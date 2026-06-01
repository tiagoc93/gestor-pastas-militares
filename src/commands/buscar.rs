use crate::core::{copiar_pasta_recursiva, encontrar_pasta_militar, Matricula};
use serde::Serialize;
use std::path::PathBuf;

#[derive(Debug, Serialize)]
pub struct BuscaResult {
    pub status: String,
    pub path_destino: Option<String>,
    pub arquivos_copiados: usize,
    pub ja_existe: bool,
}

fn downloads_path() -> Result<PathBuf, String> {
    dirs::download_dir()
        .ok_or_else(|| "Não foi possível encontrar a pasta de Downloads".to_string())
}

#[tauri::command]
pub fn buscar_matricula(raiz: String, matricula: String) -> Result<BuscaResult, String> {
    let m = Matricula::parse(&matricula).map_err(|e| e.to_string())?;
    let raiz_path = std::path::Path::new(&raiz);

    let pasta_militar = encontrar_pasta_militar(raiz_path, &m)
        .map_err(|e| e.to_string())?
        .ok_or_else(|| format!("Militar não encontrado: {}", matricula))?;

    let downloads = downloads_path()?;
    let destino = downloads.join("Busca-Matriculas").join(&m.com_hifen);

    let ja_existe = destino.exists();

    if ja_existe {
        return Ok(BuscaResult {
            status: "ja_existe".to_string(),
            path_destino: Some(destino.to_string_lossy().to_string()),
            arquivos_copiados: 0,
            ja_existe: true,
        });
    }

    let stats = copiar_pasta_recursiva(&pasta_militar, &destino).map_err(|e| e.to_string())?;

    Ok(BuscaResult {
        status: "sucesso".to_string(),
        path_destino: Some(destino.to_string_lossy().to_string()),
        arquivos_copiados: stats.arquivos_copiados,
        ja_existe: false,
    })
}

#[tauri::command]
pub fn confirmar_sobrescrita_busca(raiz: String, matricula: String) -> Result<BuscaResult, String> {
    let m = Matricula::parse(&matricula).map_err(|e| e.to_string())?;
    let raiz_path = std::path::Path::new(&raiz);

    let pasta_militar = encontrar_pasta_militar(raiz_path, &m)
        .map_err(|e| e.to_string())?
        .ok_or_else(|| format!("Militar não encontrado: {}", matricula))?;

    let downloads = downloads_path()?;
    let destino = downloads.join("Busca-Matriculas").join(&m.com_hifen);

    if destino.exists() {
        std::fs::remove_dir_all(&destino).map_err(|e| e.to_string())?;
    }

    let stats = copiar_pasta_recursiva(&pasta_militar, &destino).map_err(|e| e.to_string())?;

    Ok(BuscaResult {
        status: "sucesso".to_string(),
        path_destino: Some(destino.to_string_lossy().to_string()),
        arquivos_copiados: stats.arquivos_copiados,
        ja_existe: false,
    })
}
