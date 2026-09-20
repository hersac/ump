use crate::comandos::comun;
use crate::configuracion::ProyectoUmp;
use std::collections::HashMap;
use std::path::Path;

pub fn ejecutar(paquetes: &Vec<String>) {
    if paquetes.is_empty() {
        return mostrar_uso();
    }
    let proyecto = match comun::leer_proyecto() {
        Ok(proyecto) => proyecto,
        Err(error) => return eprintln!("{}", error),
    };
    let actuales = proyecto.dependencies.clone().unwrap_or_default();
    if actuales.is_empty() {
        return eprintln!("No hay dependencias en umpkg.yml.");
    }
    let indice = match comun::obtener_indice_global() {
        Ok(indice) => indice,
        Err(error) => return eprintln!("Error obteniendo índice global: {}", error),
    };
    match comun::asegurar_modulos() {
        Ok(()) => (),
        Err(error) => return eprintln!("{}", error),
    }
    let planes: Vec<(String, String)> = paquetes
        .iter()
        .filter_map(|spec| plan_upgrade(spec, &actuales, &indice, &proyecto.umbral))
        .collect();
    aplicar_planes(proyecto, planes)
}

fn mostrar_uso() {
    eprintln!("Uso: ump upgrade <paquete>[@version] [...]");
    eprintln!("Ejemplos:");
    eprintln!("  ump upgrade http          # actualiza a la última");
    eprintln!("  ump upgrade http@1.1.0    # actualiza a esa versión");
    eprintln!("Tip: 'ump update' lista lo que tiene versión nueva.");
}

fn plan_upgrade(
    spec: &str,
    actuales: &HashMap<String, String>,
    indice: &HashMap<String, comun::EntradaIndice>,
    umbral: &str,
) -> Option<(String, String)> {
    match resolver_upgrade(spec, actuales, indice, umbral) {
        Ok(plan) => plan,
        Err(error) => {
            eprintln!("Error con '{}': {}", spec, error);
            None
        }
    }
}

fn resolver_upgrade(
    spec: &str,
    actuales: &HashMap<String, String>,
    indice: &HashMap<String, comun::EntradaIndice>,
    umbral: &str,
) -> Result<Option<(String, String)>, String> {
    let (nombre, pedida) = comun::parsear_especificacion(spec);
    let actual = actuales.get(&nombre).ok_or_else(|| {
        format!(
            "La librería '{}' no está en las dependencias (usa 'ump add {}' para instalarla)",
            nombre, spec
        )
    })?;
    let entrada = indice
        .get(&nombre)
        .ok_or_else(|| format!("La librería '{}' no existe en el índice global", nombre))?;
    let destino = destino_upgrade(&nombre, actual, pedida, &entrada.repository)?;
    match destino {
        Some(destino) => confirmar_upgrade(
            &nombre,
            actual,
            &destino,
            &entrada.repository,
            umbral,
            &entrada.umbral,
        ),
        None => Ok(None),
    }
}

fn destino_upgrade(
    nombre: &str,
    actual: &str,
    pedida: Option<String>,
    repositorio: &str,
) -> Result<Option<String>, String> {
    match pedida {
        Some(pedida) => destino_fijo(nombre, actual, &pedida, repositorio),
        None => destino_reciente(nombre, actual, repositorio),
    }
}

fn destino_fijo(
    nombre: &str,
    actual: &str,
    pedida: &str,
    repositorio: &str,
) -> Result<Option<String>, String> {
    let normal = comun::validar_version_existe(repositorio, pedida)?;
    match comun::normalizar_version(actual) == normal {
        true => {
            println!("✓ {} ya está en v{} (nada que hacer).", nombre, actual);
            Ok(None)
        }
        false => Ok(Some(normal)),
    }
}

fn destino_reciente(
    nombre: &str,
    actual: &str,
    repositorio: &str,
) -> Result<Option<String>, String> {
    let ultima = comun::obtener_ultima_version(repositorio)?;
    match comun::es_version_nueva(actual, &ultima) {
        true => Ok(Some(ultima)),
        false => {
            println!("✓ {} ya está al día (v{}).", nombre, actual);
            Ok(None)
        }
    }
}

fn confirmar_upgrade(
    nombre: &str,
    actual: &str,
    destino: &str,
    repositorio: &str,
    umbral: &str,
    respaldo: &str,
) -> Result<Option<(String, String)>, String> {
    let requerido =
        comun::umbral_de_version(repositorio, destino).unwrap_or_else(|| respaldo.to_string());
    comun::validar_compatibilidad_umbral(umbral, &requerido)?;
    println!(
        "📦 {} v{} -> v{} (requiere Umbral {})",
        nombre, actual, destino, requerido
    );
    match comun::confirmar(&format!(
        "¿Actualizar {} de v{} a v{}?",
        nombre, actual, destino
    )) {
        false => {
            println!("  ✕ Actualización cancelada por el usuario.");
            Ok(None)
        }
        true => descargar_upgrade(nombre, repositorio, destino),
    }
}

fn descargar_upgrade(
    nombre: &str,
    repositorio: &str,
    destino: &str,
) -> Result<Option<(String, String)>, String> {
    comun::descargar_paquete(nombre, repositorio, destino, Path::new("modules_ump"))?;
    println!("✓ {} actualizado a v{}", nombre, destino);
    Ok(Some((nombre.to_string(), destino.to_string())))
}

fn aplicar_planes(proyecto: ProyectoUmp, planes: Vec<(String, String)>) {
    if planes.is_empty() {
        return;
    }
    let base = proyecto.dependencies.clone().unwrap_or_default();
    let fusionadas: HashMap<String, String> = base.into_iter().chain(planes).collect();
    let final_ = ProyectoUmp {
        dependencies: Some(fusionadas),
        ..proyecto
    };
    match comun::guardar_proyecto(&final_) {
        Ok(()) => println!("✓ Actualizado umpkg.yml"),
        Err(error) => eprintln!("{}", error),
    }
}
