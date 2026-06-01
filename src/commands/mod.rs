pub mod buscar;
pub mod config;
pub mod enviar;

pub use buscar::{buscar_matricula, confirmar_sobrescrita_busca};
pub use config::{carregar_config, salvar_config};
pub use enviar::{
    adicionar_matricula, criar_pasta_militar, enviar_documento, verificar_pasta_militar,
};
