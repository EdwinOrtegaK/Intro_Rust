pub struct Vertex {
    pub x: isize,
    pub y: isize,
}

impl Vertex {
    pub fn new(x: f32, y: f32) -> Self {
        Vertex {
            x: x.round() as isize,
            y: y.round() as isize,
        }
    }
}
