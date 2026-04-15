use std::sync::Arc;

use tauri::State;
use toto_engine::{Engine, Script, ScriptId};

pub type EngineState = Arc<Engine>;

#[tauri::command]
pub fn start_script(engine: State<'_, EngineState>, script: Script) -> Result<(), String> {
    engine.start(script).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn stop_script(engine: State<'_, EngineState>, id: String) -> Result<(), String> {
    engine.stop(&ScriptId::new(id)).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn toggle_script(engine: State<'_, EngineState>, script: Script) -> Result<bool, String> {
    engine.toggle(script).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn is_running(engine: State<'_, EngineState>, id: String) -> bool {
    engine.is_running(&ScriptId::new(id))
}

#[tauri::command]
pub fn running_scripts(engine: State<'_, EngineState>) -> Vec<String> {
    engine
        .running_ids()
        .into_iter()
        .map(|id| id.0)
        .collect()
}

#[tauri::command]
pub fn stop_all(engine: State<'_, EngineState>) {
    engine.stop_all();
}
