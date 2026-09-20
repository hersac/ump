use serde::Deserialize;
use std::collections::HashMap;
use std::fs;
use std::io::{self, Write};
use std::path::Path;
use std::process::Command;

use crate::configuracion::ProyectoUmp;

pub const URL_INDICE_GLOBAL: &str =
    "https://raw.githubusercontent.com/hersac/ump-index/main/global_directory.yml";

#[derive(Debug, Deserialize, Clone)]
pub struct EntradaIndice {
    pub repository: String,
    pub umbral: String,
    #[allow(dead_code)]
    pub exports: Option<Vec<String>>,
}

#[derive(Debug, Deserialize)]
pub struct TagGithub {
    pub name: String,
}

/// Parsea `nombre@version` con respaldo al formato `nombre/version`.
pub fn parsear_especificacion(spec: &str) -> (String, Option<String>) {
    let texto = spec.trim();
    dividir_spec(texto, '@')
        .or_else(|| dividir_spec(texto, '/'))
        .unwrap_or((texto.to_string(), None))
}

/// Quita espacios y `v` inicial de una versión.
pub fn normalizar_version(version: &str) -> String {
    version.trim().trim_start_matches('v').trim().to_string()
}

/// Convierte una versión a su tag Git (`1.0.0` -> `v1.0.0`).
pub fn tag_para_version(version: &str) -> String {
    match version.strip_prefix('v') {
        Some(_) => version.to_string(),
        None => format!("v{}", version),
    }
}

/// Descarga el índice global de librerías.
pub fn obtener_indice_global() -> Result<HashMap<String, EntradaIndice>, String> {
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

/// Extrae `dueño/repo` de una URL GitHub.
pub fn extraer_repo_github(url: &str) -> Result<String, String> {
    let limpia = url
        .trim_start_matches("https://")
        .trim_start_matches("http://")
        .trim_end_matches(".git");
    limpia
        .find("github.com/")
        .map(|pos| limpia[pos + 11..].to_string())
        .ok_or_else(|| format!("URL de repositorio no válida para GitHub: {}", url))
}

/// Lista los tags publicados del repositorio.
pub fn obtener_tags_github(repositorio: &str) -> Result<Vec<TagGithub>, String> {
    let ruta = extraer_repo_github(repositorio)?;
    let url = format!("https://api.github.com/repos/{}/tags", ruta);
    let respuesta = reqwest::blocking::Client::new()
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

/// Resuelve la última versión semántica del repositorio.
pub fn obtener_ultima_version(repositorio: &str) -> Result<String, String> {
    let tags = obtener_tags_github(repositorio)?;
    if tags.is_empty() {
        return Err(format!(
            "No se encontraron versiones para el repositorio {}",
            repositorio
        ));
    }
    tags
        .iter()
        .filter_map(|tag| semver::Version::parse(&normalizar_version(&tag.name)).ok())
        .max()
        .map(|version| version.to_string())
        .ok_or_else(|| {
            format!(
                "No se encontraron versiones semánticas válidas en {}",
                repositorio
            )
        })
}

/// Valida que la versión exista como tag y la devuelve normalizada.
pub fn validar_version_existe(repositorio: &str, version: &str) -> Result<String, String> {
    let tags = obtener_tags_github(repositorio)?;
    let limpia = normalizar_version(version);
    let conocida = tags
        .iter()
        .any(|tag| normalizar_version(&tag.name) == limpia || tag.name == version);
    match conocida {
        true => Ok(limpia),
        false => Err(explicar_faltante(repositorio, version, &tags)),
    }
}

/// Detecta la versión del binario Umbral instalado.
pub fn obtener_umbral_local() -> Option<semver::Version> {
    let salida = Command::new("umbral").arg("--version").output().ok()?;
    match salida.status.success() {
        false => None,
        true => extraer_version_salida(&String::from_utf8_lossy(&salida.stdout)),
    }
}

/// Lee el requisito Umbral declarado por una versión del repo.
pub fn umbral_de_version(repositorio: &str, version: &str) -> Option<String> {
    let ruta = extraer_repo_github(repositorio).ok()?;
    [
        tag_para_version(&normalizar_version(version)),
        normalizar_version(version),
    ]
    .into_iter()
    .find_map(|tag| leer_umbral_tag(&ruta, &tag))
}

/// Valida el requisito Umbral contra el binario local o el proyecto.
pub fn validar_compatibilidad_umbral(
    version_proyecto: &str,
    version_requerida: &str,
) -> Result<(), String> {
    let requisito = requisito_umbral(version_requerida)?;
    match obtener_umbral_local() {
        Some(local) => validar_umbral_local(&requisito, &local, version_requerida),
        None => validar_base_proyecto(version_proyecto, version_requerida),
    }
}

/// Clona el tag de la versión en `modules_ump`.
pub fn descargar_paquete(
    nombre: &str,
    repositorio: &str,
    version: &str,
    directorio_modulos: &Path,
) -> Result<(), String> {
    let destino = directorio_modulos.join(nombre);
    remover_si_existe(&destino)?;
    clonar_tag(repositorio, &tag_para_version(version), &destino)?;
    remover_si_existe(&destino.join(".git"))
}

/// Pide confirmación `S/n` con Sí por defecto.
pub fn confirmar(pregunta: &str) -> bool {
    print!("{} [S/n]: ", pregunta);
    let _ = io::stdout().flush();
    let mut entrada = String::new();
    match io::stdin().read_line(&mut entrada) {
        Ok(_) => respuesta_afirmativa(&entrada),
        Err(_) => false,
    }
}

/// Lee y valida el `umpkg.yml` del directorio actual.
pub fn leer_proyecto() -> Result<ProyectoUmp, String> {
    let ruta = Path::new("umpkg.yml");
    if !ruta.exists() {
        return Err("Error: umpkg.yml no encontrado. Ejecuta 'ump init' primero.".to_string());
    }
    let contenido =
        fs::read_to_string(ruta).map_err(|e| format!("Error leyendo umpkg.yml: {}", e))?;
    serde_yaml::from_str(&contenido).map_err(|e| format!("Error analizando umpkg.yml: {}", e))
}

/// Persiste el proyecto en `umpkg.yml`.
pub fn guardar_proyecto(proyecto: &ProyectoUmp) -> Result<(), String> {
    let texto =
        serde_yaml::to_string(proyecto).map_err(|e| format!("Error generando yaml: {}", e))?;
    fs::write("umpkg.yml", texto).map_err(|e| format!("Error escribiendo umpkg.yml: {}", e))?;
    Ok(())
}

/// Crea `modules_ump` si no existe.
pub fn asegurar_modulos() -> Result<(), String> {
    let ruta = Path::new("modules_ump");
    match ruta.exists() {
        true => Ok(()),
        false => fs::create_dir(ruta)
            .map_err(|e| format!("Error creando directorio modules_ump: {}", e)),
    }
}

/// Compara versiones normalizadas e indica si la nueva es mayor.
pub fn es_version_nueva(actual: &str, nueva: &str) -> bool {
    match (
        semver::Version::parse(&normalizar_version(actual)),
        semver::Version::parse(&normalizar_version(nueva)),
    ) {
        (Ok(base), Ok(candidata)) => candidata > base,
        _ => normalizar_version(nueva) != normalizar_version(actual),
    }
}

fn dividir_spec(spec: &str, sep: char) -> Option<(String, Option<String>)> {
    let pos = spec.rfind(sep)?;
    let nombre = spec[..pos].trim().to_string();
    let version = spec[pos + sep.len_utf8()..].trim().to_string();
    match nombre.is_empty() || version.is_empty() {
        true => None,
        false => Some((nombre, Some(version))),
    }
}

fn explicar_faltante(repositorio: &str, version: &str, tags: &[TagGithub]) -> String {
    let recientes: Vec<String> = tags.iter().take(10).map(|tag| tag.name.clone()).collect();
    format!(
        "La versión {} no existe en el repositorio {}. Disponibles (recientes): {}",
        version,
        repositorio,
        recientes.join(", ")
    )
}

fn extraer_version_salida(salida: &str) -> Option<semver::Version> {
    salida
        .split_whitespace()
        .find(es_token_version)
        .and_then(|token| semver::Version::parse(&normalizar_version(token)).ok())
}

fn es_token_version(token: &&str) -> bool {
    token.starts_with('v')
        || token
            .chars()
            .next()
            .map(|letra| letra.is_ascii_digit())
            .unwrap_or(false)
}

fn leer_umbral_tag(ruta: &str, tag: &str) -> Option<String> {
    let url = format!(
        "https://raw.githubusercontent.com/{}/{}/umpkg.yml",
        ruta, tag
    );
    let texto = reqwest::blocking::Client::new()
        .get(&url)
        .header("User-Agent", "ump-package-manager")
        .send()
        .ok()?
        .text()
        .ok()?;
    let proyecto: ProyectoUmp = serde_yaml::from_str(&texto).ok()?;
    match proyecto.umbral.trim().is_empty() {
        true => None,
        false => Some(proyecto.umbral),
    }
}

fn requisito_umbral(requerida: &str) -> Result<semver::VersionReq, String> {
    semver::VersionReq::parse(requerida.trim())
        .or_else(|_| requisito_sin_operador(requerida))
        .map_err(|_| format!("Versión Umbral requerida inválida: {}", requerida))
}

fn requisito_sin_operador(requerida: &str) -> Result<semver::VersionReq, semver::Error> {
    let base = requerida
        .trim_start_matches(">=")
        .trim_start_matches('^')
        .trim();
    semver::Version::parse(base)?;
    semver::VersionReq::parse(&format!(">={}", base))
}

fn validar_umbral_local(
    requisito: &semver::VersionReq,
    local: &semver::Version,
    requerida: &str,
) -> Result<(), String> {
    match requisito.matches(local) {
        true => Ok(()),
        false => Err(format!(
            "La librería requiere Umbral {} pero tienes instalado {}",
            requerida, local
        )),
    }
}

fn validar_base_proyecto(proyecto: &str, requerida: &str) -> Result<(), String> {
    let base_req =
        base_semver(requerida).map_err(|_| format!("Versión Umbral requerida inválida: {}", requerida))?;
    let base_proy =
        base_semver(proyecto).map_err(|_| format!("Versión Umbral del proyecto inválida: {}", proyecto))?;
    match base_proy < base_req {
        true => Err(format!(
            "La librería requiere Umbral {} pero el proyecto usa {}",
            requerida, proyecto
        )),
        false => Ok(()),
    }
}

fn base_semver(version: &str) -> Result<semver::Version, semver::Error> {
    semver::Version::parse(
        version
            .trim_start_matches(">=")
            .trim_start_matches('^')
            .trim(),
    )
}

fn remover_si_existe(ruta: &Path) -> Result<(), String> {
    match ruta.exists() {
        false => Ok(()),
        true => fs::remove_dir_all(ruta)
            .map_err(|e| format!("Error eliminando {}: {}", ruta.display(), e)),
    }
}

fn clonar_tag(repositorio: &str, tag: &str, destino: &Path) -> Result<(), String> {
    println!("  Clonando desde {} (tag {})...", repositorio, tag);
    let ruta = destino
        .to_str()
        .ok_or_else(|| format!("Ruta no válida: {}", destino.display()))?;
    let salida = Command::new("git")
        .args(["clone", "--depth", "1", "--branch", tag, repositorio, ruta])
        .output()
        .map_err(|e| format!("Error ejecutando git clone: {}", e))?;
    match salida.status.success() {
        true => Ok(()),
        false => Err(format!(
            "Git clone falló: {}",
            String::from_utf8_lossy(&salida.stderr)
        )),
    }
}

fn respuesta_afirmativa(entrada: &str) -> bool {
    let texto = entrada.trim().to_lowercase();
    texto.is_empty() || ["s", "si", "sí", "y", "yes"].contains(&texto.as_str())
}
