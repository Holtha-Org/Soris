use std::collections::HashMap;
use crate::hir::hir::{HirStmt, HirExpr, HirBinOp, HirUnaryOp};

pub struct Interpreter {
    variables: HashMap<String, HirValue>,
}

#[derive(Debug, Clone)]
enum HirValue {
    Numero(f64),
    Texto(String),
    Booleano(bool),
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            variables: HashMap::new(),
        }
    }

    pub fn interpret(&mut self, statements: &[HirStmt]) {
        for stmt in statements {
            self.execute_stmt(stmt);
        }
    }

    fn execute_stmt(&mut self, stmt: &HirStmt) {
        match stmt {
            HirStmt::Firma(_) => {}
            HirStmt::Declaracion { nombre, valor_inicial } => {
                let valor = match valor_inicial {
                    Some(init) => self.evaluate_expr(init),
                    None => HirValue::Numero(0.0),
                };
                self.variables.insert(nombre.clone(), valor);
            }
            HirStmt::Asignacion { nombre, valor } => {
                let resultado = self.evaluate_expr(valor);
                self.variables.insert(nombre.clone(), resultado);
            }
            HirStmt::Impresion(expr) => {
                let valor = self.evaluate_expr(expr);
                match valor {
                    HirValue::Numero(n) => println!("{}", n),
                    HirValue::Texto(s) => println!("{}", s),
                    HirValue::Booleano(b) => println!("{}", b),
                }
            }
            HirStmt::Si { condicion, cuerpo, sino } => {
                let cond = self.evaluate_expr(condicion);
                let es_verdadero = match cond {
                    HirValue::Numero(n) => n != 0.0,
                    HirValue::Texto(_) => true,
                    HirValue::Booleano(b) => b,
                };

                if es_verdadero {
                    for stmt in cuerpo {
                        self.execute_stmt(stmt);
                    }
                } else if let Some(sino_stmts) = sino {
                    for stmt in sino_stmts {
                        self.execute_stmt(stmt);
                    }
                }
            }
            HirStmt::Mientras { condicion, cuerpo } => {
                loop {
                    let cond = self.evaluate_expr(condicion);
                    let es_verdadero = match cond {
                        HirValue::Numero(n) => n != 0.0,
                        HirValue::Texto(_) => true,
                        HirValue::Booleano(b) => b,
                    };

                    if !es_verdadero {
                        break;
                    }

                    for stmt in cuerpo {
                        self.execute_stmt(stmt);
                    }
                }
            }
            HirStmt::LlamadaStdlib { comando, argumentos } => {
                let args: Vec<HirValue> = argumentos.iter()
                    .map(|a| self.evaluate_expr(a))
                    .collect();
                self.execute_stdlib(comando, &args);
            }
        }
    }

    fn execute_stdlib(&mut self, comando: &str, args: &[HirValue]) {
        match comando {
            "consola.limpiar" => {
                print!("\x1B[2J\x1B[1;1H");
            }
            "tiempo.dormir" => {
                if let Some(HirValue::Numero(segundos)) = args.first() {
                    std::thread::sleep(std::time::Duration::from_secs_f64(*segundos));
                }
            }
            _ => {
                println!("Comando stdlib no implementado en intérprete: {}", comando);
            }
        }
    }

    fn evaluate_expr(&self, expr: &HirExpr) -> HirValue {
        match expr {
            HirExpr::ConstanteNumero(valor) => HirValue::Numero(*valor),
            HirExpr::ConstanteTexto(valor) => HirValue::Texto(valor.clone()),
            HirExpr::Variable(nombre) => {
                self.variables.get(nombre).cloned().unwrap_or(HirValue::Numero(0.0))
            }
            HirExpr::OperacionBinaria { izquierda, operador, derecha } => {
                let izq = self.evaluate_expr(izquierda);
                let der = self.evaluate_expr(derecha);

                match (izq, der) {
                    (HirValue::Numero(a), HirValue::Numero(b)) => {
                        let resultado = match operador {
                            HirBinOp::Suma => a + b,
                            HirBinOp::Resta => a - b,
                            HirBinOp::Multiplicacion => a * b,
                            HirBinOp::Division => {
                                if b != 0.0 { a / b } else { 0.0 }
                            }
                            HirBinOp::Igual => return HirValue::Booleano(a == b),
                            HirBinOp::NoIgual => return HirValue::Booleano(a != b),
                            HirBinOp::Menor => return HirValue::Booleano(a < b),
                            HirBinOp::Mayor => return HirValue::Booleano(a > b),
                            HirBinOp::MenorIgual => return HirValue::Booleano(a <= b),
                            HirBinOp::MayorIgual => return HirValue::Booleano(a >= b),
                            HirBinOp::YLogico => return HirValue::Booleano(a != 0.0 && b != 0.0),
                            HirBinOp::OLogico => return HirValue::Booleano(a != 0.0 || b != 0.0),
                        };
                        HirValue::Numero(resultado)
                    }
                    _ => HirValue::Numero(0.0),
                }
            }
            HirExpr::OperacionUnaria { operador, operando } => {
                let op = self.evaluate_expr(operando);
                match (operador, op) {
                    (HirUnaryOp::Negacion, HirValue::Numero(n)) => HirValue::Booleano(n == 0.0),
                    (HirUnaryOp::Negativo, HirValue::Numero(n)) => HirValue::Numero(-n),
                    _ => HirValue::Numero(0.0),
                }
            }
            HirExpr::LlamadaStdlib { .. } => HirValue::Numero(0.0),
        }
    }
}