use wasm_bindgen::prelude::*;

// Esta macro permite usar console.log de JS desde Rust para depurar
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub fn hide_message(image_bytes: &[u8], secret_text: &str) -> Vec<u8> {
    // 1. Clona la imagen original para no modificar la entrada directamente
    let mut new_image = image_bytes.to_vec();
    
    // 2. Lectura dinámica del Offset de Píxeles (Bytes 10-13)
    let pixel_offset = u32::from_le_bytes([
        image_bytes[10], image_bytes[11], image_bytes[12], image_bytes[13]
    ]) as usize;

    // 3. Preparamos el mensaje: Texto + Carácter Nulo (Terminador)
    let mut message_bytes = secret_text.as_bytes().to_vec();
    message_bytes.push(0u8); // Añadimos el byte 0 al final (\0)

    // Calculamos si cabe el mensaje dinámicamente según el pixel_offset
    let required_pixels_bytes = message_bytes.len() * 8;
    if required_pixels_bytes > (new_image.len() - pixel_offset) {
        log("Error: La imagen es muy pequeña para este mensaje.");
        return new_image; // Retornamos sin cambios si no cabe
    }

    // 4. Proceso de Ocultado (LSB)
    let mut image_idx = pixel_offset; // Empezamos después de la cabecera real

    for byte_char in message_bytes {
        for bit_pos in (0..8).rev() {
            // Obtenemos el bit específico del carácter (0 o 1)
            let bit = (byte_char >> bit_pos) & 1;

            // Manipulación de Bits:
            // 1. Limpiamos el LSB del byte de la imagen con AND 0xFE (11111110)
            // 2. Insertamos nuestro bit con OR
            new_image[image_idx] = (new_image[image_idx] & 0xFE) | bit;

            image_idx += 1;
        }
    }

    log("Mensaje oculto exitosamente.");
    return new_image;
}

#[wasm_bindgen]
pub fn show_message(image_bytes: &[u8]) -> String {
    let mut decoded_bytes: Vec<u8> = Vec::new();
    
    // Lectura dinámica del Offset de Píxeles
    let pixel_offset = u32::from_le_bytes([
        image_bytes[10], image_bytes[11], image_bytes[12], image_bytes[13]
    ]) as usize;

    let mut image_idx = pixel_offset; // Saltamos cabecera dinámica

    // Bucle para reconstruir el mensaje
    loop {
        // Protección para no leer fuera de memoria
        if image_idx + 8 > image_bytes.len() {
            break; 
        }

        let mut char_byte: u8 = 0;

        // Reconstruimos el byte (8 bits) desde 8 bytes de la imagen
        for bit_pos in (0..8).rev() {
            // Extraemos el LSB del byte de la imagen
            let bit = image_bytes[image_idx] & 1;
            
            // Colocamos ese bit en su posición correcta del carácter
            char_byte |= bit << bit_pos;

            image_idx += 1;
        }

        // Si encontramos el terminador nulo (0), terminamos
        if char_byte == 0 {
            break;
        }

        decoded_bytes.push(char_byte);
    }

    // Convertimos el vector de bytes a String UTF-8
    match String::from_utf8(decoded_bytes) {
        Ok(s) => s,
        Err(_) => String::from("Error: No se encontró un mensaje válido o codificación incorrecta."),
    }
}