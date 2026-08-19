//! Módulo de construcción (build) para proyectos Soris
//! Maneja la compilación, limpieza y generación de artefactos

use crate::comando::ejecutar_comando;
use crate::configuracion::ConfiguracionProyecto;
use soris::CompiladorSoris;
use std::fs;
use std::path::Path;
use std::process::Command;

/// Resultado de una compilación
#[derive(Debug)]
pub struct ResultadoCompilacion {
    pub exito: bool,
    pub ruta_salida: Option<String>,
    pub mensajes: Vec<String>,
    pub tiempo_ms: u64,
}

/// Compila un proyecto Soris transpilándolo a Rust y usando Cargo
pub fn compilar(ruta: &str, modo: &str) -> Result<String, String> {
    let inicio = std::time::Instant::now();
    
    // Validar proyecto
    crate::proyecto::validar_proyecto(ruta)?;
    
    // Cargar configuración
    let config = ConfiguracionProyecto::cargar(ruta)?;
    
    // Determinar ruta del archivo principal
    let camino_base = Path::new(ruta);
    let archivo_sr = camino_base.join(config.ruta_principal(ruta));
    
    if !archivo_sr.exists() {
        return Err(format!("Archivo principal no encontrado: {:?}", archivo_sr));
    }
    
    // Leer código fuente Soris
    let codigo_soris = fs::read_to_string(&archivo_sr)
        .map_err(|e| format!("Error al leer {}: {}", archivo_sr.display(), e))?;
    
    // Crear directorio target si no existe
    let directorio_target = camino_base.join("target");
    fs::create_dir_all(&directorio_target)
        .map_err(|e| format!("Error al crear directorio target: {}", e))?;
    
    // Transpilar Soris a Rust usando el compilador real de Soris
    let compilador = CompiladorSoris::with_optimizacion(2);
    let resultado = compilador.compilar(&codigo_soris)
        .map_err(|e| {
            e.iter()
                .map(|err| err.mensaje.clone())
                .collect::<Vec<_>>()
                .join("\n")
        })?;
    
    let codigo_rust = resultado.codigo_rust;
    
    // Crear estructura temporal de proyecto Rust
    let rust_project = directorio_target.join("rust_temp");
    fs::create_dir_all(&rust_project)
        .map_err(|e| format!("Error al crear proyecto temporal: {}", e))?;
    
    // Crear Cargo.toml temporal
    let cargo_toml = format!(r#"[package]
name = "{}"
version = "{}"
edition = "2021"

[dependencies]
# Dependencias generadas por Soris
"#, 
        config.paquete.nombre.replace("-", "_"),
        config.paquete.version
    );
    
    fs::write(rust_project.join("Cargo.toml"), cargo_toml)
        .map_err(|e| format!("Error al crear Cargo.toml: {}", e))?;
    
    // Crear src para Rust
    let src_rust = rust_project.join("src");
    fs::create_dir_all(&src_rust)
        .map_err(|e| format!("Error al crear src: {}", e))?;
    
    // Escribir código Rust generado
    fs::write(src_rust.join("main.rs"), codigo_rust)
        .map_err(|e| format!("Error al escribir main.rs: {}", e))?;
    
    // Ejecutar cargo build
    let args_cargo = match modo {
        "release" => vec!["build", "--release"],
        _ => vec!["build"],
    };
    
    let output = Command::new("cargo")
        .args(&args_cargo)
        .current_dir(&rust_project)
        .output()
        .map_err(|e| format!("Error al ejecutar cargo: {}. ¿Tienes Rust instalado?", e))?;
    
    if !output.status.success() {
        return Err(format!(
            "Error de compilación en Rust:\n{}",
            String::from_utf8_lossy(&output.stderr)
        ));
    }
    
    // Determinar ruta del binario generado
    let subdir = if modo == "release" { "release" } else { "debug" };
    let nombre_binario = config.paquete.nombre.replace("-", "_");
    let ruta_binario = rust_project.join("target").join(subdir).join(&nombre_binario);
    
    // Copiar binario al directorio target del proyecto Soris
    let ruta_final = directorio_target.join(&nombre_binario);
    fs::copy(&ruta_binario, &ruta_final)
        .map_err(|e| format!("Error al copiar binario: {}", e))?;
    
    let tiempo = inicio.elapsed().as_millis() as u64;
    
    Ok(format!(
        "{} (compilado en {} ms)",
        ruta_final.display(),
        tiempo
    ))
}

/// Limpia los archivos generados por la compilación
pub fn limpiar(ruta: &str) -> Result<(), String> {
    let camino = Path::new(ruta);
    let directorio_target = camino.join("target");
    
    if directorio_target.exists() {
        fs::remove_dir_all(&directorio_target)
            .map_err(|e| format!("Error al eliminar target: {}", e))?;
        println!("Directorio target eliminado");
    }
    
    // Eliminar archivos temporales adicionales
    let archivos_temp = ["*.tmp", "*.log", "Cargo.lock"];
    for patron in &archivos_temp {
        // Nota: Esto es simplificado, en producción usaría glob
    }
    
    Ok(())
}

/// Genera documentación del proyecto
pub fn generar_documentacion(ruta: &str, abrir: bool) -> Result<String, String> {
    let camino = Path::new(ruta);
    let config = ConfiguracionProyecto::cargar(ruta)?;
    
    // Crear directorio de documentación
    let dir_doc = camino.join("doc");
    fs::create_dir_all(&dir_doc)
        .map_err(|e| format!("Error al crear directorio doc: {}", e))?;
    
    // Generar documentación básica en Markdown
    let contenido_doc = format!(r#"# Documentación de {}

## Versión: {}

## Descripción
{}

## Uso

```soris
// Ejemplo de uso
funcion main() {{
    consola.escribir("Hola desde {}");
}}
```

## API

### Funciones Disponibles

Consulte los archivos fuente en `src/` para detalles de implementación.

## Dependencias

{}

---
*Generado por Gotan*
"#,
        config.paquete.nombre,
        config.paquete.version,
        config.paquete.descripcion,
        config.paquete.nombre,
        if config.dependencias.is_empty() {
            "Sin dependencias externas".to_string()
        } else {
            config.dependencias
                .iter()
                .map(|(k, v)| format!("- {}: {}", k, v))
                .collect::<Vec<_>>()
                .join("\n")
        }
    );
    
    fs::write(dir_doc.join("README.md"), &contenido_doc)
        .map_err(|e| format!("Error al escribir documentación: {}", e))?;
    
    // Si se solicita abrir, intentar abrir con el navegador
    if abrir {
        #[cfg(target_os = "windows")]
        Command::new("cmd").args(["/c", "start", dir_doc.join("README.md").to_str().unwrap()]).spawn().ok();
        
        #[cfg(target_os = "macos")]
        Command::new("open").arg(dir_doc.join("README.md")).spawn().ok();
        
        #[cfg(target_os = "linux")]
        Command::new("xdg-open").arg(dir_doc.join("README.md")).spawn().ok();
    }
    
    Ok(dir_doc.join("README.md").display().to_string())
}

/// Transpila código Soris a Rust (implementación simplificada)
/// En producción, esto usaría el compilador completo de Soris
fn transpilar_a_rust(codigo_soris: &str) -> Result<String, String> {
    // Esta es una implementación placeholder
    // En producción, se integraría con el compilador real de Soris
    
    // Por ahora, generamos un programa Rust mínimo que imprima un mensaje
    // La integración real ocurrirá cuando Soris tenga el backend completo
    
    Ok(format!(r#"// Código generado por Soris/Gotan
// No editar manualmente

use std::io;

fn main() {{
    // Programa transpilado desde Soris
    println!("¡Hola desde Soris! (transpilado a Rust)");
    
    // Nota: Este es un código placeholder
    // La transpilación real se implementará cuando el backend de Soris esté completo
}}
"#))
}

/// Verifica las dependencias del proyecto
pub fn verificar_dependencias(ruta: &str) -> Result<Vec<String>, String> {
    let config = ConfiguracionProyecto::cargar(ruta)?;
    let mut faltantes = Vec::new();
    
    // Verificar que Rust/Cargo esté disponible
    if !crate::comando::comando_disponible("cargo") {
        faltantes.push("cargo (Rust)".to_string());
    }
    
    // Aquí se podrían verificar dependencias externas del proyecto
    for (nombre, _version) in &config.dependencias {
        // Lógica futura para verificar dependencias de Soris
        println!("Verificando dependencia: {}", nombre);
    }
    
    Ok(faltantes)
}
