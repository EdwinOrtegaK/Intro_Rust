mod color;
mod framebuffer;
use framebuffer::Framebuffer;

fn main() {
    // Creamos un framebuffer de 10x10 píxeles
    let mut fb = Framebuffer::new(10, 10);

    // Limpiamos el framebuffer con el color de fondo (negro por defecto)
    fb.clear();

    // Dibujamos algunos puntos con el color actual (blanco por defecto)
    fb.point(0, 0); // Esquina superior izquierda
    fb.point(9, 9); // Esquina inferior derecha
    fb.point(5, 5); // Centro

    // Cambiamos el color actual a verde y dibujamos más puntos
    fb.set_current_color(0x00FF00);
    fb.point(4, 4);
    fb.point(6, 6);

    // Mostramos el contenido del framebuffer
    fb.display();
}
