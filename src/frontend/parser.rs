use crate::utils::token::Token;
use crate::utils::span::Span;
use crate::utils::errors::ErrorCompilador;
use crate::ast::expr::{Expr, BinOp, UnaryOp, Patron, BrazoMatch};
use crate::ast::stmt::{Stmt, Funcion, VarianteEnum, FirmaMetodo};
use crate::ast::types::Tipo;
use crate::frontend::lexer::Lexer;

pub struct Parser {
    tokens: Vec<(Token, Span)>,
    posicion: usize,
}

impl Parser {
    pub fn new(entrada: &str) -> Result<Self, ErrorCompilador> {
        let mut lexer = Lexer::new(entrada);
        let tokens = lexer.tokenize()?;
        Ok(Self {
            tokens,
            posicion: 0,
        })
    }

    fn current_token(&self) -> &Token {
        &self.tokens[self.posicion].0
    }

    fn current_span(&self) -> Span {
        self.tokens[self.posicion].1
    }

    fn peek_token(&self) -> &Token {
        if self.posicion + 1 < self.tokens.len() {
            &self.tokens[self.posicion + 1].0
        } else {
            &Token::EOF
        }
    }

    fn advance(&mut self) {
        if self.posicion < self.tokens.len() - 1 {
            self.posicion += 1;
        }
    }

    fn expect(&mut self, expected: Token) -> Result<Span, ErrorCompilador> {
        if std::mem::discriminant(self.current_token()) == std::mem::discriminant(&expected) {
            let span = self.current_span();
            self.advance();
            Ok(span)
        } else {
            Err(ErrorCompilador::new(
                &format!("Se esperaba {}, pero se encontró {}", expected, self.current_token())
            ))
        }
    }

    fn check(&self, token: &Token) -> bool {
        std::mem::discriminant(self.current_token()) == std::mem::discriminant(token)
    }

    fn eat(&mut self, token: Token) -> Result<(), ErrorCompilador> {
        if self.check(&token) {
            self.advance();
            Ok(())
        } else {
            Err(ErrorCompilador::new(&format!("Se esperaba {}", token)))
        }
    }

    pub fn parse(&mut self) -> Result<Vec<Stmt>, ErrorCompilador> {
        let mut statements = Vec::new();

        while *self.current_token() != Token::EOF {
            statements.push(self.parse_statement()?);
        }

        Ok(statements)
    }

    fn parse_statement(&mut self) -> Result<Stmt, ErrorCompilador> {
        let span = self.current_span();
        
        match self.current_token() {
            // Declaraciones
            Token::Vr | Token::Mut => self.parse_var_decl(),
            Token::Const => self.parse_const_decl(),
            Token::Estatico => self.parse_static_decl(),
            Token::Fn => self.parse_fn_decl(),
            Token::Struct => self.parse_struct_decl(),
            Token::Enum => self.parse_enum_decl(),
            Token::Rasgo => self.parse_trait_decl(),
            Token::Impl => self.parse_impl_decl(),
            Token::Pub => self.parse_pub_decl(),
            
            // Control de flujo
            Token::Si => self.parse_if(),
            Token::Coin => self.parse_coin(),
            Token::Mientras => self.parse_while(),
            Token::Para => self.parse_for(),
            Token::Buc => self.parse_buc(),
            
            // Control dentro de loops
            Token::Retorna => self.parse_return_stmt(),
            Token::Rom => self.parse_break_stmt(),
            Token::Cont => self.parse_continue_stmt(),
            
            // Macros y atributos
            Token::Di => self.parse_di_macro(),
            
            // Expresiones como statements
            _ => self.parse_expr_stmt(),
        }
    }

    // ============================================
    // DECLARACIONES
    // ============================================

    fn parse_var_decl(&mut self) -> Result<Stmt, ErrorCompilador> {
        let span = self.current_span();
        self.advance(); // Consumir 'vr'

        let mut es_mut = false;
        if self.check(&Token::Mut) {
            self.advance();
            es_mut = true;
        }

        let nombre = if let Token::Identificador(ref name) = self.current_token() {
            name.clone()
        } else {
            return Err(ErrorCompilador::new("Se esperaba un identificador después de 'vr'"));
        };
        self.advance();

        // Tipo opcional
        let tipo_opcional = if self.check(&Token::DosPuntos) {
            self.advance();
            Some(self.parse_tipo()?)
        } else {
            None
        };

        // Valor inicial (requerido)
        self.expect(Token::Igual)?;
        let valor_inicial = Some(self.parse_expression()?);

        self.expect(Token::PuntoYComa)?;

        if es_mut {
            Ok(Stmt::SeaMut {
                nombre,
                tipo_opcional,
                valor_inicial,
                span,
            })
        } else {
            Ok(Stmt::Sea {
                nombre,
                tipo_opcional,
                valor_inicial,
                span,
            })
        }
    }

    fn parse_const_decl(&mut self) -> Result<Stmt, ErrorCompilador> {
        let span = self.current_span();
        self.advance(); // Consumir 'const'

        let nombre = if let Token::Identificador(ref name) = self.current_token() {
            name.clone()
        } else {
            return Err(ErrorCompilador::new("Se esperaba un identificador después de 'const'"));
        };
        self.advance();

        // Tipo requerido para const
        self.expect(Token::DosPuntos)?;
        let tipo = self.parse_tipo()?;

        self.expect(Token::Igual)?;
        let valor = self.parse_expression()?;
        self.expect(Token::PuntoYComa)?;

        Ok(Stmt::Const {
            nombre,
            tipo: tipo.nombre().to_string(),
            valor,
            span,
        })
    }

    fn parse_static_decl(&mut self) -> Result<Stmt, ErrorCompilador> {
        let span = self.current_span();
        self.advance(); // Consumir 'estatico'

        let mutable = if self.check(&Token::Mut) {
            self.advance();
            true
        } else {
            false
        };

        let nombre = if let Token::Identificador(ref name) = self.current_token() {
            name.clone()
        } else {
            return Err(ErrorCompilador::new("Se esperaba un identificador después de 'estatico'"));
        };
        self.advance();

        self.expect(Token::DosPuntos)?;
        let tipo = self.parse_tipo()?;

        self.expect(Token::Asignacion)?;
        let valor = self.parse_expression()?;
        self.expect(Token::PuntoYComa)?;

        Ok(Stmt::Estatico {
            nombre,
            tipo,
            valor,
            mutable,
            span,
        })
    }

    // ============================================
    // FUNCIONES, STRUCTS, ENUMS, TRAITS, IMPL
    // ============================================

    fn parse_fn_decl(&mut self) -> Result<Stmt, ErrorCompilador> {
        let span = self.current_span();
        self.advance(); // Consumir 'fn'

        let es_pub = if self.check(&Token::Pub) {
            self.advance();
            true
        } else {
            false
        };

        let nombre = if let Token::Identificador(ref name) = self.current_token() {
            name.clone()
        } else {
            return Err(ErrorCompilador::new("Se esperaba un identificador después de 'fn'"));
        };
        self.advance();

        // Parámetros genéricos opcionales <T, U>
        let mut generico = Vec::new();
        if self.check(&Token::Menor) {
            self.advance();
            while !self.check(&Token::Mayor) {
                if let Token::Identificador(ref name) = self.current_token() {
                    generico.push(name.clone());
                }
                self.advance();
                if self.check(&Token::Coma) {
                    self.advance();
                }
            }
            self.expect(Token::Mayor)?;
        }

        // Parámetros de la función
        self.expect(Token::ParIzq)?;
        let mut parametros = Vec::new();
        if !self.check(&Token::ParDer) {
            loop {
                let param_nombre = if let Token::Identificador(ref name) = self.current_token() {
                    name.clone()
                } else {
                    return Err(ErrorCompilador::new("Se esperaba un identificador para el parámetro"));
                };
                self.advance();

                self.expect(Token::DosPuntos)?;
                let param_tipo = self.parse_tipo()?;

                parametros.push((param_nombre, param_tipo));

                if !self.check(&Token::Coma) {
                    break;
                }
                self.advance();
            }
        }
        self.expect(Token::ParDer)?;

        // Tipo de retorno opcional
        let retorno = if self.check(&Token::Flecha) {
            self.advance();
            Some(self.parse_tipo()?)
        } else {
            None
        };

        // Cláusulas where (simplificado - por implementar)
        let donde = Vec::new();

        // Cuerpo de la función
        let cuerpo = if self.check(&Token::LlaveIzq) {
            self.parse_bloque()?
        } else {
            self.expect(Token::PuntoYComa)?;
            Vec::new()
        };

        Ok(Stmt::Funcion {
            firma: Funcion {
                nombre,
                parametros,
                retorno: retorno.map(|t| t.nombre().to_string()),
                generico,
                donde,
                cuerpo,
                es_seguro: true,
                es_async: false,
            },
            span,
        })
    }

    fn parse_struct_decl(&mut self) -> Result<Stmt, ErrorCompilador> {
        let span = self.current_span();
        self.advance(); // Consumir 'struct'

        let nombre = if let Token::Identificador(ref name) = self.current_token() {
            name.clone()
        } else {
            return Err(ErrorCompilador::new("Se esperaba un identificador después de 'struct'"));
        };
        self.advance();

        // Campos del struct
        self.expect(Token::LlaveIzq)?;
        let mut campos = Vec::new();
        while !self.check(&Token::LlaveDer) {
            let campo_nombre = if let Token::Identificador(ref name) = self.current_token() {
                name.clone()
            } else {
                return Err(ErrorCompilador::new("Se esperaba un identificador para el campo"));
            };
            self.advance();

            self.expect(Token::DosPuntos)?;
            let campo_tipo = self.parse_tipo()?;

            campos.push((campo_nombre, campo_tipo.nombre().to_string()));

            if !self.check(&Token::Coma) && !self.check(&Token::LlaveDer) {
                self.expect(Token::Coma)?;
            }
            if self.check(&Token::Coma) {
                self.advance();
            }
        }
        self.expect(Token::LlaveDer)?;

        Ok(Stmt::Estructura {
            nombre,
            campos,
            span,
        })
    }

    fn parse_enum_decl(&mut self) -> Result<Stmt, ErrorCompilador> {
        let span = self.current_span();
        self.advance(); // Consumir 'enum'

        let nombre = if let Token::Identificador(ref name) = self.current_token() {
            name.clone()
        } else {
            return Err(ErrorCompilador::new("Se esperaba un identificador después de 'enum'"));
        };
        self.advance();

        // Variantes del enum
        self.expect(Token::LlaveIzq)?;
        let mut variantes = Vec::new();
        while !self.check(&Token::LlaveDer) {
            let variante_nombre = if let Token::Identificador(ref name) = self.current_token() {
                name.clone()
            } else {
                return Err(ErrorCompilador::new("Se esperaba un identificador para la variante"));
            };
            self.advance();

            // Campos opcionales de la variante (tuple-like o struct-like)
            let mut campos = None;
            if self.check(&Token::ParIzq) {
                self.advance();
                let mut variant_campos = Vec::new();
                if !self.check(&Token::ParDer) {
                    loop {
                        let campo_tipo = self.parse_tipo()?;
                        variant_campos.push(campo_tipo.nombre().to_string());
                        if !self.check(&Token::Coma) {
                            break;
                        }
                        self.advance();
                    }
                }
                self.expect(Token::ParDer)?;
                campos = Some(variant_campos);
            }

            variantes.push(VarianteEnum {
                nombre: variante_nombre,
                campos,
                valores: None,
            });

            if !self.check(&Token::Coma) && !self.check(&Token::LlaveDer) {
                self.expect(Token::Coma)?;
            }
            if self.check(&Token::Coma) {
                self.advance();
            }
        }
        self.expect(Token::LlaveDer)?;

        Ok(Stmt::Enumeracion {
            nombre,
            variantes,
            span,
        })
    }

    fn parse_trait_decl(&mut self) -> Result<Stmt, ErrorCompilador> {
        let span = self.current_span();
        self.advance(); // Consumir 'rasgo'

        let nombre = if let Token::Identificador(ref name) = self.current_token() {
            name.clone()
        } else {
            return Err(ErrorCompilador::new("Se esperaba un identificador después de 'rasgo'"));
        };
        self.advance();

        // Métodos del trait
        self.expect(Token::LlaveIzq)?;
        let mut metodos = Vec::new();
        while !self.check(&Token::LlaveDer) {
            // Parsear firma de método simplificada
            let metodo_nombre = if let Token::Identificador(ref name) = self.current_token() {
                name.clone()
            } else {
                return Err(ErrorCompilador::new("Se esperaba un identificador para el método"));
            };
            self.advance();

            self.expect(Token::ParIzq)?;
            let mut parametros = Vec::new();
            if !self.check(&Token::ParDer) {
                // Parsear parámetros del método
            }
            self.expect(Token::ParDer)?;

            let retorno = if self.check(&Token::Flecha) {
                self.advance();
                Some(self.parse_tipo()?.nombre().to_string())
            } else {
                None
            };

            let tiene_cuerpo = self.check(&Token::LlaveIzq);
            if tiene_cuerpo {
                self.parse_bloque()?;
            } else {
                self.expect(Token::PuntoYComa)?;
            }

            metodos.push(FirmaMetodo {
                nombre: metodo_nombre,
                parametros,
                retorno,
                tiene_cuerpo,
            });
        }
        self.expect(Token::LlaveDer)?;

        Ok(Stmt::Trait {
            nombre,
            metodos,
            span,
        })
    }

    fn parse_impl_decl(&mut self) -> Result<Stmt, ErrorCompilador> {
        let span = self.current_span();
        self.advance(); // Consumir 'impl'

        let tipo_nombre = if let Token::Identificador(ref name) = self.current_token() {
            name.clone()
        } else {
            return Err(ErrorCompilador::new("Se esperaba un identificador después de 'impl'"));
        };
        self.advance();

        // Métodos del impl
        self.expect(Token::LlaveIzq)?;
        let mut metodos = Vec::new();
        while !self.check(&Token::LlaveDer) {
            // Parsear método como función pero dentro de impl
            let metodo_span = self.current_span();
            self.eat(Token::Fn)?;

            let nombre = if let Token::Identificador(ref name) = self.current_token() {
                name.clone()
            } else {
                return Err(ErrorCompilador::new("Se esperaba un identificador para el método"));
            };
            self.advance();

            self.expect(Token::ParIzq)?;
            let mut parametros = Vec::new();
            if !self.check(&Token::ParDer) {
                loop {
                    let param_nombre = if let Token::Identificador(ref name) = self.current_token() {
                        name.clone()
                    } else {
                        break;
                    };
                    self.advance();

                    self.expect(Token::DosPuntos)?;
                    let param_tipo = self.parse_tipo()?;
                    parametros.push((param_nombre, param_tipo.nombre().to_string()));

                    if !self.check(&Token::Coma) {
                        break;
                    }
                    self.advance();
                }
            }
            self.expect(Token::ParDer)?;

            let retorno = if self.check(&Token::Flecha) {
                self.advance();
                Some(self.parse_tipo()?.nombre().to_string())
            } else {
                None
            };

            let cuerpo = self.parse_bloque()?;

            metodos.push(Funcion {
                nombre,
                parametros,
                retorno,
                generico: Vec::new(),
                donde: Vec::new(),
                cuerpo,
                es_seguro: true,
                es_async: false,
            });
        }
        self.expect(Token::LlaveDer)?;

        Ok(Stmt::Implementacion {
            tipo_nombre,
            metodos,
            span,
        })
    }

    fn parse_pub_decl(&mut self) -> Result<Stmt, ErrorCompilador> {
        let _span = self.current_span();
        self.advance(); // Consumir 'pub'
        
        // Ahora parsear la declaración que sigue
        match self.current_token() {
            Token::Fn => self.parse_fn_decl(),
            Token::Struct => self.parse_struct_decl(),
            Token::Enum => self.parse_enum_decl(),
            Token::Rasgo => self.parse_trait_decl(),
            Token::Impl => self.parse_impl_decl(),
            _ => Err(ErrorCompilador::new("'pub' debe ir seguido de una declaración")),
        }
    }

    fn parse_di_macro(&mut self) -> Result<Stmt, ErrorCompilador> {
        let span = self.current_span();
        self.advance(); // Consumir 'di'
        
        // Esperar '!' si está presente
        if *self.current_token() == Token::Negacion {
            self.advance();
        }
        
        // Esperar '(' 
        self.expect(Token::ParIzq)?;
        
        // Parsear argumentos
        let mut argumentos = Vec::new();
        if *self.current_token() != Token::ParDer {
            argumentos.push(self.parse_expression()?);
            while *self.current_token() == Token::Coma {
                self.advance();
                argumentos.push(self.parse_expression()?);
            }
        }
        
        self.expect(Token::ParDer)?;
        self.expect(Token::PuntoYComa)?;
        
        Ok(Stmt::Impresion {
            expresion: if argumentos.len() == 1 {
                argumentos.into_iter().next().unwrap()
            } else {
                // Múltiples argumentos: crear un Tuple o similar
                // Por ahora, simplemente tomar el primero
                argumentos.into_iter().next().unwrap()
            },
            span,
        })
    }

    fn parse_if(&mut self) -> Result<Stmt, ErrorCompilador> {
        let span = self.current_span();
        self.advance(); // Consumir 'si'
        self.expect(Token::ParIzq)?;
        let condicion = self.parse_expression()?;
        self.expect(Token::ParDer)?;
        self.expect(Token::LlaveIzq)?;

        let mut cuerpo = Vec::new();
        while *self.current_token() != Token::LlaveDer {
            cuerpo.push(self.parse_statement()?);
        }
        self.expect(Token::LlaveDer)?;

        let mut sino = None;
        if *self.current_token() == Token::ElSi {
            self.advance();
            // elsi es "else if", así que parsear como un if anidado
            self.expect(Token::ParIzq)?;
            let _condicion_elsi = self.parse_expression()?;
            self.expect(Token::ParDer)?;
            self.expect(Token::LlaveIzq)?;
            
            let mut cuerpo_elsi = Vec::new();
            while *self.current_token() != Token::LlaveDer {
                cuerpo_elsi.push(self.parse_statement()?);
            }
            self.expect(Token::LlaveDer)?;
            
            // Crear un if anidado como el sino
            sino = Some(cuerpo_elsi);
        } else if *self.current_token() == Token::Sino {
            self.advance();
            self.expect(Token::LlaveIzq)?;
            let mut cuerpo_sino = Vec::new();
            while *self.current_token() != Token::LlaveDer {
                cuerpo_sino.push(self.parse_statement()?);
            }
            self.expect(Token::LlaveDer)?;
            sino = Some(cuerpo_sino);
        }

        Ok(Stmt::Si {
            condicion,
            cuerpo,
            sino,
            span,
        })
    }

    fn parse_while(&mut self) -> Result<Stmt, ErrorCompilador> {
        let span = self.current_span();
        self.advance(); // Consumir 'mientras'
        self.expect(Token::ParIzq)?;
        let condicion = self.parse_expression()?;
        self.expect(Token::ParDer)?;
        self.expect(Token::LlaveIzq)?;

        let mut cuerpo = Vec::new();
        while *self.current_token() != Token::LlaveDer {
            cuerpo.push(self.parse_statement()?);
        }
        self.expect(Token::LlaveDer)?;

        Ok(Stmt::Mientras {
            condicion,
            cuerpo,
            span,
        })
    }

    fn parse_for(&mut self) -> Result<Stmt, ErrorCompilador> {
        let span = self.current_span();
        self.advance(); // Consumir 'para'
        
        // Parsear variable de iteración
        let variable = if let Token::Identificador(ref name) = self.current_token() {
            name.clone()
        } else {
            return Err(ErrorCompilador::new("Se esperaba un identificador después de 'para'"));
        };
        self.advance();
        
        // Esperar 'en'
        self.expect(Token::En)?;
        
        // Parsear rango/iterable
        let _inicio = self.parse_expression()?;
        
        // Por ahora, solo soportamos rangos (0..10)
        // Esperamos un DosPuntos para el rango
        if *self.current_token() == Token::DosPuntos {
            self.advance();
            let fin = self.parse_expression()?;
            self.expect(Token::LlaveIzq)?;
            
            let mut cuerpo = Vec::new();
            while *self.current_token() != Token::LlaveDer {
                cuerpo.push(self.parse_statement()?);
            }
            self.expect(Token::LlaveDer)?;
            
            // Convertir a Stmt::Mientras equivalente
            // para i en 0..10 => mientras i < 10 { cuerpo; i += 1 }
            Ok(Stmt::Mientras {
                condicion: Expr::Binaria {
                    izquierda: Box::new(Expr::Identificador(variable, span)),
                    operador: BinOp::Menor,
                    derecha: Box::new(fin),
                    span,
                },
                cuerpo,
                span,
            })
        } else {
            Err(ErrorCompilador::new("Se esperaba '..' en bucle 'para'"))
        }
    }

    fn parse_stdlib_call(&mut self) -> Result<Stmt, ErrorCompilador> {
        let comando = if let Token::Identificador(ref name) = self.current_token() {
            name.clone()
        } else {
            return Err(ErrorCompilador::new("Se esperaba un comando de la biblioteca estándar"));
        };
        let span = self.current_span();
        self.advance();
        self.expect(Token::Punto)?;

        let metodo = if let Token::Identificador(ref name) = self.current_token() {
            name.clone()
        } else {
            return Err(ErrorCompilador::new("Se esperaba un método después del punto"));
        };
        self.advance();
        self.expect(Token::ParIzq)?;

        let mut argumentos = Vec::new();
        if *self.current_token() != Token::ParDer {
            argumentos.push(self.parse_expression()?);
            while *self.current_token() == Token::Coma {
                self.advance();
                argumentos.push(self.parse_expression()?);
            }
        }
        self.expect(Token::ParDer)?;
        self.expect(Token::PuntoYComa)?;

        Ok(Stmt::LlamadaStdlib {
            comando: format!("{}.{}", comando, metodo),
            argumentos,
            span,
        })
    }

    fn parse_expression_statement(&mut self) -> Result<Stmt, ErrorCompilador> {
        let expr = self.parse_expression()?;
        let span = expr.span();
        let _ = self.expect(Token::PuntoYComa)?;
        Ok(Stmt::Expresion(expr, span))
    }

    fn parse_expression(&mut self) -> Result<Expr, ErrorCompilador> {
        self.parse_logical_or()
    }

    fn parse_logical_or(&mut self) -> Result<Expr, ErrorCompilador> {
        let mut expr = self.parse_logical_and()?;

        while *self.current_token() == Token::OLogico {
            let operador = BinOp::OLogico;
            self.advance();
            let derecha = self.parse_logical_and()?;
            let span = expr.span();
            expr = Expr::Binaria {
                izquierda: Box::new(expr),
                operador,
                derecha: Box::new(derecha),
                span,
            };
        }

        Ok(expr)
    }

    fn parse_logical_and(&mut self) -> Result<Expr, ErrorCompilador> {
        let mut expr = self.parse_equality()?;

        while *self.current_token() == Token::YLogico {
            let operador = BinOp::YLogico;
            self.advance();
            let derecha = self.parse_equality()?;
            let span = expr.span();
            expr = Expr::Binaria {
                izquierda: Box::new(expr),
                operador,
                derecha: Box::new(derecha),
                span,
            };
        }

        Ok(expr)
    }

    fn parse_equality(&mut self) -> Result<Expr, ErrorCompilador> {
        let mut expr = self.parse_comparison()?;

        while *self.current_token() == Token::Igual || *self.current_token() == Token::NoIgual {
            let operador = if *self.current_token() == Token::Igual {
                BinOp::Igual
            } else {
                BinOp::NoIgual
            };
            self.advance();
            let derecha = self.parse_comparison()?;
            let span = expr.span();
            expr = Expr::Binaria {
                izquierda: Box::new(expr),
                operador,
                derecha: Box::new(derecha),
                span,
            };
        }

        Ok(expr)
    }

    fn parse_comparison(&mut self) -> Result<Expr, ErrorCompilador> {
        let mut expr = self.parse_term()?;

        while matches!(*self.current_token(),
            Token::Menor | Token::Mayor | Token::MenorIgual | Token::MayorIgual
        ) {
            let operador = match *self.current_token() {
                Token::Menor => BinOp::Menor,
                Token::Mayor => BinOp::Mayor,
                Token::MenorIgual => BinOp::MenorIgual,
                Token::MayorIgual => BinOp::MayorIgual,
                _ => unreachable!(),
            };
            self.advance();
            let derecha = self.parse_term()?;
            let span = expr.span();
            expr = Expr::Binaria {
                izquierda: Box::new(expr),
                operador,
                derecha: Box::new(derecha),
                span,
            };
        }

        Ok(expr)
    }

    fn parse_term(&mut self) -> Result<Expr, ErrorCompilador> {
        let mut expr = self.parse_factor()?;

        while *self.current_token() == Token::Suma || *self.current_token() == Token::Resta {
            let operador = if *self.current_token() == Token::Suma {
                BinOp::Suma
            } else {
                BinOp::Resta
            };
            self.advance();
            let derecha = self.parse_factor()?;
            let span = expr.span();
            expr = Expr::Binaria {
                izquierda: Box::new(expr),
                operador,
                derecha: Box::new(derecha),
                span,
            };
        }

        Ok(expr)
    }

    fn parse_factor(&mut self) -> Result<Expr, ErrorCompilador> {
        let mut expr = self.parse_unary()?;

        while *self.current_token() == Token::Multiplicacion || *self.current_token() == Token::Division {
            let operador = if *self.current_token() == Token::Multiplicacion {
                BinOp::Multiplicacion
            } else {
                BinOp::Division
            };
            self.advance();
            let derecha = self.parse_unary()?;
            let span = expr.span();
            expr = Expr::Binaria {
                izquierda: Box::new(expr),
                operador,
                derecha: Box::new(derecha),
                span,
            };
        }

        Ok(expr)
    }

    fn parse_unary(&mut self) -> Result<Expr, ErrorCompilador> {
        if *self.current_token() == Token::Negacion || *self.current_token() == Token::Resta {
            let operador = if *self.current_token() == Token::Negacion {
                UnaryOp::Negacion
            } else {
                UnaryOp::Negativo
            };
            let span = self.current_span();
            self.advance();
            let operando = self.parse_unary()?;
            Ok(Expr::Unaria {
                operador,
                operando: Box::new(operando),
                span,
            })
        } else {
            self.parse_primary()
        }
    }

    fn parse_primary(&mut self) -> Result<Expr, ErrorCompilador> {
        match self.current_token().clone() {
            Token::Numero(valor) => {
                let span = self.current_span();
                self.advance();
                Ok(Expr::LiteralNumero(valor, span))
            }
            Token::Texto(valor) => {
                let span = self.current_span();
                self.advance();
                Ok(Expr::LiteralTexto(valor, span))
            }
            Token::Identificador(nombre) => {
                let span = self.current_span();
                self.advance();

                if *self.current_token() == Token::ParIzq {
                    self.advance();
                    let mut argumentos = Vec::new();
                    if *self.current_token() != Token::ParDer {
                        argumentos.push(self.parse_expression()?);
                        while *self.current_token() == Token::Coma {
                            self.advance();
                            argumentos.push(self.parse_expression()?);
                        }
                    }
                    self.expect(Token::ParDer)?;
                    Ok(Expr::Llamada {
                        nombre,
                        argumentos,
                        span,
                    })
                } else {
                    Ok(Expr::Identificador(nombre, span))
                }
            }
            Token::ParIzq => {
                self.advance();
                let expr = self.parse_expression()?;
                self.expect(Token::ParDer)?;
                Ok(expr)
            }
            _ => Err(ErrorCompilador::new(
                &format!("Token inesperado {} en expresión", self.current_token())
            )),
        }
    }
}