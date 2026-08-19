//! Módulo de gestión de proyectos Soris
//! Maneja la creación, inicialización y consulta de proyectos

use crate::configuracion::{ConfiguracionProyecto, PaqueteInfo};
use std::fs;
use std::path::Path;

/// Información completa de un proyecto
#[derive(Debug)]
pub struct InformacionProyecto {
    pub nombre: String,
    pub version: String,
    pub tipo: String,
    pub autores: String,
    pub descripcion: String,
    pub dependencias: Vec<(String, String)>,
}

/// Crea un nuevo proyecto Soris con la estructura estándar
pub fn crear_proyecto(nombre: &str, tipo: &str) -> Result<(), String> {
    let camino = Path::new(nombre);
    
    // Verificar si el directorio ya existe
    if camino.exists() {
        return Err(format!("El directorio '{}' ya existe", nombre));
    }
    
    // Crear estructura de directorios
    fs::create_dir_all(camino.join("src"))
        .map_err(|e| format!("Error al crear directorios: {}", e))?;
    
    fs::create_dir_all(camino.join("pruebas"))
        .map_err(|e| format!("Error al crear directorio de pruebas: {}", e))?;
    
    // Crear archivo Proyecto.toml
    let config = ConfiguracionProyecto::nuevo(nombre, tipo);
    config.guardar(nombre)?;
    
    // Crear archivo principal según el tipo
    match tipo {
        "bin" => {
            let contenido_main = r#"// Programa principal en Soris
// Archivo: src/main.sr

funcion main() {
    consola.escribir("¡Hola desde Soris!");
}
"#;
            fs::write(camino.join("src/main.sr"), contenido_main)
                .map_err(|e| format!("Error al crear main.sr: {}", e))?;
        }
        "lib" => {
            let contenido_lib = r#"// Biblioteca en Soris
// Archivo: src/lib.sr

funcion saludar(nombre: texto) -> texto {
    retornar "¡Hola, ".concatenar(nombre).concatenar("!");
}

// Pruebas
#[prueba]
funcion prueba_saludar() {
    afirmar(saludar("Mundo") == "¡Hola, Mundo!");
}
"#;
            fs::write(camino.join("src/lib.sr"), contenido_lib)
                .map_err(|e| format!("Error al crear lib.sr: {}", e))?;
        }
        _ => return Err(format!("Tipo de proyecto no válido: {}. Use 'bin' o 'lib'", tipo)),
    }
    
    // Crear archivo .gitignore
    let gitignore = r#"# Archivos generados por Gotan
/target/
/Cargo.lock

# Archivos temporales
*.tmp
*.log

# IDE
.vscode/
.idea/
*.swp
*.swo

# Sistema operativo
.DS_Store
Thumbs.db
"#;
    fs::write(camino.join(".gitignore"), gitignore)
        .map_err(|e| format!("Error al crear .gitignore: {}", e))?;
    
    // Crear README.md
    let readme = format!(r#"# {}

{} 

## Requisitos

- [Soris](https://github.com/fundacionholtha/soris) - Lenguaje de programación en español
- [Gotan](https://github.com/fundacionholtha/gotan) - Gestor de paquetes para Soris

## Instalación

```bash
gotan compilar
```

## Uso

```bash
gotan ejecutar
```

## Desarrollo

```bash
gotan prueba
gotan documentacion
```

## Licencia

MIT
"#, nombre, config.paquete.descripcion);
    
    fs::write(camino.join("README.md"), readme)
        .map_err(|e| format!("Error al crear README.md: {}", e))?;
    
    Ok(())
}

/// Inicializa un proyecto Soris en el directorio actual
pub fn inicializar(nombre: Option<&str>) -> Result<(), String> {
    let nombre_proyecto = nombre.unwrap_or_else(|| {
        std::env::current_dir()
            .ok()
            .and_then(|p| p.file_name().map(|n| n.to_string_lossy().to_string()))
            .unwrap_or_else(|| "mi_proyecto".to_string())
    });
    
    let camino = Path::new(".");
    
    // Verificar si ya existe Proyecto.toml
    if camino.join("Proyecto.toml").exists() {
        return Err("Ya existe un Proyecto.toml en este directorio".to_string());
    }
    
    // Crear directorio src si no existe
    if !camino.join("src").exists() {
        fs::create_dir_all(camino.join("src"))
            .map_err(|e| format!("Error al crear directorio src: {}", e))?;
    }
    
    // Crear archivo Proyecto.toml
    let config = ConfiguracionProyecto::nuevo(&nombre_proyecto, "bin");
    config.guardar(".")?;
    
    // Crear main.sr si no existe
    let main_path = camino.join("src/main.sr");
    if !main_path.exists() {
        let contenido = r#"// Programa principal en Soris
funcion main() {
    consola.escribir("¡Hola desde Soris!");
}
"#;
        fs::write(&main_path, contenido)
            .map_err(|e| format!("Error al crear main.sr: {}", e))?;
    }
    
    Ok(())
}

/// Obtiene información detallada de un proyecto
pub fn informacion(ruta: &str) -> Result<InformacionProyecto, String> {
    let config = ConfiguracionProyecto::cargar(ruta)?;
    
    let autores_str = if config.paquete.autores.is_empty() {
        "No especificado".to_string()
    } else {
        config.paquete.autores.join(", ")
    };
    
    let dependencias_vec: Vec<(String, String)> = config.dependencias
        .iter()
        .map(|(k, v)| (k.clone(), v.clone()))
        .collect();
    
    Ok(InformacionProyecto {
        nombre: config.paquete.nombre,
        version: config.paquete.version,
        tipo: config.paquete.tipo,
        autores: autores_str,
        descripcion: config.paquete.descripcion,
        dependencias: dependencias_vec,
    })
}

/// Valida que un proyecto tenga la estructura correcta
pub fn validar_proyecto(ruta: &str) -> Result<(), String> {
    let camino = Path::new(ruta);
    
    // Verificar Proyecto.toml
    if !camino.join("Proyecto.toml").exists() {
        return Err("Falta el archivo Proyecto.toml".to_string());
    }
    
    // Cargar configuración para validar
    let config = ConfiguracionProyecto::cargar(ruta)?;
    
    // Verificar archivo principal
    let archivo_principal = camino.join(config.ruta_principal(ruta));
    if !archivo_principal.exists() {
        return Err(format!("Falta el archivo principal: {:?}", archivo_principal));
    }
    
    // Verificar extensión .sr
    if archivo_principal.extension().map_or(true, |ext| ext != "sr") {
        return Err("El archivo principal debe tener extensión .sr".to_string());
    }
    
    Ok(())
}
