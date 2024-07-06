mod color;
use color::Color;

fn main() {
    // Inicialización con valores RGB
    let red = Color::new(255, 0, 0);
    println!("{:?}", red);

    // Inicialización con un valor hexadecimal
    let green = Color::from_hex(0x00FF00);
    println!("{:?}", green);

    // Conversión de RGB a hexadecimal
    let red_hex = red.to_hex();
    println!("Red in hex: {:#X}", red_hex);

    // Mezcla de colores
    let mixed_color = red.mix(&green);
    println!("Mixed color: {:?}", mixed_color);
}

