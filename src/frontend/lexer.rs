use crate::utils::token::Token;
use crate::utils::span::Span;
use crate::utils::errors::ErrorCompilador;

pub struct Lexer {
    entrada: Vec<char>,
    posicion: usize,
    linea: usize,
    columna: usize,
}

impl Lexer {
    pub fn new(entrada: &str) -> Self {
        Self {
            entrada: entrada.chars().collect(),
            posicion: 0,
            linea: 1,
            columna: 1,
        }
    }

    fn current_char(&self) -> char {
        if self.posicion < self.entrada.len() {
            self.entrada[self.posicion]
        } else {
            '\0'
        }
    }

    fn peek_char(&self) -> char {
        if self.posicion + 1 < self.entrada.len() {
            self.entrada[self.posicion + 1]
        } else {
            '\0'
        }
    }

    fn advance(&mut self) {
        if self.current_char() == '\n' {
            self.linea += 1;
            self.columna = 1;
        } else {
            self.columna += 1;
        }
        self.posicion += 1;
    }

    fn skip_whitespace(&mut self) {
        while self.current_char().is_whitespace() {
            self.advance();
        }
    }

    fn read_number(&mut self) -> Token {
        let mut numero = String::new();
        
        while self.current_char().is_numeric() || self.current_char() == '.' {
            numero.push(self.current_char());
            self.advance();
            if self.current_char() == '.' && numero.contains('.') {
                break;
            }
        }

        let valor: f64 = numero.parse().expect("Error al parsear número");
        Token::Numero(valor)
    }

    fn read_identifier(&mut self) -> Token {
        let mut identificador = String::new();

        while self.current_char().is_alphanumeric() || self.current_char() == '_' {
            identificador.push(self.current_char());
            self.advance();
        }

        Token::palabra_clave_o_identificador(&identificador)
    }

    fn read_string(&mut self) -> Token {
        let mut texto = String::new();
        self.advance(); // Consumir comilla inicial

        while self.current_char() != '"' && self.current_char() != '\0' {
            if self.current_char() == '\\' {
                self.advance();
                match self.current_char() {
                    'n' => texto.push('\n'),
                    't' => texto.push('\t'),
                    '\\' => texto.push('\\'),
                    '"' => texto.push('"'),
                    _ => texto.push(self.current_char()),
                }
            } else {
                texto.push(self.current_char());
            }
            self.advance();
        }

        if self.current_char() == '"' {
            self.advance(); // Consumir comilla final
        }

        Token::Texto(texto)
    }

    pub fn next_token(&mut self) -> Result<(Token, Span), ErrorCompilador> {
        self.skip_whitespace();

        if self.posicion >= self.entrada.len() {
            return Ok((Token::EOF, Span::new(self.linea, self.columna)));
        }

        let span = Span::new(self.linea, self.columna);
        let c = self.current_char();

        match c {
            '=' => {
                self.advance();
                if self.current_char() == '=' {
                    self.advance();
                    Ok((Token::Igual, span))
                } else {
                    Ok((Token::Asignacion, span))
                }
            }
            '!' => {
                self.advance();
                if self.current_char() == '=' {
                    self.advance();
                    Ok((Token::NoIgual, span))
                } else {
                    Ok((Token::Negacion, span))
                }
            }
            '<' => {
                self.advance();
                if self.current_char() == '=' {
                    self.advance();
                    Ok((Token::MenorIgual, span))
                } else {
                    Ok((Token::Menor, span))
                }
            }
            '>' => {
                self.advance();
                if self.current_char() == '=' {
                    self.advance();
                    Ok((Token::MayorIgual, span))
                } else {
                    Ok((Token::Mayor, span))
                }
            }
            '+' => { self.advance(); Ok((Token::Suma, span)) }
            '-' => { self.advance(); Ok((Token::Resta, span)) }
            '*' => { self.advance(); Ok((Token::Multiplicacion, span)) }
            '/' => { self.advance(); Ok((Token::Division, span)) }
            ';' => { self.advance(); Ok((Token::PuntoYComa, span)) }
            ',' => { self.advance(); Ok((Token::Coma, span)) }
            '.' => { self.advance(); Ok((Token::Punto, span)) }
            ':' => { self.advance(); Ok((Token::DosPuntos, span)) }
            '{' => { self.advance(); Ok((Token::LlaveIzq, span)) }
            '}' => { self.advance(); Ok((Token::LlaveDer, span)) }
            '(' => { self.advance(); Ok((Token::ParIzq, span)) }
            ')' => { self.advance(); Ok((Token::ParDer, span)) }
            '[' => { self.advance(); Ok((Token::CorcheteIzq, span)) }
            ']' => { self.advance(); Ok((Token::CorcheteDer, span)) }
            '&' => {
                self.advance();
                if self.current_char() == '&' {
                    self.advance();
                    Ok((Token::YLogico, span))
                } else {
                    Err(ErrorCompilador::new("Carácter inesperado '&'").con_ayuda("Usa 'y' para el operador lógico"))
                }
            }
            '|' => {
                self.advance();
                if self.current_char() == '|' {
                    self.advance();
                    Ok((Token::OLogico, span))
                } else {
                    Err(ErrorCompilador::new("Carácter inesperado '|'").con_ayuda("Usa 'o' para el operador lógico"))
                }
            }
            '"' => Ok((self.read_string(), span)),
            _ if c.is_numeric() => Ok((self.read_number(), span)),
            _ if c.is_alphabetic() || c == '_' => Ok((self.read_identifier(), span)),
            _ => {
                Err(ErrorCompilador::new(
                    &format!("Carácter inesperado '{}' en {}", c, span)
                ))
            }
        }
    }

    pub fn tokenize(&mut self) -> Result<Vec<(Token, Span)>, ErrorCompilador> {
        let mut tokens = Vec::new();
        loop {
            let (token, span) = self.next_token()?;
            if token == Token::EOF {
                tokens.push((token, span));
                break;
            }
            tokens.push((token, span));
        }
        Ok(tokens)
    }
}