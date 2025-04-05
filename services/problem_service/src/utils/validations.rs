use std::fs;
use std::path::Path;

pub fn validate_limits(memory: i32, time: i32) -> bool {
    memory > 0 && time > 0
}

pub fn validate_test_cases_structure(problem_dir: &Path) -> bool {
    let inputs_path = problem_dir.join("testCases");
    let outputs_path = problem_dir.join("outputs");

    if !inputs_path.exists() || !outputs_path.exists() {
        println!("Las carpetas testCases o outputs no existen");
        return false;
    }

    let Ok(input_files) = fs::read_dir(&inputs_path) else {
        println!("No se pudieron leer los archivos en testCases");
        return false;
    };

    for entry in input_files.flatten() {
        let file_name = entry.file_name().into_string().unwrap_or_default();
        let base_name = file_name.split('.').next().unwrap_or("");
        let output_file = outputs_path.join(format!("{base_name}.out"));
        if !output_file.exists() {
            println!("No se encontró el archivo de salida para {}", base_name);
            return false;
        }
    }

    println!("Estructura de test cases válida");
    true
}
