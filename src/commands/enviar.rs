use crate::core::{
    buscar_pasta_militar_flat, criar_pasta_militar as core_criar_pasta_militar, Matricula,
    PoliticaSobrescrita,
};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};
use std::str::FromStr;
use tauri::Emitter;

// ============================================================================
// Tipos de resposta dos comandos Tauri
// ============================================================================

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

#[derive(Debug, Serialize, Deserialize)]
pub struct VerificacaoResult {
    pub existe: bool,
    pub path: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DetalheEnvio {
    pub destino: String,
    pub sucesso: bool,
    pub erro: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EnvioRelatorio {
    pub total: usize,
    pub sucesso: usize,
    pub falha: usize,
    pub detalhes: Vec<DetalheEnvio>,
}

// ============================================================================
// T-25: adicionar_matricula
// ============================================================================

#[tauri::command]
pub fn adicionar_matricula(matricula: String) -> Result<MatriculaInfo, String> {
    Matricula::parse(&matricula)
        .map(MatriculaInfo::from)
        .map_err(|e| e.to_string())
}

// ============================================================================
// T-26: verificar_pasta_militar
// ============================================================================

#[tauri::command]
pub fn verificar_pasta_militar(
    raiz: String,
    matricula: String,
) -> Result<VerificacaoResult, String> {
    let m = Matricula::parse(&matricula).map_err(|e| e.to_string())?;
    let raiz_path = Path::new(&raiz);

    match buscar_pasta_militar_flat(raiz_path, &m) {
        Ok(Some(path)) => Ok(VerificacaoResult {
            existe: true,
            path: Some(path.to_string_lossy().to_string()),
        }),
        Ok(None) => Ok(VerificacaoResult {
            existe: false,
            path: None,
        }),
        Err(e) => Err(e.to_string()),
    }
}

// ============================================================================
// T-27: criar_pasta_militar
// ============================================================================

#[tauri::command]
pub fn criar_pasta_militar(
    raiz: String,
    matricula: String,
    nome_completo: String,
) -> Result<String, String> {
    let m = Matricula::parse(&matricula).map_err(|e| e.to_string())?;
    let raiz_path = Path::new(&raiz);

    core_criar_pasta_militar(raiz_path, &m, &nome_completo)
        .map(|p| p.to_string_lossy().to_string())
        .map_err(|e| e.to_string())
}

// ============================================================================
// T-28: enviar_documento
// ============================================================================

/// Verifica se uma string parece ser uma matrícula (apenas números e hífen opcional).
fn parece_matricula(destino: &str) -> bool {
    let sem_hifen = destino.replace('-', "");
    !sem_hifen.is_empty() && sem_hifen.chars().all(|c| c.is_ascii_digit())
}

/// Separa o nome do arquivo em stem e extensão.
fn split_nome_ext(nome: &str) -> (String, String) {
    if let Some(dot_pos) = nome.rfind('.') {
        if dot_pos > 0 {
            return (nome[..dot_pos].to_string(), nome[dot_pos + 1..].to_string());
        }
    }
    (nome.to_string(), String::new())
}

/// Copia dados em memória para um diretório destino, respeitando a política de sobrescrita.
fn copiar_dados(
    dados: &[u8],
    destino_dir: &Path,
    nome_arquivo: &str,
    politica: PoliticaSobrescrita,
) -> Result<(), crate::errors::AppError> {
    fs::create_dir_all(destino_dir)?;

    let mut destino_path = destino_dir.join(nome_arquivo);

    if destino_path.exists() {
        match politica {
            PoliticaSobrescrita::Pular => return Ok(()),
            PoliticaSobrescrita::RenomearComSufixo => {
                let (stem, ext) = split_nome_ext(nome_arquivo);
                let timestamp = std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_secs();
                let novo_nome = if ext.is_empty() {
                    format!("{}_{}", stem, timestamp)
                } else {
                    format!("{}_{}.{}", stem, timestamp, ext)
                };
                destino_path = destino_dir.join(novo_nome);
            }
            PoliticaSobrescrita::Sobrescrever => {}
        }
    }

    fs::write(&destino_path, dados)?;
    Ok(())
}

#[tauri::command]
pub async fn enviar_documento(
    arquivo: String,
    destinos: Vec<String>,
    raiz: String,
    window: tauri::Window,
    politica: String,
) -> Result<EnvioRelatorio, String> {
    // 1. Parse da política de sobrescrita
    let politica = PoliticaSobrescrita::from_str(&politica).map_err(|e| e.to_string())?;

    // 2. Lê o arquivo uma única vez
    let dados = fs::read(&arquivo).map_err(|e| e.to_string())?;

    let raiz_path = Path::new(&raiz);
    let nome_arquivo = Path::new(&arquivo)
        .file_name()
        .ok_or("Caminho do arquivo inválido")?
        .to_string_lossy()
        .to_string();

    let mut relatorio = EnvioRelatorio {
        total: destinos.len(),
        sucesso: 0,
        falha: 0,
        detalhes: Vec::new(),
    };

    for destino in &destinos {
        // 3a. Se parece matrícula → parse + buscar_pasta_militar_flat
        // 3b. Se path → usa direto
        let (pasta_destino, encontrou) = if parece_matricula(destino) {
            match Matricula::parse(destino) {
                Ok(m) => match buscar_pasta_militar_flat(raiz_path, &m) {
                    Ok(Some(path)) => (path, true),
                    Ok(None) => (PathBuf::new(), false),
                    Err(e) => {
                        let detalhe = DetalheEnvio {
                            destino: destino.clone(),
                            sucesso: false,
                            erro: Some(e.to_string()),
                        };
                        let _ = window.emit("progresso-envio", &detalhe);
                        relatorio.falha += 1;
                        relatorio.detalhes.push(detalhe);
                        continue;
                    }
                },
                Err(e) => {
                    let detalhe = DetalheEnvio {
                        destino: destino.clone(),
                        sucesso: false,
                        erro: Some(e.to_string()),
                    };
                    let _ = window.emit("progresso-envio", &detalhe);
                    relatorio.falha += 1;
                    relatorio.detalhes.push(detalhe);
                    continue;
                }
            }
        } else {
            let path = PathBuf::from(destino);
            if path.exists() && path.is_dir() {
                (path, true)
            } else {
                let detalhe = DetalheEnvio {
                    destino: destino.clone(),
                    sucesso: false,
                    erro: Some(format!("Pasta de destino não existe: {destino}")),
                };
                let _ = window.emit("progresso-envio", &detalhe);
                relatorio.falha += 1;
                relatorio.detalhes.push(detalhe);
                continue;
            }
        };

        // 3d. Se não encontrou → não é possível abrir dialog de input no Tauri v2 backend.
        //     O frontend deve tratar essa falha e solicitar o nome ao usuário.
        if !encontrou {
            let detalhe = DetalheEnvio {
                destino: destino.clone(),
                sucesso: false,
                erro: Some(format!(
                    "Militar não encontrado para matrícula: {destino}. Crie a pasta antes de enviar."
                )),
            };
            let _ = window.emit("progresso-envio", &detalhe);
            relatorio.falha += 1;
            relatorio.detalhes.push(detalhe);
            continue;
        }

        // 3c. Copia o arquivo com a política escolhida
        let resultado = copiar_dados(&dados, &pasta_destino, &nome_arquivo, politica);
        let detalhe = match resultado {
            Ok(()) => {
                relatorio.sucesso += 1;
                DetalheEnvio {
                    destino: destino.clone(),
                    sucesso: true,
                    erro: None,
                }
            }
            Err(e) => {
                relatorio.falha += 1;
                DetalheEnvio {
                    destino: destino.clone(),
                    sucesso: false,
                    erro: Some(e.to_string()),
                }
            }
        };

        // 3e. Emite evento de progresso
        let _ = window.emit("progresso-envio", &detalhe);
        relatorio.detalhes.push(detalhe);
    }

    // 4. Retorna o relatório final
    Ok(relatorio)
}
