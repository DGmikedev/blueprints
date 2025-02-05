1.- Eliminar la dependencia en Cargo.toml

En  Cargo.toml, borrar líneas de librerías a desinstalar.

```python
[dependencies]
csv = "1.3"
serde = { version = "1.0", features = ["derive"] }
```
2.- Ejecutar cargo check o cargo build

```bash

cargo check

cargo build

```

esto verificará que el código sigue funcionando sin la librería eliminada.

3. Limpiar archivos compilados

Eliminar los archivos compilados relacionados con la librería eliminada:
Esto borrará los binarios y caché de compilación
```bash
cargo clean
```

Esto no elimina las librerías descargadas de ~/.cargo/registry.

4.- (Opcional) Eliminar la librería del caché local

Para borrar por completo la librería del caché de Rust en tu máquina, usa:
```bash
rm -rf ~/.cargo/registry
rm -rf ~/.cargo/git
```
<span style="color:red;">WARNINIG!</span> Esto eliminará todas las librerías descargadas en el caché, por lo que tendrás que volver a descargarlas cuando uses cargo build en otro proyecto.