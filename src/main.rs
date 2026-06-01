#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod commands;
mod core;
mod errors;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            commands::adicionar_matricula,
            commands::verificar_pasta_militar,
            commands::criar_pasta_militar,
            commands::enviar_documento,
            commands::buscar_matricula,
            commands::confirmar_sobrescrita_busca,
            commands::carregar_config,
            commands::salvar_config,
        ])
        .run(tauri::generate_context!())
        .expect("Erro ao iniciar o aplicativo");
}
