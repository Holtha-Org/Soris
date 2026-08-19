//! Módulo de gestión de dependencias para proyectos Soris
//! Maneja la búsqueda, instalación y actualización de paquetes

use crate::configuracion::ConfiguracionProyecto;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

/// Información de un paquete en el registro
#[derive(Debug, Clone)]
pub struct PaqueteRegistro {
    pub nombre: String,
    pub version: String,
    pub descripcion: String,
    pub repositorio: String,
}

/// Registro de paquetes Soris (simulado)
/// En producción, esto se conectaría a un servidor real
const REGISTRO_URL: &str = "https://registro.soris.dev";

/// Busca un paquete en el registro
pub fn buscar_paquete(nombre: &str) -> Result<Vec<PaqueteRegistro>, String> {
    // Simulación de búsqueda en registro
    // En producción, haría una petición HTTP al registro
    
    let paquetes_simulados = vec![
        PaqueteRegistro {
            nombre: "consola_plus".to_string(),
            version: "1.2.0".to_string(),
            descripcion: "Funciones avanzadas de consola para Soris".to_string(),
            repositorio: "https://github.com/soris/consola_plus".to_string(),
        },
        PaqueteRegistro {
            nombre: "matematicas".to_string(),
            version: "2.0.1".to_string(),
            descripcion: "Biblioteca matemática extendida".to_string(),
            repositorio: "https://github.com/soris/matematicas".to_string(),
        },
    ];
    
    if nombre.is_empty() {
        Ok(paquetes_simulados)
    } else {
        Ok(paquetes_simulados
            .into_iter()
            .filter(|p| p.nombre.contains(nombre))
            .collect())
    }
}

/// Añade una dependencia al proyecto actual
pub fn agregar(nombre: &str, version: &str) -> Result<(), String> {
    let ruta_actual = std::env::current_dir()
        .map_err(|e| format!("Error al obtener directorio actual: {}", e))?;
    
    let mut config = ConfiguracionProyecto::cargar(ruta_actual.to_str().unwrap())?;
    
    // Verificar si la dependencia existe en el registro
    let resultados = buscar_paquete(nombre)?;
    if resultados.is_empty() && version != "latest" {
        println!("⚠️  Advertencia: '{}' no encontrado en el registro oficial", nombre);
        println!("   Se añadirá como dependencia local");
    }
    
    // Añadir a dependencias
    config.agregar_dependencia(nombre, version);
    
    // Guardar cambios
    config.guardar(ruta_actual.to_str().unwrap())?;
    
    // Descargar e instalar la dependencia
    instalar_dependencia(nombre, version, &ruta_actual)?;
    
    println!("✅ Dependencia '{}@{}' añadida exitosamente", nombre, version);
    
    Ok(())
}

/// Instala una dependencia específica
fn instalar_dependencia(nombre: &str, version: &str, proyecto_ruta: &Path) -> Result<(), String> {
    let dir_dependencias = proyecto_ruta.join("dependencias");
    fs::create_dir_all(&dir_dependencias)
        .map_err(|e| format!("Error al crear directorio de dependencias: {}", e))?;
    
    let dir_paquete = dir_dependencias.join(format!("{}-{}", nombre, version));
    
    if dir_paquete.exists() {
        println!("ℹ️  La dependencia ya está instalada");
        return Ok(());
    }
    
    // Simular descarga del paquete
    // En producción, descargaría desde el repositorio
    
    println!("📦 Descargando {}@{}...", nombre, version);
    
    // Crear estructura básica del paquete
    fs::create_dir_all(&dir_paquete)?;
    fs::create_dir_all(dir_paquete.join("src"))?;
    
    // Crear un Proyecto.toml básico para la dependencia
    let toml_content = format!(r#"[paquete]
nombre = "{}"
version = "{}"
tipo = "lib"
descripcion = "Dependencia instalada por Gotan"
"#, nombre, version);
    
    fs::write(dir_paquete.join("Proyecto.toml"), toml_content)?;
    
    // Crear archivo lib.sr placeholder
    let lib_content = format!(r#"// Biblioteca {} v{}
// Generada por Gotan

// Las funciones reales se descargarán del repositorio
"#, nombre, version);
    
    fs::write(dir_paquete.join("src").join("lib.sr"), lib_content)?;
    
    Ok(())
}

/// Actualiza las dependencias del proyecto
pub fn actualizar(nombre: Option<&str>) -> Result<(), String> {
    let ruta_actual = std::env::current_dir()
        .map_err(|e| format!("Error al obtener directorio actual: {}", e))?;
    
    let mut config = ConfiguracionProyecto::cargar(ruta_actual.to_str().unwrap())?;
    
    if let Some(nombre_dep) = nombre {
        // Actualizar dependencia específica
        if let Some(version_actual) = config.dependencias.get(nombre_dep) {
            println!("🔄 Actualizando {} de {} a latest...", nombre_dep, version_actual);
            
            // Buscar última versión
            let resultados = buscar_paquete(nombre_dep)?;
            let nueva_version = resultados
                .first()
                .map(|p| p.version.as_str())
                .unwrap_or("latest");
            
            config.dependencias.insert(nombre_dep.to_string(), nueva_version.to_string());
            
            // Reinstalar con nueva versión
            instalar_dependencia(nombre_dep, nueva_version, &ruta_actual)?;
        } else {
            return Err(format!("La dependencia '{}' no está en el proyecto", nombre_dep));
        }
    } else {
        // Actualizar todas las dependencias
        for (nombre_dep, version_actual) in config.dependencias.clone() {
            println!("🔄 Actualizando {} de {} a latest...", nombre_dep, version_actual);
            
            let resultados = buscar_paquete(&nombre_dep)?;
            let nueva_version = resultados
                .first()
                .map(|p| p.version.as_str())
                .unwrap_or("latest");
            
            config.dependencias.insert(nombre_dep.clone(), nueva_version.to_string());
            instalar_dependencia(&nombre_dep, nueva_version, &ruta_actual)?;
        }
    }
    
    config.guardar(ruta_actual.to_str().unwrap())?;
    
    println!("✅ Dependencias actualizadas");
    
    Ok(())
}

/// Elimina una dependencia del proyecto
pub fn eliminar(nombre: &str) -> Result<(), String> {
    let ruta_actual = std::env::current_dir()
        .map_err(|e| format!("Error al obtener directorio actual: {}", e))?;
    
    let mut config = ConfiguracionProyecto::cargar(ruta_actual.to_str().unwrap())?;
    
    if !config.eliminar_dependencia(nombre) {
        return Err(format!("La dependencia '{}' no está en el proyecto", nombre));
    }
    
    config.guardar(ruta_actual.to_str().unwrap())?;
    
    // Eliminar directorio de la dependencia
    let dir_dependencias = ruta_actual.join("dependencias");
    for entrada in fs::read_dir(&dir_dependencias).ok().into_iter().flatten() {
        if let Ok(entrada) = entrada {
            let nombre_archivo = entrada.file_name().to_string_lossy().to_string();
            if nombre_archivo.starts_with(&format!("{}-", nombre)) {
                fs::remove_dir_all(entrada.path()).ok();
            }
        }
    }
    
    println!("✅ Dependencia '{}' eliminada", nombre);
    
    Ok(())
}

/// Lista las dependencias instaladas
pub fn listar(ruta: &str) -> Result<Vec<(String, String)>, String> {
    let config = ConfiguracionProyecto::cargar(ruta)?;
    
    Ok(config.dependencias
        .iter()
        .map(|(k, v)| (k.clone(), v.clone()))
        .collect())
}

/// Publica un paquete en el registro
pub fn publicar(ruta: &str) -> Result<String, String> {
    let config = ConfiguracionProyecto::cargar(ruta)?;
    
    // Validar que el paquete tenga la información requerida
    if config.paquete.nombre.is_empty() {
        return Err("El paquete debe tener un nombre".to_string());
    }
    
    if config.paquete.version.is_empty() {
        return Err("El paquete debe tener una versión".to_string());
    }
    
    // En producción, aquí se subiría el paquete al registro
    // Por ahora, simulamos la publicación
    
    let url_publicacion = format!("{}/paquetes/{}@{}", 
        REGISTRO_URL, 
        config.paquete.nombre, 
        config.paquete.version);
    
    println!("📤 Preparando publicación de {}@{}...", 
        config.paquete.nombre, 
        config.paquete.version);
    
    // Simular empaquetado
    println!("📦 Empaquetando...");
    
    // Simular subida
    println!("⬆️  Subiendo al registro...");
    
    println!("✅ Paquete publicado exitosamente");
    
    Ok(url_publicacion)
}

/// Verifica el árbol de dependencias
pub fn arbol_dependencias(ruta: &str) -> Result<String, String> {
    let config = ConfiguracionProyecto::cargar(ruta)?;
    
    let mut resultado = String::new();
    resultado.push_str(&format!("{} {}\n", config.paquete.nombre, config.paquete.version));
    
    for (nombre, version) in &config.dependencias {
        resultado.push_str(&format!("├── {}@{}\n", nombre, version));
        
        // Aquí se podrían cargar las dependencias transitivas
        // de cada paquete si estuvieran disponibles
    }
    
    Ok(resultado)
}
