pub mod console;
pub mod io;
pub mod math;
pub mod random;
pub mod string;
pub mod time;

use std::collections::HashMap;

pub trait StdlibFunc {
    fn nombre(&self) -> &str;
    fn comandos_soportados(&self) -> Vec<&str>;
    fn generar_codigo(&self, comando: &str, argumentos: &[String]) -> String;
}

pub struct GestorStdlib {
    modulos: HashMap<String, Box<dyn StdlibFunc>>,
}

impl GestorStdlib {
    pub fn new() -> Self {
        let mut gestor = Self {
            modulos: HashMap::new(),
        };

        gestor.registrar(Box::new(console::Console));
        gestor.registrar(Box::new(io::Io));
        gestor.registrar(Box::new(math::Math));
        gestor.registrar(Box::new(random::Random));
        gestor.registrar(Box::new(string::StringFunc));
        gestor.registrar(Box::new(time::Time));

        gestor
    }

    fn registrar(&mut self, modulo: Box<dyn StdlibFunc>) {
        self.modulos.insert(modulo.nombre().to_string(), modulo);
    }

    pub fn generar_llamada(&self, nombre_completo: &str, argumentos: &[String]) -> Option<String> {
        let partes: Vec<&str> = nombre_completo.splitn(2, '.').collect();
        if partes.len() != 2 {
            return None;
        }

        let (modulo_nombre, comando) = (partes[0], partes[1]);

        if let Some(modulo) = self.modulos.get(modulo_nombre) {
            if modulo.comandos_soportados().contains(&comando) {
                return Some(modulo.generar_codigo(comando, argumentos));
            }
        }

        None
    }
}