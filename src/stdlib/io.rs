use crate::stdlib::StdlibFunc;

pub struct Io;

impl StdlibFunc for Io {
    fn nombre(&self) -> &str {
        "io"
    }

    fn comandos_soportados(&self) -> Vec<&str> {
        vec!["leer_archivo", "escribir_archivo"]
    }

    fn generar_codigo(&self, comando: &str, argumentos: &[String]) -> String {
        match comando {
            "leer_archivo" => {
                if argumentos.is_empty() {
                    "std::fs::read_to_string(\"\")?".to_string()
                } else {
                    format!("std::fs::read_to_string({})?", argumentos[0])
                }
            }
            "escribir_archivo" => {
                if argumentos.len() >= 2 {
                    format!("std::fs::write({}, {})?", argumentos[0], argumentos[1])
                } else {
                    "std::fs::write(\"\", \"\")?".to_string()
                }
            }
            _ => String::new(),
        }
    }
}