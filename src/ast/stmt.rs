use crate::ast::expr::Expr;
use crate::utils::span::Span;

#[derive(Debug, Clone)]
pub enum Stmt {
    Firma {
        autor: String,
        span: Span,
    },
    Declaracion {
        nombre: String,
        valor_inicial: Option<Expr>,
        span: Span,
    },
    Asignacion {
        nombre: String,
        valor: Expr,
        span: Span,
    },
    Impresion {
        expresion: Expr,
        span: Span,
    },
    Si {
        condicion: Expr,
        cuerpo: Vec<Stmt>,
        sino: Option<Vec<Stmt>>,
        span: Span,
    },
    Mientras {
        condicion: Expr,
        cuerpo: Vec<Stmt>,
        span: Span,
    },
    Expresion(Expr, Span),
    LlamadaStdlib {
        comando: String,
        argumentos: Vec<Expr>,
        span: Span,
    },
}

impl Stmt {
    pub fn span(&self) -> Span {
        match self {
            Stmt::Firma { span, .. } => *span,
            Stmt::Declaracion { span, .. } => *span,
            Stmt::Asignacion { span, .. } => *span,
            Stmt::Impresion { span, .. } => *span,
            Stmt::Si { span, .. } => *span,
            Stmt::Mientras { span, .. } => *span,
            Stmt::Expresion(_, span) => *span,
            Stmt::LlamadaStdlib { span, .. } => *span,
        }
    }
}