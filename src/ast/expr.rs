use crate::utils::span::Span;
use crate::ast::types::Tipo;
use crate::ast::stmt::Stmt;

#[derive(Debug, Clone)]
pub enum Expr {
    LiteralNumero(f64, Span),
    LiteralTexto(String, Span),
    LiteralCaracter(char, Span),
    LiteralBool(bool, Span),
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
    // Nuevas expresiones tipo Rust
    OpcionAlguna(Option<Box<Expr>>, Span),      // alguna(expr) o ninguna
    OpcionNinguna(Span),                         // ninguna
    ResultadoOk(Box<Expr>, Span),               // ok(valor)
    ResultadoErr(Box<Expr>, Span),              // err(valor)
    Tupla(Vec<Expr>, Span),                     // (a, b, c)
    Array(Vec<Expr>, Span),                     // [a, b, c]
    Vector(Vec<Expr>, Span),                    // vec![a, b, c]
    Indexacion {
        colección: Box<Expr>,
        indice: Box<Expr>,
        span: Span,
    },
    Rango {
        inicio: Option<Box<Expr>>,
        fin: Option<Box<Expr>>,
        inclusivo: bool,
        span: Span,
    },
    Cierre {
        parametros: Vec<(String, Option<Tipo>)>,
        cuerpo: Box<Expr>,
        span: Span,
    },
    Match {
        expresion: Box<Expr>,
        brazos: Vec<BrazoMatch>,
        span: Span,
    },
    IfExpresion {
        condicion: Box<Expr>,
        entonces: Box<Expr>,
        sino: Option<Box<Expr>>,
        span: Span,
    },
    Bloque(Vec<Stmt>, Span),
    Desestructurar {
        patron: Box<Patron>,
        valor: Box<Expr>,
        span: Span,
    },
}

#[derive(Debug, Clone)]
pub struct BrazoMatch {
    pub patron: Box<Patron>,
    pub guardia: Option<Expr>,
    pub cuerpo: Expr,
}

#[derive(Debug, Clone)]
pub enum Patron {
    GuionBajo,                      // _
    Literal(Box<Expr>),             // 5, "hola", true
    Identificador(String),          // x
    Tupla(Vec<Box<Patron>>),        // (a, b)
    Array(Vec<Box<Patron>>),        // [a, b, ..]
    Estructura {
        nombre: String,
        campos: Vec<(String, Box<Patron>)>,
        resto: bool,                 // ..
    },
    EnumVariante {
        nombre: String,
        variante: String,
        campos: Vec<Box<Patron>>,
    },
    Rango {
        inicio: Expr,
        fin: Expr,
    },
    Referencia(Box<Patron>),
    Mutabilidad(Box<Patron>),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BinOp {
    Suma,
    Resta,
    Multiplicacion,
    Division,
    Modulo,
    Igual,
    NoIgual,
    Menor,
    Mayor,
    MenorIgual,
    MayorIgual,
    YLogico,
    OLogico,
    BitAnd,
    BitOr,
    BitXor,
    DesplazaIzq,
    DesplazaDer,
    SumaAsig,
    RestaAsig,
    MultAsig,
    DivAsig,
}

impl BinOp {
    pub fn nombre_rust(&self) -> &str {
        match self {
            BinOp::Suma => "+",
            BinOp::Resta => "-",
            BinOp::Multiplicacion => "*",
            BinOp::Division => "/",
            BinOp::Modulo => "%",
            BinOp::Igual => "==",
            BinOp::NoIgual => "!=",
            BinOp::Menor => "<",
            BinOp::Mayor => ">",
            BinOp::MenorIgual => "<=",
            BinOp::MayorIgual => ">=",
            BinOp::YLogico => "&&",
            BinOp::OLogico => "||",
            BinOp::BitAnd => "&",
            BinOp::BitOr => "|",
            BinOp::BitXor => "^",
            BinOp::DesplazaIzq => "<<",
            BinOp::DesplazaDer => ">>",
            BinOp::SumaAsig => "+=",
            BinOp::RestaAsig => "-=",
            BinOp::MultAsig => "*=",
            BinOp::DivAsig => "/=",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnaryOp {
    Negacion,
    Negativo,
    Dereferencia,
    NotBit,
}

impl Expr {
    pub fn span(&self) -> Span {
        match self {
            Expr::LiteralNumero(_, span) => *span,
            Expr::LiteralTexto(_, span) => *span,
            Expr::LiteralCaracter(_, span) => *span,
            Expr::LiteralBool(_, span) => *span,
            Expr::Identificador(_, span) => *span,
            Expr::Binaria { span, .. } => *span,
            Expr::Unaria { span, .. } => *span,
            Expr::Llamada { span, .. } => *span,
            Expr::AccesoMiembro { span, .. } => *span,
            Expr::OpcionAlguna(_, span) => *span,
            Expr::OpcionNinguna(span) => *span,
            Expr::ResultadoOk(_, span) => *span,
            Expr::ResultadoErr(_, span) => *span,
            Expr::Tupla(_, span) => *span,
            Expr::Array(_, span) => *span,
            Expr::Vector(_, span) => *span,
            Expr::Indexacion { span, .. } => *span,
            Expr::Rango { span, .. } => *span,
            Expr::Cierre { span, .. } => *span,
            Expr::Match { span, .. } => *span,
            Expr::IfExpresion { span, .. } => *span,
            Expr::Bloque(_, span) => *span,
            Expr::Desestructurar { span, .. } => *span,
        }
    }
}