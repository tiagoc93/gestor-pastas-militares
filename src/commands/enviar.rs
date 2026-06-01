use crate::core::{AppConfig, Matricula};
use serde::{Deserialize, Serialize};
use tauri::Emitter;

#[derive(Debug, Serialize, Deserialize)]
pub struct MatriculaInfo {
    pub digitos: String,
    pub com_hifen: String,
    pub subpasta: String,
}

impl From<Matricula> for MatriculaInfo {
    fn from(m: Matricula) -> Self {
        MatriculaInfo {
            digitos: m.digitos,
            com_hifen: m.com_hifen,
            subpasta: m.subpasta,
        }
    }
}

#[tauri::command]
pub fn adicionar_matricula(matricula: String) -> Result<MatriculaInfo, String> {
    Matricula::parse(&matricula).map(MatriculaInfo::from).map_err(|e| e.to_string())
}

#[derive(Debug, Serialize)]
pub struct VerificacaoResult {
    pub existe: bool,
    pub path: Option<String>,
}

#[tauri::command]
pub fn verificar_pasta_militar(raiz: String, matricula: String) -> Result<VerificacaoResult, String> {
    let m = Matricula::parse(&matricula).map_err(|e| e.to_string())?;
    let raiz_path = std::path::Path::new(&raiz);

    match crate::core::encontrar_pasta_militar(raiz_path, &m) {
        Ok(Some(path)) => Ok(VerificacaoResult {
            existe: true,
            path: Some(path.to_string_lossy().to_string()),
        }),
        Ok(None) => Ok(VerificacaoResult { existe: false, path: None }),
        Err(e) => Err(e.to_string()),
    }
}

#[tauri::command]
pub fn criar_pasta_militar(
    raiz: String,
    matricula: String,
    nome_completo: String,
) -> Result<String, String> {
    let m = Matricula::parse(&matricula).map_err(|e| e.to_string())?;
    let raiz_path = std::path::Path::new(&raiz);

    crate::core::criar_pasta_militar(raiz_path, &m, &nome_completo)
        .map(|p| p.to_string_lossy().to_string())
        .map_err(|e| e.to_string())
}

#[derive(Debug, Deserialize)]
pub struct Destino {
    pub tipo: String,
    pub valor: String,
}

#[derive(Debug, Deserialize)]
pub struct EnviarPayload {
    pub arquivo: String,
    pub destinos: Vec<Destino>,
    pub raiz: String,
}

#[derive(Debug, Serialize)]
pub struct DetalheEnvio {
    pub destino: String,
    pub sucesso: bool,
    pub erro: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct RelatorioEnvio {
    pub total: usize,
    pub sucesso: usize,
    pub falha: usize,
    pub detalhes: Vec<DetalheEnvio>,
}

#[tauri::command]
pub fn enviar_documento(
    payload: EnviarPayload,
    window: tauri::Window,
) -> Result<RelatorioEnvio, String> {
    let arquivo_path = std::path::Path::new(&payload.arquivo);
    let raiz_path = std::path::Path::new(&payload.raiz);
    let config = AppConfig::carregar().map_err(|e| e.to_string())?;
    let politica = config.politica_sobrescrita;

    let mut relatorio = RelatorioEnvio {
        total: payload.destinos.len(),
        sucesso: 0,
        falha: 0,
        detalhes: Vec::new(),
    };

    for destino in &payload.destinos {
        let resultado = match destino.tipo.as_str() {
            "matricula" => {
                let m = Matricula::parse(&destino.valor).map_err(|e| e.to_string())?;
                match crate::core::encontrar_pasta_militar(raiz_path, &m) {
                    Ok(Some(pasta)) => {
                        crate::core::copiar_arquivo(arquivo_path, &pasta, politica)
                    }
                    Ok(None) => Err(crate::errors::AppError::MilitarNaoEncontrado(destino.valor.clone())),
                    Err(e) => Err(e),
                }
            }
            "pasta" => {
                let pasta_path = std::path::Path::new(&destino.valor);
                crate::core::copiar_arquivo(arquivo_path, pasta_path, politica)
            }
            _ => Err(crate::errors::AppError::PastaDestinoInvalida(std::path::PathBuf::from(&destino.valor))),
        };

        let detalhe = match resultado {
            Ok(()) => {
                relatorio.sucesso += 1;
                DetalheEnvio {
                    destino: destino.valor.clone(),
                    sucesso: true,
                    erro: None,
                }
            }
            Err(e) => {
                relatorio.falha += 1;
                DetalheEnvio {
                    destino: destino.valor.clone(),
                    sucesso: false,
                    erro: Some(e.to_string()),
                }
            }
        };

        let _ = window.emit("progresso-envio", &detalhe);
        relatorio.detalhes.push(detalhe);
    }

    Ok(relatorio)
}
