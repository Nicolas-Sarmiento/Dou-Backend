use std::path::Path;
use axum::{extract:: {Extension, Multipart}, response::IntoResponse, Json};
use sqlx::{PgPool, Row};
use mime_guess::from_path;
use serde_json::json;
use tokio::{fs::{self, File}, io::AsyncWriteExt};
use reqwest::StatusCode;
use reqwest::Client;
use uuid::Uuid;
use std::collections::{HashMap, HashSet};
use crate::models::compile_models::{CompileStruct, CompileRequest, CompileResponse, SourceFile};


pub async fn upload(Extension(pool): Extension<PgPool>, mut multipart: Multipart) -> Result<impl IntoResponse, impl IntoResponse> {
    let mut user_id:i32 = 0;
    let mut problem_id:i32 = 0;
    let mut upload_path:String = String::new();
    let mut lang:String = String::new();
    let mut source_code:String = String::new();
    let save_path = "/app/submissions";

    let mut field_check = HashSet::new();
    let mut avaliable_langs: HashMap<String, String> = HashMap::new();
    avaliable_langs.insert("cpp".to_string(), "10.2.0".to_string());
    avaliable_langs.insert("c".to_string(), "10.2.0".to_string());
    avaliable_langs.insert("python".to_string(), "3.11.0".to_string());
    avaliable_langs.insert("java".to_string(), "15.0.2".to_string());

    let judge_ip = std::env::var("JUDGE_IP").map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": "Judge ip not found!" }))
        ).into_response()
    })?;

    let judge_url:String = format!("http://{}/api/v2/execute", judge_ip);

    while let Some(field) = multipart
        .next_field()
        .await
        .map_err(|_| {
            (
                StatusCode::PAYLOAD_TOO_LARGE,
                Json(json!({ "error": "Max 2MB file size" }))
            ).into_response()
        })?
    {
        if let Some("user_id") = field.name() {
            if field_check.contains("user_id") {
                return Err( (
                    StatusCode::BAD_REQUEST,
                    Json(json!({ "error": "Multiple user id's" }))
                ).into_response() );
            }
            let content = field.text().await.map_err(|_| {
                (
                    StatusCode::BAD_REQUEST,
                    Json(json!({ "error": "User id not valid" }))
                ).into_response()
            })?;
            user_id = content.parse().map_err(|_| {
                (
                    StatusCode::BAD_REQUEST,
                    Json(json!({ "error": "User id not valid" }))
                ).into_response()
            })?;

            field_check.insert("user_id");

            continue;
        }

        if let Some("problem_id") = field.name() {
            if field_check.contains("problem_id") {
                return Err( (
                    StatusCode::BAD_REQUEST,
                    Json(json!({ "error": "Multiple Problem id " }))
                ).into_response() );
            }
            let content = field.text().await.map_err(|_| {
                (
                    StatusCode::BAD_REQUEST,
                    Json(json!({ "error": "Problem id not valid" }))
                ).into_response()
            })?;
            problem_id = content.parse().map_err(|_| {
                (
                    StatusCode::BAD_REQUEST,
                    Json(json!({ "error": "Problem id not valid" }))
                ).into_response()
            })?;

            field_check.insert("problem_id");
            continue;
        }

        if let Some("lang") = field.name() {
            if field_check.contains("lang") {
                return Err( (
                    StatusCode::BAD_REQUEST,
                    Json(json!({ "error": "Multiple languages" }))
                ).into_response() );
            }
            lang = field.text().await.map_err(|_| {
                (
                    StatusCode::BAD_REQUEST,
                    Json(json!({ "error": "language not valid" }))
                ).into_response()
            })?;
        
            if !avaliable_langs.contains_key(&lang) || lang.is_empty(){
                return Err((
                    StatusCode::BAD_REQUEST,
                    Json(json!({ "error": "language not valid" }))
                ).into_response())
            }

            field_check.insert("lang");
            continue;
        }


        if let Some("source") = field.name(){

            if field_check.contains("source") {
               return Err( (
                StatusCode::BAD_REQUEST,
                Json(json!({ "error": "multiple source files" }))
            ).into_response() );
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
                    tokio::fs::create_dir(&save_path).await.map_err(|e| {
                        (
                            StatusCode::INTERNAL_SERVER_ERROR,
                            Json(json!({ "error": format!("Creating Server: {}", e) }))
                        ).into_response()
                    })?;
                }
                
                let bytes = field.bytes().await.map_err(|e| {
                    (
                        StatusCode::BAD_REQUEST,
                        Json(json!({ "error": format!("Parsing to source file: {}", e) }))
                    ).into_response()
                })?;

                source_code = String::from_utf8(bytes.to_vec()).map_err(|e| {
                    (
                        StatusCode::BAD_REQUEST,
                        Json(json!({ "error": format!("Parsing to UTF-8 source file: {}", e) }))
                    ).into_response()
                })?;
  
            }
            
        }

    }

    for required in ["user_id", "lang", "source", "problem_id"] {
        if !field_check.contains(required) {
            return Err((
                StatusCode::BAD_REQUEST,
                Json(json!({ "error": format!("Missing field: {}", required) }))
            ).into_response());
        }
    }

    // Compilar y ejecutar
    let user_id_query = sqlx::query("SELECT EXISTS (SELECT 1 FROM USERS WHERE USER_ID = $1)")
        .bind(user_id)
        .fetch_one(&pool)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "error": format!("DB error: {}", e) }))
            ).into_response()
        })?;

    let exists: bool = user_id_query.get(0); 

    if !exists {
        return Err((
            StatusCode::NOT_FOUND,
            Json(json!({ "error": "User not found!" }))
        ).into_response());
    }

    let problem_query = "
        SELECT PROBLEM_TEST_CASES_URL, PROBLEM_OUTPUTS_URL, PROBLEM_MEMORY_MB_LIMIT, PROBLEM_TIME_MS_LIMIT 
        FROM PROBLEMS
        WHERE PROBLEM_ID = $1;        
    ";
    
    let problem_query_result = sqlx::query(problem_query)
        .bind(problem_id)
        .fetch_one(&pool)
        .await;
    
    let row = problem_query_result.map_err(|_| {
        (
            StatusCode::NOT_FOUND,
            Json(json!({ "error": "Problem not found!" }))
        ).into_response()
    })?;
    
    let test_cases_url: String = row.get("problem_test_cases_url");
    let outputs_url: String = row.get("problem_outputs_url");
    let time_limit: i32 = row.get("problem_time_ms_limit");
    let memory_limit: i32 = row.get("problem_memory_mb_limit");
    

    let version =  avaliable_langs.get(&lang).cloned().ok_or_else(|| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": "Language version not found!" }))
        ).into_response()
    })?;

    let source = CompileStruct {
        lang: lang.clone(),
        version: version.clone(),
        source_code: source_code.clone(),
        memory_limit: memory_limit,
        time_limit: time_limit,
    };

    let compile_result:Result<String,String> = compile(&judge_url, source, &test_cases_url, &outputs_url).await;
    
    let veredict = compile_result.map_err( |e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": format!("Cannot compile the source file! Error: {}", e) }))
        ).into_response()
    })?;

    // Guardar código fuente
    let mut saved_file = File::create(&upload_path).await.map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": "Error writing the source code file!" }))
        ).into_response()
    })?;

    saved_file.write(source_code.as_bytes()).await.map_err(|_|{
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": "Error writing the source code file!" }))
        ).into_response()
    })?;

    println!("File {} saved!", upload_path);
    // Guardar en la DB

    let save_query = "
        INSERT INTO SUBMISSIONS(
            USER_ID,
            PROBLEM_ID,
            SUBMISSION_URL,
            SUBMISSION_ANSWER_CODE
        )
        VALUES ($1, $2, $3, $4)
        RETURNING
            SUBMISSION_ID,
            USER_ID,
            PROBLEM_ID,
            SUBMISSION_URL,
            SUBMISSION_ANSWER_CODE
    ";

    let result_save_submission = sqlx::query(save_query)
        .bind(user_id)
        .bind(problem_id)
        .bind(upload_path.clone())
        .bind(veredict)
        .fetch_one(&pool)
        .await;


    match result_save_submission {
        Ok(row) => {
            let body = json!({
                "status": true,
                "submission_id":row.get::<i32, _>("submission_id"),
                "user_id": row.get::<i32, _>("user_id"),
                "problem_id": row.get::<i32, _>("problem_id"),
                "upload_path" : row.get::<String,_>("submission_url"),
                "veredict": row.get::<String, _>("submission_answer_code"),
                "lang": lang,
            });

            Ok(Json(body))
        }

        Err(e) => {
            let error_body = json!({
                "error": format!("Database error: {}", e)
            });

            Err((StatusCode::INTERNAL_SERVER_ERROR, Json(error_body)).into_response())
        }
        }

}

async fn compile(judge_url: &str, source: CompileStruct ,inputs_url: &str, outputs_url : &str) -> Result<String, String> {
    let client = Client::new();
    let inputs = match load_cases(&inputs_url).await {
        Ok(mapa) => mapa,
        Err(e) => return Err(format!("Error loading inputs: {}", e)),
    };
    let outputs = match load_cases(&outputs_url).await {
        Ok(mapa) => mapa,
        Err(e) => return Err(format!("Error loading outputs: {}", e)),
    };

    let mut request = CompileRequest {
        language: source.lang.clone(),
        version: source.version.clone(),
        files: vec![SourceFile {
            content: source.source_code.clone(),
        }],
        stdin: String::new(), 
    };
    println!("{}", source.source_code);
    print!("{}", judge_url);
    for (file, input) in &inputs {
        if let Some(expected_stdout) = outputs.get(file) {
            println!("Caso: {}", file);
            println!("Input: {}", input);
            println!("Expected: {}", expected_stdout);
            
            request.stdin = input.clone();

            let res = client
                .post(judge_url)
                .json(&request)
                .send()
                .await
                .map_err(|e| format!("error sending sorce code to compile {e}"))?;
            
           
            //let result1 = res1.text().await.map_err(|e| format!("Error reading response text: {e}"))?;
            //let result: CompileResponse = res.json().await.map_err(|e| format!("Invalid response from judge {e}"))?;
            let text = res.text().await.map_err(|e| format!("No se pudo leer el body: {e}"))?;
            println!("Respuesta cruda del juez:\n{text}");

            // Ahora convertís el texto en JSON
            let result: CompileResponse = serde_json::from_str(&text)
                .map_err(|e| format!("Respuesta inválida del juez: {e}"))?;
            
            println!("{:?}", result);


            let result_case = get_verdict(&result, &expected_stdout,source.time_limit,source.memory_limit);
            if result_case != "AC".to_string() {
                return Ok(result_case)
            }

        } else {
            println!("Archivo {} no tiene salida esperada", file);
            return Err(format!("File {} doesn't have expected output", file));
        }
    }



    Ok("AC".to_string())
}


async fn load_cases(dir: &str) -> Result<HashMap<String, String>, String> {
    let mut files = HashMap::new();
    let mut entries = fs::read_dir(dir)
        .await
        .map_err(|_| format!("Error reading {}", dir))?;

    while let Some(entry) = entries.next_entry().await.map_err(|_| "Error reading the file")? {
        let path = entry.path();
        if path.is_file() {
            if let Some(file_name) = path.file_stem().and_then(|f| f.to_str()) {
                let contenido = fs::read_to_string(&path)
                    .await
                    .map_err(|_| format!("Error reading the file {}", file_name))?;
                files.insert(file_name.to_string(), contenido);
            }
        }
    }

    Ok(files)
}


fn get_verdict(response: &CompileResponse, expected_output: &str, time_limit: i32, memory_limit: i32) -> String {
    if let Some(compile) = &response.compile{
        if let Some(run_code) = compile.code   {
            if run_code != 0{
                return "CE".into(); 
            }
        }
    }


    let run = &response.run;

    if let Some(signal) = &run.signal {
        if signal == "SIGKILL" {
            if let (Some(wall_time), Some(memory)) = (run.wall_time, run.memory) {
                if wall_time >= time_limit - 5 {
                    return "TLE".into(); 
                } else if memory >= memory_limit - 5000 {
                    return "MLE".into(); 
                } else {
                    return "RTE".into(); 
                }
            } else {
                return "RTE".into();
            }
        } else {
            return "RTE".into(); 
        }
    }
    if let Some(code) = run.code {
        if code != 0 {
            return "RTE".into();
        }
    } else {
        return "RTE".into();
    }

    if run.stdout.trim() == expected_output.trim() {
        "AC".into()
    } else {
        "WA".into()
    }
}