use crate::stdlib::StdlibFunc;

pub struct Time;

impl StdlibFunc for Time {
    fn nombre(&self) -> &str {
        "tiempo"
    }

    fn comandos_soportados(&self) -> Vec<&str> {
        vec!["dormir", "ahora", "medir"]
    }

    fn generar_codigo(&self, comando: &str, argumentos: &[String]) -> String {
        match comando {
            "dormir" => {
                if argumentos.is_empty() {
                    "std::thread::sleep(std::time::Duration::from_secs(0));".to_string()
                } else {
                    format!(
                        "std::thread::sleep(std::time::Duration::from_secs_f64({}));",
                        argumentos[0]
                    )
                }
            }
            "ahora" => {
                "std::time::SystemTime::now()".to_string()
            }
            "medir" => {
                if argumentos.is_empty() {
                    "std::time::Instant::now()".to_string()
                } else {
                    format!("std::time::Instant::now()")
                }
            }
            _ => String::new(),
        }
    }
}