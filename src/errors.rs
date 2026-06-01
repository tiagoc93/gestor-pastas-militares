use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Matrícula inválida: {0}")]
    MatriculaInvalida(String),

    #[error("Subpasta inexistente: {0}")]
    SubpastaInexistente(String),

    #[error("Militar não encontrado: {0}")]
    MilitarNaoEncontrado(String),

    #[error("Erro de E/S em {path}: {fonte}")]
    ErroDeIO { path: String, fonte: String },

    #[error("Pasta raiz não configurada")]
    PastaRaizNaoConfigurada,

    #[error("Pasta destino inválida: {0}")]
    PastaDestinoInvalida(String),

    #[error("Arquivo não encontrado: {0}")]
    ArquivoNaoEncontrado(String),

    #[error("Erro de serialização: {0}")]
    ErroDeSerializacao(String),
}

impl From<std::io::Error> for AppError {
    fn from(e: std::io::Error) -> Self {
        AppError::ErroDeIO {
            path: String::new(),
            fonte: e.to_string(),
        }
    }
}
