use std::fs::File;
use std::io::{self, BufRead, Write};

pub fn obtener_lineas_ordenadas(ruta: &str) -> String {
    // 1. Leer el archivo
    let archivo_lectura = match File::open(ruta) {
        Ok(f) => f,
        Err(_) => return "ERROR: No se encontró el archivo".to_string(),
    };

    let lector = io::BufReader::new(archivo_lectura);
    let mut lineas: Vec<String> = lector.lines().map_while(Result::ok).collect();

    // 2. Ordenar
    lineas.sort();

    // 3. Sobreescribir el archivo (File::create trunca el archivo si ya existe)
    let archivo_escritura = match File::create(ruta) {
        Ok(f) => f,
        Err(_) => return "ERROR: No se pudo escribir en el archivo".to_string(),
    };

    let mut escritor = io::BufWriter::new(archivo_escritura);

    for linea in lineas {
        if let Err(_) = writeln!(escritor, "{}", linea) {
            return "ERROR: Fallo al escribir línea".to_string();
        }
    }

    "DONE".to_string()
}
