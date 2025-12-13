use crate::configuracion::ProyectoUmp;
use std::fs::{self, File};
use std::io::Write;
use std::path::Path;

pub fn ejecutar(nombre: &String) {
    let ruta_proyecto = Path::new(nombre);
    if ruta_proyecto.exists() {
        eprintln!("Error: El directorio '{}' ya existe.", nombre);
        return;
    }

    if let Err(e) = fs::create_dir(ruta_proyecto) {
        eprintln!("Error creando directorio '{}': {}", nombre, e);
        return;
    }
    println!("Directorio '{}' creado.", nombre);

    let mut proyecto = ProyectoUmp::default();
    proyecto.name = nombre.clone();

    let yaml = match serde_yaml::to_string(&proyecto) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Error generando yaml: {}", e);
            return;
        }
    };

    let ruta_config = ruta_proyecto.join("umpkg.yml");
    if let Err(e) = fs::write(ruta_config, yaml) {
        eprintln!("Error escribiendo umpkg.yml: {}", e);
        return;
    }
    println!("Creado {}/umpkg.yml", nombre);

    let ruta_src = ruta_proyecto.join("src");
    if let Err(e) = fs::create_dir(&ruta_src) {
        eprintln!("Error creando directorio src: {}", e);
        return;
    }

    let ruta_main = ruta_src.join("main.um");
    let contenido = r#"f: main() {
    r: ("Hola Mundo");
}
"#;

    if escribir_archivo(ruta_main.as_path(), contenido) {
        println!("Creado {}/src/main.um", nombre);
    }
}

fn escribir_archivo(ruta: &Path, contenido: &str) -> bool {
    let mut archivo = match File::create(ruta) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Error creando {}: {}", ruta.display(), e);
            return false;
        }
    };

    if let Err(e) = archivo.write_all(contenido.as_bytes()) {
        eprintln!("Error escribiendo en {}: {}", ruta.display(), e);
        return false;
    }
    true
}
