use std::collections::HashMap;
use crate::resolver::symbols::Simbolo;
use crate::ast::expr::Expr;
use crate::ast::stmt::Stmt;
use crate::ast::visitor::{Visitor, recorrer_expr, recorrer_stmt};
use crate::utils::span::Span;
use crate::utils::errors::ErrorCompilador;

pub struct ScopeResolver {
    scopes: Vec<HashMap<String, Simbolo>>,
    errores: Vec<ErrorCompilador>,
}

impl ScopeResolver {
    pub fn new() -> Self {
        Self {
            scopes: vec![HashMap::new()],
            errores: Vec::new(),
        }
    }

    pub fn resolver(&mut self, statements: &[Stmt]) -> Result<(), Vec<ErrorCompilador>> {
        for stmt in statements {
            self.visitar_stmt(stmt);
        }

        if self.errores.is_empty() {
            Ok(())
        } else {
            Err(std::mem::take(&mut self.errores))
        }
    }

    fn begin_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }

    fn end_scope(&mut self) {
        self.scopes.pop();
    }

    fn declarar(&mut self, nombre: &str, _span: Span) {
        if let Some(scope) = self.scopes.last_mut() {
            if scope.contains_key(nombre) {
                self.errores.push(
                    ErrorCompilador::new(
                        &format!("La variable '{}' ya está declarada en este ámbito", nombre)
                    )
                );
            } else {
                scope.insert(
                    nombre.to_string(),
                    Simbolo::new(nombre, crate::ast::types::Tipo::Desconocido, true, Span::cero()),
                );
            }
        }
    }

    fn resolver_acceso(&mut self, nombre: &str, _span: Span) {
        let mut encontrada = false;
        for scope in self.scopes.iter().rev() {
            if scope.contains_key(nombre) {
                encontrada = true;
                break;
            }
        }

        if !encontrada {
            self.errores.push(
                ErrorCompilador::new(
                    &format!("La variable '{}' no está declarada", nombre)
                )
            );
        }
    }
}

impl Visitor<()> for ScopeResolver {
    fn visitar_expr(&mut self, expr: &Expr) {
        recorrer_expr(self, expr);
    }

    fn visitar_stmt(&mut self, stmt: &Stmt) {
        recorrer_stmt(self, stmt);
    }

    fn visitar_expr_literal_numero(&mut self, _valor: f64) {}

    fn visitar_expr_literal_texto(&mut self, _valor: &str) {}

    fn visitar_expr_identificador(&mut self, nombre: &str) {
        self.resolver_acceso(nombre, Span::cero());
    }

    fn visitar_expr_binaria(&mut self, izquierda: &Expr, _operador: &crate::ast::expr::BinOp, derecha: &Expr) {
        self.visitar_expr(izquierda);
        self.visitar_expr(derecha);
    }

    fn visitar_expr_unaria(&mut self, _operador: &crate::ast::expr::UnaryOp, operando: &Expr) {
        self.visitar_expr(operando);
    }

    fn visitar_expr_llamada(&mut self, _nombre: &str, argumentos: &[Expr]) {
        for arg in argumentos {
            self.visitar_expr(arg);
        }
    }

    fn visitar_expr_acceso_miembro(&mut self, objeto: &Expr, _miembro: &str) {
        self.visitar_expr(objeto);
    }

    fn visitar_stmt_firma(&mut self, _autor: &str) {}

    fn visitar_stmt_declaracion(&mut self, nombre: &str, valor_inicial: &Option<Expr>) {
        if let Some(init) = valor_inicial {
            self.visitar_expr(init);
        }
        self.declarar(nombre, Span::cero());
    }

    fn visitar_stmt_asignacion(&mut self, nombre: &str, valor: &Expr) {
        self.resolver_acceso(nombre, Span::cero());
        self.visitar_expr(valor);
    }

    fn visitar_stmt_impresion(&mut self, expresion: &Expr) {
        self.visitar_expr(expresion);
    }

    fn visitar_stmt_si(&mut self, condicion: &Expr, cuerpo: &[Stmt], sino: &Option<Vec<Stmt>>) {
        self.visitar_expr(condicion);
        self.begin_scope();
        for stmt in cuerpo {
            self.visitar_stmt(stmt);
        }
        self.end_scope();

        if let Some(sino_stmts) = sino {
            self.begin_scope();
            for stmt in sino_stmts {
                self.visitar_stmt(stmt);
            }
            self.end_scope();
        }
    }

    fn visitar_stmt_mientras(&mut self, condicion: &Expr, cuerpo: &[Stmt]) {
        self.visitar_expr(condicion);
        self.begin_scope();
        for stmt in cuerpo {
            self.visitar_stmt(stmt);
        }
        self.end_scope();
    }

    fn visitar_stmt_expresion(&mut self, expr: &Expr) {
        self.visitar_expr(expr);
    }

    fn visitar_stmt_llamada_stdlib(&mut self, _comando: &str, argumentos: &[Expr]) {
        for arg in argumentos {
            self.visitar_expr(arg);
        }
    }
}