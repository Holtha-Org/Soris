# Guía de Sintaxis de Soris

**Autor: @holtha**

## Introducción

Soris es un lenguaje transpilado a Rust nativo. Soporta 40 comandos de la Fundación Holtha.

## Firma obligatoria

Todo archivo `.sr` debe comenzar con:
firma "autor:@holtha"


## Tipos de datos

- Números: `42`, `3.14`
- Booleanos: `verdadero`, `falso`
- Texto: `"hola mundo"`

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

- Y: `y` o `&&`
- O: `o` o `||`
- No: `no` o `!`

## Estructuras de control

### Condicional Si
si (condicion) {
// bloque de código
} sino {
// bloque alternativo
}



### Bucle Mientras
mientras (condicion) {
// bloque de código
}



## Declaración y Asignación

### Declarar variable
declarar nombre = valor



### Asignar valor
nombre = nuevo_valor



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