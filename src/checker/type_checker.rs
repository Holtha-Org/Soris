use crate::ast::expr::Expr;
use crate::ast::stmt::Stmt;
use crate::ast::types::Tipo;
use crate::ast::visitor::{Visitor, recorrer_expr, recorrer_stmt};
use crate::utils::errors::ErrorCompilador;

pub struct TypeChecker {
    errores: Vec<ErrorCompilador>,
}

impl TypeChecker {
    pub fn new() -> Self {
        Self {
            errores: Vec::new(),
        }
    }

    pub fn check(&mut self, statements: &[Stmt]) -> Result<(), Vec<ErrorCompilador>> {
        for stmt in statements {
            self.visitar_stmt(stmt);
        }

        if self.errores.is_empty() {
            Ok(())
        } else {
            Err(std::mem::take(&mut self.errores))
        }
    }
}

impl Visitor<Tipo> for TypeChecker {
    fn visitar_expr(&mut self, expr: &Expr) -> Tipo {
        recorrer_expr(self, expr)
    }

    fn visitar_stmt(&mut self, stmt: &Stmt) -> Tipo {
        recorrer_stmt(self, stmt)
    }

    fn visitar_expr_literal_numero(&mut self, _valor: f64) -> Tipo {
        Tipo::Numero
    }

    fn visitar_expr_literal_texto(&mut self, _valor: &str) -> Tipo {
        Tipo::Texto
    }

    fn visitar_expr_identificador(&mut self, _nombre: &str) -> Tipo {
        Tipo::Desconocido
    }

    fn visitar_expr_binaria(&mut self, izquierda: &Expr, _operador: &crate::ast::expr::BinOp, derecha: &Expr) -> Tipo {
        let tipo_izq = self.visitar_expr(izquierda);
        let tipo_der = self.visitar_expr(derecha);

        match (tipo_izq, tipo_der) {
            (Tipo::Numero, Tipo::Numero) => Tipo::Numero,
            _ => Tipo::Desconocido,
        }
    }

    fn visitar_expr_unaria(&mut self, _operador: &crate::ast::expr::UnaryOp, operando: &Expr) -> Tipo {
        self.visitar_expr(operando)
    }

    fn visitar_expr_llamada(&mut self, _nombre: &str, _argumentos: &[Expr]) -> Tipo {
        Tipo::Desconocido
    }

    fn visitar_expr_acceso_miembro(&mut self, objeto: &Expr, _miembro: &str) -> Tipo {
        self.visitar_expr(objeto)
    }

    fn visitar_stmt_firma(&mut self, _autor: &str) -> Tipo {
        Tipo::Vacio
    }

    fn visitar_stmt_declaracion(&mut self, _nombre: &str, valor_inicial: &Option<Expr>) -> Tipo {
        if let Some(init) = valor_inicial {
            self.visitar_expr(init);
        }
        Tipo::Vacio
    }

    fn visitar_stmt_asignacion(&mut self, _nombre: &str, valor: &Expr) -> Tipo {
        self.visitar_expr(valor);
        Tipo::Vacio
    }

    fn visitar_stmt_impresion(&mut self, expresion: &Expr) -> Tipo {
        self.visitar_expr(expresion);
        Tipo::Vacio
    }

    fn visitar_stmt_si(&mut self, condicion: &Expr, cuerpo: &[Stmt], sino: &Option<Vec<Stmt>>) -> Tipo {
        self.visitar_expr(condicion);
        for stmt in cuerpo {
            self.visitar_stmt(stmt);
        }
        if let Some(sino_stmts) = sino {
            for stmt in sino_stmts {
                self.visitar_stmt(stmt);
            }
        }
        Tipo::Vacio
    }

    fn visitar_stmt_mientras(&mut self, condicion: &Expr, cuerpo: &[Stmt]) -> Tipo {
        self.visitar_expr(condicion);
        for stmt in cuerpo {
            self.visitar_stmt(stmt);
        }
        Tipo::Vacio
    }

    fn visitar_stmt_expresion(&mut self, expr: &Expr) -> Tipo {
        self.visitar_expr(expr);
        Tipo::Vacio
    }

    fn visitar_stmt_llamada_stdlib(&mut self, _comando: &str, _argumentos: &[Expr]) -> Tipo {
        Tipo::Vacio
    }
}