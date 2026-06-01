pub mod config;
pub mod filesystem;
pub mod matricula;

pub use config::{AppConfig, PoliticaSobrescrita, Tema};
pub use filesystem::{copiar_arquivo, copiar_pasta_recursiva, criar_pasta_militar, encontrar_pasta_militar, CopyStats};
pub use matricula::Matricula;
