# Resumen
En este capítulo hemos conocido la máquina y como programarla. Hemos aprendido a configurar el SSOO para ejecutar Rust y en concreto programar sistemas embebidos, hemos configurado la toolchain adecuada para la MB2 de Rust y hemos aprendido a compilar, flashear y ejecutar. Además, hemos visto cómo utilizar el depurador para analizar el comportamiento de nuestros programas. En resumen, hemos adquirido los conocimientos necesarios para desarrollar software en sistemas embebidos utilizando Rust.

A partir de ahora, avanzaremos hacia la programación de sistemas embebidos más complejos, a través de proyectos específicos. En concreto cada capítulo se dedicará a una parte del hardware específica.

## Para terminar
Hay un tema que no hemos tratado, el uso de IDEs específicos. Si bien Rust no requiere un IDE para programar, existen algunos que pueden facilitar el desarrollo, como Visual Studio Code o RustRover. Cualquiera de los dos puede ser utilizado para programar en Rust, y ambos ofrecen características como ia, autocompletado, depuración y gestión de proyectos que pueden mejorar la productividad. Sin embargo, es importante recordar que el conocimiento de la línea de comandos y las herramientas básicas sigue siendo fundamental para un desarrollador de sistemas embebidos.

### Como configurar IntelliJ o RustRover

Al editar la configuración de compilación de RustRover, estos son algunos valores no predeterminados:
* Hay que modificar el comando. Cuando se indique que hay que ejecutar `cargo embed FLAGS`, se deberá cambiar el valor predeterminado `run` por el comando `embed FLAGS` si queremos depurar, si solo lo lanzamos no es necesario.
* Se tiene que activar la opción "Emular terminal en la consola de salida". De lo contrario, el programa no podrá mostrar texto en un terminal.
* Nos aseguraremos que el directorio de trabajo sea `./src/N-nombre_capitulo`, siendo `N-nombre_capitulo` el directorio del capítulo que estamos leyendo. No se puede ejecutar desde el directorio `src` o `FreenoveManual`.
