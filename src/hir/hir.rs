use crate::ast::expr::{Expr, BinOp, UnaryOp};
use crate::ast::stmt::Stmt;

#[derive(Debug, Clone)]
pub enum HirExpr {
    ConstanteNumero(f64),
    ConstanteTexto(String),
    Variable(String),
    OperacionBinaria {
        izquierda: Box<HirExpr>,
        operador: HirBinOp,
        derecha: Box<HirExpr>,
    },
    OperacionUnaria {
        operador: HirUnaryOp,
        operando: Box<HirExpr>,
    },
    LlamadaStdlib {
        comando: String,
        argumentos: Vec<HirExpr>,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HirBinOp {
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
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HirUnaryOp {
    Negacion,
    Negativo,
}

#[derive(Debug, Clone)]
pub enum HirStmt {
    Firma(String),
    Declaracion {
        nombre: String,
        valor_inicial: Option<HirExpr>,
    },
    Asignacion {
        nombre: String,
        valor: HirExpr,
    },
    Impresion(HirExpr),
    Si {
        condicion: HirExpr,
        cuerpo: Vec<HirStmt>,
        sino: Option<Vec<HirStmt>>,
    },
    Mientras {
        condicion: HirExpr,
        cuerpo: Vec<HirStmt>,
    },
    LlamadaStdlib {
        comando: String,
        argumentos: Vec<HirExpr>,
    },
}

pub fn ast_to_hir(statements: &[Stmt]) -> Vec<HirStmt> {
    statements.iter().map(stmt_to_hir).collect()
}

fn stmt_to_hir(stmt: &Stmt) -> HirStmt {
    match stmt {
        Stmt::Firma { autor, .. } => HirStmt::Firma(autor.clone()),
        Stmt::Declaracion { nombre, valor_inicial, .. } => HirStmt::Declaracion {
            nombre: nombre.clone(),
            valor_inicial: valor_inicial.as_ref().map(expr_to_hir),
        },
        Stmt::Asignacion { nombre, valor, .. } => HirStmt::Asignacion {
            nombre: nombre.clone(),
            valor: expr_to_hir(valor),
        },
        Stmt::Impresion { expresion, .. } => HirStmt::Impresion(expr_to_hir(expresion)),
        Stmt::Si { condicion, cuerpo, sino, .. } => HirStmt::Si {
            condicion: expr_to_hir(condicion),
            cuerpo: cuerpo.iter().map(stmt_to_hir).collect(),
            sino: sino.as_ref().map(|s| s.iter().map(stmt_to_hir).collect()),
        },
        Stmt::Mientras { condicion, cuerpo, .. } => HirStmt::Mientras {
            condicion: expr_to_hir(condicion),
            cuerpo: cuerpo.iter().map(stmt_to_hir).collect(),
        },
        Stmt::LlamadaStdlib { comando, argumentos, .. } => HirStmt::LlamadaStdlib {
            comando: comando.clone(),
            argumentos: argumentos.iter().map(expr_to_hir).collect(),
        },
        Stmt::Expresion(expr, _) => HirStmt::Impresion(expr_to_hir(expr)),
    }
}

fn expr_to_hir(expr: &Expr) -> HirExpr {
    match expr {
        Expr::LiteralNumero(valor, _) => HirExpr::ConstanteNumero(*valor),
        Expr::LiteralTexto(valor, _) => HirExpr::ConstanteTexto(valor.clone()),
        Expr::Identificador(nombre, _) => HirExpr::Variable(nombre.clone()),
        Expr::Binaria { izquierda, operador, derecha, .. } => HirExpr::OperacionBinaria {
            izquierda: Box::new(expr_to_hir(izquierda)),
            operador: match operador {
                BinOp::Suma => HirBinOp::Suma,
                BinOp::Resta => HirBinOp::Resta,
                BinOp::Multiplicacion => HirBinOp::Multiplicacion,
                BinOp::Division => HirBinOp::Division,
                BinOp::Igual => HirBinOp::Igual,
                BinOp::NoIgual => HirBinOp::NoIgual,
                BinOp::Menor => HirBinOp::Menor,
                BinOp::Mayor => HirBinOp::Mayor,
                BinOp::MenorIgual => HirBinOp::MenorIgual,
                BinOp::MayorIgual => HirBinOp::MayorIgual,
                BinOp::YLogico => HirBinOp::YLogico,
                BinOp::OLogico => HirBinOp::OLogico,
                BinOp::Modulo => HirBinOp::Modulo,
                BinOp::BitAnd => HirBinOp::BitAnd,
                BinOp::BitOr => HirBinOp::BitOr,
                BinOp::BitXor => HirBinOp::BitXor,
                BinOp::DesplazaIzq => HirBinOp::DesplazaIzq,
                BinOp::DesplazaDer => HirBinOp::DesplazaDer,
                BinOp::Potencia => HirBinOp::Potencia,
                BinOp::Concatenar => HirBinOp::Concatenar,
            },
            derecha: Box::new(expr_to_hir(derecha)),
        },
        Expr::Unaria { operador, operando, .. } => HirExpr::OperacionUnaria {
            operador: match operador {
                UnaryOp::Negacion => HirUnaryOp::Negacion,
                UnaryOp::Negativo => HirUnaryOp::Negativo,
                UnaryOp::Dereferencia => HirUnaryOp::Dereferencia,
                UnaryOp::NotBit => HirUnaryOp::NotBit,
            },
            operando: Box::new(expr_to_hir(operando)),
        },
        Expr::Llamada { nombre, argumentos, .. } => HirExpr::LlamadaStdlib {
            comando: nombre.clone(),
            argumentos: argumentos.iter().map(expr_to_hir).collect(),
        },
        Expr::AccesoMiembro { objeto, miembro, .. } => {
            let objeto_hir = expr_to_hir(objeto);
            if let HirExpr::Variable(obj_name) = objeto_hir {
                HirExpr::LlamadaStdlib {
                    comando: format!("{}.{}", obj_name, miembro),
                    argumentos: vec![],
                }
            } else {
                HirExpr::LlamadaStdlib {
                    comando: format!("{}.{}", "objeto", miembro),
                    argumentos: vec![objeto_hir],
                }
            }
        }
    }
}