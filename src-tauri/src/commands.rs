use std::sync::Arc;

use tauri::{image::Image, AppHandle, State};
use toto_engine::{Engine, Script, ScriptId};

use crate::TRAY_ID;

pub type EngineState = Arc<Engine>;

const TRAY_IDLE_PNG: &[u8] = include_bytes!("../icons/tray-idle-32.png");
const TRAY_ACTIVE_PNG: &[u8] = include_bytes!("../icons/tray-active-32.png");

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
    engine.running_ids().into_iter().map(|id| id.0).collect()
}

#[tauri::command]
pub fn stop_all(engine: State<'_, EngineState>) {
    engine.stop_all();
}

#[tauri::command]
pub fn set_tray_active(app: AppHandle, active: bool) -> Result<(), String> {
    let tray = app
        .tray_by_id(TRAY_ID)
        .ok_or_else(|| "tray icon not found".to_string())?;
    let bytes = if active {
        TRAY_ACTIVE_PNG
    } else {
        TRAY_IDLE_PNG
    };
    let icon = Image::from_bytes(bytes).map_err(|e| e.to_string())?;
    tray.set_icon(Some(icon)).map_err(|e| e.to_string())
}
