use once_cell::sync::Lazy;
use std::collections::HashMap;
use walkdir::WalkDir;

use csv::Reader;
use std::error::Error;
use std::fs::File;

pub static FUNC_MAP: Lazy<HashMap<&'static str, fn(&'static str) -> Vec<Vec<i32>>>> =
    Lazy::new(|| {
        let mut m = HashMap::new();

        m.insert(
            "boundary",
            import_csv_layout as fn(&'static str) -> Vec<Vec<i32>>,
        );
        m.insert(
            "grass",
            import_csv_layout as fn(&'static str) -> Vec<Vec<i32>>,
        );
        m
    });

fn import_csv_layout(path: &'static str) -> Vec<Vec<i32>> {
    let mut map: Vec<Vec<i32>> = Vec::new();

    if let Ok(file) = File::open(path) {
        let mut reader = Reader::from_reader(file);
        for result in reader.records() {
            let mut row: Vec<i32> = Vec::new();
            match result {
                Ok(record) => {
                    for cell in record.iter() {
                        if let Ok(value) = cell.parse::<i32>() {
                            row.push(value);
                        } else {
                            eprintln!("No se pudo convertir '{}' a u16", cell);
                        }
                    }
                    map.push(row);
                }
                Err(e) => eprintln!("Error: {}", e),
            }
        }
    } else {
        eprintln!("No se pudo abrir el archivo: {}", path);
    }
    map
}

pub fn import_folder(path: &str) -> Vec<String> {
    WalkDir::new(path)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|entry| entry.path() != std::path::Path::new(path)) // excluye el directorio raíz
        .map(|entry| entry.path().display().to_string())
        .collect()
}
