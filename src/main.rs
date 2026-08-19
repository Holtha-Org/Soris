//! CLI de Soris - Compilador del lenguaje en español a Rust
//! 
//! Uso:
//!   soris compilar <archivo.sr> [-o salida]
//!   soris ejecutar <archivo.sr>
//!   soris nuevo <nombre_proyecto>
//!   soris --version

use std::env;
use std::fs;
use std::path::Path;
use std::process::Command;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        mostrar_ayuda();
        return;
    }

    let comando = &args[1];

    match comando.as_str() {
        "compilar" => compilar(&args[2..]),
        "ejecutar" | "correr" | "run" => ejecutar(&args[2..]),
        "nuevo" => nuevo_proyecto(&args[2..]),
        "version" | "--version" | "-v" => mostrar_version(),
        "ayuda" | "--help" | "-h" => mostrar_ayuda(),
        _ => {
            eprintln!("Comando desconocido: {}", comando);
            mostrar_ayuda();
        }
    }
}

fn mostrar_ayuda() {
    println!(r#"
╔═══════════════════════════════════════════════════════════╗
║                    SORIS v0.1.0                           ║
║         Lenguaje de Programación en Español               ║
╚═══════════════════════════════════════════════════════════╝

USO:
    soris <comando> [opciones]

COMANDOS:
    compilar <archivo.sr> [-o salida]   Compila código Soris a Rust
    ejecutar <archivo.sr>               Compila y ejecuta el código
    nuevo <nombre_proyecto>             Crea un nuevo proyecto Soris
    version                             Muestra la versión del compilador
    ayuda                               Muestra esta ayuda

EJEMPLOS:
    soris compilar programa.sr
    soris compilar programa.sr -o mi_programa
    soris ejecutar hola_mundo.sr
    soris nuevo mi_proyecto

"#);
}

fn mostrar_version() {
    println!("Soris v0.1.0");
    println!("Lenguaje de programación en español que transpila a Rust");
    println!("Fundación Holtha © 2024");
}

fn compilar(args: &[String]) {
    if args.is_empty() {
        eprintln!("Error: Debes especificar un archivo .sr para compilar");
        eprintln!("Uso: soris compilar <archivo.sr> [-o salida]");
        std::process::exit(1);
    }

    let archivo = &args[0];
    let mut salida = String::from("main.rs");

    // Parsear opción -o
    let mut i = 1;
    while i < args.len() {
        if args[i] == "-o" && i + 1 < args.len() {
            salida = args[i + 1].clone();
            i += 2;
        } else {
            i += 1;
        }
    }

    println!("🔵 Compilando {}...", archivo);

    // Leer archivo fuente
    let codigo_fuente = match fs::read_to_string(archivo) {
        Ok(codigo) => codigo,
        Err(e) => {
            eprintln!("❌ Error leyendo archivo '{}': {}", archivo, e);
            std::process::exit(1);
        }
    };

    // Compilar usando la librería
    let compilador = soris::CompiladorSoris::new();
    match compilador.compilar(&codigo_fuente) {
        Ok(resultado) => {
            // Escribir código Rust generado
            if let Err(e) = fs::write(&salida, &resultado.codigo_rust) {
                eprintln!("❌ Error escribiendo archivo de salida: {}", e);
                std::process::exit(1);
            }

            println!("✅ Compilación exitosa!");
            println!("   Código Rust generado: {}", salida);
            
            // Mostrar estadísticas
            let lineas_originales = codigo_fuente.lines().count();
            let lineas_rust = resultado.codigo_rust.lines().count();
            println!("   Líneas originales: {}", lineas_originales);
            println!("   Líneas Rust: {}", lineas_rust);

            if !resultado.advertencias.is_empty() {
                println!("\n⚠️  Advertencias:");
                for adv in &resultado.advertencias {
                    println!("   {}", adv);
                }
            }
        }
        Err(errores) => {
            eprintln!("❌ Errores de compilación:");
            for error in &errores {
                eprintln!("   {}", error);
            }
            std::process::exit(1);
        }
    }
}

fn ejecutar(args: &[String]) {
    if args.is_empty() {
        eprintln!("Error: Debes especificar un archivo .sr para ejecutar");
        eprintln!("Uso: soris ejecutar <archivo.sr>");
        std::process::exit(1);
    }

    let archivo = &args[0];
    let archivo_temp = "target_temp.rs";

    println!("🔵 Compilando y ejecutando {}...", archivo);

    // Primero compilar
    compilar(&[archivo.to_string(), "-o".to_string(), archivo_temp.to_string()]);

    // Crear Cargo.toml temporal si no existe
    let cargo_toml = r#"[package]
name = "soris_temp"
version = "0.1.0"
edition = "2021"

[[bin]]
name = "soris_temp"
path = "target_temp.rs"

[dependencies]
rand = "0.8"
"#;

    if !Path::new("Cargo.toml").exists() {
        fs::write("Cargo.toml", cargo_toml).expect("Error creando Cargo.toml temporal");
    }

    // Ejecutar con cargo
    println!("\n🚀 Ejecutando...");
    let output = Command::new("cargo")
        .args(&["run", "--bin", "soris_temp", "--quiet"])
        .output()
        .expect("Error ejecutando cargo");

    if output.status.success() {
        println!("{}", String::from_utf8_lossy(&output.stdout));
    } else {
        eprintln!("❌ Error en ejecución:");
        eprintln!("{}", String::from_utf8_lossy(&output.stderr));
        std::process::exit(1);
    }

    // Limpieza
    let _ = fs::remove_file(archivo_temp);
}

fn nuevo_proyecto(args: &[String]) {
    if args.is_empty() {
        eprintln!("Error: Debes especificar un nombre para el proyecto");
        eprintln!("Uso: soris nuevo <nombre_proyecto>");
        std::process::exit(1);
    }

    let nombre = &args[0];

    // Validar nombre
    if !nombre.chars().all(|c| c.is_alphanumeric() || c == '_') {
        eprintln!("Error: El nombre del proyecto solo puede contener letras, números y guiones bajos");
        std::process::exit(1);
    }

    println!("📦 Creando proyecto Soris: {}", nombre);

    // Crear estructura de directorios
    let dirs = vec![
        format!("{}/src", nombre),
        format!("{}/tests", nombre),
    ];

    for dir in &dirs {
        if let Err(e) = fs::create_dir_all(dir) {
            eprintln!("❌ Error creando directorio '{}': {}", dir, e);
            std::process::exit(1);
        }
    }

    // Crear Cargo.toml
    let cargo_toml = format!(r#"[package]
name = "{}"
version = "0.1.0"
edition = "2021"

[dependencies]
soris = {{ path = "../" }}
rand = "0.8"
"#, nombre);

    if let Err(e) = fs::write(format!("{}/Cargo.toml", nombre), cargo_toml) {
        eprintln!("❌ Error creando Cargo.toml: {}", e);
        std::process::exit(1);
    }

    // Crear archivo principal
    let main_sr = r#"// Programa Soris
// Autor: Tu Nombre

var mensaje = "¡Hola desde Soris!";
di!(mensaje);

var numero = 42;
di!("El número es: " + numero);

// Estructura de control
var contador = 0;
mientras (contador < 5) {
    di!("Contando: " + contador);
    contador = contador + 1;
}

// Condicionales
si (numero > 40) {
    di!("El número es mayor que 40");
} sino {
    di!("El número es menor o igual a 40");
}

// Funciones de la biblioteca estándar
texto.longitud(mensaje);
mat.raiz(16);
aleatorio.generar_entre(1, 100);
"#;

    if let Err(e) = fs::write(format!("{}/src/main.sr", nombre), main_sr) {
        eprintln!("❌ Error creando main.sr: {}", e);
        std::process::exit(1);
    }

    // Crear README
    let readme = format!(r#"# {}

Proyecto creado con Soris - Lenguaje de programación en español

## Estructura

```
{}/
├── Cargo.toml      # Configuración del proyecto
├── src/
│   └── main.sr     # Código fuente principal
└── tests/          # Pruebas unitarias
```

## Comandos útiles

```bash
# Compilar
soris compilar src/main.sr

# Ejecutar
soris ejecutar src/main.sr
```

## Documentación

Visita https://github.com/Holtha-HT/soris para más información.
"#, nombre, nombre);

    if let Err(e) = fs::write(format!("{}/README.md", nombre), readme) {
        eprintln!("❌ Error creando README.md: {}", e);
        std::process::exit(1);
    }

    println!("✅ Proyecto '{}' creado exitosamente!", nombre);
    println!("\nSiguientes pasos:");
    println!("  cd {}", nombre);
    println!("  soris ejecutar src/main.sr");
}
