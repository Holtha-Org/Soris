pub mod expr;
pub mod stmt;
pub mod types;
pub mod visitor;

// Re-export principal para facilitar imports
pub use expr::{Expr, BinOp, UnaryOp, Patron, BrazoMatch};
pub use stmt::{Stmt, Programa};
pub use types::Tipo;

// Alias para Literal (usamos Patron::Literal como fallback)
pub type Literal = Patron;