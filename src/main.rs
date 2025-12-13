mod comandos;
mod configuracion;
mod interfaz;

use clap::Parser;
use interfaz::{Comandos, Interfaz};

fn main() {
    let interfaz = Interfaz::parse();

    match &interfaz.comando {
        Comandos::Inicio => {
            comandos::inicio::ejecutar();
        }
        Comandos::Agregar { paquetes } => {
            comandos::agregar::ejecutar(paquetes);
        }
        Comandos::Ejecutar { script } => {
            comandos::ejecutar::ejecutar(script);
        }
        Comandos::Crear { nombre } => {
            comandos::crear::ejecutar(nombre);
        }
    }
}
