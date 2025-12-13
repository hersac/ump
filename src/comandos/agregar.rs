use crate::configuracion::ProyectoUmp;
use std::collections::HashMap;
use std::fs::{self};
use std::path::Path;

pub fn ejecutar(paquetes: &Vec<String>) {
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

    let mut proyecto: ProyectoUmp = match serde_yaml::from_str(&contenido) {
        Ok(p) => p,
        Err(e) => {
            eprintln!("Error analizando umpkg.yml: {}", e);
            return;
        }
    };

    if proyecto.dependencies.is_none() {
        proyecto.dependencies = Some(HashMap::new());
    }

    let deps = proyecto.dependencies.as_mut().unwrap();
    let directorio_modulos = Path::new("modules_ump");

    if !directorio_modulos.exists() {
        if let Err(e) = fs::create_dir(directorio_modulos) {
            eprintln!("Error creando directorio modules_ump: {}", e);
            return;
        }
    }

    for paquete in paquetes {
        deps.insert(paquete.clone(), "^1.0.0".to_string());
        println!("Agregando dependencia: {}", paquete);

        if let Err(e) = instalar_paquete_simulado(directorio_modulos, paquete) {
            eprintln!("Falló instalación simulada de {}: {}", paquete, e);
        }
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

fn instalar_paquete_simulado(dir_modulos: &Path, nombre_paquete: &str) -> std::io::Result<()> {
    let dir_paquete = dir_modulos.join(nombre_paquete);
    if !dir_paquete.exists() {
        fs::create_dir(&dir_paquete)?;
    }

    let config_paquete = format!(
        "name: {}\nversion: 1.0.0\numbral: \">=0.1.0\"\nentry: src/index.um\nexports: []\n",
        nombre_paquete
    );
    fs::write(dir_paquete.join("umpkg.yml"), config_paquete)?;

    let dir_src = dir_paquete.join("src");
    if !dir_src.exists() {
        fs::create_dir(&dir_src)?;
    }

    let contenido_index = format!("// Punto de entrada para {}\n", nombre_paquete);
    fs::write(dir_src.join("index.um"), contenido_index)?;

    Ok(())
}
