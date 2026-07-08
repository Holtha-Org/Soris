use crate::utils::span::Span;
use std::fmt;

#[derive(Debug)]
pub struct Diagnostico {
    pub nivel: NivelDiagnostico,
    pub mensaje: String,
    pub span: Span,
    pub ayuda: Option<String>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum NivelDiagnostico {
    Error,
    Advertencia,
    Nota,
}

impl Diagnostico {
    pub fn error(mensaje: &str, span: Span) -> Self {
        Self {
            nivel: NivelDiagnostico::Error,
            mensaje: mensaje.to_string(),
            span,
            ayuda: None,
        }
    }

    pub fn con_ayuda(mut self, ayuda: &str) -> Self {
        self.ayuda = Some(ayuda.to_string());
        self
    }
}

impl fmt::Display for Diagnostico {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let nivel_str = match self.nivel {
            NivelDiagnostico::Error => "ERROR",
            NivelDiagnostico::Advertencia => "ADVERTENCIA",
            NivelDiagnostico::Nota => "NOTA",
        };
        write!(f, "{} en {}: {}", nivel_str, self.span, self.mensaje)?;
        if let Some(ref ayuda) = self.ayuda {
            write!(f, "\n  Ayuda: {}", ayuda)?;
        }
        Ok(())
    }
}