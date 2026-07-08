use crate::mir::mir::{MirInstruccion, MirValor};

pub struct Optimizer;

impl Optimizer {
    pub fn new() -> Self {
        Self
    }

    pub fn optimize(&self, instructions: &[MirInstruccion]) -> Vec<MirInstruccion> {
        self.constant_folding(instructions)
    }

    fn constant_folding(&self, instructions: &[MirInstruccion]) -> Vec<MirInstruccion> {
        instructions.iter().map(|inst| self.optimize_instruction(inst)).collect()
    }

    fn optimize_instruction(&self, inst: &MirInstruccion) -> MirInstruccion {
        match inst {
            MirInstruccion::Imprimir(valor) => {
                MirInstruccion::Imprimir(self.optimize_valor(valor))
            }
            MirInstruccion::AsignarVariable { nombre, valor } => {
                MirInstruccion::AsignarVariable {
                    nombre: nombre.clone(),
                    valor: self.optimize_valor(valor),
                }
            }
            MirInstruccion::DeclararVariable { nombre, valor_inicial } => {
                MirInstruccion::DeclararVariable {
                    nombre: nombre.clone(),
                    valor_inicial: valor_inicial.as_ref().map(|v| self.optimize_valor(v)),
                }
            }
            MirInstruccion::SaltarSi { condicion, etiqueta_verdadero, etiqueta_falso } => {
                MirInstruccion::SaltarSi {
                    condicion: self.optimize_valor(condicion),
                    etiqueta_verdadero: *etiqueta_verdadero,
                    etiqueta_falso: *etiqueta_falso,
                }
            }
            _ => inst.clone(),
        }
    }

    fn optimize_valor(&self, valor: &MirValor) -> MirValor {
        match valor {
            MirValor::OperacionBinaria { izquierda, operador, derecha } => {
                let izq_opt = self.optimize_valor(izquierda);
                let der_opt = self.optimize_valor(derecha);

                // Plegado de constantes
                match (&izq_opt, &der_opt) {
                    (MirValor::ConstanteNumero(a), MirValor::ConstanteNumero(b)) => {
                        let resultado = match operador {
                            crate::mir::mir::MirBinOp::Suma => a + b,
                            crate::mir::mir::MirBinOp::Resta => a - b,
                            crate::mir::mir::MirBinOp::Multiplicacion => a * b,
                            crate::mir::mir::MirBinOp::Division => {
                                if *b != 0.0 { a / b } else { 0.0 }
                            }
                            _ => return MirValor::OperacionBinaria {
                                izquierda: Box::new(izq_opt),
                                operador: operador.clone(),
                                derecha: Box::new(der_opt),
                            },
                        };
                        MirValor::ConstanteNumero(resultado)
                    }
                    _ => MirValor::OperacionBinaria {
                        izquierda: Box::new(izq_opt),
                        operador: operador.clone(),
                        derecha: Box::new(der_opt),
                    },
                }
            }
            _ => valor.clone(),
        }
    }
}