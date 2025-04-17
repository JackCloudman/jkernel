# JKernel

Un kernel minimalista experimental escrito en Rust.

## 🚀 Descripción

Este proyecto es un kernel básico desarrollado en Rust sin dependencia de la biblioteca estándar (no_std), diseñado para ejecutarse directamente en hardware sin un sistema operativo subyacente.

## ⚙️ Requisitos Previos

- Rust (última versión estable)
- cargo
- Para desarrollo cruzado:
  - Target `thumbv7em-none-eabihf` (u otro target bare metal)
  ```bash
  rustup target add thumbv7em-none-eabihf
  ```

## 🛠️ Compilación

### Para target bare metal:
```bash
cargo build --target thumbv7em-none-eabihf
```

### Para sistemas host específicos:

**Linux:**
```bash
cargo rustc -- -C link-arg=-nostartfiles
```

**Windows:**
```bash
cargo rustc -- -C link-args="/ENTRY:_start /SUBSYSTEM:console"
```

**macOS:**
```bash
cargo rustc -- -C link-args="-e __start -static -nostartfiles"
```

## 🔍 Características

- Binario autónomo sin dependencia de la biblioteca estándar
- Implementación personalizada del manejador de pánico
- Punto de entrada personalizado (_start)
- Sin desenrollado de pila en caso de pánico

## 📝 Referencias

Este proyecto está inspirado en la serie de blog [Writing an OS in Rust](https://os.phil-opp.com/es/freestanding-rust-binary/) por Philipp Oppermann.

## 📄 Licencia

Este proyecto está licenciado bajo la GNU General Public License v3.0 (GPLv3) - vea el archivo [LICENSE](LICENSE) para más detalles.

[GPLv3](https://www.gnu.org/licenses/gpl-3.0.html): Las libertades del software libre necesitan ser protegidas. La GPLv3 permite:
- Usar el software para cualquier propósito
- Modificar el software según sus necesidades
- Compartir el software con sus amigos y vecinos
- Compartir los cambios que realice

