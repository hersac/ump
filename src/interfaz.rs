use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "ump")]
#[command(about = "Gestor de Paquetes Umbral", long_about = None)]
#[command(version)]
pub struct Interfaz {
    #[command(subcommand)]
    pub comando: Comandos,
}

#[derive(Subcommand)]
pub enum Comandos {
    #[command(name = "init")]
    Inicio,
    #[command(name = "add")]
    Agregar { paquetes: Vec<String> },
    #[command(name = "create")]
    Crear { nombre: String },
    #[command(name = "run")]
    Ejecutar { script: String },
}
