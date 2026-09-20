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
    let indice = match comun::obtener_indice_global() {
        Ok(indice) => indice,
        Err(error) => return eprintln!("Error obteniendo índice global: {}", error),
    };
    match comun::asegurar_modulos() {
        Ok(()) => (),
        Err(error) => return eprintln!("{}", error),
    }
    let nuevos: Vec<(String, String)> = paquetes
        .iter()
        .filter_map(|spec| plan_agregado(spec, &proyecto, &indice))
        .collect();
    guardar_nuevos(proyecto, nuevos)
}

fn mostrar_uso() {
    eprintln!("Uso: ump add <paquete>[@version] [...]");
    eprintln!("Ejemplos: ump add http | ump add http@1.1.0");
}

fn plan_agregado(
    spec: &str,
    proyecto: &ProyectoUmp,
    indice: &HashMap<String, comun::EntradaIndice>,
) -> Option<(String, String)> {
    match resolver_agregado(spec, proyecto, indice) {
        Ok(plan) => plan,
        Err(error) => {
            eprintln!("Error procesando {}: {}", spec, error);
            None
        }
    }
}

fn resolver_agregado(
    spec: &str,
    proyecto: &ProyectoUmp,
    indice: &HashMap<String, comun::EntradaIndice>,
) -> Result<Option<(String, String)>, String> {
    let (nombre, pedida) = comun::parsear_especificacion(spec);
    let entrada = indice
        .get(&nombre)
        .ok_or_else(|| format!("La librería '{}' no existe en el índice global", nombre))?;
    rechazar_existente(&nombre, proyecto)?;
    let version = match pedida {
        Some(pedida) => comun::validar_version_existe(&entrada.repository, &pedida)?,
        None => comun::obtener_ultima_version(&entrada.repository)?,
    };
    confirmar_agregado(
        &nombre,
        &version,
        &entrada.repository,
        &entrada.umbral,
        &proyecto.umbral,
    )
}

fn rechazar_existente(nombre: &str, proyecto: &ProyectoUmp) -> Result<(), String> {
    let existe = proyecto
        .dependencies
        .as_ref()
        .and_then(|deps| deps.get(nombre))
        .is_some();
    match existe {
        true => Err(format!(
            "La librería '{}' ya existe en el proyecto (usa 'ump upgrade {}' para cambiar de versión)",
            nombre, nombre
        )),
        false => Ok(()),
    }
}

fn confirmar_agregado(
    nombre: &str,
    version: &str,
    repositorio: &str,
    respaldo: &str,
    umbral: &str,
) -> Result<Option<(String, String)>, String> {
    let requerido = comun::umbral_de_version(repositorio, version)
        .unwrap_or_else(|| respaldo.to_string());
    comun::validar_compatibilidad_umbral(umbral, &requerido)?;
    println!("📦 {} v{} (requiere Umbral {})", nombre, version, requerido);
    match comun::confirmar(&format!("¿Instalar {} v{}?", nombre, version)) {
        false => {
            println!("  ✕ Instalación cancelada por el usuario.");
            Ok(None)
        }
        true => instalar_agregado(nombre, version, repositorio),
    }
}

fn instalar_agregado(
    nombre: &str,
    version: &str,
    repositorio: &str,
) -> Result<Option<(String, String)>, String> {
    comun::descargar_paquete(nombre, repositorio, version, Path::new("modules_ump"))?;
    println!("✓ {} v{} instalado correctamente", nombre, version);
    Ok(Some((nombre.to_string(), version.to_string())))
}

fn guardar_nuevos(proyecto: ProyectoUmp, nuevos: Vec<(String, String)>) {
    if nuevos.is_empty() {
        return;
    }
    let base = proyecto.dependencies.clone().unwrap_or_default();
    let fusionadas: HashMap<String, String> = base.into_iter().chain(nuevos).collect();
    let final_ = ProyectoUmp {
        dependencies: Some(fusionadas),
        ..proyecto
    };
    match comun::guardar_proyecto(&final_) {
        Ok(()) => println!("✓ Actualizado umpkg.yml"),
        Err(error) => eprintln!("{}", error),
    }
}
