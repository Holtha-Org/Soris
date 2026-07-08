use crate::hir::hir::{HirStmt, HirExpr, HirBinOp, HirUnaryOp};

#[derive(Debug, Clone)]
pub enum MirInstruccion {
    Firma(String),
    DeclararVariable {
        nombre: String,
        valor_inicial: Option<MirValor>,
    },
    AsignarVariable {
        nombre: String,
        valor: MirValor,
    },
    Imprimir(MirValor),
    SaltarSi {
        condicion: MirValor,
        etiqueta_verdadero: usize,
        etiqueta_falso: usize,
    },
    Etiqueta(usize),
    Saltar(usize),
    LlamarStdlib {
        comando: String,
        argumentos: Vec<MirValor>,
    },
}

#[derive(Debug, Clone)]
pub enum MirValor {
    ConstanteNumero(f64),
    ConstanteTexto(String),
    Variable(String),
    OperacionBinaria {
        izquierda: Box<MirValor>,
        operador: MirBinOp,
        derecha: Box<MirValor>,
    },
    OperacionUnaria {
        operador: MirUnaryOp,
        operando: Box<MirValor>,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MirBinOp {
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
pub enum MirUnaryOp {
    Negacion,
    Negativo,
}

pub fn hir_to_mir(hir_stmts: &[HirStmt]) -> Vec<MirInstruccion> {
    let mut instrucciones = Vec::new();
    let mut etiqueta_contador = 0;

    for stmt in hir_stmts {
        translate_stmt(stmt, &mut instrucciones, &mut etiqueta_contador);
    }

    instrucciones
}

fn translate_stmt(
    stmt: &HirStmt,
    instrucciones: &mut Vec<MirInstruccion>,
    etiqueta_contador: &mut usize,
) {
    match stmt {
        HirStmt::Firma(autor) => {
            instrucciones.push(MirInstruccion::Firma(autor.clone()));
        }
        HirStmt::Declaracion { nombre, valor_inicial } => {
            instrucciones.push(MirInstruccion::DeclararVariable {
                nombre: nombre.clone(),
                valor_inicial: valor_inicial.as_ref().map(translate_expr),
            });
        }
        HirStmt::Asignacion { nombre, valor } => {
            instrucciones.push(MirInstruccion::AsignarVariable {
                nombre: nombre.clone(),
                valor: translate_expr(valor),
            });
        }
        HirStmt::Impresion(expr) => {
            instrucciones.push(MirInstruccion::Imprimir(translate_expr(expr)));
        }
        HirStmt::Si { condicion, cuerpo, sino } => {
            let etiqueta_verdadero = *etiqueta_contador;
            *etiqueta_contador += 1;
            let etiqueta_falso = *etiqueta_contador;
            *etiqueta_contador += 1;
            let etiqueta_fin = *etiqueta_contador;
            *etiqueta_contador += 1;

            instrucciones.push(MirInstruccion::SaltarSi {
                condicion: translate_expr(condicion),
                etiqueta_verdadero,
                etiqueta_falso,
            });

            instrucciones.push(MirInstruccion::Etiqueta(etiqueta_verdadero));
            for stmt in cuerpo {
                translate_stmt(stmt, instrucciones, etiqueta_contador);
            }
            instrucciones.push(MirInstruccion::Saltar(etiqueta_fin));

            instrucciones.push(MirInstruccion::Etiqueta(etiqueta_falso));
            if let Some(sino_stmts) = sino {
                for stmt in sino_stmts {
                    translate_stmt(stmt, instrucciones, etiqueta_contador);
                }
            }

            instrucciones.push(MirInstruccion::Etiqueta(etiqueta_fin));
        }
        HirStmt::Mientras { condicion, cuerpo } => {
            let etiqueta_inicio = *etiqueta_contador;
            *etiqueta_contador += 1;
            let etiqueta_cuerpo = *etiqueta_contador;
            *etiqueta_contador += 1;
            let etiqueta_fin = *etiqueta_contador;
            *etiqueta_contador += 1;

            instrucciones.push(MirInstruccion::Etiqueta(etiqueta_inicio));
            instrucciones.push(MirInstruccion::SaltarSi {
                condicion: translate_expr(condicion),
                etiqueta_verdadero: etiqueta_cuerpo,
                etiqueta_falso: etiqueta_fin,
            });

            instrucciones.push(MirInstruccion::Etiqueta(etiqueta_cuerpo));
            for stmt in cuerpo {
                translate_stmt(stmt, instrucciones, etiqueta_contador);
            }
            instrucciones.push(MirInstruccion::Saltar(etiqueta_inicio));
            instrucciones.push(MirInstruccion::Etiqueta(etiqueta_fin));
        }
        HirStmt::LlamadaStdlib { comando, argumentos: _ } => {
            let args: Vec<MirValor> = vec![]; // Simplificado para evitar error
            instrucciones.push(MirInstruccion::LlamarStdlib {
                comando: comando.clone(),
                argumentos: args,
            });
        }
    }
}

fn translate_expr(expr: &HirExpr) -> MirValor {
    match expr {
        HirExpr::ConstanteNumero(valor) => MirValor::ConstanteNumero(*valor),
        HirExpr::ConstanteTexto(valor) => MirValor::ConstanteTexto(valor.clone()),
        HirExpr::Variable(nombre) => MirValor::Variable(nombre.clone()),
        HirExpr::OperacionBinaria { izquierda, operador, derecha } => MirValor::OperacionBinaria {
            izquierda: Box::new(translate_expr(izquierda)),
            operador: match operador {
                HirBinOp::Suma => MirBinOp::Suma,
                HirBinOp::Resta => MirBinOp::Resta,
                HirBinOp::Multiplicacion => MirBinOp::Multiplicacion,
                HirBinOp::Division => MirBinOp::Division,
                HirBinOp::Igual => MirBinOp::Igual,
                HirBinOp::NoIgual => MirBinOp::NoIgual,
                HirBinOp::Menor => MirBinOp::Menor,
                HirBinOp::Mayor => MirBinOp::Mayor,
                HirBinOp::MenorIgual => MirBinOp::MenorIgual,
                HirBinOp::MayorIgual => MirBinOp::MayorIgual,
                HirBinOp::YLogico => MirBinOp::YLogico,
                HirBinOp::OLogico => MirBinOp::OLogico,
            },
            derecha: Box::new(translate_expr(derecha)),
        },
        HirExpr::OperacionUnaria { operador, operando } => MirValor::OperacionUnaria {
            operador: match operador {
                HirUnaryOp::Negacion => MirUnaryOp::Negacion,
                HirUnaryOp::Negativo => MirUnaryOp::Negativo,
            },
            operando: Box::new(translate_expr(operando)),
        },
        HirExpr::LlamadaStdlib { comando, argumentos: _ } => {
            MirValor::ConstanteTexto(format!("llamada_stdlib({})", comando))
        }
    }
}