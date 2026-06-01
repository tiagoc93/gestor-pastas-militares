use crate::core::{
    copiar_arquivo, copiar_pasta_recursiva, encontrar_todos_militares, Matricula,
    PoliticaSobrescrita,
};
use serde::Serialize;
use std::path::PathBuf;

#[derive(Debug, Serialize)]
pub struct BuscaResult {
    pub status: String,
    pub path_destino: Option<String>,
    pub arquivos_copiados: usize,
    pub ja_existe: bool,
    pub matches_encontrados: usize,
}

fn downloads_path() -> Result<PathBuf, String> {
    dirs::download_dir()
        .ok_or_else(|| "Não foi possível encontrar a pasta de Downloads".to_string())
}

fn copiar_matches(matches: &[PathBuf], destino: &PathBuf) -> Result<usize, String> {
    std::fs::create_dir_all(destino).map_err(|e| e.to_string())?;

    let mut total_arquivos = 0usize;

    for item in matches {
        if item.is_dir() {
            let stats = copiar_pasta_recursiva(item, destino).map_err(|e| e.to_string())?;
            total_arquivos += stats.arquivos_copiados;
        } else {
            copiar_arquivo(item, destino, PoliticaSobrescrita::Sobrescrever)
                .map_err(|e| e.to_string())?;
            total_arquivos += 1;
        }
    }

    Ok(total_arquivos)
}

#[tauri::command]
pub fn buscar_matricula(raiz: String, matricula: String) -> Result<BuscaResult, String> {
    let m = Matricula::parse(&matricula).map_err(|e| e.to_string())?;
    let raiz_path = std::path::Path::new(&raiz);

    let matches = encontrar_todos_militares(raiz_path, &m).map_err(|e| e.to_string())?;

    if matches.is_empty() {
        return Err(format!("Militar não encontrado: {}", matricula));
    }

    let downloads = downloads_path()?;
    let destino = downloads.join("Busca-Matriculas").join(&m.com_hifen);

    let ja_existe = destino.exists();

    if ja_existe {
        return Ok(BuscaResult {
            status: "ja_existe".to_string(),
            path_destino: Some(destino.to_string_lossy().to_string()),
            arquivos_copiados: 0,
            ja_existe: true,
            matches_encontrados: matches.len(),
        });
    }

    let total_arquivos = copiar_matches(&matches, &destino)?;

    Ok(BuscaResult {
        status: "sucesso".to_string(),
        path_destino: Some(destino.to_string_lossy().to_string()),
        arquivos_copiados: total_arquivos,
        ja_existe: false,
        matches_encontrados: matches.len(),
    })
}

#[tauri::command]
pub fn confirmar_sobrescrita_busca(raiz: String, matricula: String) -> Result<BuscaResult, String> {
    let m = Matricula::parse(&matricula).map_err(|e| e.to_string())?;
    let raiz_path = std::path::Path::new(&raiz);

    let matches = encontrar_todos_militares(raiz_path, &m).map_err(|e| e.to_string())?;

    if matches.is_empty() {
        return Err(format!("Militar não encontrado: {}", matricula));
    }

    let downloads = downloads_path()?;
    let destino = downloads.join("Busca-Matriculas").join(&m.com_hifen);

    if destino.exists() {
        std::fs::remove_dir_all(&destino).map_err(|e| e.to_string())?;
    }

    let total_arquivos = copiar_matches(&matches, &destino)?;

    Ok(BuscaResult {
        status: "sucesso".to_string(),
        path_destino: Some(destino.to_string_lossy().to_string()),
        arquivos_copiados: total_arquivos,
        ja_existe: false,
        matches_encontrados: matches.len(),
    })
}
