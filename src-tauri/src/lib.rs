use std::sync::Arc;

use tauri::{
    image::Image,
    menu::{Menu, MenuItem, PredefinedMenuItem},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    Emitter, Manager, WindowEvent,
};
use toto_engine::{Engine, EnigoBackend};

mod commands;

pub const TRAY_ID: &str = "main";

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let engine: commands::EngineState =
        Arc::new(Engine::new(|| Ok(Box::new(EnigoBackend::new()?))));

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_store::Builder::default().build())
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .manage(engine)
        .setup(|app| {
            let show_item = MenuItem::with_id(app, "show", "Show", true, None::<&str>)?;
            let about_item = MenuItem::with_id(app, "about", "About", true, None::<&str>)?;
            let quit_item = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;
            let separator = PredefinedMenuItem::separator(app)?;
            let menu = Menu::with_items(
                app,
                &[&show_item, &about_item, &separator, &quit_item],
            )?;

            let idle_icon = Image::from_bytes(include_bytes!("../icons/tray-idle-32.png"))?;

            TrayIconBuilder::with_id(TRAY_ID)
                .icon(idle_icon)
                .menu(&menu)
                .show_menu_on_left_click(false)
                .on_menu_event(|app, event| match event.id().as_ref() {
                    "show" => show_main_window(app),
                    "about" => {
                        let _ = app.emit("show-about", ());
                    }
                    "quit" => app.exit(0),
                    _ => {}
                })
                .on_tray_icon_event(|tray, event| {
                    if let TrayIconEvent::Click {
                        button: MouseButton::Left,
                        button_state: MouseButtonState::Up,
                        ..
                    } = event
                    {
                        show_main_window(tray.app_handle());
                    }
                })
                .build(app)?;

            if let Some(main) = app.get_webview_window("main") {
                let hide_target = main.clone();
                main.on_window_event(move |event| {
                    if let WindowEvent::CloseRequested { api, .. } = event {
                        api.prevent_close();
                        let _ = hide_target.hide();
                    }
                });
            }

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::start_script,
            commands::stop_script,
            commands::toggle_script,
            commands::is_running,
            commands::running_scripts,
            commands::stop_all,
            commands::set_tray_active,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn show_main_window<R: tauri::Runtime>(app: &tauri::AppHandle<R>) {
    if let Some(window) = app.get_webview_window("main") {
        let _ = window.show();
        let _ = window.unminimize();
        let _ = window.set_focus();
    }
}
