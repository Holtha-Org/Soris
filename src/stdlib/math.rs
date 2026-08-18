use crate::stdlib::StdlibFunc;

pub struct Math;

impl StdlibFunc for Math {
    fn nombre(&self) -> &str {
        "mat"
    }

    fn comandos_soportados(&self) -> Vec<&str> {
        vec![
            "raiz", "potencia", "absoluto", "redondear", "aleatorio",
            "truncar", "log", "seno", "coseno", "tangente",
            "arcoseno", "arcocoseno", "arcotangente", "maximo", "minimo",
            "clamp", "sumar", "restar", "multiplicar", "dividir"
        ]
    }

    fn generar_codigo(&self, comando: &str, argumentos: &[String]) -> String {
        match comando {
            "raiz" => {
                if argumentos.is_empty() {
                    "0.0_f64.sqrt()".to_string()
                } else {
                    format!("({}).sqrt()", argumentos[0])
                }
            }
            "potencia" => {
                if argumentos.len() >= 2 {
                    format!("({}).powf({})", argumentos[0], argumentos[1])
                } else {
                    "0.0_f64.powi(0)".to_string()
                }
            }
            "absoluto" | "abs" => {
                if argumentos.is_empty() {
                    "0.0_f64.abs()".to_string()
                } else {
                    format!("({}).abs()", argumentos[0])
                }
            }
            "redondear" => {
                if argumentos.is_empty() {
                    "0.0_f64.round()".to_string()
                } else {
                    format!("({}).round()", argumentos[0])
                }
            }
            "truncar" => {
                if argumentos.is_empty() {
                    "0.0_f64.trunc()".to_string()
                } else {
                    format!("({}).trunc()", argumentos[0])
                }
            }
            "log" => {
                if argumentos.is_empty() {
                    "0.0_f64.ln()".to_string()
                } else {
                    format!("({}).ln()", argumentos[0])
                }
            }
            "seno" => {
                if argumentos.is_empty() {
                    "0.0_f64.sin()".to_string()
                } else {
                    format!("({}).sin()", argumentos[0])
                }
            }
            "coseno" => {
                if argumentos.is_empty() {
                    "0.0_f64.cos()".to_string()
                } else {
                    format!("({}).cos()", argumentos[0])
                }
            }
            "tangente" => {
                if argumentos.is_empty() {
                    "0.0_f64.tan()".to_string()
                } else {
                    format!("({}).tan()", argumentos[0])
                }
            }
            "arcoseno" => {
                if argumentos.is_empty() {
                    "0.0_f64.asin()".to_string()
                } else {
                    format!("({}).asin()", argumentos[0])
                }
            }
            "arcocoseno" => {
                if argumentos.is_empty() {
                    "0.0_f64.acos()".to_string()
                } else {
                    format!("({}).acos()", argumentos[0])
                }
            }
            "arcotangente" => {
                if argumentos.is_empty() {
                    "0.0_f64.atan()".to_string()
                } else {
                    format!("({}).atan()", argumentos[0])
                }
            }
            "maximo" => {
                if argumentos.len() >= 2 {
                    format!("({}).max({})", argumentos[0], argumentos[1])
                } else {
                    "0.0_f64".to_string()
                }
            }
            "minimo" => {
                if argumentos.len() >= 2 {
                    format!("({}).min({})", argumentos[0], argumentos[1])
                } else {
                    "0.0_f64".to_string()
                }
            }
            "clamp" => {
                if argumentos.len() >= 3 {
                    format!("({}).max({}).min({})", argumentos[0], argumentos[1], argumentos[2])
                } else {
                    "0.0_f64".to_string()
                }
            }
            "sumar" => {
                if argumentos.len() >= 2 {
                    format!("({}) + ({})", argumentos[0], argumentos[1])
                } else {
                    "0.0".to_string()
                }
            }
            "restar" => {
                if argumentos.len() >= 2 {
                    format!("({}) - ({})", argumentos[0], argumentos[1])
                } else {
                    "0.0".to_string()
                }
            }
            "multiplicar" => {
                if argumentos.len() >= 2 {
                    format!("({}) * ({})", argumentos[0], argumentos[1])
                } else {
                    "0.0".to_string()
                }
            }
            "dividir" => {
                if argumentos.len() >= 2 {
                    format!("({}) / ({})", argumentos[0], argumentos[1])
                } else {
                    "0.0".to_string()
                }
            }
            _ => String::new(),
        }
    }
}