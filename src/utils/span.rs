use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Span {
    pub linea: usize,
    pub columna: usize,
}

impl Span {
    pub fn new(linea: usize, columna: usize) -> Self {
        Self { linea, columna }
    }

    pub fn cero() -> Self {
        Self { linea: 0, columna: 0 }
    }
}

impl fmt::Display for Span {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "línea {}, columna {}", self.linea, self.columna)
    }
}