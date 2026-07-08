use crate::stdlib::StdlibFunc;

pub struct Math;

impl StdlibFunc for Math {
    fn nombre(&self) -> &str {
        "mat"
    }

    fn comandos_soportados(&self) -> Vec<&str> {
        vec!["raiz", "potencia", "absoluto", "redondear", "aleatorio"]
    }

    fn generar_codigo(&self, comando: &str, argumentos: &[String]) -> String {
        match comando {
            "raiz" => {
                if argumentos.is_empty() {
                    "0.0_f64.sqrt()".to_string()
                } else {
                    format!("{}f64.sqrt()", argumentos[0])
                }
            }
            "potencia" => {
                if argumentos.len() >= 2 {
                    format!("{}.powf({})", argumentos[0], argumentos[1])
                } else {
                    "0.0_f64.powi(0)".to_string()
                }
            }
            "absoluto" => {
                if argumentos.is_empty() {
                    "0.0_f64.abs()".to_string()
                } else {
                    format!("{}.abs()", argumentos[0])
                }
            }
            "redondear" => {
                if argumentos.is_empty() {
                    "0.0_f64.round()".to_string()
                } else {
                    format!("{}.round()", argumentos[0])
                }
            }
            _ => String::new(),
        }
    }
}