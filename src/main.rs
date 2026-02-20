mod frame;
mod logic;

pub const RUTA: &str = "./file_test.m3u";

fn main() -> iced::Result {
    iced::run(update, frame::view)
}

// fn main() {
//     println!("Buscando archivo en: {}", RUTA);
//     println!("{}", logic::obtener_lineas_ordenadas(RUTA));
// }

// Función de actualización mínima
fn update(_state: &mut (), _message: frame::Message) {
    // Por ahora no hace nada
}
