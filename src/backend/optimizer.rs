use crate::mir::mir::{MirInstruccion, MirValor};

pub struct Optimizer {
    nivel: u8,
}

impl Optimizer {
    pub fn new() -> Self {
        Self { nivel: 1 }
    }

    /// Optimizar MIR con un nivel específico (0-3)
    pub fn optimizar(&mut self, instructions: Vec<MirInstruccion>, nivel: u8) -> Vec<MirInstruccion> {
        self.nivel = nivel.min(3);
        
        let mut result = instructions;
        
        // Nivel 1: Plegado de constantes básico
        if self.nivel >= 1 {
            result = self.constant_folding(&result);
        }
        
        // Nivel 2: Eliminación de código muerto
        if self.nivel >= 2 {
            result = self.dead_code_elimination(&result);
        }
        
        // Nivel 3: Optimizaciones avanzadas
        if self.nivel >= 3 {
            result = self.advanced_optimizations(&result);
        }
        
        result
    }

    fn constant_folding(&self, instructions: &[MirInstruccion]) -> Vec<MirInstruccion> {
        instructions.iter().map(|inst| self.optimize_instruction(inst)).collect()
    }

    fn dead_code_elimination(&self, instructions: &[MirInstruccion]) -> Vec<MirInstruccion> {
        // Eliminar etiquetas inalcanzables y saltos redundantes
        let mut result = Vec::new();
        let mut i = 0;
        
        while i < instructions.len() {
            match &instructions[i] {
                // Eliminar salto incondicional seguido de etiqueta inmediata
                MirInstruccion::Saltar(etiqueta) => {
                    if i + 1 < instructions.len() {
                        if let MirInstruccion::Etiqueta(sig_etiqueta) = &instructions[i + 1] {
                            if etiqueta == sig_etiqueta {
                                // Salto redundante, lo eliminamos
                                i += 1;
                                continue;
                            }
                        }
                    }
                    result.push(instructions[i].clone());
                }
                _ => result.push(instructions[i].clone()),
            }
            i += 1;
        }
        
        result
    }

    fn advanced_optimizations(&self, instructions: &[MirInstruccion]) -> Vec<MirInstruccion> {
        // Optimizaciones avanzadas: inline de funciones simples, 
        // propagación de copias, etc.
        // Por ahora, aplicamos dead code elimination dos veces
        let mut result = self.dead_code_elimination(instructions);
        result = self.dead_code_elimination(&result);
        result
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

impl Default for Optimizer {
    fn default() -> Self {
        Self::new()
    }
}