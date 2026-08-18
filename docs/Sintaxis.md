# Guía de Sintaxis de Soris

**Autor: @holtha**

## Introducción

Soris es un lenguaje de programación en español transpilado a Rust nativo. Ofrece una sintaxis clara y expresiva para desarrollar aplicaciones eficientes.

## Primer Programa

```soris
// Hola Mundo
di!("Hola, mundo");
```

## Tipos de datos

### Tipos numéricos con signo
- `ent` - Entero de 32 bits (predeterminado)
- `ent8` - Entero de 8 bits
- `ent16` - Entero de 16 bits
- `ent64` - Entero de 64 bits
- `ent128` - Entero de 128 bits

### Tipos numéricos sin signo
- `ent8s` - Entero sin signo de 8 bits
- `ent16s` - Entero sin signo de 16 bits
- `ent32s` - Entero sin signo de 32 bits
- `ent64s` - Entero sin signo de 64 bits
- `ent128s` - Entero sin signo de 128 bits

### Tipos flotantes
- `flot` - Número flotante de 64 bits (predeterminado)
- `f32` - Número flotante de 32 bits
- `f64` - Número flotante de 64 bits

### Otros tipos
- `car` - Carácter individual
- `cad` - Cadena de texto (String)
- `txt` - Referencia a cadena (&str)
- `bool` - Booleano (verdadero/falso)

### Tipos especiales
- `opt` - Tipo opcional
- `result` - Tipo para manejar errores
- `alg` - Tipo genérico

### Literales

- Números: `42`, `3.14`, `ent 100`, `flot 2.5`
- Booleanos: `verdadero`, `falso`
- Texto: `"hola mundo"`
- Nada: `nada`
- Error: `err`


## Operadores

### Aritméticos

- Suma: `+`
- Resta: `-`
- Multiplicación: `*`
- División: `/`
- Módulo: `%`

### Comparación

- Igual: `==`
- No igual: `!=`
- Mayor: `>`
- Menor: `<`
- Mayor o igual: `>=`
- Menor o igual: `<=`

### Lógicos

- Y: `y` (equivalente a `&&`)
- O: `o` (equivalente a `||`)
- No: `!` (negación)

### Asignación

- Asignación: `=`
- Suma y asignación: `+=`
- Resta y asignación: `-=`
- Multiplicación y asignación: `*=`
- División y asignación: `/=`

## Estructuras de control

### Condicional Si/ElSi/Sino

```soris
si (condicion) {
    di!("condición verdadera");
} elsi (otra_condicion) {
    di!("otra condición verdadera");
} sino {
    di!("ninguna condición se cumplió");
}
```

### Bucle Mientras

```soris
mientras (condicion) {
    di!("se repite mientras la condición sea verdadera");
}
```

### Bucle Para

```soris
para i en 0..10 {
    di!(i);
}
```

## Declaración y Asignación

### Declarar variable

```soris
var nombre = valor;
var edad: ent = 25;
var altura: flot = 1.75;
var activo: bool = verdadero;
```

### Asignar valor

```soris
nombre = nuevo_valor;
```

## Macro di!

La macro `di!` es usada para imprimir valores en la consola:

```soris
di!("Hola");
di!(42);
di!(variable);
di!(x + y);
```

## Funciones

### Definir función

```soris
fn nombre_funcion() {
    di!("cuerpo de la función");
}

fn suma(a: ent, b: ent) -> ent {
    retorna a + b;
}

fn dividir(a: flot, b: flot) -> result {
    si (b == 0.0) {
        retorna err "División por cero";
    }
    retorna a / b;
}
```

### Llamar función

```soris
nombre_funcion();
var resultado = suma(5, 3);
```

## Rasgos (Traits)

```soris
rasgo Mostrable {
    fn mostrar();
}
```

## Comentarios

Los comentarios se escriben con `//`:

```soris
// Esto es un comentario de línea
di!("Código"); // Comentario al final de la línea
```




## Comandos (40 disponibles)

### Entrada/Salida

- `escribir(texto)` - Muestra texto en consola
- `leer(variable)` - Lee entrada del usuario
- `limpiar()` - Limpia la consola

### Matemáticos

- `sumar(a, b)` - Suma dos números
- `restar(a, b)` - Resta dos números
- `multiplicar(a, b)` - Multiplica dos números
- `dividir(a, b)` - Divide dos números
- `aleatorio()` - Genera un número aleatorio
- `raiz(n)` - Calcula la raíz cuadrada
- `potencia(base, exp)` - Potencia de un número
- `log(n)` - Logaritmo natural
- `seno(x)` - Seno de un ángulo
- `coseno(x)` - Coseno de un ángulo
- `tangente(x)` - Tangente de un ángulo
- `arcoseno(x)` - Arcoseno
- `arcocoseno(x)` - Arcocoseno
- `arcotangente(x)` - Arcotangente
- `redondear(n)` - Redondea un número
- `truncar(n)` - Trunca un número
- `abs(n)` - Valor absoluto
- `maximo(a, b)` - Máximo de dos números
- `minimo(a, b)` - Mínimo de dos números
- `clamp(n, min, max)` - Limita un número entre min y max

### Texto

- `longitud(texto)` - Longitud de un texto
- `concatenar(a, b)` - Concatena dos textos
- `subcadena(texto, inicio, longitud)` - Extrae una subcadena
- `reemplazar(texto, buscar, reemplazar)` - Reemplaza texto
- `mayusculas(texto)` - Convierte a mayúsculas
- `minusculas(texto)` - Convierte a minúsculas
- `trim(texto)` - Elimina espacios en blanco
- `dividir_texto(texto, separador)` - Divide texto en partes
- `unir_texto(lista, separador)` - Une una lista con separador
- `contener(texto, subtexto)` - Verifica si contiene subtexto
- `iniciar_con(texto, prefijo)` - Verifica si inicia con prefijo
- `terminar_con(texto, sufijo)` - Verifica si termina con sufijo
- `posicion(texto, subtexto)` - Posición de subtexto
- `ultimo_indice(texto, subtexto)` - Última posición
- `eliminar(texto, indice)` - Elimina carácter en posición
- `insertar(texto, indice, caracter)` - Inserta un carácter

### Sistema

- `dormir(ms)` - Pausa en milisegundos

## Tipos de retorno de comandos

### Comandos que retornan valor (expresiones)

- Todos los matemáticos
- Todos los de texto
- `aleatorio()`
- `longitud(texto)`
- `posicion(texto, subtexto)`
- `ultimo_indice(texto, subtexto)`
- `contener(texto, subtexto)`
- `iniciar_con(texto, prefijo)`
- `terminar_con(texto, sufijo)`

### Comandos de acción (no retornan valor)

- `escribir(texto)`
- `leer(variable)`
- `limpiar()`
- `dormir(ms)`

## Ejemplo completo
firma "autor:@holtha"

declarar nombre = "visitante"
declarar edad = 25

escribir("Como te llamas?")
leer(nombre)

si (edad >= 18) {
escribir("Eres mayor de edad, " + nombre)
} sino {
escribir("Eres menor de edad, " + nombre)
}

declarar contador = 0
mientras (contador < 10) {
contador = contador + 1
escribir(contador)
}

declarar raiz_cuadrada = raiz(edad)
escribir("La raiz cuadrada de tu edad es: " + raiz_cuadrada)

declarar saludo = concatenar("Hola ", nombre)
escribir(saludo)



## Transpilación

1. Guarda tu código en un archivo con extensión `.sr`
2. Ejecuta: `soris programa.sr`
3. Se generará un archivo `out.rs` y luego se compilará a un ejecutable nativo con `rustc`

## Notas importantes

- La firma `firma "autor:@holtha"` es obligatoria al inicio de cada archivo
- Las variables deben declararse antes de usarse
- Los tipos se infieren automáticamente
- Error de tipo: `Texto + Numero` no esta permitido
- Los comandos pueden ser usados como expresiones o sentencias según su tipo
- Los bloques de código usan llaves `{}`
- Las condiciones en `si` y `mientras` deben ser booleanas
- El punto y coma `;` es opcional al final de cada linea