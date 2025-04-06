use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct CreateProblem {
    pub problem_name: String,
    pub time_limit: i32,
    pub memory_limit: i32,
}

#[derive(Serialize)]
pub struct Problem {
    pub problem_id: i32,
    pub problem_name: String,
    pub problem_statement_url: String,
    pub problem_test_cases_url: String,
    pub problem_outputs_url: String,
    pub problem_memory_mb_limit: i32,
    pub problem_time_ms_limit: i32,
}


