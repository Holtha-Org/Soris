use std::fmt;
use crate::utils::span::Span;

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    // Palabras clave
    Autor,
    Declarar,
    Imprimir,
    Si,
    Sino,
    Mientras,
    // Literales e identificadores
    Identificador(String),
    Numero(f64),
    Texto(String),
    // Símbolos y operadores
    Asignacion,
    PuntoYComa,
    Coma,
    Punto,
    DosPuntos,
    LlaveIzq,
    LlaveDer,
    ParIzq,
    ParDer,
    CorcheteIzq,
    CorcheteDer,
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
    Negacion,
    // Fin de archivo
    EOF,
}

impl Token {
    pub fn palabra_clave_o_identificador(palabra: &str) -> Token {
        match palabra {
            "autor" => Token::Autor,
            "declarar" => Token::Declarar,
            "imprimir" => Token::Imprimir,
            "si" => Token::Si,
            "sino" => Token::Sino,
            "mientras" => Token::Mientras,
            _ => Token::Identificador(palabra.to_string()),
        }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Token::Autor => write!(f, "'autor'"),
            Token::Declarar => write!(f, "'declarar'"),
            Token::Imprimir => write!(f, "'imprimir'"),
            Token::Si => write!(f, "'si'"),
            Token::Sino => write!(f, "'sino'"),
            Token::Mientras => write!(f, "'mientras'"),
            Token::Identificador(nombre) => write!(f, "identificador '{}'", nombre),
            Token::Numero(valor) => write!(f, "número '{}'", valor),
            Token::Texto(valor) => write!(f, "texto \"{}\"", valor),
            Token::Asignacion => write!(f, "'='"),
            Token::PuntoYComa => write!(f, "';'"),
            Token::Coma => write!(f, "','"),
            Token::Punto => write!(f, "'.'"),
            Token::DosPuntos => write!(f, "':'"),
            Token::LlaveIzq => write!(f, "'{{'"),
            Token::LlaveDer => write!(f, "'}}'"),
            Token::ParIzq => write!(f, "'('"),
            Token::ParDer => write!(f, "')'"),
            Token::CorcheteIzq => write!(f, "'['"),
            Token::CorcheteDer => write!(f, "']'"),
            Token::Suma => write!(f, "'+'"),
            Token::Resta => write!(f, "'-'"),
            Token::Multiplicacion => write!(f, "'*'"),
            Token::Division => write!(f, "'/'"),
            Token::Igual => write!(f, "'=='"),
            Token::NoIgual => write!(f, "'!='"),
            Token::Menor => write!(f, "'<'"),
            Token::Mayor => write!(f, "'>'"),
            Token::MenorIgual => write!(f, "'<='"),
            Token::MayorIgual => write!(f, "'>='"),
            Token::YLogico => write!(f, "'y'"),
            Token::OLogico => write!(f, "'o'"),
            Token::Negacion => write!(f, "'no'"),
            Token::EOF => write!(f, "fin de archivo"),
        }
    }
}