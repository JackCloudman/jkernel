# JKernel

Un kernel minimalista experimental escrito en Rust basado en el tutorial [Writing an OS in Rust](https://os.phil-opp.com/es/minimal-rust-kernel/).

## 🚀 Descripción

Este proyecto es un kernel básico desarrollado en Rust sin dependencia de la biblioteca estándar (no_std), diseñado para ejecutarse directamente en hardware sin un sistema operativo subyacente. Actualmente muestra un mensaje "JKernel Started" con un borde animado en la pantalla.

## ⚙️ Requisitos Previos

- Rust (última versión estable)
- cargo
- QEMU (para emular el kernel)
- `rustup component add llvm-tools-preview` (para la herramienta bootimage)
- `cargo install bootimage` (para crear la imagen de arranque)

## 🛠️ Compilación y Ejecución

### Compilar el kernel
```bash
cargo build
```

### Crear la imagen de arranque
```bash
cargo bootimage
```

### Ejecutar en QEMU (forma manual)
```bash
qemu-system-x86_64 -drive format=raw,file=target/x86_64-jack_os/debug/bootimage-jkernel.bin
```

### Ejecutar con cargo run (más sencillo)
```bash
cargo run
```
Esto automáticamente compila el kernel, crea la imagen de arranque y la ejecuta en QEMU.

## 🚀 Ejecutar en hardware real

Para ejecutar en hardware real, puedes transferir la imagen a una memoria USB:
```bash
dd if=target/x86_64-jack_os/debug/bootimage-jkernel.bin of=/dev/sdX && sync
```
¡ADVERTENCIA! Reemplaza `/dev/sdX` con la ruta correcta a tu dispositivo USB. Todo el contenido del dispositivo será sobrescrito.

## 🔍 Características

- Binario autónomo sin dependencia de la biblioteca estándar
- Implementación personalizada del manejador de pánico
- Punto de entrada personalizado (_start)
- Animación de colores en el texto VGA
- Sin desenrollado de pila en caso de pánico

## 📝 Referencias

Este proyecto está basado en la serie de blog [Writing an OS in Rust](https://os.phil-opp.com/es/minimal-rust-kernel/) por Philipp Oppermann.

## 📄 Licencia

Este proyecto está licenciado bajo la GNU General Public License v3.0 (GPLv3) - vea el archivo [LICENSE](LICENSE) para más detalles.

[GPLv3](https://www.gnu.org/licenses/gpl-3.0.html): Las libertades del software libre necesitan ser protegidas. La GPLv3 permite:
- Usar el software para cualquier propósito
- Modificar el software según sus necesidades
- Compartir el software con sus amigos y vecinos
- Compartir los cambios que realice

