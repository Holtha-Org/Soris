use std::fmt;
use crate::utils::span::Span;

#[derive(Debug, Clone)]
pub struct ErrorCompilador {
    pub mensaje: String,
    pub ayuda: Option<String>,
}

impl ErrorCompilador {
    pub fn new(mensaje: &str) -> Self {
        Self {
            mensaje: mensaje.to_string(),
            ayuda: None,
        }
    }

    pub fn con_ayuda(mut self, ayuda: &str) -> Self {
        self.ayuda = Some(ayuda.to_string());
        self
    }
}

impl fmt::Display for ErrorCompilador {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Error: {}", self.mensaje)?;
        if let Some(ref ayuda) = self.ayuda {
            write!(f, "\n  Ayuda: {}", ayuda)?;
        }
        Ok(())
    }
}

impl std::error::Error for ErrorCompilador {}

// Implementar From para permitir conversión con operador ?
impl From<ErrorCompilador> for Vec<ErrorCompilador> {
    fn from(err: ErrorCompilador) -> Self {
        vec![err]
    }
}

// Alias para compatibilidad
pub type SorisError = ErrorCompilador;

// Resultado personalizado
pub type Resultado<T> = Result<T, ErrorCompilador>;