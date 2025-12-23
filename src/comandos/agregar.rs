use crate::configuracion::ProyectoUmp;
use serde::Deserialize;
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::process::Command;

const URL_INDICE_GLOBAL: &str =
    "https://raw.githubusercontent.com/hersac/ump-index/main/global_directory.yml";

#[derive(Debug, Deserialize)]
struct EntradaIndice {
    repository: String,
    umbral: String,
    #[allow(dead_code)]
    exports: Option<Vec<String>>,
}

#[derive(Debug, Deserialize)]
struct TagGithub {
    name: String,
}

pub fn ejecutar(paquetes: &Vec<String>) {
    let ruta_config = Path::new("umpkg.yml");
    if !ruta_config.exists() {
        eprintln!("Error: umpkg.yml no encontrado. Ejecuta 'ump init' primero.");
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

    let indice_global = match obtener_indice_global() {
        Ok(indice) => indice,
        Err(e) => {
            eprintln!("Error obteniendo índice global: {}", e);
            return;
        }
    };

    let directorio_modulos = Path::new("modules_ump");
    if !directorio_modulos.exists() {
        if let Err(e) = fs::create_dir(directorio_modulos) {
            eprintln!("Error creando directorio modules_ump: {}", e);
            return;
        }
    }

    for paquete_spec in paquetes {
        if let Err(e) = procesar_paquete(
            paquete_spec,
            &mut proyecto,
            &indice_global,
            directorio_modulos,
        ) {
            eprintln!("Error procesando {}: {}", paquete_spec, e);
            continue;
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
        println!("✓ Actualizado umpkg.yml");
    }
}

fn obtener_indice_global() -> Result<HashMap<String, EntradaIndice>, String> {
    let respuesta = reqwest::blocking::get(URL_INDICE_GLOBAL)
        .map_err(|e| format!("Error descargando índice global: {}", e))?;

    if !respuesta.status().is_success() {
        return Err(format!(
            "Error HTTP al obtener índice: {}",
            respuesta.status()
        ));
    }

    let texto = respuesta
        .text()
        .map_err(|e| format!("Error leyendo respuesta: {}", e))?;

    serde_yaml::from_str(&texto).map_err(|e| format!("Error parseando índice global: {}", e))
}

fn procesar_paquete(
    paquete_spec: &str,
    proyecto: &mut ProyectoUmp,
    indice_global: &HashMap<String, EntradaIndice>,
    directorio_modulos: &Path,
) -> Result<(), String> {
    let (nombre_paquete, version_especifica) = parsear_especificacion(paquete_spec);

    let entrada = indice_global.get(&nombre_paquete).ok_or_else(|| {
        format!(
            "La librería '{}' no existe en el índice global",
            nombre_paquete
        )
    })?;

    let deps = proyecto.dependencies.as_ref().unwrap();
    if deps.contains_key(&nombre_paquete) {
        return Err(format!(
            "La librería '{}' ya existe en el proyecto",
            nombre_paquete
        ));
    }

    validar_compatibilidad_umbral(&proyecto.umbral, &entrada.umbral)?;

    let version_a_instalar = if let Some(version) = version_especifica {
        validar_version_existe(&entrada.repository, &version)?;
        version
    } else {
        obtener_ultima_version(&entrada.repository)?
    };

    println!("📦 Instalando {} v{}", nombre_paquete, version_a_instalar);

    descargar_paquete(
        &nombre_paquete,
        &entrada.repository,
        &version_a_instalar,
        directorio_modulos,
    )?;

    let deps_mut = proyecto.dependencies.as_mut().unwrap();
    deps_mut.insert(nombre_paquete.clone(), version_a_instalar.clone());

    println!(
        "✓ {} v{} instalado correctamente",
        nombre_paquete, version_a_instalar
    );

    Ok(())
}

fn parsear_especificacion(spec: &str) -> (String, Option<String>) {
    if let Some(pos) = spec.rfind('/') {
        let nombre = spec[..pos].to_string();
        let version = spec[pos + 1..].to_string();
        (nombre, Some(version))
    } else {
        (spec.to_string(), None)
    }
}

fn validar_compatibilidad_umbral(
    version_proyecto: &str,
    version_requerida: &str,
) -> Result<(), String> {
    let req_limpia = version_requerida
        .trim_start_matches(">=")
        .trim_start_matches("^");
    let proj_limpia = version_proyecto
        .trim_start_matches(">=")
        .trim_start_matches("^");

    let req_sem = semver::Version::parse(req_limpia)
        .map_err(|_| format!("Versión Umbral requerida inválida: {}", version_requerida))?;

    let proj_sem = semver::Version::parse(proj_limpia)
        .map_err(|_| format!("Versión Umbral del proyecto inválida: {}", version_proyecto))?;

    if proj_sem < req_sem {
        return Err(format!(
            "La librería requiere Umbral {} pero el proyecto usa {}",
            version_requerida, version_proyecto
        ));
    }

    Ok(())
}

fn obtener_ultima_version(repositorio: &str) -> Result<String, String> {
    let tags = obtener_tags_github(repositorio)?;

    if tags.is_empty() {
        return Err(format!(
            "No se encontraron versiones para el repositorio {}",
            repositorio
        ));
    }

    let mut versiones: Vec<semver::Version> = tags
        .iter()
        .filter_map(|tag| {
            let nombre_limpio = tag.name.trim_start_matches('v');
            semver::Version::parse(nombre_limpio).ok()
        })
        .collect();

    if versiones.is_empty() {
        return Err(format!(
            "No se encontraron versiones semánticas válidas en {}",
            repositorio
        ));
    }

    versiones.sort();
    versiones.reverse();

    Ok(versiones[0].to_string())
}

fn validar_version_existe(repositorio: &str, version: &str) -> Result<(), String> {
    let tags = obtener_tags_github(repositorio)?;

    let version_limpia = version.trim_start_matches('v');

    let existe = tags.iter().any(|tag| {
        let tag_limpio = tag.name.trim_start_matches('v');
        tag_limpio == version_limpia || tag.name == version
    });

    if !existe {
        return Err(format!(
            "La versión {} no existe en el repositorio {}",
            version, repositorio
        ));
    }

    Ok(())
}

fn obtener_tags_github(repositorio: &str) -> Result<Vec<TagGithub>, String> {
    let repo_path = extraer_repo_github(repositorio)?;
    let url = format!("https://api.github.com/repos/{}/tags", repo_path);

    let cliente = reqwest::blocking::Client::new();
    let respuesta = cliente
        .get(&url)
        .header("User-Agent", "ump-package-manager")
        .send()
        .map_err(|e| format!("Error consultando GitHub API: {}", e))?;

    if !respuesta.status().is_success() {
        return Err(format!(
            "Error HTTP al consultar tags: {} - Repositorio: {}",
            respuesta.status(),
            repositorio
        ));
    }
    respuesta
        .json::<Vec<TagGithub>>()
        .map_err(|e| format!("Error parseando respuesta de GitHub: {}", e))
}

fn extraer_repo_github(url: &str) -> Result<String, String> {
    let url_limpia = url
        .trim_start_matches("https://")
        .trim_start_matches("http://")
        .trim_end_matches(".git");

    if let Some(pos) = url_limpia.find("github.com/") {
        Ok(url_limpia[pos + 11..].to_string())
    } else {
        Err(format!("URL de repositorio no válida para GitHub: {}", url))
    }
}

fn descargar_paquete(
    nombre: &str,
    repositorio: &str,
    version: &str,
    directorio_modulos: &Path,
) -> Result<(), String> {
    let url_repo = repositorio;
    let ruta_destino = directorio_modulos.join(nombre);

    if ruta_destino.exists() {
        fs::remove_dir_all(&ruta_destino)
            .map_err(|e| format!("Error eliminando directorio existente: {}", e))?;
    }

    let tag = if version.starts_with('v') {
        version.to_string()
    } else {
        format!("v{}", version)
    };

    println!("  Clonando desde {}...", url_repo);

    let salida = Command::new("git")
        .args(&[
            "clone",
            "--depth",
            "1",
            "--branch",
            &tag,
            &url_repo,
            ruta_destino.to_str().unwrap(),
        ])
        .output()
        .map_err(|e| format!("Error ejecutando git clone: {}", e))?;

    if !salida.status.success() {
        let error = String::from_utf8_lossy(&salida.stderr);
        return Err(format!("Git clone falló: {}", error));
    }

    let git_dir = ruta_destino.join(".git");
    if git_dir.exists() {
        fs::remove_dir_all(&git_dir)
            .map_err(|e| format!("Error eliminando directorio .git: {}", e))?;
    }

    Ok(())
}
