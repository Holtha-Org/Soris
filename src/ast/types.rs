use crate::utils::span::Span;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Tipo {
    // Tipos enteros con signo
    I8,
    I16,
    I32,
    I64,
    I128,
    
    // Tipos enteros sin signo
    U8,
    U16,
    U32,
    U64,
    U128,
    
    // Tipos Soris personalizados
    Ent,        // i32 por defecto
    Ent8,       // i8
    Ent16,      // i16
    Ent64,      // i64
    Ent128,     // i128
    Ent8s,      // u8
    Ent16s,     // u16
    Ent32s,     // u32
    Ent64s,     // u64
    Ent128s,    // u128
    
    // Tipos flotantes
    Flot,       // f64 por defecto
    F32,
    F64,
    
    // Otros tipos
    Car,        // char
    Cad,        // String
    Txt,        // &str
    Bool,       // bool
    
    // Tipos especiales - Sistema de errores como Rust
    Opt(Box<Tipo>),        // Option<T>
    Result(Box<Tipo>, Box<Tipo>),  // Result<T, E>
    Alg,        // Any / Generic
    Nunca,      // ! (never type)
    
    // Constantes especiales
    Nada,       // None
    Err,        // Error
    
    // Tipos antiguos (compatibilidad)
    Numero,     // Numero genérico
    Texto,      // String/Texto genérico
    Booleano,   // bool
    Vacio,      // ()
    
    // Tipos especiales
    Desconocido,
    Unit,       // ()
    
    // Tipos compuestos
    Fn(Vec<Tipo>, Box<Tipo>),         // Function type fn(A, B) -> C
    Array(Box<Tipo>, usize),  // [T; N]
    Vector(Box<Tipo>),        // Vec<T>
    Slice(Box<Tipo>),         // [T]
    Pointer(Box<Tipo>),       // *T
    Referencia(Box<Tipo>, Mutabilidad),     // &T o &mut T
    Tupla(Vec<Tipo>),         // (T1, T2, ...)
    
    // Genéricos y Traits
    Generico(String),
    ConTrait(String, String), // Tipo que implementa Trait
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Mutabilidad {
    Inmutable,
    Mutable,
}

impl Tipo {
    pub fn nombre(&self) -> &str {
        match self {
            // Tipos enteros con signo
            Tipo::I8 => "i8",
            Tipo::I16 => "i16",
            Tipo::I32 => "i32",
            Tipo::I64 => "i64",
            Tipo::I128 => "i128",
            
            // Tipos enteros sin signo
            Tipo::U8 => "u8",
            Tipo::U16 => "u16",
            Tipo::U32 => "u32",
            Tipo::U64 => "u64",
            Tipo::U128 => "u128",
            
            // Tipos Soris personalizados
            Tipo::Ent => "ent",
            Tipo::Ent8 => "ent8",
            Tipo::Ent16 => "ent16",
            Tipo::Ent64 => "ent64",
            Tipo::Ent128 => "ent128",
            Tipo::Ent8s => "ent8s",
            Tipo::Ent16s => "ent16s",
            Tipo::Ent32s => "ent32s",
            Tipo::Ent64s => "ent64s",
            Tipo::Ent128s => "ent128s",
            
            // Tipos flotantes
            Tipo::Flot => "flot",
            Tipo::F32 => "f32",
            Tipo::F64 => "f64",
            
            // Otros tipos
            Tipo::Car => "car",
            Tipo::Cad => "cad",
            Tipo::Txt => "txt",
            Tipo::Bool => "bool",
            
            // Tipos especiales
            Tipo::Opt(_) => "opt",
            Tipo::Result(_, _) => "result",
            Tipo::Alg => "alg",
            Tipo::Nunca => "nunca",
            
            // Constantes especiales
            Tipo::Nada => "nada",
            Tipo::Err => "err",
            
            // Tipos antiguos
            Tipo::Numero => "número",
            Tipo::Texto => "texto",
            Tipo::Booleano => "booleano",
            Tipo::Vacio => "vacío",
            
            // Tipos especiales
            Tipo::Unit => "unit",
            Tipo::Nunca => "nunca",
            Tipo::Fn(_, _) => "fn",
            Tipo::Desconocido => "desconocido",
            
            // Tipos compuestos
            Tipo::Array(_, _) => "array",
            Tipo::Vector(_) => "vector",
            Tipo::Slice(_) => "slice",
            Tipo::Pointer(_) => "puntero",
            Tipo::Referencia(_, _) => "referencia",
            Tipo::Tupla(_) => "tupla",
            
            // Genéricos y Traits
            Tipo::Generico(_) => "genérico",
            Tipo::ConTrait(_, _) => "trait",
        }
    }
    
    /// Convierte un tipo Soris a su equivalente en Rust
    pub fn a_rust(&self) -> String {
        match self {
            Tipo::I8 => "i8".to_string(),
            Tipo::I16 => "i16".to_string(),
            Tipo::I32 => "i32".to_string(),
            Tipo::I64 => "i64".to_string(),
            Tipo::I128 => "i128".to_string(),
            Tipo::U8 => "u8".to_string(),
            Tipo::U16 => "u16".to_string(),
            Tipo::U32 => "u32".to_string(),
            Tipo::U64 => "u64".to_string(),
            Tipo::U128 => "u128".to_string(),
            
            Tipo::Ent => "i32".to_string(),
            Tipo::Ent8 => "i8".to_string(),
            Tipo::Ent16 => "i16".to_string(),
            Tipo::Ent64 => "i64".to_string(),
            Tipo::Ent128 => "i128".to_string(),
            Tipo::Ent8s => "u8".to_string(),
            Tipo::Ent16s => "u16".to_string(),
            Tipo::Ent32s => "u32".to_string(),
            Tipo::Ent64s => "u64".to_string(),
            Tipo::Ent128s => "u128".to_string(),
            
            Tipo::Flot => "f64".to_string(),
            Tipo::F32 => "f32".to_string(),
            Tipo::F64 => "f64".to_string(),
            
            Tipo::Car => "char".to_string(),
            Tipo::Cad => "String".to_string(),
            Tipo::Txt => "&str".to_string(),
            Tipo::Bool => "bool".to_string(),
            
            Tipo::Opt(inner) => format!("Option<{}>", inner.a_rust()),
            Tipo::Result(ok, err) => format!("Result<{}, {}>", ok.a_rust(), err.a_rust()),
            Tipo::Alg => "Box<dyn std::any::Any>".to_string(),
            Tipo::Nunca => "!".to_string(),
            
            Tipo::Nada => "None".to_string(),
            Tipo::Err => "Err".to_string(),
            
            Tipo::Numero => "f64".to_string(),
            Tipo::Texto => "String".to_string(),
            Tipo::Booleano => "bool".to_string(),
            Tipo::Vacio | Tipo::Unit => "()".to_string(),
            Tipo::Nunca => "!".to_string(),
            Tipo::Fn(_, _) => "fn".to_string(),
            Tipo::Desconocido => "unknown".to_string(),
            
            Tipo::Array(inner, size) => format!("[{}; {}]", inner.a_rust(), size),
            Tipo::Vector(inner) => format!("Vec<{}>", inner.a_rust()),
            Tipo::Slice(inner) => format!("[{}]", inner.a_rust()),
            Tipo::Pointer(inner) => format!("*const {}", inner.a_rust()),
            Tipo::Referencia(inner, Mutabilidad::Inmutable) => format!("&{}", inner.a_rust()),
            Tipo::Referencia(inner, Mutabilidad::Mutable) => format!("&mut {}", inner.a_rust()),
            Tipo::Tupla(types) => {
                let types_str: Vec<String> = types.iter().map(|t| t.a_rust()).collect();
                format!("({})", types_str.join(", "))
            }
            
            Tipo::Generico(name) => name.clone(),
            Tipo::ConTrait(_, trait_name) => format!("Box<dyn {}>", trait_name),
        }
    }
    
    /// Verifica si dos tipos son compatibles
    pub fn es_compatible_con(&self, otro: &Tipo) -> bool {
        match (self, otro) {
            (Tipo::Numero, Tipo::Numero) => true,
            (Tipo::Texto, Tipo::Texto) => true,
            (Tipo::Booleano, Tipo::Booleano) => true,
            (Tipo::Ent, Tipo::I32) | (Tipo::I32, Tipo::Ent) => true,
            (Tipo::Flot, Tipo::F64) | (Tipo::F64, Tipo::Flot) => true,
            (Tipo::Opt(inner_self), Tipo::Opt(inner_otro)) => inner_self.es_compatible_con(inner_otro),
            (Tipo::Result(ok1, err1), Tipo::Result(ok2, err2)) => {
                ok1.es_compatible_con(ok2) && err1.es_compatible_con(err2)
            }
            _ => self == otro,
        }
    }
}