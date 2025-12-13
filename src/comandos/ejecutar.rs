use crate::configuracion::ProyectoUmp;
use std::fs;
use std::path::Path;
use std::process::{self, Command};

pub fn ejecutar(nombre_script: &String) {
    let ruta_config = Path::new("umpkg.yml");
    if !ruta_config.exists() {
        eprintln!("Error: umpkg.yml no encontrado. Ejecuta 'ump inicio' primero.");
        return;
    }

    let contenido = match fs::read_to_string(ruta_config) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Error leyendo umpkg.yml: {}", e);
            return;
        }
    };

    let proyecto: ProyectoUmp = match serde_yaml::from_str(&contenido) {
        Ok(p) => p,
        Err(e) => {
            eprintln!("Error analizando umpkg.yml: {}", e);
            return;
        }
    };

    if proyecto.scripts.is_none() {
        eprintln!("Error: No hay scripts definidos en umpkg.yml");
        return;
    }

    let scripts = proyecto.scripts.unwrap();
    let comando_str = match scripts.get(nombre_script) {
        Some(s) => s,
        None => {
            eprintln!(
                "Error: Script '{}' no encontrado en umpkg.yml",
                nombre_script
            );
            println!("Scripts disponibles: {:?}", scripts.keys());
            return;
        }
    };

    println!("> {}", comando_str);

    let estado = Command::new("sh").arg("-c").arg(comando_str).status();

    match estado {
        Ok(s) => {
            if !s.success() {
                eprintln!("Script falló con estado: {}", s);
                process::exit(s.code().unwrap_or(1));
            }
        }
        Err(e) => {
            eprintln!("Falló ejecución del script: {}", e);
        }
    }
}
