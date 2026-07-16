# Capítulo 2 - Botones
Los teclados o botones son herramientas importantes para la interacción nosotros y el ordenador. A menudo utilizamos los teclados para introducir texto, escribir comandos, controlar dispositivos, etc. La micro:bit incorpora dos botones programables, A y B, que permiten controlarla para que realice acciones.

## Descripción del proyecto 2.1
Este proyecto tiene como objetivo mostrar dos patrones diferentes en función del botón que se pulse. Si se pulsa el botón A, se mostrará un patrón de flecha hacia la izquierda en la pantalla LED de la MB2. Si se pulsa el botón B, se mostrará un patrón de flecha hacia la derecha.

### Hardware necesario
- Micro:bit
- Micro Usb

### Esquema de conexión
<p style="text-align: center;">
    <img title="micro:bit" src="./img/circuito.png" alt="Micro:bit" />
</p>

### Código fuente
``` rust
{{#include src/main.rs}}
```
