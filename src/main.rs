#![no_std] // no stdlib
#![no_main] // no main

use core::panic::PanicInfo;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    let vga_buffer = 0xb8000 as *mut u8;
    
    // Dibujar el recuadro con el mensaje "JKernel Started"
    let width = 50;
    let height = 6;
    let top_left_x = 15;
    let top_left_y = 10;
    
    // Caracteres para el recuadro
    let horizontal = b'=';
    let vertical = b'|';
    let corner = b'+';
    
    // Lista de colores para hacer el efecto
    let colors = [
        0x1F, // Azul con blanco brillante
        0x2F, // Verde con blanco brillante
        0x4F, // Rojo con blanco brillante
        0x5F, // Magenta con blanco brillante
        0x6F, // Café/marrón con blanco brillante
        0x9F, // Azul claro con blanco brillante
        0xAF, // Verde claro con blanco brillante
        0xCF, // Rojo claro con blanco brillante
    ];
    
    // Contador para animar
    let mut counter: u64 = 0;
    
    loop {
        // Seleccionar el color actual basado en el contador
        let color_index = ((counter / 50000) % colors.len() as u64) as usize;
        let color = colors[color_index];
        
        // Parpadeo: cada cierto tiempo, hacer invisible el texto
        let visible = (counter / 20000) % 5 != 0;
        let display_color = if visible { color } else { color & 0xF0 }; // Solo mantener el color de fondo
        
        // Dibujar bordes horizontales
        for x in 0..width {
            // Borde superior
            unsafe {
                let pos = (top_left_y * 80 + top_left_x + x) * 2;
                *vga_buffer.offset(pos as isize) = if x == 0 || x == width - 1 { corner } else { horizontal };
                *vga_buffer.offset(pos as isize + 1) = display_color;
            }
            
            // Borde inferior
            unsafe {
                let pos = ((top_left_y + height - 1) * 80 + top_left_x + x) * 2;
                *vga_buffer.offset(pos as isize) = if x == 0 || x == width - 1 { corner } else { horizontal };
                *vga_buffer.offset(pos as isize + 1) = display_color;
            }
        }
        
        // Dibujar bordes verticales
        for y in 1..height-1 {
            // Borde izquierdo
            unsafe {
                let pos = ((top_left_y + y) * 80 + top_left_x) * 2;
                *vga_buffer.offset(pos as isize) = vertical;
                *vga_buffer.offset(pos as isize + 1) = display_color;
            }
            
            // Borde derecho
            unsafe {
                let pos = ((top_left_y + y) * 80 + top_left_x + width - 1) * 2;
                *vga_buffer.offset(pos as isize) = vertical;
                *vga_buffer.offset(pos as isize + 1) = display_color;
            }
        }
        
        // Escribir el mensaje centrado
        let message = b"JKernel Started";
        let msg_len = message.len();
        let msg_x = top_left_x + (width - msg_len) / 2;
        let msg_y = top_left_y + height / 2;
        
        for (i, &byte) in message.iter().enumerate() {
            unsafe {
                let pos = (msg_y * 80 + msg_x + i) * 2;
                *vga_buffer.offset(pos as isize) = byte;
                *vga_buffer.offset(pos as isize + 1) = display_color;
            }
        }
        
        // Incrementar contador para la animación
        counter = counter.wrapping_add(1);
    }
}

/// This function is called when a panic occurs.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}