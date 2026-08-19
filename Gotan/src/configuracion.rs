//! Módulo de configuración para proyectos Soris
//! Maneja la lectura y escritura de archivos Proyecto.toml

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

/// Estructura principal de configuración de un proyecto Soris
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfiguracionProyecto {
    /// Información del paquete
    pub paquete: PaqueteInfo,
    
    /// Dependencias del proyecto
    #[serde(default)]
    pub dependencias: HashMap<String, String>,
    
    /// Dependencias de desarrollo
    #[serde(default)]
    pub dependencias_desarrollo: HashMap<String, String>,
    
    /// Configuración de compilación
    #[serde(default)]
    pub compilacion: ConfiguracionCompilacion,
    
    /// Metadatos adicionales
    #[serde(default)]
    pub metadatos: HashMap<String, String>,
}

/// Información básica del paquete
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaqueteInfo {
    /// Nombre del proyecto
    pub nombre: String,
    
    /// Versión del proyecto (formato semántico)
    pub version: String,
    
    /// Autores del proyecto
    #[serde(default)]
    pub autores: Vec<String>,
    
    /// Descripción del proyecto
    #[serde(default)]
    pub descripcion: String,
    
    /// Licencia del proyecto
    #[serde(default)]
    pub licencia: Option<String>,
    
    /// Tipo de proyecto: "bin" o "lib"
    #[serde(default = "tipo_por_defecto")]
    pub tipo: String,
    
    /// Punto de entrada principal
    #[serde(default)]
    pub entrada: Option<String>,
}

fn tipo_por_defecto() -> String {
    "bin".to_string()
}

/// Configuración de compilación
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ConfiguracionCompilacion {
    /// Nivel de optimización (0-3)
    #[serde(default)]
    pub optimizacion: u8,
    
    /// Características a habilitar
    #[serde(default)]
    pub caracteristicas: Vec<String>,
    
    /// Rutas de búsqueda adicionales
    #[serde(default)]
    pub rutas_busqueda: Vec<String>,
    
    /// Configuración específica para debug
    #[serde(default)]
    pub debug: ConfiguracionPerfil,
    
    /// Configuración específica para release
    #[serde(default)]
    pub release: ConfiguracionPerfil,
}

/// Configuración de un perfil de compilación
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ConfiguracionPerfil {
    /// Habilitar símbolos de debug
    #[serde(default = "bool_true")]
    pub simbolos: bool,
    
    /// Nivel de optimización específico
    pub optimizacion: Option<u8>,
    
    /// Características específicas del perfil
    #[serde(default)]
    pub caracteristicas: Vec<String>,
}

fn bool_true() -> bool {
    true
}

impl ConfiguracionProyecto {
    /// Carga la configuración desde un archivo Proyecto.toml
    pub fn cargar(ruta: &str) -> Result<Self, String> {
        let camino = Path::new(ruta);
        let archivo_toml = if camino.is_dir() {
            camino.join("Proyecto.toml")
        } else {
            camino.to_path_buf()
        };
        
        let contenido = fs::read_to_string(&archivo_toml)
            .map_err(|e| format!("Error al leer '{}': {}", archivo_toml.display(), e))?;
        
        toml::from_str(&contenido)
            .map_err(|e| format!("Error al parsear Proyecto.toml: {}", e))
    }
    
    /// Guarda la configuración en un archivo Proyecto.toml
    pub fn guardar(&self, ruta: &str) -> Result<(), String> {
        let camino = Path::new(ruta);
        let archivo_toml = if camino.is_dir() {
            camino.join("Proyecto.toml")
        } else {
            camino.to_path_buf()
        };
        
        let contenido = toml::to_string_pretty(self)
            .map_err(|e| format!("Error al serializar: {}", e))?;
        
        fs::write(&archivo_toml, contenido)
            .map_err(|e| format!("Error al escribir '{}': {}", archivo_toml.display(), e))
    }
    
    /// Crea una configuración por defecto para un nuevo proyecto
    pub fn nuevo(nombre: &str, tipo: &str) -> Self {
        ConfiguracionProyecto {
            paquete: PaqueteInfo {
                nombre: nombre.to_string(),
                version: "0.1.0".to_string(),
                autores: vec![],
                descripcion: format!("Proyecto Soris: {}", nombre),
                licencia: None,
                tipo: tipo.to_string(),
                entrada: None,
            },
            dependencias: HashMap::new(),
            dependencias_desarrollo: HashMap::new(),
            compilacion: ConfiguracionCompilacion::default(),
            metadatos: HashMap::new(),
        }
    }
    
    /// Añade una dependencia al proyecto
    pub fn agregar_dependencia(&mut self, nombre: &str, version: &str) {
        self.dependencias.insert(nombre.to_string(), version.to_string());
    }
    
    /// Elimina una dependencia del proyecto
    pub fn eliminar_dependencia(&mut self, nombre: &str) -> bool {
        self.dependencias.remove(nombre).is_some()
    }
    
    /// Obtiene la ruta del archivo principal según el tipo de proyecto
    pub fn ruta_principal(&self, base: &str) -> String {
        if let Some(entrada) = &self.paquete.entrada {
            return entrada.clone();
        }
        
        match self.paquete.tipo.as_str() {
            "bin" => format!("{}/src/main.sr", base),
            "lib" => format!("{}/src/lib.sr", base),
            _ => format!("{}/src/main.sr", base),
        }
    }
}

/// Obtiene información básica de un proyecto sin cargar toda la configuración
pub fn obtener_info_basica(ruta: &str) -> Result<(String, String, String), String> {
    let config = ConfiguracionProyecto::cargar(ruta)?;
    Ok((
        config.paquete.nombre,
        config.paquete.version,
        config.paquete.tipo,
    ))
}
