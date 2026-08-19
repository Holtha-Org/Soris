//! Módulo de comandos utilitarios para Gotan

use std::process::Command;
use colored::Colorize;

/// Ejecuta un comando externo y devuelve su salida
pub fn ejecutar_comando(comando: &str, argumentos: &[&str]) -> Result<String, String> {
    let output = Command::new(comando)
        .args(argumentos)
        .output()
        .map_err(|e| format!("Error al ejecutar '{}': {}", comando, e))?;
    
    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    } else {
        Err(format!(
            "Comando '{}' falló: {}",
            comando,
            String::from_utf8_lossy(&output.stderr)
        ))
    }
}

/// Verifica si un comando está disponible en el sistema
pub fn comando_disponible(comando: &str) -> bool {
    Command::new(comando)
        .arg("--version")
        .output()
        .is_ok()
}

/// Formatea un mensaje de éxito
pub fn mensaje_exito(mensaje: &str) -> String {
    format!("{} {}", "✅".bright_green(), mensaje.bright_green())
}

/// Formatea un mensaje de error
pub fn mensaje_error(mensaje: &str) -> String {
    format!("{} {}", "❌".bright_red(), mensaje.bright_red())
}

/// Formatea un mensaje de advertencia
pub fn mensaje_advertencia(mensaje: &str) -> String {
    format!("{} {}", "⚠️".bright_yellow(), mensaje.bright_yellow())
}

/// Formatea un mensaje de información
pub fn mensaje_info(mensaje: &str) -> String {
    format!("{} {}", "ℹ️".bright_cyan(), mensaje.bright_cyan())
}
