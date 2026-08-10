
use image::GenericImageView;
fn main() {
    let img = image::open(r"c:\Users\freit\Pictures\Screenshots\Captura de tela 2026-08-09 222520.png").unwrap();
    println!("Dimensions: {:?}", img.dimensions());
}

