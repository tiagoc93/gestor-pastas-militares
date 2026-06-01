use crate::core::{AppConfig, PoliticaSobrescrita, Tema};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppConfigDto {
    #[serde(default)]
    pub pasta_raiz: Option<String>,
    #[serde(default = "default_politica")]
    pub politica_sobrescrita: String,
    #[serde(default = "default_tema")]
    pub tema: String,
}

fn default_politica() -> String {
    "sobrescrever".to_string()
}

fn default_tema() -> String {
    "escuro".to_string()
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
pub async fn selecionar_pasta_raiz(app: tauri::AppHandle) -> Result<Option<String>, String> {
    use tauri_plugin_dialog::DialogExt;

    let path = app.dialog().file().blocking_pick_folder();

    match path {
        Some(p) => Ok(Some(
            p.into_path()
                .map(|pb| pb.to_string_lossy().to_string())
                .map_err(|e| format!("Erro ao converter caminho: {:?}", e))?,
        )),
        None => Ok(None),
    }
}

#[tauri::command]
pub async fn selecionar_arquivo(app: tauri::AppHandle) -> Result<Option<String>, String> {
    use tauri_plugin_dialog::DialogExt;

    let path = app.dialog().file().blocking_pick_file();

    match path {
        Some(p) => Ok(Some(
            p.into_path()
                .map(|pb| pb.to_string_lossy().to_string())
                .map_err(|e| format!("Erro ao converter caminho: {:?}", e))?,
        )),
        None => Ok(None),
    }
}
