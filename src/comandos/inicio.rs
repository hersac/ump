use crate::configuracion::ProyectoUmp;
use std::fs::{self, File};
use std::io::Write;
use std::path::Path;

pub fn ejecutar() {
    let ruta_config = Path::new("umpkg.yml");
    if ruta_config.exists() {
        eprintln!("Error: umpkg.yml ya existe.");
        return;
    }

    let proyecto = ProyectoUmp::default();
    let yaml = match serde_yaml::to_string(&proyecto) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Error generando yaml: {}", e);
            return;
        }
    };

    if let Err(e) = fs::write(ruta_config, yaml) {
        eprintln!("Error escribiendo umpkg.yml: {}", e);
        return;
    }
    println!("Creado umpkg.yml");

    let ruta_src = Path::new("src");
    if !ruta_src.exists() {
        if let Err(e) = fs::create_dir(ruta_src) {
            eprintln!("Error creando directorio src: {}", e);
            return;
        }
    }

    let ruta_main = ruta_src.join("main.um");
    let contenido = r#"f: main() {
    r: ("Hola Mundo");
}
"#;

    if escribir_archivo(ruta_main.as_path(), contenido) {
        println!("Creado src/main.um");
    }
}

fn escribir_archivo(ruta: &Path, contenido: &str) -> bool {
    if ruta.exists() {
        eprintln!("Advertencia: {} ya existe, omitiendo.", ruta.display());
        return false;
    }

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
