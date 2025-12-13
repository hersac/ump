use crate::configuracion::ProyectoUmp;
use std::fs::{self};
use std::path::Path;

pub fn ejecutar(paquetes: &Vec<String>) {
    let ruta_config = Path::new("umpkg.yml");
    if !ruta_config.exists() {
        eprintln!("Error: umpkg.yml no encontrado.");
        return;
    }

    let contenido = match fs::read_to_string(ruta_config) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Error leyendo umpkg.yml: {}", e);
            return;
        }
    };

    let mut proyecto: ProyectoUmp = match serde_yaml::from_str(&contenido) {
        Ok(p) => p,
        Err(e) => {
            eprintln!("Error analizando umpkg.yml: {}", e);
            return;
        }
    };

    if let Some(deps) = proyecto.dependencies.as_mut() {
        let directorio_modulos = Path::new("modules_ump");

        for paquete in paquetes {
            if deps.remove(paquete).is_some() {
                println!("Removiendo dependencia: {}", paquete);
                let ruta_paquete = directorio_modulos.join(paquete);
                if ruta_paquete.exists() {
                    if let Err(e) = fs::remove_dir_all(&ruta_paquete) {
                        eprintln!("Error eliminando directorio {}: {}", paquete, e);
                    }
                }
            } else {
                eprintln!(
                    "Advertencia: El paquete '{}' no estaba en las dependencias.",
                    paquete
                );
            }
        }
    } else {
        eprintln!("Advertencia: No hay dependencias para remover.");
        return;
    }

    let nuevo_yaml = match serde_yaml::to_string(&proyecto) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Error generando yaml: {}", e);
            return;
        }
    };

    if let Err(e) = fs::write(ruta_config, nuevo_yaml) {
        eprintln!("Error escribiendo umpkg.yml: {}", e);
    } else {
        println!("Actualizado umpkg.yml");
    }
}
