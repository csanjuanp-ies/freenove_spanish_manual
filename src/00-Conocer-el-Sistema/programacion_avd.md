# Un poco más de programación
## Comprendiendo el primer programa
Vamos a echar un vistazo a nuestro primer programa. Comprobemos el fichero `src/main.rs`:

``` rust
{{#include src/main.rs}}
```

Los programas para microcontroladores son diferentes de los programas estándar en dos aspectos: `#![no_std]` y `#![no_main]`.

El atributo `no_std` indica que este programa no usará la biblioteca estándar de Rust, la cual asume un sistema operativo subyacente; el programa utilizará en su lugar el crate `core`, un subconjunto de `std` que puede ejecutarse en sistemas hardware directamente.

El atributo `no_main` indica que este programa no usará la interfaz estándar 'main', que está diseñada para aplicaciones de línea de comandos que reciben argumentos. En lugar del 'main' estándar, usaremos el atributo 'entry' del crate [`cortex-m-rt`] para definir un punto de entrada personalizado.
En este programa hemos definido el punto de entrada como `main`, pero se podría haber usado cualquier otro nombre. La función del punto de entrada debe tener la firma `fn() -> !`; este tipo indica que la función no termina. Significa que el programa nunca finaliza: si el compilador detecta que esto sería posible, se negará a compilarlo.

[`cortex-m-rt`]: https://crates.io/crates/cortex-m-rt

Si observamos con cuidado el directorio del proyecto notaremos que hay un directorio `.cargo` posiblemente oculto en el proyecto. Este directorio contiene un archivo de configuración de Cargo `.cargo/config.toml`.

```toml
{{#include .cargo/config.toml}}
```

Este fichero modifica el proceso de enlace para adaptar la disposición de memoria del programa a los requisitos del dispositivo de desarrollo. Este proceso de enlace es un requisito del crate `cortex-m-rt`. El archivo `.cargo/config.toml` también le dice a Cargo cómo construir y ejecutar el código en nuestra MB2.

Hay también un fichero `Embed.toml`:

```toml
{{#include Embed.toml}}
```

Este fichero informa a `cargo-embed` que:

- Trabaja con un chip NRF52833.
- Queremos detener la ejecución en el chip después de flashearlo, por lo que nuestro programa se detiene antes de `main`.
- Queremos deshabilitar/habilitar RTT. RTT es un protocolo que permite al chip enviar texto a un depurador. Ya has visto RTT en acción fue la primera versión del programa inicial.
- Queremos Deshabilitar/habilitar GDB. Este paso es necesario para procesos de depuración tal y como veremos al final de este capítulo.

## Generando el binario
El primer paso es construir el binario. Dado que el microcontrolador tiene una arquitectura diferente a la de nuestro ordenador, tendremos que realizar una compilación cruzada. Hacerlo en Rust es tan simple como pasar un flag extra `--target` a `rustc` o Cargo. La parte complicada es averiguar el argumento de ese flag: el *nombre* del destino.

Como ya hemos visto, el microcontrolador del micro:bit tiene un procesador Cortex-M4F. `rustc` sabe cómo compilar de forma cruzada para la arquitectura Cortex-M y proporciona varios destinos que cubren las diferentes familias de procesadores dentro de esa arquitectura:

- `thumbv6m-none-eabi`, para los procesadores Cortex-M0 y Cortex-M1
- `thumbv7m-none-eabi`, para el procesador Cortex-M3
- `thumbv7em-none-eabi`, para los procesadores Cortex-M4 y Cortex-M7
- `thumbv7em-none-eabihf`, para los procesadores Cortex-M4**F** y Cortex-M7**F**
- `thumbv8m.main-none-eabi`, para los procesadores Cortex-M33 y Cortex-M35P
- `thumbv8m.main-none-eabihf`, para los procesadores Cortex-M33**F** y Cortex-M35P**F**

"Thumb" aquí se refiere a una versión del conjunto de instrucciones Arm que tiene instrucciones más pequeñas para reducir el tamaño del código. La denominación `hf`/`F` significa que implementa aceleración de punto flotante por hardware.

Para la MB2, micro:bit v2, queremos el destino `thumbv7em-none-eabihf`.

Antes de realizar la compilación cruzada, es necesario descargar una versión precompilada de la biblioteca estándar (en realidad, una versión reducida de ella) en el host local. Se hace usando `rustup`:
``` console
$ rustup target add thumbv7em-none-eabihf
```

Solo hay que hacerlo una vez; `rustup` actualizará este destino (reinstalando un nuevo componente de la biblioteca estándar `rust-std` que contiene la biblioteca `core` que usamos) cada vez que actualicemos la cadena de compilación. Por lo tanto, **se puede omitir este paso si ya se agregó la toolchain necesaria anteriormente**.

Con el componente `rust-std` en su lugar, ya es posible compilar el programa de forma cruzada usando Cargo. Nos aseguraremos de estar en el directorio `src/00-Conocer-el-Sistema`, luego lo construimos. Este código inicial es un ejemplo, así que lo compilamos como tal.

``` console
$ cargo build 
   Compiling semver-parser v0.7.0
   Compiling proc-macro2 v1.0.86
   ...

    Finished dev [unoptimized + debuginfo] target(s) in 33.67s
```

Ok, ya tenemos un ejecutable. ¡No hará mucho!, imprime el mensaje "Hello, World!".

## Flashearlo
Flashear es el proceso de mover el programa a la memoria del microcontrolador. Una vez hecho, el microcontrolador ejecutará el programa cada vez que se encienda.

El programa será el único existente en la memoria. Por esto me refiero a que no hay nada más ejecutándose en el microcontrolador: ni un sistema operativo, ni un "daemon", nada. Nuestro programa tiene control total sobre el dispositivo.

Pasarlo al microcontrolador es muy simple, gracias a `cargo embed\run`.
```console
$ cargo run
  (...)
     Erasing sectors ✔ [00:00:00] [####################################################################################################################################################]  2.00KiB/ 2.00KiB @  4.21KiB/s (eta 0s )
 Programming pages   ✔ [00:00:00] [####################################################################################################################################################]  2.00KiB/ 2.00KiB @  2.71KiB/s (eta 0s )
    Finished flashing in 0.608s
```

Es importante fijarse que si queremos depurar se tiene que lanzar con `cargo embed`, y de esta forma no termina después de mostrar la última línea. Esto es intencionado: no hay que cerrar `cargo embed`, ya que lo necesitamos en este estado para depurar.

## Depuración de programas
La depuración de programas es una parte fundamental del desarrollo de software, especialmente cuando se trabaja con microcontroladores como la Micro::bit. Para un funcionamiento correcto de la depuración, necesitamos configurar el proyecto para que utilice `gdb` en lugar de `rtt`. Para ello, editamos el archivo `Embed.toml` y establecemos `enabled = false` para `rtt` y `enabled = true` para `gdb`. El contenido del archivo debe ser el siguiente:
```
[default.rtt]
enabled = false

[default.gdb]
enabled = true
```

Tras modificar el fichero `Embed.toml` en el directorio `00-Conocer-el-Sistema`, ejecutamos pero lanzando la depuración con `cargo embed`:
```
$ cargo embed
```

Nos conectamos a la sesión de depuración con `gdb` desde otra consola (recordar que solo se muestran ejemplos para el SSOO Windows). Para ello, abrimos otra terminal y nos situamos en la raíz del proyecto. Ejecutamos `arm-none-eabi-gdb` con el binario que queremos depurar como argumento:
```shell
$ cd raiz_del_libro
$ arm-none-eabi-gdb ./target/thumbv7em-none-eabihf/debug/introduccion
```
Si nos da un fallo donde no encuentra el binario, recordar que el directorio `target` se encuentra en la raíz del proyecto, no en `src/00-Conocer-el-Sistema`. Ya dentro de gdb nos unimos a la sesión de depuración remota con el comando `target remote` y el puerto que vemos al ejecutar con `cargo embed` (en este caso, el puerto 1337):
```shell
(gdb) target remote :1337
Remote debugging using :1337
```

Los puntos de ruptura se pueden usar para detener el flujo normal de un programa. El comando `continue` permitirá que el binario se ejecute libremente *hasta* que alcance un punto de ruptura. En este caso, hasta que alcance la función `main` porque hemos establecido uno allí.
```
(gdb) break main
...
(gdb) continue
...
```

Para ver el contenido de las variables, podemos usar el comando `print` seguido del nombre de la variable. Por ejemplo, para ver el contenido de la variable `x`:
```
(gdb) print x
$1 = 0
``` 

Si queremos ejecutar el programa paso a paso, podemos usar el comando `next` para avanzar una línea de código a la vez. Esto nos permite observar cómo cambian las variables y el flujo del programa en tiempo real.
```
(gdb) next
16          loop {}
```

El comando `monitor reset` resetea el microcontrolador y lo para en el punto de entrada.
```
(gdb) monitor reset
```

Ya hemos terminado la sesión de depuración. Podemos salir con el comando `quit`.
```
(gdb) quit
A debugging session is active.

        Inferior 1 [Remote target] will be detached.

Quit anyway? (y or n) y
Detaching from program: $PWD/target/thumbv7em-none-eabihf/debug/meet-your-software, Remote target
Ending remote debugging.
[Inferior 1 (Remote target) detached]
```