# 🕵️ Sistema LSB de Esteganografía en Imágenes BMP

Aplicación web que permite **ocultar y revelar mensajes secretos** dentro de imágenes BMP utilizando la técnica de **Least Significant Bit (LSB)** — todo procesado localmente en el navegador gracias a **Rust compilado a WebAssembly**.

> Práctica 0 — Manipulación de Bits (TXT + BMP)  
> Materia: Introduction to Cryptography · ESCOM - IPN

---

## 🔍 ¿Qué hace?

### Módulo Hide (Ocultar)
Toma una **imagen BMP** y un **archivo de texto (.txt)** con el mensaje secreto. Modifica el **bit menos significativo (LSB)** de cada byte de los píxeles de la imagen para codificar el mensaje carácter por carácter. La imagen resultante se ve **idéntica al ojo humano**, pero contiene el mensaje oculto en su interior.

### Módulo Show (Revelar)
Recibe una imagen BMP que contiene un mensaje oculto y **extrae los bits LSB** de los píxeles para reconstruir el texto original. Genera automáticamente la descarga del mensaje revelado como archivo `.txt`.

---

## 🧠 ¿Cómo funciona el LSB?

Cada píxel en una imagen BMP se compone de bytes (valores de 0 a 255). La técnica LSB modifica únicamente el **último bit** de cada byte — un cambio tan pequeño que es imperceptible visualmente.

```
Byte original:   11010110  (214)
Bit a ocultar:   1
Byte modificado: 11010111  (215)  ← Solo cambia el último bit
```

Para ocultar un carácter (8 bits), se necesitan **8 bytes consecutivos** de la imagen. El mensaje termina con un byte nulo (`0x00`) que actúa como terminador.

---

## ⚙️ Stack Tecnológico

| Componente | Tecnología |
|------------|------------|
| **Lógica de esteganografía** | Rust → WebAssembly (wasm-bindgen) |
| **Frontend** | HTML + CSS + JavaScript (ES Modules) |
| **UI Framework** | Bootstrap 5 |
| **Estética** | Tema Matrix (efecto lluvia de código binario en canvas) |

---

## 📁 Estructura del Proyecto

```
P0/
├── index.html                  # Interfaz web principal
├── yo.png                      # Foto de perfil del alumno
├── pkg/                        # Módulo WASM compilado (listo para usar)
│   ├── steganography_core.js       # Glue code JS generado por wasm-bindgen
│   ├── steganography_core_bg.wasm  # Binario WebAssembly
│   ├── steganography_core.d.ts     # Tipos TypeScript
│   └── package.json
└── steganography_core/         # Código fuente Rust
    ├── Cargo.toml
    └── src/
        └── lib.rs                  # Funciones hide_message() y show_message()
```

---

## 🧩 Funciones principales (Rust)

- **`hide_message(image_bytes, secret_text) → Vec<u8>`**: Lee el offset de píxeles del header BMP, codifica cada bit del mensaje en el LSB de los bytes de la imagen, y retorna la imagen modificada.

- **`show_message(image_bytes) → String`**: Extrae los LSBs de los bytes de la imagen desde el offset de píxeles, reconstruye los caracteres hasta encontrar el terminador nulo, y retorna el mensaje como texto UTF-8.

---

## 👤 Autor

**Gael Martín Ramírez Lozano**  
Grupo 6CM1 · ESCOM - IPN
