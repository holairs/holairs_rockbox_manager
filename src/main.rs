use crate::tools::disk_actions::{get_playlists_list, get_playlists_path, read_external_disks, validate_rockbox_device};

mod frame;
mod logic;
mod tools;

pub const PATH: &str = "./Musical.m3u8";
pub const PATH_1: &str = "./a.txt";
pub const PATH_2: &str = "./b.txt";

// fn main() -> iced::Result {
//     iced::run(update, frame::view)
// }

// fn main() {
//     println!("{:?}", logic::get_sorted_lines(PATH));
//     // println!("{}", logic::merge_playlists(PATH_1, PATH_2));
// }

// fn main() {
//     println!("Running file explorer");
//     println!("{:?}", read_external_disks());
// }

// Función de actualización mínima
// fn update(_state: &mut (), _message: frame::Message) {
//     // Por ahora no hace nada
// }


fn main() {
    let workflow = || -> Result<(), String> {
        // 1. Obtener discos
        let disks = read_external_disks()?;

        for disk in disks {
            // 2. Validar dispositivo Rockbox
            if let Ok(mount_point) = validate_rockbox_device(&disk) {
                
                // 3. Obtener ruta de la carpeta Playlists
                let pg_path = get_playlists_path(&mount_point)?;

                // 4. Listar los archivos m3u/m3u8
                let playlists = get_playlists_list(&pg_path)?;

                println!("Found {} playlist(s) in {:?}", playlists.len(), mount_point);
                
                for pl in playlists {
                    println!(" -> {:?}", pl.file_name().unwrap_or_default());
                    let opened_file = open_files_by_path(mount_point);
                }
            }
        }
        Ok(())
    };

    if let Err(e) = workflow() {
        eprintln!("Aviso: {}", e);
    }
}
