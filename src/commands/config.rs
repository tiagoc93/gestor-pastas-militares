use crate::core::{AppConfig, PoliticaSobrescrita, Tema};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct AppConfigDto {
    pub pasta_raiz: Option<String>,
    pub politica_sobrescrita: String,
    pub tema: String,
}

impl From<AppConfig> for AppConfigDto {
    fn from(c: AppConfig) -> Self {
        AppConfigDto {
            pasta_raiz: c.pasta_raiz,
            politica_sobrescrita: match c.politica_sobrescrita {
                PoliticaSobrescrita::Sobrescrever => "sobrescrever".to_string(),
                PoliticaSobrescrita::Pular => "pular".to_string(),
                PoliticaSobrescrita::RenomearComSufixo => "renomear".to_string(),
            },
            tema: match c.tema {
                Tema::Claro => "claro".to_string(),
                Tema::Escuro => "escuro".to_string(),
            },
        }
    }
}

impl From<AppConfigDto> for AppConfig {
    fn from(dto: AppConfigDto) -> Self {
        AppConfig {
            pasta_raiz: dto.pasta_raiz,
            politica_sobrescrita: match dto.politica_sobrescrita.as_str() {
                "pular" => PoliticaSobrescrita::Pular,
                "renomear" => PoliticaSobrescrita::RenomearComSufixo,
                _ => PoliticaSobrescrita::Sobrescrever,
            },
            tema: match dto.tema.as_str() {
                "claro" => Tema::Claro,
                _ => Tema::Escuro,
            },
        }
    }
}

#[tauri::command]
pub fn carregar_config() -> Result<AppConfigDto, String> {
    AppConfig::carregar()
        .map(AppConfigDto::from)
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn salvar_config(config: AppConfigDto) -> Result<(), String> {
    let app_config: AppConfig = config.into();
    app_config.salvar().map_err(|e| e.to_string())
}

#[tauri::command]
pub fn selecionar_pasta_raiz(
    app: tauri::AppHandle,
) -> Result<Option<String>, String> {
    use tauri_plugin_dialog::DialogExt;
    use std::sync::mpsc::channel;

    let (tx, rx) = channel();
    app.dialog().file().pick_folder(move |path| {
        let _ = tx.send(path);
    });

    let path = rx.recv().map_err(|e| e.to_string())?;

    Ok(path.map(|p| p.into_path().map(|pb| pb.to_string_lossy().to_string()))
        .transpose()
        .map_err(|e| format!("Erro ao converter caminho: {:?}", e))?)
}

#[tauri::command]
pub fn selecionar_arquivo(
    app: tauri::AppHandle,
) -> Result<Option<String>, String> {
    use tauri_plugin_dialog::DialogExt;
    use std::sync::mpsc::channel;

    let (tx, rx) = channel();
    app.dialog().file().pick_file(move |path| {
        let _ = tx.send(path);
    });

    let path = rx.recv().map_err(|e| e.to_string())?;

    Ok(path.map(|p| p.into_path().map(|pb| pb.to_string_lossy().to_string()))
        .transpose()
        .map_err(|e| format!("Erro ao converter caminho: {:?}", e))?)
}
