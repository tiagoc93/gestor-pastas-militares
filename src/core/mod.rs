pub mod config;
pub mod filesystem;
pub mod matricula;

pub use config::{AppConfig, Tema};
pub use filesystem::{
    buscar_pasta_militar_flat, copiar_arquivo, copiar_pasta_recursiva, criar_pasta_militar,
    encontrar_todos_militares, PoliticaSobrescrita,
};
pub use matricula::Matricula;
