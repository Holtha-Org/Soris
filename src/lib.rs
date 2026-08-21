//! Soris - Lenguaje de programación en español que transpila a Rust
//! 
//! # Arquitectura
//! 
//! Fuente (.sr) → Lexer → Parser → AST → Resolver → Type Checker
//!                                           ↓
//! Código Rust ← Compiler ← Optimizer ← MIR ← HIR

pub mod ast;
pub mod frontend;
pub mod resolver;
pub mod checker;
pub mod hir;
pub mod mir;
pub mod backend;
pub mod stdlib;
pub mod utils;

use frontend::lexer::Lexer;
use frontend::parser::Parser;
use resolver::scope::ScopeResolver;
use checker::type_checker::TypeChecker;
use hir::hir::{ast_to_hir, HirStmt};
use mir::mir::{hir_to_mir, MirInstruccion};
use backend::optimizer::Optimizer;
use backend::compiler::Compiler;
use backend::interpreter::Interprete;
use utils::errors::ErrorCompilador;

/// Resultado de la compilación
#[derive(Debug)]
pub struct ResultadoCompilacion {
    pub codigo_rust: String,
    pub advertencias: Vec<String>,
}

/// Compilador principal de Soris
pub struct CompiladorSoris {
    optimizacion_nivel: u8,
    modo_interpretado: bool,
}

impl CompiladorSoris {
    /// Crear un nuevo compilador con nivel de optimización por defecto
    pub fn new() -> Self {
        Self {
            optimizacion_nivel: 1,
            modo_interpretado: false,
        }
    }

    /// Crear compilador con nivel de optimización personalizado
    pub fn with_optimizacion(nivel: u8) -> Self {
        Self {
            optimizacion_nivel: nivel.min(3),
            modo_interpretado: false,
        }
    }

    /// Activar modo interpretado (sin necesidad de Rust instalado)
    pub fn with_modo_interpretado(mut self, activado: bool) -> Self {
        self.modo_interpretado = activado;
        self
    }

    /// Ejecutar código Soris directamente (modo interpretado)
    pub fn ejecutar(&self, codigo_fuente: &str) -> Result<(), Vec<ErrorCompilador>> {
        if !self.modo_interpretado {
            return Err(vec![ErrorCompilador::new("El modo interpretado no está activado. Usa with_modo_interpretado(true)")]);
        }

        // Fase 1: Lexing
        let mut lexer = Lexer::new(codigo_fuente);
        let _tokens = lexer.tokenize()
            .map_err(|e| vec![e])?;

        // Fase 2: Parsing
        let mut parser = Parser::new(codigo_fuente)?;
        let ast = parser.parse()?;

        // Fase 3: Resolución de scopes
        let mut resolver = ScopeResolver::new();
        resolver.resolver(&ast)?;

        // Fase 4: Verificación de tipos (opcional en modo interpretado)
        let mut type_checker = TypeChecker::new();
        type_checker.check(&ast)?;

        // Fase 5: Ejecutar con el intérprete nativo
        let mut interprete = Interprete::nuevo();
        interprete.ejecutar(&ast)
            .map_err(|e| vec![ErrorCompilador::new(&e.to_string())])?;

        Ok(())
    }

    /// Compilar código fuente Soris a Rust
    pub fn compilar(&self, codigo_fuente: &str) -> Result<ResultadoCompilacion, Vec<ErrorCompilador>> {
        // Si está en modo interpretado, ejecutar directamente
        if self.modo_interpretado {
            self.ejecutar(codigo_fuente)?;
            // En modo interpretado no generamos código Rust
            return Ok(ResultadoCompilacion {
                codigo_rust: String::new(),
                advertencias: Vec::new(),
            });
        }

        // Modo transpilación normal (.sr → Rust)
        // Fase 1: Lexing
        let mut lexer = Lexer::new(codigo_fuente);
        let _tokens = lexer.tokenize()
            .map_err(|e| vec![e])?;

        // Fase 2: Parsing
        let mut parser = Parser::new(codigo_fuente)?;
        let ast = parser.parse()?;

        // Fase 3: Resolución de scopes
        let mut resolver = ScopeResolver::new();
        resolver.resolver(&ast)?;

        // Fase 4: Verificación de tipos
        let mut type_checker = TypeChecker::new();
        type_checker.check(&ast)?;

        // Fase 5: Convertir a HIR
        let hir: Vec<HirStmt> = ast_to_hir(&ast);

        // Fase 6: Convertir a MIR
        let mut mir: Vec<MirInstruccion> = hir_to_mir(&hir);

        // Fase 7: Optimización
        let mut optimizer = Optimizer::new();
        mir = optimizer.optimizar(mir, self.optimizacion_nivel);

        // Fase 8: Generar código Rust
        let mut compiler = Compiler::new();
        let codigo_rust = compiler.compile(&mir);

        Ok(ResultadoCompilacion {
            codigo_rust,
            advertencias: Vec::new(),
        })
    }

    /// Compilar archivo .sr a Rust (o ejecutar si es modo interpretado)
    pub fn compilar_archivo(&self, ruta: &str) -> Result<ResultadoCompilacion, Vec<ErrorCompilador>> {
        let codigo_fuente = std::fs::read_to_string(ruta)
            .map_err(|e| vec![ErrorCompilador::new(&format!("Error leyendo archivo: {}", e))])?;
        
        if self.modo_interpretado {
            self.ejecutar(&codigo_fuente)?;
            return Ok(ResultadoCompilacion {
                codigo_rust: String::new(),
                advertencias: Vec::new(),
            });
        }
        
        self.compilar(&codigo_fuente)
    }
}

impl Default for CompiladorSoris {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compilacion_basica() {
        let codigo = r#"
            fn inicio() {
                vr x = 10;
                vr y = 20;
                di!(x + y);
            }
        "#;

        let compilador = CompiladorSoris::new();
        let resultado = compilador.compilar(codigo);
        assert!(resultado.is_ok());
    }

    #[test]
    fn test_ejecucion_interpretada() {
        let codigo = r#"
            fn inicio() {
                vr x = 10;
                vr y = 20;
                di!(x + y);
            }
        "#;

        let compilador = CompiladorSoris::new().with_modo_interpretado(true);
        let resultado = compilador.ejecutar(codigo);
        assert!(resultado.is_ok());
    }
}
