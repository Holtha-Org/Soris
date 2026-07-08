#[derive(Debug, Clone, PartialEq)]
pub enum Tipo {
    Numero,
    Texto,
    Booleano,
    Vacio,
    Desconocido,
}

impl Tipo {
    pub fn nombre(&self) -> &str {
        match self {
            Tipo::Numero => "número",
            Tipo::Texto => "texto",
            Tipo::Booleano => "booleano",
            Tipo::Vacio => "vacío",
            Tipo::Desconocido => "desconocido",
        }
    }
}