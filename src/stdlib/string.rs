use crate::stdlib::StdlibFunc;

pub struct StringFunc;

impl StdlibFunc for StringFunc {
    fn nombre(&self) -> &str {
        "texto"
    }

    fn comandos_soportados(&self) -> Vec<&str> {
        vec!["longitud", "mayuscula", "minuscula", "concatenar", "reemplazar"]
    }

    fn generar_codigo(&self, comando: &str, argumentos: &[String]) -> String {
        match comando {
            "longitud" => {
                if argumentos.is_empty() {
                    "\"\".len()".to_string()
                } else {
                    format!("{}.len()", argumentos[0])
                }
            }
            "mayuscula" => {
                if argumentos.is_empty() {
                    "\"\".to_uppercase()".to_string()
                } else {
                    format!("{}.to_uppercase()", argumentos[0])
                }
            }
            "minuscula" => {
                if argumentos.is_empty() {
                    "\"\".to_lowercase()".to_string()
                } else {
                    format!("{}.to_lowercase()", argumentos[0])
                }
            }
            "concatenar" => {
                let args: Vec<String> = argumentos.iter()
                    .map(|a| format!("&{}", a))
                    .collect();
                format!("vec![{}].concat()", args.join(", "))
            }
            "reemplazar" => {
                if argumentos.len() >= 3 {
                    format!("{}.replace({}, {})", argumentos[0], argumentos[1], argumentos[2])
                } else {
                    "\"\".to_string()".to_string()
                }
            }
            _ => String::new(),
        }
    }
}