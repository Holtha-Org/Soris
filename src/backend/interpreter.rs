use crate::ast::{Expr, Stmt, Program, Literal, BinOp};
use crate::utils::errors::SorisError;
use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;

/// Valor dinámico en tiempo de ejecución
#[derive(Debug, Clone)]
pub enum Valor {
    Entero(i64),
    Flotante(f64),
    Booleano(bool),
    Texto(String),
    Caracter(char),
    Nada,
    Vector(Vec<Valor>),
}

impl std::fmt::Display for Valor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Valor::Entero(v) => write!(f, "{}", v),
            Valor::Flotante(v) => write!(f, "{}", v),
            Valor::Booleano(v) => write!(f, "{}", v),
            Valor::Texto(v) => write!(f, "{}", v),
            Valor::Caracter(v) => write!(f, "'{}'", v),
            Valor::Nada => write!(f, "nada"),
            Valor::Vector(v) => {
                let strs: Vec<String> = v.iter().map(|val| val.to_string()).collect();
                write!(f, "[{}]", strs.join(", "))
            },
        }
    }
}

type Entorno = Rc<RefCell<HashMap<String, Valor>>>;

pub struct Interprete {
    global: Entorno,
    salida: String,
}

impl Interprete {
    pub fn nuevo() -> Self {
        Interprete {
            global: Rc::new(RefCell::new(HashMap::new())),
            salida: String::new(),
        }
    }

    /// Ejecuta un programa completo (.sr)
    pub fn ejecutar(&mut self, programa: &Program) -> Result<(), SorisError> {
        // Buscar función 'inicio' (equivalente a main en Rust)
        let mut main_found = false;
        for stmt in &programa.sentencias {
            if let Stmt::Funcion { nombre, cuerpo, .. } = stmt {
                if nombre == "inicio" {
                    main_found = true;
                    self.ejecutar_bloque(cuerpo, Rc::clone(&self.global))?;
                    break;
                }
            }
        }

        if !main_found {
            return Err(SorisError::new("No se encontró la función 'inicio()'"));
        }

        Ok(())
    }

    fn ejecutar_bloque(&mut self, bloque: &[Stmt], entorno: Entorno) -> Result<Option<Valor>, SorisError> {
        for stmt in bloque {
            match self.ejecutar_stmt(stmt, Rc::clone(&entorno)) {
                Ok(Some(val)) => return Ok(Some(val)), // Return temprano
                Ok(None) => continue,
                Err(e) => return Err(e),
            }
        }
        Ok(None)
    }

    fn ejecutar_stmt(&mut self, stmt: &Stmt, entorno: Entorno) -> Result<Option<Valor>, SorisError> {
        match stmt {
            Stmt::Declaracion { nombre, inicializador, .. } => {
                let valor = if let Some(expr) = inicializador {
                    self.evaluar_expr(expr, Rc::clone(&entorno))?
                } else {
                    Valor::Nada
                };
                entorno.borrow_mut().insert(nombre.clone(), valor);
                Ok(None)
            },
            
            Stmt::Asignacion { objetivo, valor, .. } => {
                let val_evaluado = self.evaluar_expr(valor, Rc::clone(&entorno))?;
                if let Expr::Identificador(nombre) = objetivo {
                    entorno.borrow_mut().insert(nombre.clone(), val_evaluado);
                } else {
                    return Err(SorisError::new("Asignación inválida"));
                }
                Ok(None)
            },

            Stmt::Expresion(expr) => {
                self.evaluar_expr(expr, Rc::clone(&entorno))?;
                Ok(None)
            },

            Stmt::Si { condicion, entonces, sino } => {
                let cond_val = self.evaluar_expr(condicion, Rc::clone(&entorno))?;
                if let Valor::Booleano(true) = cond_val {
                    self.ejecutar_bloque(entonces, Rc::clone(&entorno))?;
                } else if let Some(sino_block) = sino {
                    self.ejecutar_bloque(sino_block, Rc::clone(&entorno))?;
                }
                Ok(None)
            },

            Stmt::Mientras { condicion, cuerpo } => {
                loop {
                    let cond_val = self.evaluar_expr(condicion, Rc::clone(&entorno))?;
                    if let Valor::Booleano(true) = cond_val {
                        if let Some(_) = self.ejecutar_bloque(cuerpo, Rc::clone(&entorno))? {
                            break; 
                        }
                    } else {
                        break;
                    }
                }
                Ok(None)
            },

            Stmt::Para { variable, iterable, cuerpo } => {
                // Simplificado: asume rango numérico
                if let Expr::Rango { inicio, fin } = iterable.as_ref() {
                    let start = self.evaluar_expr(inicio, Rc::clone(&entorno))?;
                    let end = self.evaluar_expr(fin, Rc::clone(&entorno))?;
                    
                    if let (Valor::Entero(s), Valor::Entero(e)) = (start, end) {
                        for i in s..e {
                            entorno.borrow_mut().insert(variable.clone(), Valor::Entero(i));
                            if let Some(_) = self.ejecutar_bloque(cuerpo, Rc::clone(&entorno))? {
                                break;
                            }
                        }
                    }
                }
                Ok(None)
            },

            Stmt::Retorno { valor } => {
                let val = if let Some(expr) = valor {
                    self.evaluar_expr(expr, Rc::clone(&entorno))?
                } else {
                    Valor::Nada
                };
                Ok(Some(val))
            },

            Stmt::Di { argumentos } => {
                // Implementación nativa de 'di!' (println!)
                let mut mensaje = String::new();
                for (i, arg) in argumentos.iter().enumerate() {
                    let val = self.evaluar_expr(arg, Rc::clone(&entorno))?;
                    if i > 0 { mensaje.push(' '); }
                    mensaje.push_str(&val.to_string());
                }
                println!("{}", mensaje);
                self.salida.push_str(&mensaje);
                self.salida.push('\n');
                Ok(None)
            },

            Stmt::Coin { expresion, brazos } => {
                // Pattern matching simplificado
                let val = self.evaluar_expr(expresion, Rc::clone(&entorno))?;
                for brazo in brazos {
                    if let Stmt::BrazoCoin { patron, guarda, cuerpo } = brazo {
                        // Simplificación: match exacto o wildcard
                        let coincide = match patron {
                            Expr::Literal(Literal::Nada) => val == Valor::Nada,
                            Expr::Literal(Literal::Booleano(b)) => val == Valor::Booleano(*b),
                            Expr::Literal(Literal::Entero(n)) => val == Valor::Entero(*n),
                            Expr::Literal(Literal::Flotante(f)) => val == Valor::Flotante(*f),
                            Expr::Literal(Literal::Texto(s)) => val == Valor::Texto(s.clone()),
                            Expr::Identificador(_) => true, // Wildcard
                            _ => false,
                        };

                        if coincide {
                            if let Some(guarda_expr) = guarda {
                                let guard_val = self.evaluar_expr(guarda_expr, Rc::clone(&entorno))?;
                                if let Valor::Booleano(true) = guard_val {
                                    self.ejecutar_bloque(cuerpo, Rc::clone(&entorno))?;
                                    break;
                                }
                            } else {
                                self.ejecutar_bloque(cuerpo, Rc::clone(&entorno))?;
                                break;
                            }
                        }
                    }
                }
                Ok(None)
            },

            _ => Ok(None)
        }
    }

    fn evaluar_expr(&self, expr: &Expr, entorno: Entorno) -> Result<Valor, SorisError> {
        match expr {
            Expr::Literal(lit) => {
                match lit {
                    Literal::Entero(v) => Ok(Valor::Entero(*v)),
                    Literal::Flotante(v) => Ok(Valor::Flotante(*v)),
                    Literal::Texto(s) => Ok(Valor::Texto(s.clone())),
                    Literal::Booleano(b) => Ok(Valor::Booleano(*b)),
                    Literal::Nada => Ok(Valor::Nada),
                    Literal::Caracter(c) => Ok(Valor::Caracter(*c)),
                    _ => Err(SorisError::new("Literal no soportado"))
                }
            },
            Expr::Identificador(nombre) => {
                let env_lock = entorno.borrow();
                if let Some(val) = env_lock.get(nombre) {
                    Ok(val.clone())
                } else {
                    Err(SorisError::new(&format!("Variable '{}' no definida", nombre)))
                }
            },
            Expr::Binaria { izquierda, operador, derecha } => {
                let izq = self.evaluar_expr(izquierda, Rc::clone(&entorno))?;
                let der = self.evaluar_expr(derecha, Rc::clone(&entorno))?;

                match (izq, der) {
                    (Valor::Entero(a), Valor::Entero(b)) => {
                        match operador {
                            BinOp::Suma => Ok(Valor::Entero(a + b)),
                            BinOp::Resta => Ok(Valor::Entero(a - b)),
                            BinOp::Multiplicacion => Ok(Valor::Entero(a * b)),
                            BinOp::Division => if b != 0 { Ok(Valor::Entero(a / b)) } else { Err(SorisError::new("División por cero")) },
                            BinOp::Modulo => Ok(Valor::Entero(a % b)),
                            BinOp::Igual => Ok(Valor::Booleano(a == b)),
                            BinOp::NoIgual => Ok(Valor::Booleano(a != b)),
                            BinOp::Menor => Ok(Valor::Booleano(a < b)),
                            BinOp::Mayor => Ok(Valor::Booleano(a > b)),
                            BinOp::MenorIgual => Ok(Valor::Booleano(a <= b)),
                            BinOp::MayorIgual => Ok(Valor::Booleano(a >= b)),
                            _ => Err(SorisError::new("Operador no válido para enteros"))
                        }
                    },
                    (Valor::Flotante(a), Valor::Flotante(b)) => {
                        match operador {
                            BinOp::Suma => Ok(Valor::Flotante(a + b)),
                            BinOp::Resta => Ok(Valor::Flotante(a - b)),
                            BinOp::Multiplicacion => Ok(Valor::Flotante(a * b)),
                            BinOp::Division => Ok(Valor::Flotante(a / b)),
                            BinOp::Igual => Ok(Valor::Booleano((a - b).abs() < f64::EPSILON)),
                            BinOp::Menor => Ok(Valor::Booleano(a < b)),
                            BinOp::Mayor => Ok(Valor::Booleano(a > b)),
                            _ => Err(SorisError::new("Operador no válido para flotantes"))
                        }
                    },
                    (Valor::Texto(a), Valor::Texto(b)) => {
                        match operador {
                            BinOp::Suma => Ok(Valor::Texto(format!("{}{}", a, b))),
                            _ => Err(SorisError::new("Solo '+' soportado para texto"))
                        }
                    },
                    _ => Err(SorisError::new("Tipos incompatibles en operación binaria"))
                }
            },
            Expr::Llamada { nombre, argumentos } => {
                // Nativas hardcodeadas para el MVP
                match nombre.as_str() {
                    "raiz_cuadrada" => {
                        if argumentos.len() != 1 { return Err(SorisError::new("raiz_cuadrada requiere 1 arg")); }
                        let val = self.evaluar_expr(&argumentos[0], Rc::clone(&entorno))?;
                        if let Valor::Flotante(f) = val {
                            return Ok(Valor::Flotante(f.sqrt()));
                        }
                        return Err(SorisError::new("raiz_cuadrada requiere flotante"));
                    },
                    "generar_entre" => {
                        use rand::Rng;
                        if argumentos.len() != 2 { return Err(SorisError::new("generar_entre requiere 2 args")); }
                        let a = self.evaluar_expr(&argumentos[0], Rc::clone(&entorno))?;
                        let b = self.evaluar_expr(&argumentos[1], Rc::clone(&entorno))?;
                        if let (Valor::Entero(min), Valor::Entero(max)) = (a, b) {
                            let mut rng = rand::thread_rng();
                            return Ok(Valor::Entero(rng.gen_range(min..=max)));
                        }
                        return Err(SorisError::new("generar_entre requiere enteros"));
                    },
                    "limpiar" => {
                        print!("\x1B[2J\x1B[1;1H");
                        return Ok(Valor::Nada);
                    },
                    "dormir" => {
                        if argumentos.len() != 1 { return Err(SorisError::new("dormir requiere 1 arg")); }
                        let val = self.evaluar_expr(&argumentos[0], Rc::clone(&entorno))?;
                        if let Valor::Flotante(segs) = val {
                            std::thread::sleep(std::time::Duration::from_secs_f64(segs));
                            return Ok(Valor::Nada);
                        }
                        return Err(SorisError::new("dormir requiere flotante"));
                    },
                    _ => {}
                }
                
                Err(SorisError::new(&format!("Función '{}' no encontrada o no implementada en intérprete", nombre)))
            },
            Expr::Rango { inicio, fin } => {
                // Los rangos se evalúan en el contexto del for
                Ok(Valor::Nada)
            },
            _ => Err(SorisError::new("Expresión no soportada en intérprete"))
        }
    }

    pub fn obtener_salida(&self) -> &str {
        &self.salida
    }
}