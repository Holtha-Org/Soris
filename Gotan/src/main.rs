//! Gotan - Gestor de paquetes y compilador para Soris
//! 
//! Gotan es el equivalente a Cargo para el ecosistema Soris.
//! Permite crear, compilar, ejecutar y gestionar dependencias de proyectos .sr

pub mod comando;
pub mod proyecto;
pub mod configuracion;
pub mod construcccion;
pub mod ejecucion;
pub mod dependencias;

use clap::{Parser, Subcommand};
use colored::Colorize;

#[derive(Parser)]
#[command(name = "gotan")]
#[command(author = "Fundación Holtha")]
#[command(version = "0.1.0")]
#[command(about = "Gestor de paquetes y compilador para proyectos Soris", long_about = None)]
struct Cli {
    #[command(subcommand)]
    comando: Comandos,
}

#[derive(Subcommand)]
enum Comandos {
    /// Crea un nuevo proyecto Soris
    Nuevo {
        /// Nombre del proyecto
        nombre: String,
        
        /// Tipo de proyecto: binario o biblioteca
        #[arg(short, long, default_value = "bin")]
        tipo: String,
    },
    
    /// Compila un proyecto Soris
    Compilar {
        /// Ruta al proyecto (por defecto directorio actual)
        #[arg(default_value = ".")]
        ruta: String,
        
        /// Modo de compilación: debug o release
        #[arg(short, long, default_value = "debug")]
        modo: String,
    },
    
    /// Ejecuta un proyecto Soris
    Ejecutar {
        /// Ruta al proyecto (por defecto directorio actual)
        #[arg(default_value = ".")]
        ruta: String,
        
        /// Argumentos para pasar al programa
        argumentos: Vec<String>,
    },
    
    /// Limpia los archivos generados
    Limpiar {
        /// Ruta al proyecto (por defecto directorio actual)
        #[arg(default_value = ".")]
        ruta: String,
    },
    
    /// Muestra información del proyecto
    Info {
        /// Ruta al proyecto (por defecto directorio actual)
        #[arg(default_value = ".")]
        ruta: String,
    },
    
    /// Busca y añade dependencias
    Agregar {
        /// Nombre de la dependencia
        nombre: String,
        
        /// Versión de la dependencia
        version: Option<String>,
    },
    
    /// Actualiza las dependencias
    Actualizar {
        /// Nombre de la dependencia específica (opcional)
        nombre: Option<String>,
    },
    
    /// Elimina una dependencia
    Eliminar {
        /// Nombre de la dependencia a eliminar
        nombre: String,
    },
    
    /// Lista las dependencias
    Listar {
        /// Ruta al proyecto (por defecto directorio actual)
        #[arg(default_value = ".")]
        ruta: String,
    },
    
    /// Ejecuta pruebas
    Prueba {
        /// Ruta al proyecto (por defecto directorio actual)
        #[arg(default_value = ".")]
        ruta: String,
    },
    
    /// Genera documentación
    Documentacion {
        /// Ruta al proyecto (por defecto directorio actual)
        #[arg(default_value = ".")]
        ruta: String,
        
        /// Abrir en navegador
        #[arg(short, long)]
        abrir: bool,
    },
    
    /// Publica el paquete en el registro
    Publicar {
        /// Ruta al proyecto (por defecto directorio actual)
        #[arg(default_value = ".")]
        ruta: String,
    },
    
    /// Inicializa un proyecto Soris en el directorio actual
    Iniciar {
        /// Nombre del proyecto
        #[arg(short, long)]
        nombre: Option<String>,
    },
    
    /// Muestra el árbol de dependencias
    Arbol {
        /// Ruta al proyecto (por defecto directorio actual)
        #[arg(default_value = ".")]
        ruta: String,
    },
}

fn main() {
    let cli = Cli::parse();
    
    match cli.comando {
        Comandos::Nuevo { nombre, tipo } => {
            println!("{} Creando nuevo proyecto '{}' de tipo {}", 
                "✨".bright_yellow(), 
                nombre.bright_cyan(), 
                tipo.bright_green());
            
            match proyecto::crear_proyecto(&nombre, &tipo) {
                Ok(_) => println!("{} Proyecto '{}' creado exitosamente", 
                    "✅".bright_green(), 
                    nombre.bright_cyan()),
                Err(e) => {
                    eprintln!("{} Error al crear proyecto: {}", 
                        "❌".bright_red(), 
                        e.to_string().bright_red());
                    std::process::exit(1);
                }
            }
        }
        
        Comandos::Compilar { ruta, modo } => {
            println!("{} Compilando proyecto en '{}' (modo: {})", 
                "🔨".bright_yellow(), 
                ruta.bright_cyan(), 
                modo.bright_green());
            
            match construcccion::compilar(&ruta, &modo) {
                Ok(salida) => println!("{} Compilación exitosa: {}", 
                    "✅".bright_green(), 
                    salida.bright_cyan()),
                Err(e) => {
                    eprintln!("{} Error de compilación: {}", 
                        "❌".bright_red(), 
                        e.to_string().bright_red());
                    std::process::exit(1);
                }
            }
        }
        
        Comandos::Ejecutar { ruta, argumentos } => {
            println!("{} Ejecutando proyecto en '{}'", 
                "🚀".bright_yellow(), 
                ruta.bright_cyan());
            
            match ejecucion::ejecutar(&ruta, &argumentos) {
                Ok(codigo) => {
                    if codigo == 0 {
                        println!("{} Ejecución completada", "✅".bright_green());
                    } else {
                        println!("{} Programa finalizó con código {}", 
                            "⚠️".bright_yellow(), 
                            codigo);
                    }
                }
                Err(e) => {
                    eprintln!("{} Error de ejecución: {}", 
                        "❌".bright_red(), 
                        e.to_string().bright_red());
                    std::process::exit(1);
                }
            }
        }
        
        Comandos::Limpiar { ruta } => {
            println!("{} Limpiando proyecto en '{}'", 
                "🧹".bright_yellow(), 
                ruta.bright_cyan());
            
            match construcccion::limpiar(&ruta) {
                Ok(_) => println!("{} Limpieza completada", "✅".bright_green()),
                Err(e) => {
                    eprintln!("{} Error al limpiar: {}", 
                        "❌".bright_red(), 
                        e.to_string().bright_red());
                    std::process::exit(1);
                }
            }
        }
        
        Comandos::Info { ruta } => {
            println!("{} Información del proyecto en '{}'", 
                "ℹ️".bright_yellow(), 
                ruta.bright_cyan());
            
            match proyecto::informacion(&ruta) {
                Ok(info) => {
                    println!("\n{}", "=== Información del Proyecto ===".bright_cyan());
                    println!("Nombre: {}", info.nombre.bright_green());
                    println!("Versión: {}", info.version.bright_green());
                    println!("Tipo: {}", info.tipo.bright_green());
                    println!("Autor(es): {}", info.autores.bright_green());
                    println!("Descripción: {}", info.descripcion.bright_green());
                    println!("Dependencias: {}", info.dependencias.len());
                    for dep in &info.dependencias {
                        println!("  - {}: {}", dep.0.bright_yellow(), dep.1.bright_yellow());
                    }
                }
                Err(e) => {
                    eprintln!("{} Error al obtener información: {}", 
                        "❌".bright_red(), 
                        e.to_string().bright_red());
                    std::process::exit(1);
                }
            }
        }
        
        Comandos::Agregar { nombre, version } => {
            let ver = version.unwrap_or_else(|| "latest".to_string());
            println!("{} Añadiendo dependencia '{}' (versión: {})", 
                "📦".bright_yellow(), 
                nombre.bright_cyan(), 
                ver.bright_green());
            
            match dependencias::agregar(&nombre, &ver) {
                Ok(_) => println!("{} Dependencia añadida exitosamente", "✅".bright_green()),
                Err(e) => {
                    eprintln!("{} Error al añadir dependencia: {}", 
                        "❌".bright_red(), 
                        e.to_string().bright_red());
                    std::process::exit(1);
                }
            }
        }
        
        Comandos::Actualizar { nombre } => {
            if let Some(n) = nombre {
                println!("{} Actualizando dependencia '{}'", 
                    "🔄".bright_yellow(), 
                    n.bright_cyan());
            } else {
                println!("{} Actualizando todas las dependencias", 
                    "🔄".bright_yellow());
            }
            
            match dependencias::actualizar(nombre.as_deref()) {
                Ok(_) => println!("{} Dependencias actualizadas", "✅".bright_green()),
                Err(e) => {
                    eprintln!("{} Error al actualizar: {}", 
                        "❌".bright_red(), 
                        e.to_string().bright_red());
                    std::process::exit(1);
                }
            }
        }
        
        Comandos::Eliminar { nombre } => {
            println!("{} Eliminando dependencia '{}'", 
                "🗑️".bright_yellow(), 
                nombre.bright_cyan());
            
            match dependencias::eliminar(&nombre) {
                Ok(_) => println!("{} Dependencia eliminada", "✅".bright_green()),
                Err(e) => {
                    eprintln!("{} Error al eliminar: {}", 
                        "❌".bright_red(), 
                        e.to_string().bright_red());
                    std::process::exit(1);
                }
            }
        }
        
        Comandos::Listar { ruta } => {
            println!("{} Listando dependencias", "📋".bright_yellow());
            
            match dependencias::listar(&ruta) {
                Ok(deps) => {
                    if deps.is_empty() {
                        println!("{} Sin dependencias", "ℹ️".bright_cyan());
                    } else {
                        for (nombre, version) in deps {
                            println!("  - {}: {}", nombre.bright_yellow(), version.bright_yellow());
                        }
                    }
                }
                Err(e) => {
                    eprintln!("{} Error al listar: {}", 
                        "❌".bright_red(), 
                        e.to_string().bright_red());
                    std::process::exit(1);
                }
            }
        }
        
        Comandos::Prueba { ruta } => {
            println!("{} Ejecutando pruebas en '{}'", 
                "🧪".bright_yellow(), 
                ruta.bright_cyan());
            
            match ejecucion::ejecutar_pruebas(&ruta) {
                Ok(resultados) => {
                    println!("{} Pruebas completadas: {} pasaron, {} fallaron de {}", 
                        "✅".bright_green(),
                        resultados.pasados,
                        resultados.fallidos,
                        resultados.total);
                    for detalle in resultados.detalles {
                        println!("  {}", detalle);
                    }
                }
                Err(e) => {
                    eprintln!("{} Error en pruebas: {}", 
                        "❌".bright_red(), 
                        e.to_string().bright_red());
                    std::process::exit(1);
                }
            }
        }
        
        Comandos::Documentacion { ruta, abrir } => {
            println!("{} Generando documentación para '{}'", 
                "📚".bright_yellow(), 
                ruta.bright_cyan());
            
            match construcccion::generar_documentacion(&ruta, abrir) {
                Ok(ruta_doc) => println!("{} Documentación generada en: {}", 
                    "✅".bright_green(), 
                    ruta_doc.bright_cyan()),
                Err(e) => {
                    eprintln!("{} Error al generar documentación: {}", 
                        "❌".bright_red(), 
                        e.to_string().bright_red());
                    std::process::exit(1);
                }
            }
        }
        
        Comandos::Publicar { ruta } => {
            println!("{} Publicando proyecto en '{}'", 
                "📤".bright_yellow(), 
                ruta.bright_cyan());
            
            match dependencias::publicar(&ruta) {
                Ok(url) => println!("{} Proyecto publicado en: {}", 
                    "✅".bright_green(), 
                    url.bright_cyan()),
                Err(e) => {
                    eprintln!("{} Error al publicar: {}", 
                        "❌".bright_red(), 
                        e.to_string().bright_red());
                    std::process::exit(1);
                }
            }
        }
        
        Comandos::Iniciar { nombre } => {
            println!("{} Iniciando proyecto Soris", "🎯".bright_yellow());
            
            match proyecto::inicializar(nombre.as_deref()) {
                Ok(_) => println!("{} Proyecto inicializado", "✅".bright_green()),
                Err(e) => {
                    eprintln!("{} Error al inicializar: {}", 
                        "❌".bright_red(), 
                        e.to_string().bright_red());
                    std::process::exit(1);
                }
            }
        }
        
        Comandos::Arbol { ruta } => {
            println!("{} Árbol de dependencias", "🌳".bright_yellow());
            
            match dependencias::arbol_dependencias(&ruta) {
                Ok(arbol) => println!("{}", arbol.bright_green()),
                Err(e) => {
                    eprintln!("{} Error al mostrar árbol: {}", 
                        "❌".bright_red(), 
                        e.to_string().bright_red());
                    std::process::exit(1);
                }
            }
        }
    }
}
