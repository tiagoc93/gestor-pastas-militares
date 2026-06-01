#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod commands;
mod core;
mod errors;

use commands::{
    adicionar_matricula, buscar_matricula, carregar_config, confirmar_sobrescrita_busca,
    criar_pasta_militar, enviar_documento, salvar_config, selecionar_arquivo,
    selecionar_pasta_raiz, verificar_pasta_militar,
};

fn main() {
    env_logger::init();
    errors::__garantir_variants_completeness();

    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .invoke_handler(tauri::generate_handler![
            adicionar_matricula,
            verificar_pasta_militar,
            criar_pasta_militar,
            enviar_documento,
            buscar_matricula,
            confirmar_sobrescrita_busca,
            carregar_config,
            salvar_config,
            selecionar_pasta_raiz,
            selecionar_arquivo,
        ])
        .run(tauri::generate_context!())
        .expect("Erro ao iniciar o aplicativo");
}
