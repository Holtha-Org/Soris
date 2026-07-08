use crate::ast::types::Tipo;
use crate::utils::span::Span;

#[derive(Debug, Clone)]
pub struct Simbolo {
    pub nombre: String,
    pub tipo: Tipo,
    pub mutable: bool,
    pub span: Span,
}

impl Simbolo {
    pub fn new(nombre: &str, tipo: Tipo, mutable: bool, span: Span) -> Self {
        Self {
            nombre: nombre.to_string(),
            tipo,
            mutable,
            span,
        }
    }
}