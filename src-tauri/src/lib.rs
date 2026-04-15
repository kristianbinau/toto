use std::sync::Arc;

use toto_engine::{Engine, EnigoBackend};

mod commands;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let engine: commands::EngineState = Arc::new(Engine::new(|| {
        Ok(Box::new(EnigoBackend::new()?))
    }));

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_store::Builder::default().build())
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .manage(engine)
        .invoke_handler(tauri::generate_handler![
            commands::start_script,
            commands::stop_script,
            commands::toggle_script,
            commands::is_running,
            commands::running_scripts,
            commands::stop_all,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
