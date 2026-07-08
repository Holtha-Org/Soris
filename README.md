# 🦀 Soris - Lenguaje de Programación en Español

![Logo](https://img.shields.io/badge/Soris-v0.1.0-orange)
![Rust](https://img.shields.io/badge/Rust-1.70+-red)
![Licencia](https://img.shields.io/badge/Licencia-MIT-blue)

**Soris** es un lenguaje de programación de bajo nivel en español, desarrollado por la **Fundación Holtha**. 
Sorris transpila a Rust, combinando la expresividad del español con el rendimiento y seguridad de Rust.

## ✨ Características

- 📝 **Sintaxis en español** - Pensá en español, programá en español
- 🚀 **Alto rendimiento** - Transpila a Rust, hereda su velocidad
- 🔒 **Seguro** - Sin null, sin data races, sin memory leaks
- 🎯 **Bajo nivel** - Control total sobre la memoria
- 📚 **Biblioteca estándar** - comandos útiles listos para usar
- 🔄 **Modo intérprete** - Ejecutá sin compilar

## 🏗️ Arquitectura del Compilador
Fuente .sr → Lexer → Parser → AST → Resolver → Type Checker
↓
Código Rust ← Compiler ← Optimizer ← MIR ← HIR



## 📦 Instalación

### Requisitos
- Rust 1.70 o superior
- Cargo

### Compilar desde fuente
```bash
git clone https://github.com/tuusuario/soris.git
cd soris
cargo build --release
🚀 Uso
Compilar un archivo Soris a Rust
bash
./target/release/soris programa.sr
# Genera: programa.rs
Interpretar directamente
bash
./target/release/soris programa.sr --interpretar
📝 Ejemplo
soris
autor:holtha;

declarar nombre = "Mundo";
imprimir texto.concatenar("¡Hola ", nombre, "!");

declarar contador = 0;
mientras (contador < 5) {
    imprimir contador;
    contador = contador + 1;
}
📚 Comandos Disponibles
Módulo	Comandos
consola	limpiar
io	leer_archivo, escribir_archivo
mat	raiz, potencia, absoluto, redondear
aleatorio	generar, generar_entre
texto	longitud, mayuscula, minuscula, concatenar, reemplazar
tiempo	dormir, ahora, medir
🗺️ Roadmap
Compilador funcional

19 comandos de stdlib

Structs y tipos personalizados

Pattern matching

Sistema de módulos

Async/Await

WASM como target

👤 Autor
Fundación Holtha - Creando herramientas en español para el mundo.

📄 Licencia
MIT © 2024 Fundación Holtha

Hecho con ❤️ en español



### Paso 3: Inicializar Git y subir

Ejecutá estos comandos en PowerShell desde la carpeta `C:\Users\oshir\Desktop\Soris`:

```bash
# Inicializar repositorio
git init

# Agregar todos los archivos
git add .

# Primer commit
git commit -m "🎉 Primer commit: Soris v0.1.0 - Compilador funcional con 19 comandos stdlib"

# Conectar con GitHub (cambiá la URL por la tuya)
git remote add origin https://github.com/TU-USUARIO/soris.git

# Subir
git branch -M main
git push -u origin main