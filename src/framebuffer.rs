pub struct Framebuffer {
    pub width: usize,
    pub height: usize,
    buffer: Vec<u32>,
    background_color: u32,
    current_color: u32,
}

impl Framebuffer {
    // Constructor para inicializar el framebuffer
    pub fn new(width: usize, height: usize, background_color: u32, current_color: u32) -> Framebuffer {
        let buffer = vec![background_color; width * height];
        Framebuffer {
            width,
            height,
            buffer,
            background_color,
            current_color,
        }
    }

    // Método para establecer el color de un píxel
    pub fn set_pixel(&mut self, x: usize, y: usize) {
        if x < self.width && y < self.height {
            let index = y * self.width + x;
            self.buffer[index] = self.current_color;
        }
    }

    // Método para obtener el color de un píxel
    pub fn get_pixel(&self, x: usize, y: usize) -> Option<u32> {
        if x < self.width && y < self.height {
            Some(self.buffer[y * self.width + x])
        } else {
            None
        }
    }

    // Método para mostrar el contenido del framebuffer (para depuración)
    pub fn display(&self) {
        for y in 0..self.height {
            for x in 0..self.width {
                let color = self.buffer[y * self.width + x];
                print!("{:#X} ", color);
            }
            println!();
        }
    }

    // Método para cambiar el color de primer plano
    pub fn set_current_color(&mut self, color: u32) {
        self.current_color = color;
    }

    // Método para cambiar el color de fondo
    pub fn set_background_color(&mut self, color: u32) {
        self.background_color = color;
        for i in 0..self.buffer.len() {
            if self.buffer[i] == self.background_color {
                self.buffer[i] = color;
            }
        }
    }
}
