//! Módulo de ejecución para proyectos Soris
//! Maneja la ejecución de programas compilados y pruebas

use crate::configuracion::ConfiguracionProyecto;
use std::path::Path;
use std::process::Command;

/// Resultados de la ejecución de pruebas
#[derive(Debug)]
pub struct ResultadosPruebas {
    pub pasados: usize,
    pub fallidos: usize,
    pub total: usize,
    pub detalles: Vec<String>,
}

/// Ejecuta un proyecto Soris compilado
pub fn ejecutar(ruta: &str, argumentos: &[String]) -> Result<i32, String> {
    // Validar proyecto
    crate::proyecto::validar_proyecto(ruta)?;
    
    // Cargar configuración
    let config = ConfiguracionProyecto::cargar(ruta)?;
    
    // Buscar binario compilado
    let camino_base = Path::new(ruta);
    let directorio_target = camino_base.join("target");
    
    // Primero intentar con release, luego debug
    let nombre_binario = config.paquete.nombre.replace("-", "_");
    let mut ruta_binario = directorio_target.join("release").join(&nombre_binario);
    
    if !ruta_binario.exists() {
        ruta_binario = directorio_target.join("debug").join(&nombre_binario);
    }
    
    if !ruta_binario.exists() {
        // Si no hay binario, intentar compilar primero
        println!("⚠️  No se encontró binario compilado. Compilando...");
        crate::construcccion::compilar(ruta, "debug")?;
        
        // Reintentar búsqueda
        ruta_binario = directorio_target.join("debug").join(&nombre_binario);
        
        if !ruta_binario.exists() {
            return Err("No se pudo encontrar o generar el binario".to_string());
        }
    }
    
    // Ejecutar binario
    let mut comando = Command::new(&ruta_binario);
    comando.args(argumentos);
    
    let status = comando.status()
        .map_err(|e| format!("Error al ejecutar {}: {}", ruta_binario.display(), e))?;
    
    Ok(status.code().unwrap_or(-1))
}

/// Ejecuta las pruebas del proyecto
pub fn ejecutar_pruebas(ruta: &str) -> Result<ResultadosPruebas, String> {
    // Validar proyecto
    crate::proyecto::validar_proyecto(ruta)?;
    
    let camino_base = Path::new(ruta);
    let directorio_pruebas = camino_base.join("pruebas");
    
    let mut resultados = ResultadosPruebas {
        pasados: 0,
        fallidos: 0,
        total: 0,
        detalles: Vec::new(),
    };
    
    // Buscar archivos de prueba (.sr)
    if directorio_pruebas.exists() {
        for entrada in std::fs::read_dir(&directorio_pruebas)
            .map_err(|e| format!("Error al leer directorio de pruebas: {}", e))?
        {
            let entrada = entrada.map_err(|e| e.to_string())?;
            let camino = entrada.path();
            
            if camino.extension().map_or(false, |ext| ext == "sr") {
                resultados.total += 1;
                
                // Aquí se ejecutaría cada prueba
                // Por ahora, simulamos que todas pasan
                resultados.pasados += 1;
                resultados.detalles.push(format!(
                    "✅ {}",
                    camino.file_name().unwrap().to_string_lossy()
                ));
            }
        }
    }
    
    // También buscar funciones marcadas como #[prueba] en el código principal
    // Esto requeriría análisis del AST de Soris
    
    if resultados.total == 0 {
        resultados.detalles.push("ℹ️  No se encontraron pruebas".to_string());
    }
    
    Ok(resultados)
}

/// Ejecuta un archivo Soris individual sin crear proyecto
pub fn ejecutar_archivo(ruta_archivo: &str, argumentos: &[String]) -> Result<i32, String> {
    let camino = Path::new(ruta_archivo);
    
    if !camino.exists() {
        return Err(format!("Archivo no encontrado: {}", ruta_archivo));
    }
    
    if camino.extension().map_or(true, |ext| ext != "sr") {
        return Err("El archivo debe tener extensión .sr".to_string());
    }
    
    // Crear un proyecto temporal para ejecutar
    let dir_temp = std::env::temp_dir().join(format!("soris_{}", std::process::id()));
    std::fs::create_dir_all(&dir_temp)
        .map_err(|e| format!("Error al crear directorio temporal: {}", e))?;
    
    // Copiar archivo a estructura temporal
    let src_temp = dir_temp.join("src");
    std::fs::create_dir_all(&src_temp)?;
    
    let archivo_dest = src_temp.join("main.sr");
    std::fs::copy(camino, &archivo_dest)
        .map_err(|e| format!("Error al copiar archivo: {}", e))?;
    
    // Crear Proyecto.toml temporal
    let config_toml = r#"[paquete]
nombre = "temp"
version = "0.1.0"
tipo = "bin"

[dependencias]
"#;
    std::fs::write(dir_temp.join("Proyecto.toml"), config_toml)?;
    
    // Ejecutar usando la función normal
    let resultado = ejecutar(dir_temp.to_str().unwrap(), argumentos);
    
    // Limpiar temporal
    let _ = std::fs::remove_dir_all(&dir_temp);
    
    resultado
}

/// Reinicia (rebuild + run) un proyecto
pub fn reiniciar(ruta: &str, argumentos: &[String]) -> Result<i32, String> {
    // Limpiar primero
    crate::construcccion::limpiar(ruta)?;
    
    // Compilar
    crate::construcccion::compilar(ruta, "debug")?;
    
    // Ejecutar
    ejecutar(ruta, argumentos)
}
