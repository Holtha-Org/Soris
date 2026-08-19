# Gotan 🛠️

**Gestor de paquetes y compilador para proyectos Soris**

Gotan es el equivalente a Cargo para el ecosistema Soris. Permite crear, compilar, ejecutar y gestionar dependencias de proyectos escritos en Soris (.sr).

## Características

- 📦 **Gestión de proyectos**: Crea nuevos proyectos con estructura estándar
- 🔨 **Compilación**: Transpila código Soris a Rust y genera binarios nativos
- 🚀 **Ejecución**: Ejecuta proyectos compilados
- 📚 **Documentación**: Genera documentación automática
- 🧪 **Pruebas**: Sistema de testing integrado
- 🔄 **Dependencias**: Gestión completa de paquetes y dependencias
- 📤 **Publicación**: Publica paquetes en el registro

## Instalación

```bash
cd Gotan
cargo build --release
```

El binario se encontrará en `target/release/gotan`.

## Comandos Disponibles

### Crear nuevo proyecto
```bash
gotan nuevo mi_proyecto
gotan nuevo mi_biblioteca --tipo lib
```

### Compilar
```bash
gotan compilar              # Compila en modo debug
gotan compilar --modo release  # Compila en modo release
```

### Ejecutar
```bash
gotan ejecutar              # Ejecuta el proyecto
gotan ejecutar arg1 arg2    # Con argumentos
```

### Información del proyecto
```bash
gotan info                  # Muestra detalles del proyecto
```

### Gestión de dependencias
```bash
gotan agregar consola_plus      # Añade una dependencia
gotan agregar matematicas@2.0   # Versión específica
gotan listar                    # Lista dependencias
gotan actualizar                # Actualiza todas
gotan actualizar nombre         # Actualiza específica
gotan eliminar nombre           # Elimina dependencia
gotan arbol                     # Muestra árbol de dependencias
```

### Pruebas
```bash
gotan prueba                # Ejecuta pruebas
```

### Documentación
```bash
gotan documentacion         # Genera documentación
gotan documentacion --abrir # Abre en navegador
```

### Publicación
```bash
gotan publicar              # Publica en el registro
```

### Inicializar proyecto existente
```bash
gotan iniciar               # Inicializa en directorio actual
gotan iniciar --nombre mi_proyecto
```

### Limpieza
```bash
gotan limpiar               # Elimina archivos generados
```

## Estructura de Proyecto

Un proyecto Soris típico tiene esta estructura:

```
mi_proyecto/
├── Proyecto.toml       # Configuración (equivalente a Cargo.toml)
├── src/
│   ├── main.sr         # Punto de entrada (para binarios)
│   └── lib.sr          # Biblioteca (para libs)
├── pruebas/            # Archivos de prueba
├── dependencias/       # Dependencias instaladas
├── target/             # Artefactos de compilación
└── doc/                # Documentación generada
```

## Proyecto.toml

Ejemplo de archivo de configuración:

```toml
[paquete]
nombre = "mi_proyecto"
version = "0.1.0"
autores = ["Tu Nombre"]
descripcion = "Mi proyecto en Soris"
tipo = "bin"

[dependencias]
consola_plus = "1.2.0"
matematicas = "2.0"

[compilacion]
optimizacion = 2
```

## Integración con Soris

Gotan utiliza el compilador de Soris para transpilar código `.sr` a Rust, y luego usa Cargo internamente para generar el binario final.

Flujo completo:
```
main.sr → Compilador Soris → main.rs → Cargo → Binario nativo
```

## Comandos Equivalentes a Cargo

| Cargo | Gotan |
|-------|-------|
| `cargo new` | `gotan nuevo` |
| `cargo build` | `gotan compilar` |
| `cargo run` | `gotan ejecutar` |
| `cargo test` | `gotan prueba` |
| `cargo doc` | `gotan documentacion` |
| `cargo add` | `gotan agregar` |
| `cargo update` | `gotan actualizar` |
| `cargo publish` | `gotan publicar` |
| `cargo clean` | `gotan limpiar` |

## Requisitos

- [Rust](https://rust-lang.org) - Para compilación final
- [Soris](https://github.com/fundacionholtha/soris) - Compilador Soris

## Desarrollo

Para contribuir al desarrollo de Gotan:

```bash
cargo check        # Verifica el código
cargo test         # Ejecuta pruebas
cargo clippy       # Análisis de código
cargo fmt          # Formatea el código
```

## Licencia

MIT - Fundación Holtha

---

**Hecho con ❤️ para la comunidad Soris**
