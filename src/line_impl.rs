use crate::framebuffer::Framebuffer;
use crate::vertex::Vertex;

pub trait Line {
    fn line(&mut self, start: Vertex, end: Vertex);
}

impl Line for Framebuffer {
    fn line(&mut self, start: Vertex, end: Vertex) {
        let (mut x, mut y, _) = (start.x, start.y, start.z);
        let (x2, y2, _) = (end.x, end.y, end.z);
        let dx = (x2 - x).abs();
        let dy = -(y2 - y).abs();
        let sx = if x < x2 { 1.0 } else { -1.0 };
        let sy = if y < y2 { 1.0 } else { -1.0 };
        let mut err = dx + dy;

        loop {
            self.point(x as isize, y as isize); // Dibujar el punto en el framebuffer

            if x == x2 && y == y2 { break; }
            let e2 = 2.0 * err;
            if e2 >= dy {
                err += dy;
                x += sx;
            }
            if e2 <= dx {
                err += dx;
                y += sy;
            }
        }
    }
}
