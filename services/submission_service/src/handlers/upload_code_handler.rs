use std::path::Path;
use axum::{extract:: Multipart, response::{IntoResponse, Response}};
use mime_guess::from_path;
use serde_json::json;
use tokio::{fs::File, io::AsyncWriteExt};
use reqwest::StatusCode;
use uuid::Uuid;
use std::collections::HashSet;


pub async fn upload(mut multipart: Multipart) -> Result<Response, Response> {
    let mut user_id:i32 = 0;
    let mut problem_id:i32 = 0;
    let mut veredict:String = String::new();
    let mut upload_path:String = String::new();
    let mut lang:String = String::new();
    let mut source_code:String = String::new();

    let save_path = "/app/submissions";

    let mut field_check = HashSet::new();
    let avaliable_langs: [&str;4] = ["cpp", "python", "c", "java"];

    let judge_url = std::env::var("JUDGE_URL").map_err(|_| {
        (StatusCode::INTERNAL_SERVER_ERROR, "JUDGE_URL not set in environment").into_response()
    })?;

    while let Some(field) = multipart
        .next_field()
        .await
        .map_err(|_| {(StatusCode::PAYLOAD_TOO_LARGE, "Files are bigger than 2MB").into_response()})?
    {
        if let Some("user_id") = field.name() {
            if field_check.contains("user_id") {
                return Err( (StatusCode::BAD_REQUEST, "Just one user id!").into_response() );
            }
            let content = field.text().await.map_err(|_| {
                (StatusCode::BAD_REQUEST, "Invalid user id").into_response()
            })?;
            user_id = content.parse().map_err(|_| {
                (StatusCode::BAD_REQUEST, "Invalid user id").into_response()
            })?;

            field_check.insert("user_id");

            continue;
        }

        if let Some("problem_id") = field.name() {
            if field_check.contains("problem_id") {
                return Err( (StatusCode::BAD_REQUEST, "Just one problem id!").into_response() );
            }
            let content = field.text().await.map_err(|_| {
                (StatusCode::BAD_REQUEST, "Invalid problem id").into_response()
            })?;
            problem_id = content.parse().map_err(|_| {
                (StatusCode::BAD_REQUEST, "Invalid problem id").into_response()
            })?;

            field_check.insert("problem_id");
            continue;
        }

        if let Some("lang") = field.name() {
            if field_check.contains("lang") {
                return Err( (StatusCode::BAD_REQUEST, "Just one language!").into_response() );
            }
            lang = field.text().await.map_err(|_| {
                (StatusCode::BAD_REQUEST, "Invalid language").into_response()
            })?;

            if !avaliable_langs.contains(&lang.as_str()) || lang.is_empty(){
                return Err((StatusCode::BAD_REQUEST, "Invalid language").into_response())
            }

            field_check.insert("lang");
            continue;
        }


        if let Some("source") = field.name(){

            if field_check.contains("source") {
               return Err( (StatusCode::BAD_REQUEST, "Just one source code file").into_response() );
            }
            field_check.insert("source");
            if let Some(file_name) = field.file_name(){

                let extension = from_path(file_name)
                    .first_raw()
                    .and_then(|mime| mime.split('/').nth(1))
                    .unwrap_or("");


                let file_name =  if extension.is_empty(){
                    Uuid::new_v4().to_string()
                }else{
                    format!("{}.{}", Uuid::new_v4(), extension)
                };

                upload_path = format!("{}/{}",save_path,file_name);

                if !Path::new(&save_path).exists(){
                    tokio::fs::create_dir(&save_path).await.map_err(|_| {
                        (StatusCode::INTERNAL_SERVER_ERROR, "Error creating uploads directory!").into_response()
                    })?;
                }
                
                let bytes = field.bytes().await.map_err(|_| {
                    (StatusCode::BAD_REQUEST, "Error reading source code").into_response()
                })?;

                source_code = String::from_utf8(bytes.to_vec()).map_err(|_| {
                    (StatusCode::BAD_REQUEST, "Source code must be in UTF-8").into_response()
                })?;
  
            }
            
        }

    }

    for required in ["user_id", "lang", "source", "problem_id"] {
        if !field_check.contains(required) {
            return Err((
                StatusCode::BAD_REQUEST,
                format!("Missing field: {}", required),
            ).into_response());
        }
    }


    // Compilar y ejecutar
    // Guardar en la DB


    Ok(Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "application/json")
        .body(json!(
            {
                "status": true,
                "user_id" : user_id.to_string(),
                "problem_id" : problem_id.to_string(),
                "source_code_url" : upload_path.clone(),
                "veredict": veredict.clone(),
                "lang": lang.clone(),
                "source_code": source_code.clone()
            }
        ).to_string().into())
        .unwrap())

}

// async fn compile() -> String {

// }

// asyng fn save_submission() -> {

// }




// let mut saved_file = File::create(&upload_path).await.map_err(|_| {
//     (StatusCode::INTERNAL_SERVER_ERROR, "Error saving the source code!").into_response()
// })?;

// saved_file.write(&bytes).await.map_err(|_|{
//     (StatusCode::INTERNAL_SERVER_ERROR, "Error writing the source code file!").into_response()
// })?;

// println!("File {} saved!", upload_path);