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
    
    // Tipos especiales
    Opt,        // Option<T>
    Result,     // Result<T, E>
    Alg,        // Any / Generic
    
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
    Never,      // !
    
    // Tipos compuestos (para futuro)
    Fn,         // Function type
    Array(Box<Tipo>, usize),  // [T; N]
    Slice(Box<Tipo>),         // [T]
    Pointer(Box<Tipo>),       // *T
    Reference(Box<Tipo>),     // &T
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
            Tipo::Opt => "opt",
            Tipo::Result => "result",
            Tipo::Alg => "alg",
            
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
            Tipo::Never => "never",
            Tipo::Fn => "fn",
            Tipo::Desconocido => "desconocido",
            
            // Tipos compuestos
            Tipo::Array(_, _) => "array",
            Tipo::Slice(_) => "slice",
            Tipo::Pointer(_) => "pointer",
            Tipo::Reference(_) => "reference",
        }
    }
    
    /// Convierte un tipo Soris a su equivalente en Rust
    pub fn a_rust(&self) -> &str {
        match self {
            Tipo::I8 => "i8",
            Tipo::I16 => "i16",
            Tipo::I32 => "i32",
            Tipo::I64 => "i64",
            Tipo::I128 => "i128",
            Tipo::U8 => "u8",
            Tipo::U16 => "u16",
            Tipo::U32 => "u32",
            Tipo::U64 => "u64",
            Tipo::U128 => "u128",
            
            Tipo::Ent => "i32",
            Tipo::Ent8 => "i8",
            Tipo::Ent16 => "i16",
            Tipo::Ent64 => "i64",
            Tipo::Ent128 => "i128",
            Tipo::Ent8s => "u8",
            Tipo::Ent16s => "u16",
            Tipo::Ent32s => "u32",
            Tipo::Ent64s => "u64",
            Tipo::Ent128s => "u128",
            
            Tipo::Flot => "f64",
            Tipo::F32 => "f32",
            Tipo::F64 => "f64",
            
            Tipo::Car => "char",
            Tipo::Cad => "String",
            Tipo::Txt => "&str",
            Tipo::Bool => "bool",
            
            Tipo::Opt => "Option",
            Tipo::Result => "Result",
            Tipo::Alg => "Any",
            
            Tipo::Nada => "None",
            Tipo::Err => "Error",
            
            Tipo::Numero => "f64",
            Tipo::Texto => "String",
            Tipo::Booleano => "bool",
            Tipo::Vacio | Tipo::Unit => "()",
            Tipo::Never => "!",
            Tipo::Fn => "fn",
            Tipo::Desconocido => "unknown",
            
            Tipo::Array(_, _) => "array",
            Tipo::Slice(_) => "slice",
            Tipo::Pointer(_) => "*const T",
            Tipo::Reference(_) => "&T",
        }
    }
}