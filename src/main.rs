mod color;
mod framebuffer;
mod line_impl;
mod bmp;
mod vertex;

use framebuffer::Framebuffer;
use line_impl::Line;
use vertex::Vertex;

fn main() {
    let width = 800;
    let height = 600;
    let mut framebuffer = Framebuffer::new(width, height);

    // Limpiar el framebuffer con un fondo blanco
    framebuffer.set_background_color(0xFFFFFF);
    framebuffer.clear();

    // Establecer el color de dibujo actual a negro
    framebuffer.set_current_color(0x000000);

    // Dibujar un cuadrado usando el algoritmo de Bresenham
    framebuffer.line(Vertex::new(100.0, 100.0), Vertex::new(700.0, 100.0));
    framebuffer.line(Vertex::new(700.0, 100.0), Vertex::new(700.0, 500.0));
    framebuffer.line(Vertex::new(700.0, 500.0), Vertex::new(100.0, 500.0));
    framebuffer.line(Vertex::new(100.0, 500.0), Vertex::new(100.0, 100.0));

    // Guardar el framebuffer como un archivo BMP
    framebuffer.render_buffer("square.bmp");

    println!("Framebuffer rendered to square.bmp");
}

