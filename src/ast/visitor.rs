use crate::ast::expr::{Expr, BinOp, UnaryOp};
use crate::ast::stmt::Stmt;

pub trait Visitor<T> {
    fn visitar_expr(&mut self, expr: &Expr) -> T;
    fn visitar_stmt(&mut self, stmt: &Stmt) -> T;

    fn visitar_expr_literal_numero(&mut self, valor: f64) -> T;
    fn visitar_expr_literal_texto(&mut self, valor: &str) -> T;
    fn visitar_expr_identificador(&mut self, nombre: &str) -> T;
    fn visitar_expr_binaria(&mut self, izquierda: &Expr, operador: &BinOp, derecha: &Expr) -> T;
    fn visitar_expr_unaria(&mut self, operador: &UnaryOp, operando: &Expr) -> T;
    fn visitar_expr_llamada(&mut self, nombre: &str, argumentos: &[Expr]) -> T;
    fn visitar_expr_acceso_miembro(&mut self, objeto: &Expr, miembro: &str) -> T;

    fn visitar_stmt_firma(&mut self, autor: &str) -> T;
    fn visitar_stmt_declaracion(&mut self, nombre: &str, valor_inicial: &Option<Expr>) -> T;
    fn visitar_stmt_asignacion(&mut self, nombre: &str, valor: &Expr) -> T;
    fn visitar_stmt_impresion(&mut self, expresion: &Expr) -> T;
    fn visitar_stmt_si(&mut self, condicion: &Expr, cuerpo: &[Stmt], sino: &Option<Vec<Stmt>>) -> T;
    fn visitar_stmt_mientras(&mut self, condicion: &Expr, cuerpo: &[Stmt]) -> T;
    fn visitar_stmt_expresion(&mut self, expr: &Expr) -> T;
    fn visitar_stmt_llamada_stdlib(&mut self, comando: &str, argumentos: &[Expr]) -> T;
}

pub fn recorrer_expr<T>(visitor: &mut dyn Visitor<T>, expr: &Expr) -> T {
    match expr {
        Expr::LiteralNumero(valor, _) => visitor.visitar_expr_literal_numero(*valor),
        Expr::LiteralTexto(valor, _) => visitor.visitar_expr_literal_texto(valor),
        Expr::Identificador(nombre, _) => visitor.visitar_expr_identificador(nombre),
        Expr::Binaria { izquierda, operador, derecha, .. } => {
            visitor.visitar_expr_binaria(izquierda, operador, derecha)
        }
        Expr::Unaria { operador, operando, .. } => {
            visitor.visitar_expr_unaria(operador, operando)
        }
        Expr::Llamada { nombre, argumentos, .. } => {
            visitor.visitar_expr_llamada(nombre, argumentos)
        }
        Expr::AccesoMiembro { objeto, miembro, .. } => {
            visitor.visitar_expr_acceso_miembro(objeto, miembro)
        }
        Expr::LiteralCaracter(_, _) | Expr::LiteralBool(_, _) | 
        Expr::OpcionAlguna(_, _) | Expr::OpcionNinguna(_) |
        Expr::ResultadoOk(_, _) | Expr::ResultadoErr(_, _) |
        Expr::Tupla(_, _) | Expr::Array(_, _) | Expr::Vector(_, _) |
        Expr::Indexacion { .. } | Expr::Rango { .. } | Expr::Cierre { .. } |
        Expr::Match { .. } | Expr::IfExpresion { .. } | Expr::Bloque(_, _) |
        Expr::Desestructurar { .. } => todo!("Expresión no implementada en visitor"),
    }
}

pub fn recorrer_stmt<T>(visitor: &mut dyn Visitor<T>, stmt: &Stmt) -> T {
    match stmt {
        Stmt::Firma { autor, .. } => visitor.visitar_stmt_firma(autor),
        Stmt::Sea { nombre, valor_inicial, .. } |
        Stmt::SeaMut { nombre, valor_inicial, .. } => {
            visitor.visitar_stmt_declaracion(nombre, valor_inicial)
        }
        Stmt::Expresion(expr, _) => visitor.visitar_stmt_expresion(expr),
        Stmt::Si { condicion, cuerpo, sino, .. } => {
            visitor.visitar_stmt_si(condicion, cuerpo, sino)
        }
        Stmt::Mientras { condicion, cuerpo, .. } => {
            visitor.visitar_stmt_mientras(condicion, cuerpo)
        }
        Stmt::LlamadaStdlib { comando, argumentos, .. } => {
            visitor.visitar_stmt_llamada_stdlib(comando, argumentos)
        }
        Stmt::Const { .. } | Stmt::Estatico { .. } | Stmt::Tipo { .. } |
        Stmt::Estructura { .. } | Stmt::Enumeracion { .. } |
        Stmt::Implementacion { .. } | Stmt::Trait { .. } |
        Stmt::Funcion { .. } | Stmt::Coincidir { .. } |
        Stmt::Para { .. } | Stmt::Retorno { .. } | Stmt::Di { .. } |
        Stmt::Pausa { .. } | Stmt::Continuar { .. } |
        Stmt::Bloque { .. } | Stmt::Asignacion { .. } |
        Stmt::BrazoCoin { .. } => todo!("Sentencia no implementada en visitor"),
    }
}