use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct AttachmentResponse {
    pub file_name: String,
    pub base64_content: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct MaterialResponse {
    pub material_id: i32,
    pub description_path: String,
    pub attachments: Vec<AttachmentResponse>,
}

#[derive(Serialize, Deserialize)]
pub struct MaterialResponseIdOnly {
    pub material_id: i32,
    pub description_path: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String, 
    pub name: String,
    pub email : String,
    pub role: String,
    pub exp: usize,  
}

