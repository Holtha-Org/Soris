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

Ejemplo de código Soris:

```soris
// Hola Mundo
di!("Hola, mundo");

// Variables
var x: ent = 10;
var y: flot = 3.14;
var nombre: cad = "Juan";

// Condicionales
si (x > 5) {
    di!("x es mayor que 5");
} elsi (x == 5) {
    di!("x es igual a 5");
} sino {
    di!("x es menor que 5");
}

// Bucles
mientras (x > 0) {
    di!(x);
    x = x - 1;
}

para i en 0..5 {
    di!(i);
}

// Funciones
fn suma(a: ent, b: ent) -> ent {
    retorna a + b;
}

var resultado = suma(3, 4);
di!(resultado);
```

## 📚 Palabras Clave

### Control de Flujo
- `si` - condicional
- `elsi` - else if
- `sino` - else
- `mientras` - while loop
- `para en` - for loop
- `retorna` - return

### Declaraciones
- `var` - variable
- `const` - constante
- `fn` - función
- `rasgo` - trait
- `struct` - estructura
- `enum` - enumeración

### Tipos
- **Enteros**: `ent`, `ent8`, `ent16`, `ent64`, `ent128`
- **Enteros sin signo**: `ent8s`, `ent16s`, `ent32s`, `ent64s`, `ent128s`
- **Flotantes**: `flot`, `f32`, `f64`
- **Otros**: `car`, `cad`, `txt`, `bool`, `opt`, `result`

### Operadores Lógicos
- `y` - AND lógico
- `o` - OR lógico
- `!` - NOT lógico

### Otros
- `pub` - público
- `mut` - mutable
- `verdadero` - true
- `falso` - false
- `nada` - None
- `err` - Error

## 📚 Biblioteca Estándar

Soris proporciona acceso a funciones de la biblioteca estándar de Rust a través de módulos:

### Módulo `consola`
- `consola.limpiar()` - Limpia la pantalla

### Módulo `io` 
- `io.leer()` - Lee entrada del usuario
- `io.escribir(texto)` - Escribe a archivo

### Módulo `mat` (Matemáticas)
- `mat.raiz(n)` - Raíz cuadrada
- `mat.potencia(base, exp)` - Potencia
- `mat.absoluto(n)` - Valor absoluto
- `mat.redondear(n)` - Redondear

### Módulo `aleatorio`
- `aleatorio.generar()` - Número aleatorio
- `aleatorio.generar_entre(min, max)` - Aleatorio en rango

### Módulo `texto` (Strings)
- `texto.longitud(s)` - Longitud de cadena
- `texto.mayuscula(s)` - Convertir a mayúscula
- `texto.minuscula(s)` - Convertir a minúscula
- `texto.concatenar(...)` - Concatenar cadenas
- `texto.reemplazar(s, from, to)` - Reemplazar

### Módulo `tiempo`
- `tiempo.dormir(ms)` - Dormir milisegundos
- `tiempo.ahora()` - Tiempo actual
- `tiempo.medir()` - Medir tiempo de ejecución

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