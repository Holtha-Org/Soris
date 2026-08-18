use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    // Palabras clave de control de flujo
    Si,
    Sino,
    ElSi,
    Mientras,
    Para,
    En,
    Pausa,
    Continuar,
    Retorna,
    
    // Palabras clave de declaraciones
    Fn,
    Var,
    Const,
    Rasgo,
    Struct,
    Enum,
    Impl,
    
    // Palabras clave de tipos
    I8, I16, I32, I64, I128,
    U8, U16, U32, U64, U128,
    Ent, Ent8, Ent16, Ent64, Ent128,
    Ent8s, Ent16s, Ent32s, Ent64s, Ent128s,
    Flot, F32, F64,
    Car, Cad, Txt,
    Bool,
    Opt, Result, Alg, Nada, Err,
    
    // Otros keywords
    Pub, Priv, Mut, Ref,
    Verdadero, Falso,
    Self_,
    
    // Macro di!
    Di,
    
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
    Flecha,
    FlechaGorda,
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
    Modulo,
    Igual,
    NoIgual,
    Menor,
    Mayor,
    MenorIgual,
    MayorIgual,
    YLogico,
    OLogico,
    Negacion,
    And,
    Or,
    PluIgual,
    MenosIgual,
    MulIgual,
    DivIgual,
    
    // Fin de archivo
    EOF,
}

impl Token {
    pub fn palabra_clave_o_identificador(palabra: &str) -> Token {
        match palabra {
            // Control de flujo
            "si" => Token::Si,
            "sino" => Token::Sino,
            "elsi" => Token::ElSi,
            "mientras" => Token::Mientras,
            "para" => Token::Para,
            "en" => Token::En,
            "pausa" => Token::Pausa,
            "continuar" => Token::Continuar,
            "retorna" => Token::Retorna,
            
            // Declaraciones
            "fn" => Token::Fn,
            "var" => Token::Var,
            "const" => Token::Const,
            "rasgo" => Token::Rasgo,
            "struct" => Token::Struct,
            "enum" => Token::Enum,
            "impl" => Token::Impl,
            
            // Tipos numéricos enteros
            "i8" => Token::I8,
            "i16" => Token::I16,
            "i32" => Token::I32,
            "i64" => Token::I64,
            "i128" => Token::I128,
            "u8" => Token::U8,
            "u16" => Token::U16,
            "u32" => Token::U32,
            "u64" => Token::U64,
            "u128" => Token::U128,
            
            // Tipos personalizados de Soris
            "ent" => Token::Ent,
            "ent8" => Token::Ent8,
            "ent16" => Token::Ent16,
            "ent64" => Token::Ent64,
            "ent128" => Token::Ent128,
            "ent8s" => Token::Ent8s,
            "ent16s" => Token::Ent16s,
            "ent32s" => Token::Ent32s,
            "ent64s" => Token::Ent64s,
            "ent128s" => Token::Ent128s,
            
            // Tipos flotantes
            "flot" => Token::Flot,
            "f32" => Token::F32,
            "f64" => Token::F64,
            
            // Otros tipos
            "car" => Token::Car,
            "cad" => Token::Cad,
            "txt" => Token::Txt,
            "bool" => Token::Bool,
            
            // Result/Option/Algun
            "opt" => Token::Opt,
            "result" => Token::Result,
            "alg" => Token::Alg,
            "nada" => Token::Nada,
            "err" => Token::Err,
            
            // Otros keywords
            "pub" => Token::Pub,
            "priv" => Token::Priv,
            "mut" => Token::Mut,
            "ref" => Token::Ref,
            "verdadero" => Token::Verdadero,
            "falso" => Token::Falso,
            "self" => Token::Self_,
            
            // Macro di!
            "di" => Token::Di,
            
            _ => Token::Identificador(palabra.to_string()),
        }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            // Control de flujo
            Token::Si => write!(f, "'si'"),
            Token::Sino => write!(f, "'sino'"),
            Token::ElSi => write!(f, "'elsi'"),
            Token::Mientras => write!(f, "'mientras'"),
            Token::Para => write!(f, "'para'"),
            Token::En => write!(f, "'en'"),
            Token::Pausa => write!(f, "'pausa'"),
            Token::Continuar => write!(f, "'continuar'"),
            Token::Retorna => write!(f, "'retorna'"),
            
            // Declaraciones
            Token::Fn => write!(f, "'fn'"),
            Token::Var => write!(f, "'var'"),
            Token::Const => write!(f, "'const'"),
            Token::Rasgo => write!(f, "'rasgo'"),
            Token::Struct => write!(f, "'struct'"),
            Token::Enum => write!(f, "'enum'"),
            Token::Impl => write!(f, "'impl'"),
            
            // Tipos
            Token::I8 => write!(f, "'i8'"),
            Token::I16 => write!(f, "'i16'"),
            Token::I32 => write!(f, "'i32'"),
            Token::I64 => write!(f, "'i64'"),
            Token::I128 => write!(f, "'i128'"),
            Token::U8 => write!(f, "'u8'"),
            Token::U16 => write!(f, "'u16'"),
            Token::U32 => write!(f, "'u32'"),
            Token::U64 => write!(f, "'u64'"),
            Token::U128 => write!(f, "'u128'"),
            Token::Ent => write!(f, "'ent'"),
            Token::Ent8 => write!(f, "'ent8'"),
            Token::Ent16 => write!(f, "'ent16'"),
            Token::Ent64 => write!(f, "'ent64'"),
            Token::Ent128 => write!(f, "'ent128'"),
            Token::Ent8s => write!(f, "'ent8s'"),
            Token::Ent16s => write!(f, "'ent16s'"),
            Token::Ent32s => write!(f, "'ent32s'"),
            Token::Ent64s => write!(f, "'ent64s'"),
            Token::Ent128s => write!(f, "'ent128s'"),
            Token::Flot => write!(f, "'flot'"),
            Token::F32 => write!(f, "'f32'"),
            Token::F64 => write!(f, "'f64'"),
            Token::Car => write!(f, "'car'"),
            Token::Cad => write!(f, "'cad'"),
            Token::Txt => write!(f, "'txt'"),
            Token::Bool => write!(f, "'bool'"),
            Token::Opt => write!(f, "'opt'"),
            Token::Result => write!(f, "'result'"),
            Token::Alg => write!(f, "'alg'"),
            Token::Nada => write!(f, "'nada'"),
            Token::Err => write!(f, "'err'"),
            
            // Otros
            Token::Pub => write!(f, "'pub'"),
            Token::Priv => write!(f, "'priv'"),
            Token::Mut => write!(f, "'mut'"),
            Token::Ref => write!(f, "'ref'"),
            Token::Verdadero => write!(f, "'verdadero'"),
            Token::Falso => write!(f, "'falso'"),
            Token::Self_ => write!(f, "'self'"),
            Token::Di => write!(f, "'di!'"),
            
            // Literales e identificadores
            Token::Identificador(nombre) => write!(f, "identificador '{}'", nombre),
            Token::Numero(valor) => write!(f, "número '{}'", valor),
            Token::Texto(valor) => write!(f, "texto \"{}\"", valor),
            
            // Símbolos y operadores
            Token::Asignacion => write!(f, "'='"),
            Token::PuntoYComa => write!(f, "';'"),
            Token::Coma => write!(f, "','"),
            Token::Punto => write!(f, "'.'"),
            Token::DosPuntos => write!(f, "':'"),
            Token::Flecha => write!(f, "'->'"),
            Token::FlechaGorda => write!(f, "'=>'"),
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
            Token::Modulo => write!(f, "'%'"),
            Token::Igual => write!(f, "'=='"),
            Token::NoIgual => write!(f, "'!='"),
            Token::Menor => write!(f, "'<'"),
            Token::Mayor => write!(f, "'>'"),
            Token::MenorIgual => write!(f, "'<='"),
            Token::MayorIgual => write!(f, "'>='"),
            Token::YLogico => write!(f, "'y'"),
            Token::OLogico => write!(f, "'o'"),
            Token::Negacion => write!(f, "'!'"),
            Token::And => write!(f, "'&'"),
            Token::Or => write!(f, "'|'"),
            Token::PluIgual => write!(f, "'+='"),
            Token::MenosIgual => write!(f, "'-='"),
            Token::MulIgual => write!(f, "'*='"),
            Token::DivIgual => write!(f, "'/='"),
            
            Token::EOF => write!(f, "fin de archivo"),
        }
    }
}