use crate::core::filesystem::PoliticaSobrescrita;
use crate::errors::AppError;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Tema {
    Claro,
    Escuro,
}

impl Default for Tema {
    fn default() -> Self {
        Tema::Escuro
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub pasta_raiz: Option<String>,
    pub politica_sobrescrita: PoliticaSobrescrita,
    pub tema: Tema,
}

impl Default for AppConfig {
    fn default() -> Self {
        AppConfig {
            pasta_raiz: None,
            politica_sobrescrita: PoliticaSobrescrita::Sobrescrever,
            tema: Tema::default(),
        }
    }
}

impl AppConfig {
    fn path() -> PathBuf {
        dirs::config_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("gestor-militar")
            .join("config.json")
    }

    pub fn carregar() -> Result<AppConfig, AppError> {
        let path = Self::path();

        if !path.exists() {
            return Ok(AppConfig::default());
        }

        let conteudo = fs::read_to_string(&path).map_err(|e| {
            AppError::ErroDeIO {
                path: path.to_string_lossy().to_string(),
                fonte: e.to_string(),
            }
        })?;

        serde_json::from_str(&conteudo).map_err(|e| AppError::ErroDeSerializacao(e.to_string()))
    }

    pub fn salvar(&self) -> Result<(), AppError> {
        let path = Self::path();

        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).map_err(|e| {
                AppError::ErroDeIO {
                    path: parent.to_string_lossy().to_string(),
                    fonte: e.to_string(),
                }
            })?;
        }

        let conteudo = serde_json::to_string_pretty(self)
            .map_err(|e| AppError::ErroDeSerializacao(e.to_string()))?;

        fs::write(&path, conteudo).map_err(|e| {
            AppError::ErroDeIO {
                path: path.to_string_lossy().to_string(),
                fonte: e.to_string(),
            }
        })?;

        Ok(())
    }
}
