use crate::core::matricula::Matricula;
use crate::errors::AppError;
use serde::{Deserialize, Serialize};
use std::ffi::OsStr;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
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

        if (nome_str.starts_with(&matricula.digitos)
            || nome_str.starts_with(&matricula.com_hifen)
            || nome_str.replace("-", "").starts_with(&matricula.digitos))
            && entry.file_type()?.is_dir()
        {
            return Ok(Some(entry.path()));
        }
    }

    Ok(None)
}

/// Busca flat: encontra TODOS os entries (pastas e arquivos) cujo nome começa
/// com a matrícula, em todas as subpastas da raiz.
pub fn encontrar_todos_militares(
    raiz: &Path,
    matricula: &Matricula,
) -> Result<Vec<PathBuf>, AppError> {
    let mut resultados = Vec::new();

    if !raiz.exists() {
        return Ok(resultados);
    }

    let digitos_lower = matricula.digitos.to_lowercase();
    let hifen_lower = matricula.com_hifen.to_lowercase();

    for subpasta_entry in fs::read_dir(raiz)? {
        let subpasta_entry = subpasta_entry?;
        let subpasta_path = subpasta_entry.path();

        if !subpasta_path.is_dir() {
            continue;
        }

        for entry in fs::read_dir(&subpasta_path)? {
            let entry = entry?;
            let nome = entry.file_name();
            let nome_lower = nome.to_string_lossy().to_lowercase();
            let nome_sem_hifen = nome_lower.replace("-", "");

            let matchou = nome_lower.starts_with(&digitos_lower)
                || nome_lower.starts_with(&hifen_lower)
                || nome_sem_hifen.starts_with(&digitos_lower);

            if matchou {
                resultados.push(entry.path());
            }
        }
    }

    Ok(resultados)
}

/// Busca flat: encontra o PRIMEIRO match (pasta ou arquivo) cujo nome come\u{00e7}a
/// com a matr\u{00ed}cula, iterando todas as subpastas diretas da raiz (um n\u{00ed}vel apenas).
/// Retorna apenas o primeiro resultado encontrado.
pub fn buscar_pasta_militar_flat(
    raiz: &Path,
    matricula: &Matricula,
) -> Result<Option<PathBuf>, AppError> {
    if !raiz.exists() {
        return Ok(None);
    }

    let digitos_lower = matricula.digitos.to_lowercase();
    let hifen_lower = matricula.com_hifen.to_lowercase();

    for subpasta_entry in fs::read_dir(raiz)? {
        let subpasta_entry = subpasta_entry?;
        let subpasta_path = subpasta_entry.path();

        if !subpasta_path.is_dir() {
            continue;
        }

        for entry in fs::read_dir(&subpasta_path)? {
            let entry = entry?;
            let nome = entry.file_name();
            let nome_lower = nome.to_string_lossy().to_lowercase();
            let nome_sem_hifen = nome_lower.replace("-", "");

            let matchou = nome_lower.starts_with(&digitos_lower)
                || nome_lower.starts_with(&hifen_lower)
                || nome_sem_hifen.starts_with(&digitos_lower);

            if matchou {
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
        .ok_or_else(|| AppError::PastaDestinoInvalida(arquivo_origem.to_path_buf()))?;

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
        return Err(AppError::PastaDestinoInvalida(origem.to_path_buf()));
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

fn split_file_stem_ext(nome: &OsStr) -> (String, String) {
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

    #[test]
    fn test_encontrar_todos_multiplas_subpastas() {
        let tmp = TempDir::new().unwrap();
        let raiz = tmp.path();

        let sub1 = raiz.join("11111");
        fs::create_dir_all(&sub1).unwrap();
        fs::create_dir(sub1.join("111111-1 - FULANO DE TAL")).unwrap();

        let sub2 = raiz.join("outra");
        fs::create_dir_all(&sub2).unwrap();
        fs::create_dir(sub2.join("111111-1 - FULANO COPIA")).unwrap();

        let m = Matricula::parse("111111-1").unwrap();
        let resultados = encontrar_todos_militares(raiz, &m).unwrap();

        assert_eq!(resultados.len(), 2);
    }

    #[test]
    fn test_encontrar_todos_arquivo_solto() {
        let tmp = TempDir::new().unwrap();
        let raiz = tmp.path();

        let sub = raiz.join("11111");
        fs::create_dir_all(&sub).unwrap();
        fs::write(sub.join("111111-1 - documento.pdf"), b"conteudo").unwrap();

        let m = Matricula::parse("111111-1").unwrap();
        let resultados = encontrar_todos_militares(raiz, &m).unwrap();

        assert_eq!(resultados.len(), 1);
        assert!(resultados[0].is_file());
    }

    #[test]
    fn test_encontrar_todos_vazio() {
        let tmp = TempDir::new().unwrap();
        let raiz = tmp.path();

        let m = Matricula::parse("999999-9").unwrap();
        let resultados = encontrar_todos_militares(raiz, &m).unwrap();

        assert!(resultados.is_empty());
    }

    #[test]
    fn test_buscar_flat_primeiro_match() {
        let tmp = TempDir::new().unwrap();
        let raiz = tmp.path();

        let sub1 = raiz.join("aaa");
        fs::create_dir_all(&sub1).unwrap();
        fs::create_dir(sub1.join("111111-1 - FULANO")).unwrap();

        let sub2 = raiz.join("bbb");
        fs::create_dir_all(&sub2).unwrap();
        fs::create_dir(sub2.join("111111-1 - CICLANO")).unwrap();

        let m = Matricula::parse("111111-1").unwrap();
        let resultado = buscar_pasta_militar_flat(raiz, &m).unwrap();

        assert!(resultado.is_some());
        let path = resultado.unwrap();
        let path_str = path.to_string_lossy();
        assert!(path_str.contains("FULANO") || path_str.contains("CICLANO"));
    }

    #[test]
    fn test_buscar_flat_arquivo_solto() {
        let tmp = TempDir::new().unwrap();
        let raiz = tmp.path();

        let sub = raiz.join("docs");
        fs::create_dir_all(&sub).unwrap();
        fs::write(sub.join("111111-1 - doc.pdf"), b"conteudo").unwrap();

        let m = Matricula::parse("111111-1").unwrap();
        let resultado = buscar_pasta_militar_flat(raiz, &m).unwrap();

        assert!(resultado.is_some());
        assert!(resultado.unwrap().is_file());
    }

    #[test]
    fn test_buscar_flat_nenhum_match() {
        let tmp = TempDir::new().unwrap();
        let raiz = tmp.path();

        let sub = raiz.join("aaa");
        fs::create_dir_all(&sub).unwrap();
        fs::create_dir(sub.join("999999-9 - OUTRO")).unwrap();

        let m = Matricula::parse("111111-1").unwrap();
        let resultado = buscar_pasta_militar_flat(raiz, &m).unwrap();

        assert!(resultado.is_none());
    }

    #[test]
    fn test_buscar_flat_case_insensitive() {
        let tmp = TempDir::new().unwrap();
        let raiz = tmp.path();

        let sub = raiz.join("aaa");
        fs::create_dir_all(&sub).unwrap();
        fs::create_dir(sub.join("111111-1 - FULANO")).unwrap();

        let m = Matricula::parse("111111-1").unwrap();
        let resultado = buscar_pasta_militar_flat(raiz, &m).unwrap();

        assert!(resultado.is_some());
    }
}
