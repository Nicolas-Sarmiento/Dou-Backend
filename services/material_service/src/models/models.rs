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

