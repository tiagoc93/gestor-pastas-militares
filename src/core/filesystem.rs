use crate::core::matricula::Matricula;
use crate::errors::AppError;
use std::fs;
use std::path::{Path, PathBuf};

/// Enum que define o comportamento quando o arquivo já existe no destino
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PoliticaSobrescrita {
    Sobrescrever,
    Pular,
    RenomearComSufixo,
}

/// Estatísticas de uma cópia recursiva de pasta
#[derive(Debug, Clone)]
pub struct CopyStats {
    pub arquivos_copiados: usize,
    pub bytes_copiados: u64,
}

/// Encontra a pasta de um militar dentro da pasta raiz.
///
/// Busca por prefixo da matrícula (com ou sem hífen), case-insensitive.
pub fn encontrar_pasta_militar(
    raiz: &Path,
    matricula: &Matricula,
) -> Result<Option<PathBuf>, AppError> {
    let subpasta_path = raiz.join(&matricula.subpasta);

    if !subpasta_path.exists() {
        return Ok(None);
    }

    for entry in fs::read_dir(&subpasta_path)? {
        let entry = entry?;
        let nome = entry.file_name();
        let nome_str = nome.to_string_lossy();

        if nome_str.starts_with(&matricula.digitos)
            || nome_str.starts_with(&matricula.com_hifen)
            || nome_str.replace("-", "").starts_with(&matricula.digitos)
        {
            if entry.file_type()?.is_dir() {
                return Ok(Some(entry.path()));
            }
        }
    }

    Ok(None)
}

/// Cria a pasta de um novo militar.
///
/// Nome: `{COM_HIFEN} - {NOME_EM_MAIUSCULAS}`
pub fn criar_pasta_militar(
    raiz: &Path,
    matricula: &Matricula,
    nome_completo: &str,
) -> Result<PathBuf, AppError> {
    let subpasta_path = raiz.join(&matricula.subpasta);
    fs::create_dir_all(&subpasta_path)?;

    let nome_maiuscular = nome_completo.trim().to_uppercase();
    let nome_pasta = format!("{} - {}", matricula.com_hifen, nome_maiuscular);
    let caminho_pasta = subpasta_path.join(&nome_pasta);

    fs::create_dir_all(&caminho_pasta)?;

    Ok(caminho_pasta)
}

/// Copia um arquivo para um diretório destino.
///
/// O arquivo é lido uma única vez em memória.
pub fn copiar_arquivo(
    arquivo_origem: &Path,
    destino_dir: &Path,
    politica: PoliticaSobrescrita,
) -> Result<(), AppError> {
    if !arquivo_origem.exists() {
        return Err(AppError::ArquivoNaoEncontrado(
            arquivo_origem.to_string_lossy().to_string(),
        ));
    }

    fs::create_dir_all(destino_dir)?;

    let nome_arquivo = arquivo_origem
        .file_name()
        .ok_or_else(|| AppError::PastaDestinoInvalida("Nome de arquivo inválido".to_string()))?;

    let mut destino_path = destino_dir.join(nome_arquivo);

    if destino_path.exists() {
        match politica {
            PoliticaSobrescrita::Pular => return Ok(()),
            PoliticaSobrescrita::RenomearComSufixo => {
                let (stem, ext) = split_file_stem_ext(nome_arquivo);
                let timestamp = std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_secs();
                let novo_nome = format!("{}_{}.{}", stem, timestamp, ext);
                destino_path = destino_dir.join(novo_nome);
            }
            PoliticaSobrescrita::Sobrescrever => {}
        }
    }

    let dados = fs::read(arquivo_origem)?;
    fs::write(&destino_path, dados)?;

    Ok(())
}

/// Copia uma pasta recursivamente para o destino.
pub fn copiar_pasta_recursiva(origem: &Path, destino: &Path) -> Result<CopyStats, AppError> {
    if !origem.exists() {
        return Err(AppError::PastaDestinoInvalida(format!(
            "Pasta de origem não existe: {}",
            origem.display()
        )));
    }

    fs::create_dir_all(destino)?;

    let mut stats = CopyStats {
        arquivos_copiados: 0,
        bytes_copiados: 0,
    };

    if origem.is_dir() {
        for entry in fs::read_dir(origem)? {
            let entry = entry?;
            let path_origem = entry.path();
            let path_destino = destino.join(entry.file_name());

            if path_origem.is_dir() {
                let sub_stats = copiar_pasta_recursiva(&path_origem, &path_destino)?;
                stats.arquivos_copiados += sub_stats.arquivos_copiados;
                stats.bytes_copiados += sub_stats.bytes_copiados;
            } else {
                let dados = fs::read(&path_origem)?;
                let bytes = dados.len() as u64;
                fs::write(&path_destino, dados)?;
                stats.arquivos_copiados += 1;
                stats.bytes_copiados += bytes;
            }
        }
    }

    Ok(stats)
}

fn split_file_stem_ext(nome: &Path) -> (String, String) {
    let nome_str = nome.to_string_lossy();
    if let Some(dot_pos) = nome_str.rfind('.') {
        if dot_pos > 0 {
            return (
                nome_str[..dot_pos].to_string(),
                nome_str[dot_pos + 1..].to_string(),
            );
        }
    }
    (nome_str.to_string(), String::new())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_encontrar_pasta_existente() {
        let tmp = TempDir::new().unwrap();
        let raiz = tmp.path();

        let subpasta = raiz.join("11111");
        fs::create_dir_all(&subpasta).unwrap();
        fs::create_dir(subpasta.join("111111-1 - FULANO DE TAL")).unwrap();

        let m = Matricula::parse("111111-1").unwrap();
        let resultado = encontrar_pasta_militar(raiz, &m).unwrap();

        assert!(resultado.is_some());
        assert!(resultado.unwrap().to_string_lossy().contains("FULANO"));
    }

    #[test]
    fn test_encontrar_pasta_inexistente() {
        let tmp = TempDir::new().unwrap();
        let raiz = tmp.path();

        let m = Matricula::parse("999999-9").unwrap();
        let resultado = encontrar_pasta_militar(raiz, &m).unwrap();

        assert!(resultado.is_none());
    }

    #[test]
    fn test_criar_pasta_militar() {
        let tmp = TempDir::new().unwrap();
        let raiz = tmp.path();

        let m = Matricula::parse("111111-1").unwrap();
        let resultado = criar_pasta_militar(raiz, &m, "Fulano de Tal").unwrap();

        assert!(resultado.exists());
        assert!(resultado.to_string_lossy().contains("FULANO DE TAL"));
    }

    #[test]
    fn test_copiar_arquivo() {
        let tmp = TempDir::new().unwrap();
        let arquivo = tmp.path().join("origem.txt");
        fs::write(&arquivo, "conteúdo teste").unwrap();

        let destino = tmp.path().join("destino");
        fs::create_dir_all(&destino).unwrap();

        copiar_arquivo(&arquivo, &destino, PoliticaSobrescrita::Sobrescrever).unwrap();

        assert!(destino.join("origem.txt").exists());
    }
}
