use crate::comandos::comun;
use std::collections::HashMap;

enum Revision {
    Nueva((String, String, String)),
    Vigente,
    Omitida,
}

pub fn ejecutar() {
    let dependencias = match comun::leer_proyecto() {
        Ok(proyecto) => proyecto.dependencies.unwrap_or_default(),
        Err(error) => return eprintln!("{}", error),
    };
    if dependencias.is_empty() {
        return println!("No hay dependencias en umpkg.yml.");
    }
    let indice = match comun::obtener_indice_global() {
        Ok(indice) => indice,
        Err(error) => return eprintln!("Error obteniendo índice global: {}", error),
    };
    println!("🔍 Buscando actualizaciones...\n");
    let (nuevas, vigentes) = clasificar(&dependencias, &indice);
    mostrar_reporte(nuevas, vigentes)
}

fn clasificar(
    dependencias: &HashMap<String, String>,
    indice: &HashMap<String, comun::EntradaIndice>,
) -> (Vec<(String, String, String)>, usize) {
    let revisiones: Vec<Revision> = dependencias
        .iter()
        .map(|(nombre, actual)| revisar(nombre, actual, indice))
        .collect();
    let nuevas = revisiones.iter().filter_map(extraer_nueva).collect();
    let vigentes = revisiones
        .iter()
        .filter(|revision| matches!(revision, Revision::Vigente))
        .count();
    (nuevas, vigentes)
}

fn revisar(
    nombre: &str,
    actual: &str,
    indice: &HashMap<String, comun::EntradaIndice>,
) -> Revision {
    let entrada = match indice.get(nombre) {
        Some(entrada) => entrada,
        None => {
            println!(
                "  ⚠ {}: no está en el índice global (actual: {})",
                nombre, actual
            );
            return Revision::Omitida;
        }
    };
    let ultima = match comun::obtener_ultima_version(&entrada.repository) {
        Ok(ultima) => ultima,
        Err(error) => {
            eprintln!(
                "  ⚠ {}: no se pudo consultar versiones: {}",
                nombre, error
            );
            return Revision::Omitida;
        }
    };
    match comun::es_version_nueva(actual, &ultima) {
        true => Revision::Nueva((nombre.to_string(), actual.to_string(), ultima)),
        false => Revision::Vigente,
    }
}

fn extraer_nueva(revision: &Revision) -> Option<(String, String, String)> {
    match revision {
        Revision::Nueva(datos) => Some(datos.clone()),
        Revision::Vigente => None,
        Revision::Omitida => None,
    }
}

fn mostrar_reporte(nuevas: Vec<(String, String, String)>, vigentes: usize) {
    match nuevas.is_empty() {
        true => println!("✓ Todo al día ({} dependencias sin novedades).", vigentes),
        false => mostrar_nuevas(&nuevas),
    }
}

fn mostrar_nuevas(nuevas: &[(String, String, String)]) {
    println!("Actualizaciones disponibles:\n");
    println!("{:<20} {:<12} {:<12}", "PAQUETE", "ACTUAL", "NUEVA");
    println!("{}", "-".repeat(46));
    nuevas.iter().for_each(mostrar_fila);
    println!("\n{} paquete(s) con actualización. Usa:", nuevas.len());
    nuevas.iter().for_each(mostrar_sugerencia);
    println!("\nO fija una versión:  ump upgrade <paquete>@<version>");
}

fn mostrar_fila(datos: &(String, String, String)) {
    println!("{:<20} {:<12} {:<12}", datos.0, datos.1, datos.2);
}

fn mostrar_sugerencia(datos: &(String, String, String)) {
    println!("  ump upgrade {}    # última ({})", datos.0, datos.2);
}
