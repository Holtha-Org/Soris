use crate::utils::span::Span;

#[derive(Debug, Clone)]
pub enum Expr {
    LiteralNumero(f64, Span),
    LiteralTexto(String, Span),
    Identificador(String, Span),
    Binaria {
        izquierda: Box<Expr>,
        operador: BinOp,
        derecha: Box<Expr>,
        span: Span,
    },
    Unaria {
        operador: UnaryOp,
        operando: Box<Expr>,
        span: Span,
    },
    Llamada {
        nombre: String,
        argumentos: Vec<Expr>,
        span: Span,
    },
    AccesoMiembro {
        objeto: Box<Expr>,
        miembro: String,
        span: Span,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BinOp {
    Suma,
    Resta,
    Multiplicacion,
    Division,
    Igual,
    NoIgual,
    Menor,
    Mayor,
    MenorIgual,
    MayorIgual,
    YLogico,
    OLogico,
}

impl BinOp {
    pub fn nombre_rust(&self) -> &str {
        match self {
            BinOp::Suma => "+",
            BinOp::Resta => "-",
            BinOp::Multiplicacion => "*",
            BinOp::Division => "/",
            BinOp::Igual => "==",
            BinOp::NoIgual => "!=",
            BinOp::Menor => "<",
            BinOp::Mayor => ">",
            BinOp::MenorIgual => "<=",
            BinOp::MayorIgual => ">=",
            BinOp::YLogico => "&&",
            BinOp::OLogico => "||",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnaryOp {
    Negacion,
    Negativo,
}

impl Expr {
    pub fn span(&self) -> Span {
        match self {
            Expr::LiteralNumero(_, span) => *span,
            Expr::LiteralTexto(_, span) => *span,
            Expr::Identificador(_, span) => *span,
            Expr::Binaria { span, .. } => *span,
            Expr::Unaria { span, .. } => *span,
            Expr::Llamada { span, .. } => *span,
            Expr::AccesoMiembro { span, .. } => *span,
        }
    }
}