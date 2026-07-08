use crate::mir::mir::{MirInstruccion, MirValor, MirBinOp, MirUnaryOp};
use crate::stdlib::GestorStdlib;

pub struct Compiler {
    indentacion: usize,
    output: String,
    gestor_stdlib: GestorStdlib,
}

impl Compiler {
    pub fn new() -> Self {
        Self {
            indentacion: 0,
            output: String::new(),
            gestor_stdlib: GestorStdlib::new(),
        }
    }

    pub fn compile(&mut self, instructions: &[MirInstruccion]) -> String {
        self.emit_line("fn main() {");
        self.indentacion += 1;
        
        // Añadir imports necesarios
        self.emit_line("use std::io::{self, Write};");
        self.emit_line("");

        for inst in instructions {
            self.compile_instruction(inst);
        }

        self.indentacion -= 1;
        self.emit_line("}");

        self.output.clone()
    }

    fn emit_line(&mut self, line: &str) {
        if line.is_empty() {
            self.output.push('\n');
        } else {
            for _ in 0..self.indentacion {
                self.output.push_str("    ");
            }
            self.output.push_str(line);
            self.output.push('\n');
        }
    }

    fn compile_instruction(&mut self, inst: &MirInstruccion) {
        match inst {
            MirInstruccion::Firma(autor) => {
                self.emit_line(&format!("// Programa de {}", autor));
            }
            MirInstruccion::DeclararVariable { nombre, valor_inicial } => {
                let valor = match valor_inicial {
                    Some(v) => self.compile_valor(v),
                    None => "0.0".to_string(),
                };
                self.emit_line(&format!("let mut {} = {};", nombre, valor));
            }
            MirInstruccion::AsignarVariable { nombre, valor } => {
                let val = self.compile_valor(valor);
                self.emit_line(&format!("{} = {};", nombre, val));
            }
            MirInstruccion::Imprimir(valor) => {
                let val = self.compile_valor(valor);
                self.emit_line(&format!("println!(\"{{}}\", {});", val));
            }
            MirInstruccion::SaltarSi { condicion, etiqueta_verdadero, etiqueta_falso } => {
                let cond = self.compile_valor(condicion);
                self.emit_line(&format!(
                    "if {} != 0.0 {{ goto label_{}; }} else {{ goto label_{}; }}",
                    cond, etiqueta_verdadero, etiqueta_falso
                ));
            }
            MirInstruccion::Etiqueta(etiqueta) => {
                self.indentacion = self.indentacion.saturating_sub(1);
                self.emit_line(&format!("// label_{}:", etiqueta));
                self.indentacion += 1;
            }
            MirInstruccion::Saltar(etiqueta) => {
                self.emit_line(&format!("goto label_{};", etiqueta));
            }
            MirInstruccion::LlamarStdlib { comando, argumentos } => {
                let args: Vec<String> = argumentos.iter()
                    .map(|a| self.compile_valor(a))
                    .collect();

                if let Some(codigo) = self.gestor_stdlib.generar_llamada(comando, &args) {
                    self.emit_line(&codigo);
                } else {
                    self.emit_line(&format!(
                        "// Comando stdlib no implementado: {}({})",
                        comando,
                        args.join(", ")
                    ));
                }
            }
        }
    }

    fn compile_valor(&self, valor: &MirValor) -> String {
        match valor {
            MirValor::ConstanteNumero(n) => {
                if *n == n.floor() && n.is_finite() {
                    format!("{}", n)
                } else {
                    format!("{}", n)
                }
            }
            MirValor::ConstanteTexto(s) => {
                format!("\"{}\"", s.escape_default())
            }
            MirValor::Variable(nombre) => nombre.clone(),
            MirValor::OperacionBinaria { izquierda, operador, derecha } => {
                let izq = self.compile_valor(izquierda);
                let der = self.compile_valor(derecha);
                let op = match operador {
                    MirBinOp::Suma => "+",
                    MirBinOp::Resta => "-",
                    MirBinOp::Multiplicacion => "*",
                    MirBinOp::Division => "/",
                    MirBinOp::Igual => "==",
                    MirBinOp::NoIgual => "!=",
                    MirBinOp::Menor => "<",
                    MirBinOp::Mayor => ">",
                    MirBinOp::MenorIgual => "<=",
                    MirBinOp::MayorIgual => ">=",
                    MirBinOp::YLogico => "&&",
                    MirBinOp::OLogico => "||",
                };
                format!("({} {} {})", izq, op, der)
            }
            MirValor::OperacionUnaria { operador, operando } => {
                let op = self.compile_valor(operando);
                match operador {
                    MirUnaryOp::Negacion => format!("({} == 0.0)", op),
                    MirUnaryOp::Negativo => format!("(-{})", op),
                }
            }
        }
    }
}