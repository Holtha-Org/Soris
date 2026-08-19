use crate::ast::expr::{Expr, Patron, BrazoMatch};
use crate::utils::span::Span;

#[derive(Debug, Clone)]
pub enum Stmt {
    Firma {
        autor: String,
        span: Span,
    },
    // Declaraciones tipo Rust
    Sea {
        nombre: String,
        tipo_opcional: Option<String>,
        valor_inicial: Option<Expr>,
        mutable: bool,
        span: Span,
    },
    SeaMut {
        nombre: String,
        tipo_opcional: Option<String>,
        valor_inicial: Option<Expr>,
        span: Span,
    },
    Const {
        nombre: String,
        tipo: String,
        valor: Expr,
        span: Span,
    },
    Estatico {
        nombre: String,
        tipo: String,
        valor: Expr,
        mutable: bool,
        span: Span,
    },
    Tipo {
        nombre: String,
        alias: String,
        span: Span,
    },
    Estructura {
        nombre: String,
        campos: Vec<(String, String)>,  // (nombre, tipo)
        span: Span,
    },
    Enumeracion {
        nombre: String,
        variantes: Vec<VarianteEnum>,
        span: Span,
    },
    Implementacion {
        tipo_nombre: String,
        metodos: Vec<Funcion>,
        span: Span,
    },
    Trait {
        nombre: String,
        metodos: Vec<FirmaMetodo>,
        span: Span,
    },
    Funcion {
        firma: Funcion,
        span: Span,
    },
    // Sentencias de control
    Si {
        condicion: Expr,
        cuerpo: Vec<Stmt>,
        sino: Option<Vec<Stmt>>,
        es_expresion: bool,
        span: Span,
    },
    Coincidir {
        expresion: Expr,
        brazos: Vec<BrazoMatch>,
        span: Span,
    },
    Mientras {
        condicion: Expr,
        cuerpo: Vec<Stmt>,
        span: Span,
    },
    ParaCiclo {
        variable: String,
        rango: Expr,
        cuerpo: Vec<Stmt>,
        span: Span,
    },
    Bucle {
        etiqueta: Option<String>,
        cuerpo: Vec<Stmt>,
        span: Span,
    },
    // Control de flujo
    Retornar(Option<Expr>, Span),
    Romper(Option<String>, Span),      // break con etiqueta opcional
    Continuar(Option<String>, Span),   // continue con etiqueta opcional
    // Expresiones como statements
    Expresion(Expr, Span),
    // Llamadas a stdlib
    LlamadaStdlib {
        comando: String,
        argumentos: Vec<Expr>,
        span: Span,
    },
    // Atributos y macros
    Atributo {
        nombre: String,
        argumentos: Vec<Expr>,
        item: Box<Stmt>,
        span: Span,
    },
    Macro {
        nombre: String,
        argumentos: Vec<Expr>,
        span: Span,
    },
}

// Alias para el programa completo
pub type Programa = Vec<Stmt>;

#[derive(Debug, Clone)]
pub struct Funcion {
    pub nombre: String,
    pub parametros: Vec<(String, String)>,  // (nombre, tipo)
    pub retorno: Option<String>,
    pub generico: Vec<String>,
    pub donde: Vec<String>,  // Cláusulas where
    pub cuerpo: Vec<Stmt>,
    pub es_seguro: bool,      // fn vs unsafe fn
    pub es_async: bool,       // async fn
}

#[derive(Debug, Clone)]
pub struct VarianteEnum {
    pub nombre: String,
    pub campos: Option<Vec<String>>,  // None para unit, Some para tuple/struct
    pub valores: Option<Vec<Expr>>,   // Para discriminantes explícitos
}

#[derive(Debug, Clone)]
pub struct FirmaMetodo {
    pub nombre: String,
    pub parametros: Vec<(String, String)>,
    pub retorno: Option<String>,
    pub tiene_cuerpo: bool,
}

impl Stmt {
    pub fn span(&self) -> Span {
        match self {
            Stmt::Firma { span, .. } => *span,
            Stmt::Sea { span, .. } => *span,
            Stmt::SeaMut { span, .. } => *span,
            Stmt::Const { span, .. } => *span,
            Stmt::Estatico { span, .. } => *span,
            Stmt::Tipo { span, .. } => *span,
            Stmt::Estructura { span, .. } => *span,
            Stmt::Enumeracion { span, .. } => *span,
            Stmt::Implementacion { span, .. } => *span,
            Stmt::Trait { span, .. } => *span,
            Stmt::Funcion { span, .. } => *span,
            Stmt::Si { span, .. } => *span,
            Stmt::Coincidir { span, .. } => *span,
            Stmt::Mientras { span, .. } => *span,
            Stmt::ParaCiclo { span, .. } => *span,
            Stmt::Bucle { span, .. } => *span,
            Stmt::Retornar(_, span) => *span,
            Stmt::Romper(_, span) => *span,
            Stmt::Continuar(_, span) => *span,
            Stmt::Expresion(_, span) => *span,
            Stmt::LlamadaStdlib { span, .. } => *span,
            Stmt::Atributo { span, .. } => *span,
            Stmt::Macro { span, .. } => *span,
        }
    }
}