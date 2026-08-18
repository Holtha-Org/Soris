use crate::stdlib::StdlibFunc;

pub struct StringFunc;

impl StdlibFunc for StringFunc {
    fn nombre(&self) -> &str {
        "texto"
    }

    fn comandos_soportados(&self) -> Vec<&str> {
        vec![
            "longitud", "mayuscula", "minuscula", "concatenar", "reemplazar",
            "subcadena", "dividir_texto", "unir_texto", "contener",
            "iniciar_con", "terminar_con", "posicion", "ultimo_indice",
            "eliminar", "insertar", "trim"
        ]
    }

    fn generar_codigo(&self, comando: &str, argumentos: &[String]) -> String {
        match comando {
            "longitud" => {
                if argumentos.is_empty() {
                    "\"\".len()".to_string()
                } else {
                    format!("({}).len()", argumentos[0])
                }
            }
            "mayuscula" => {
                if argumentos.is_empty() {
                    "\"\".to_uppercase()".to_string()
                } else {
                    format!("({}).to_uppercase()", argumentos[0])
                }
            }
            "minuscula" => {
                if argumentos.is_empty() {
                    "\"\".to_lowercase()".to_string()
                } else {
                    format!("({}).to_lowercase()", argumentos[0])
                }
            }
            "mayusculas" => {
                if argumentos.is_empty() {
                    "\"\".to_uppercase()".to_string()
                } else {
                    format!("({}).to_uppercase()", argumentos[0])
                }
            }
            "minusculas" => {
                if argumentos.is_empty() {
                    "\"\".to_lowercase()".to_string()
                } else {
                    format!("({}).to_lowercase()", argumentos[0])
                }
            }
            "concatenar" => {
                if argumentos.len() >= 2 {
                    format!("format!(\"{{}}{{}}\", {}, {})", argumentos[0], argumentos[1])
                } else {
                    "String::new()".to_string()
                }
            }
            "reemplazar" => {
                if argumentos.len() >= 3 {
                    format!("({}).replace({}, {})", argumentos[0], argumentos[1], argumentos[2])
                } else {
                    "String::new()".to_string()
                }
            }
            "subcadena" => {
                if argumentos.len() >= 3 {
                    format!("({}).chars().skip({}).take({}).collect::<String>()", 
                            argumentos[0], argumentos[1], argumentos[2])
                } else {
                    "String::new()".to_string()
                }
            }
            "dividir_texto" => {
                if argumentos.len() >= 2 {
                    format!("({}).split({})", argumentos[0], argumentos[1])
                } else {
                    "vec![]".to_string()
                }
            }
            "unir_texto" => {
                if argumentos.len() >= 2 {
                    format!("({}).join({})", argumentos[0], argumentos[1])
                } else {
                    "String::new()".to_string()
                }
            }
            "contener" => {
                if argumentos.len() >= 2 {
                    format!("({}).contains({})", argumentos[0], argumentos[1])
                } else {
                    "false".to_string()
                }
            }
            "iniciar_con" => {
                if argumentos.len() >= 2 {
                    format!("({}).starts_with({})", argumentos[0], argumentos[1])
                } else {
                    "false".to_string()
                }
            }
            "terminar_con" => {
                if argumentos.len() >= 2 {
                    format!("({}).ends_with({})", argumentos[0], argumentos[1])
                } else {
                    "false".to_string()
                }
            }
            "posicion" => {
                if argumentos.len() >= 2 {
                    format!("({}).find({}).unwrap_or(0)", argumentos[0], argumentos[1])
                } else {
                    "0".to_string()
                }
            }
            "ultimo_indice" => {
                if argumentos.len() >= 2 {
                    format!("({}).rfind({}).unwrap_or(0)", argumentos[0], argumentos[1])
                } else {
                    "0".to_string()
                }
            }
            "eliminar" => {
                if argumentos.len() >= 2 {
                    format!("{{ let mut s = {}.to_string(); s.remove({} as usize); s }}", 
                            argumentos[0], argumentos[1])
                } else {
                    "String::new()".to_string()
                }
            }
            "insertar" => {
                if argumentos.len() >= 3 {
                    format!("{{ let mut s = {}.to_string(); s.insert({} as usize, {}); s }}", 
                            argumentos[0], argumentos[1], argumentos[2])
                } else {
                    "String::new()".to_string()
                }
            }
            "trim" => {
                if argumentos.is_empty() {
                    "\"\"".to_string()
                } else {
                    format!("({}).trim()", argumentos[0])
                }
            }
            _ => String::new(),
        }
    }
}