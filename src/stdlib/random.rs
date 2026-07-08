use crate::stdlib::StdlibFunc;

pub struct Random;

impl StdlibFunc for Random {
    fn nombre(&self) -> &str {
        "aleatorio"
    }

    fn comandos_soportados(&self) -> Vec<&str> {
        vec!["generar", "generar_entre"]
    }

    fn generar_codigo(&self, comando: &str, argumentos: &[String]) -> String {
        match comando {
            "generar" => {
                "rand::random::<f64>()".to_string()
            }
            "generar_entre" => {
                if argumentos.len() >= 2 {
                    format!("rand::thread_rng().gen_range({}..{})", argumentos[0], argumentos[1])
                } else {
                    "rand::random::<f64>()".to_string()
                }
            }
            _ => String::new(),
        }
    }
}