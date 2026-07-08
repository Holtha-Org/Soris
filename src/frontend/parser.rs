use crate::utils::token::Token;
use crate::utils::span::Span;
use crate::utils::errors::ErrorCompilador;
use crate::ast::expr::{Expr, BinOp, UnaryOp};
use crate::ast::stmt::Stmt;
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

    pub fn parse(&mut self) -> Result<Vec<Stmt>, ErrorCompilador> {
        let mut statements = Vec::new();

        // Verificar firma de la Fundación Holtha
        if let Token::Identificador(ref ident) = self.current_token() {
            if ident == "autor" {
                self.advance();
                let _ = self.expect(Token::DosPuntos)?;
                if let Token::Identificador(ref autor) = self.current_token() {
                    let span = self.current_span();
                    let firma = Stmt::Firma {
                        autor: autor.clone(),
                        span,
                    };
                    self.advance();
                    let _ = self.expect(Token::PuntoYComa)?;
                    statements.push(firma);
                } else {
                    return Err(ErrorCompilador::new("Se esperaba un identificador después de 'autor:'"));
                }
            } else {
                return Err(ErrorCompilador::new(
                    "El programa debe comenzar con la firma 'autor:holtha'"
                ).con_ayuda("Añade 'autor:holtha;' al inicio de tu archivo .sr"));
            }
        } else {
            return Err(ErrorCompilador::new(
                "El programa debe comenzar con la firma 'autor:holtha'"
            ).con_ayuda("Añade 'autor:holtha;' al inicio de tu archivo .sr"));
        }

        // Parsear el resto del programa
        while *self.current_token() != Token::EOF {
            statements.push(self.parse_statement()?);
        }

        Ok(statements)
    }

    fn parse_statement(&mut self) -> Result<Stmt, ErrorCompilador> {
        match self.current_token() {
            Token::Declarar => self.parse_declaration(),
            Token::Imprimir => self.parse_print(),
            Token::Si => self.parse_if(),
            Token::Mientras => self.parse_while(),
            Token::Identificador(_) => {
                if self.posicion + 1 < self.tokens.len() {
                    match self.tokens[self.posicion + 1].0 {
                        Token::Asignacion => self.parse_assignment(),
                        Token::Punto => self.parse_stdlib_call(),
                        _ => self.parse_expression_statement(),
                    }
                } else {
                    self.parse_expression_statement()
                }
            }
            _ => self.parse_expression_statement(),
        }
    }

    fn parse_declaration(&mut self) -> Result<Stmt, ErrorCompilador> {
        let span = self.current_span();
        self.advance(); // Consumir 'declarar'
        let nombre = if let Token::Identificador(ref name) = self.current_token() {
            name.clone()
        } else {
            return Err(ErrorCompilador::new("Se esperaba un nombre de variable después de 'declarar'"));
        };
        self.advance();

        let mut valor_inicial = None;
        if *self.current_token() == Token::Asignacion {
            self.advance();
            valor_inicial = Some(self.parse_expression()?);
        }

        let _ = self.expect(Token::PuntoYComa)?;

        Ok(Stmt::Declaracion {
            nombre,
            valor_inicial,
            span,
        })
    }

    fn parse_assignment(&mut self) -> Result<Stmt, ErrorCompilador> {
        let span = self.current_span();
        let nombre = if let Token::Identificador(ref name) = self.current_token() {
            name.clone()
        } else {
            return Err(ErrorCompilador::new("Se esperaba un nombre de variable para la asignación"));
        };
        self.advance();
        self.expect(Token::Asignacion)?;

        let valor = self.parse_expression()?;
        let _ = self.expect(Token::PuntoYComa)?;

        Ok(Stmt::Asignacion {
            nombre,
            valor,
            span,
        })
    }

    fn parse_print(&mut self) -> Result<Stmt, ErrorCompilador> {
        let span = self.current_span();
        self.advance(); // Consumir 'imprimir'
        let expr = self.parse_expression()?;
        let _ = self.expect(Token::PuntoYComa)?;
        Ok(Stmt::Impresion {
            expresion: expr,
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
        if *self.current_token() == Token::Sino {
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