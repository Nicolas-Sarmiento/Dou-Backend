use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct CompileStruct {
    pub lang:String,
    pub version: String,
    pub source_code: String,
    pub memory_limit: i32,
    pub time_limit: i32,
}

#[derive(Deserialize, Serialize)]
pub struct SourceFile {
    pub content: String,
}

#[derive(Deserialize, Serialize)]
pub struct CompileRequest {
    pub language:String,
    pub version:String,
    pub files: Vec<SourceFile>,
    pub stdin: String,
}

#[derive(Debug,Deserialize, Serialize)]
pub struct ResultObject {
    pub stdout:String,
    pub code:Option<i32>,
    pub signal:Option<String>,
    pub wall_time:Option<i32>,
    pub memory:Option<i32>,
}

#[derive(Debug,Serialize,Deserialize)]
pub struct CompileResponse {
    pub language: String,
    pub version: String,
    pub run: ResultObject,
    pub compile: Option<ResultObject>,
}

