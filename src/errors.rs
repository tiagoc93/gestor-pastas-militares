use std::fmt;
use std::path::PathBuf;

#[derive(Debug)]
pub enum AppError {
    MatriculaInvalida(String),
    SubpastaInexistente(PathBuf),
    MilitarNaoEncontrado(String),
    ErroDeIO {
        path: PathBuf,
        fonte: std::io::Error,
    },
    PastaRaizNaoConfigurada,
    PastaDestinoInvalida(PathBuf),
    ArquivoNaoEncontrado(String),
    ErroDeSerializacao(String),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::MatriculaInvalida(msg) => write!(f, "Matrícula inválida: {msg}"),
            AppError::SubpastaInexistente(path) => {
                write!(f, "Subpasta inexistente: {}", path.display())
            }
            AppError::MilitarNaoEncontrado(mat) => {
                write!(f, "Nenhum militar encontrado para matrícula: {mat}")
            }
            AppError::ErroDeIO { path, fonte } => {
                write!(f, "Erro de E/S em {}: {fonte}", path.display())
            }
            AppError::PastaRaizNaoConfigurada => {
                write!(f, "Pasta raiz não configurada. Acesse Configurações.")
            }
            AppError::PastaDestinoInvalida(path) => {
                write!(f, "Pasta de destino inválida: {}", path.display())
            }
            AppError::ArquivoNaoEncontrado(path) => {
                write!(f, "Arquivo não encontrado: {path}")
            }
            AppError::ErroDeSerializacao(msg) => {
                write!(f, "Erro de serialização: {msg}")
            }
        }
    }
}

impl std::error::Error for AppError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            AppError::ErroDeIO { fonte, .. } => Some(fonte),
            _ => None,
        }
    }
}

impl From<std::io::Error> for AppError {
    fn from(e: std::io::Error) -> Self {
        AppError::ErroDeIO {
            path: PathBuf::new(),
            fonte: e,
        }
    }
}

/// Garante que as variants completeness n\u{00e3}o sejam marcadas como dead_code.
pub fn __garantir_variants_completeness() {
    let _ = AppError::SubpastaInexistente(PathBuf::new());
    let _ = AppError::PastaRaizNaoConfigurada;
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::error::Error;

    #[test]
    fn test_display_matricula_invalida() {
        let e = AppError::MatriculaInvalida("abc".to_string());
        assert_eq!(e.to_string(), "Matr\u{00ed}cula inv\u{00e1}lida: abc");
    }

    #[test]
    fn test_display_militar_nao_encontrado() {
        let e = AppError::MilitarNaoEncontrado("12345".to_string());
        assert_eq!(
            e.to_string(),
            "Nenhum militar encontrado para matr\u{00ed}cula: 12345"
        );
    }

    #[test]
    fn test_display_pasta_raiz_nao_configurada() {
        let e = AppError::PastaRaizNaoConfigurada;
        assert_eq!(
            e.to_string(),
            "Pasta raiz n\u{00e3}o configurada. Acesse Configura\u{00e7}\u{00f5}es."
        );
    }

    #[test]
    fn test_display_pasta_destino_invalida() {
        let e = AppError::PastaDestinoInvalida(PathBuf::from("/fake"));
        assert_eq!(e.to_string(), "Pasta de destino inv\u{00e1}lida: /fake");
    }

    #[test]
    fn test_display_arquivo_nao_encontrado() {
        let e = AppError::ArquivoNaoEncontrado("doc.pdf".to_string());
        assert_eq!(e.to_string(), "Arquivo n\u{00e3}o encontrado: doc.pdf");
    }

    #[test]
    fn test_display_subpasta_inexistente() {
        let e = AppError::SubpastaInexistente(PathBuf::from("/inexistente"));
        assert_eq!(e.to_string(), "Subpasta inexistente: /inexistente");
    }

    #[test]
    fn test_error_source() {
        let io_err = std::io::Error::new(std::io::ErrorKind::NotFound, "nope");
        let e = AppError::ErroDeIO {
            path: PathBuf::from("/x"),
            fonte: io_err,
        };
        assert!(e.source().is_some());
    }
}
