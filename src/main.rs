mod color;
mod framebuffer;
mod line_impl;
mod bmp;
mod vertex;
mod polygon_impl;

use framebuffer::Framebuffer;
use line_impl::Line;
use vertex::new_vertex;
use polygon_impl::Polygon;

fn main() {
    let width = 800;
    let height = 600;
    let mut framebuffer = Framebuffer::new(width, height);

    // Limpiar el framebuffer con un fondo blanco
    framebuffer.set_background_color(0xFFFFFF);
    framebuffer.clear();

    // Establecer el color de dibujo actual a negro
    framebuffer.set_current_color(0x000000);

    // Definir los puntos del polígono usando vértices 3D
    let points = vec![
        new_vertex(100.0, 100.0, 0.0),
        new_vertex(700.0, 100.0, 0.0),
        new_vertex(700.0, 500.0, 0.0),
        new_vertex(100.0, 500.0, 0.0),
    ];

    // Dibujar el polígono
    framebuffer.draw_polygon(&points);

    // Guardar el framebuffer como un archivo BMP
    framebuffer.render_buffer("polygon.bmp");

    println!("Framebuffer rendered to polygon.bmp");
}
