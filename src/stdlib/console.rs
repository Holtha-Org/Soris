use crate::stdlib::StdlibFunc;

pub struct Console;

impl StdlibFunc for Console {
    fn nombre(&self) -> &str {
        "consola"
    }

    fn comandos_soportados(&self) -> Vec<&str> {
        vec!["limpiar", "escribir"]
    }

    fn generar_codigo(&self, comando: &str, argumentos: &[String]) -> String {
        match comando {
            "limpiar" => r#"print!("\x1B[2J\x1B[1;1H"); std::io::stdout().flush().unwrap();"#.to_string(),
            "escribir" => {
                if argumentos.is_empty() {
                    r#"println!("");"#.to_string()
                } else {
                    format!(r#"println!("{{}}", {});"#, argumentos[0])
                }
            }
            _ => String::new(),
        }
    }
}
