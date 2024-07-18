mod color;
mod framebuffer;
mod line_impl;
mod bmp;
mod vertex;
mod polygon_impl;

use framebuffer::Framebuffer;
use line_impl::Line;
use vertex::{new_vertex, Vertex};
use polygon_impl::Polygon;

fn draw_polygon(framebuffer: &mut Framebuffer, vertices: &[Vertex]) {
    if vertices.len() < 3 {
        eprintln!("Error: Se requieren al menos 3 vértices para formar un polígono.");
        return;
    }

    for i in 0..vertices.len() {
        let start = vertices[i];
        let end = if i == vertices.len() - 1 {
            vertices[0]
        } else {
            vertices[i + 1]
        };
        framebuffer.line(start, end);
    }

    framebuffer.line(vertices[vertices.len() - 1], vertices[0]);
}

fn fill_polygon(framebuffer: &mut Framebuffer, vertices: &[Vertex]) {
    let min_y = vertices.iter().map(|v| v.y).fold(f32::INFINITY, |a, b| a.min(b)).ceil() as isize;
    let max_y = vertices.iter().map(|v| v.y).fold(f32::NEG_INFINITY, |a, b| a.max(b)).floor() as isize;

    for y in min_y..=max_y {
        let mut intersections = Vec::new();

        for i in 0..vertices.len() {
            let start = vertices[i];
            let end = if i == vertices.len() - 1 { vertices[0] } else { vertices[i + 1] };

            if (start.y <= y as f32 && end.y > y as f32) || (end.y <= y as f32 && start.y > y as f32) {
                let t = (y as f32 - start.y) / (end.y - start.y);
                let x = start.x + t * (end.x - start.x);
                intersections.push(x);
            }
        }

        intersections.sort_by(|a, b| a.partial_cmp(b).unwrap());

        for i in (0..intersections.len()).step_by(2) {
            if i + 1 < intersections.len() {
                let x_start = intersections[i].ceil() as isize;
                let x_end = intersections[i + 1].floor() as isize;

                for x in x_start..=x_end {
                    framebuffer.point(x, y);
                }
            }
        }
    }
}

fn main() {
    let width = 800;
    let height = 600;
    let mut framebuffer = Framebuffer::new(width, height);

    framebuffer.set_background_color(0xFFFFFF);
    framebuffer.clear();

    let points = vec![
    new_vertex(400.0, 100.0, 0.0),  // Punto superior
    new_vertex(550.0, 250.0, 0.0),  // Punto superior derecho
    new_vertex(475.0, 450.0, 0.0),  // Punto inferior derecho
    new_vertex(325.0, 450.0, 0.0),  // Punto inferior izquierdo
    new_vertex(250.0, 250.0, 0.0),  // Punto superior izquierdo
    ];

    framebuffer.set_current_color(0xFFFF00); // Amarillo
    fill_polygon(&mut framebuffer, &points);

    framebuffer.set_current_color(0x000000); // Negro
    draw_polygon(&mut framebuffer, &points);

    framebuffer.render_buffer("pentagon_filled.bmp").unwrap();

    println!("Framebuffer rendered to pentagon_filled.bmp");
}
