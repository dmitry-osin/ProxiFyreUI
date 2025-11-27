use crate::configuration::AppSettings;
use serde::Serialize;
use std::fs;
use tauri::menu::{Menu, MenuItem};
use tauri::tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent};
use tauri::{AppHandle, Emitter, Manager};

#[derive(Clone, Serialize)]
struct TrayAction {
    action: String,
}

fn tray_service_action(action: &str) -> TrayAction {
    TrayAction {
        action: action.to_string(),
    }
}

fn notify_frontend(app: &AppHandle, payload: TrayAction) {
    let _ = app.emit("tray-action", payload);
}

fn show_main_window(app: &AppHandle) {
    if let Some(win) = app.get_webview_window("main") {
        let _ = win.show();
        let _ = win.set_focus();
    }
}

pub fn handle_tray_menu(app: &AppHandle, id: &str) {
    match id {
        "show" => show_main_window(app),

        "install" => {
            notify_frontend(app, tray_service_action("install"));
        }
        "uninstall" => {
            notify_frontend(app, tray_service_action("uninstall"));
        }
        "restart" => {
            notify_frontend(app, tray_service_action("restart"));
        }

        "quit" => app.exit(0),
        _ => {}
    }
}

pub fn setup_tray(app: &mut tauri::App) -> Result<(), Box<dyn std::error::Error>> {
    let show_item = MenuItem::with_id(app, "show", "Show", true, None::<&str>)?;
    let install_item = MenuItem::with_id(app, "install", "Install Service", true, None::<&str>)?;
    let uninstall_item =
        MenuItem::with_id(app, "uninstall", "Uninstall Service", true, None::<&str>)?;
    let restart_item = MenuItem::with_id(app, "restart", "Restart Service", true, None::<&str>)?;
    let quit_item = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;

    let menu = Menu::with_items(
        app,
        &[
            &show_item,
            &install_item,
            &uninstall_item,
            &restart_item,
            &quit_item,
        ],
    )?;

    TrayIconBuilder::new()
        .icon(app.default_window_icon().unwrap().clone())
        .tooltip("ProxiFyre UI")
        .menu(&menu)
        .show_menu_on_left_click(false)
        .on_menu_event(|app, event| handle_tray_menu(app, event.id.as_ref()))
        .on_tray_icon_event(|tray, event| {
            if let TrayIconEvent::Click {
                button: MouseButton::Left,
                button_state: MouseButtonState::Up,
                ..
            } = event
            {
                let app = tray.app_handle();
                show_main_window(app);
            }
        })
        .build(app)?;

    let settings_path = app
        .path()
        .app_config_dir()
        .ok()
        .map(|d| d.join("settings.json"));
    if let Some(path) = settings_path {
        if let Ok(content) = fs::read_to_string(&path) {
            if let Ok(settings) = serde_json::from_str::<AppSettings>(&content) {
                if settings.start_minimized {
                    if let Some(win) = app.get_webview_window("main") {
                        let _ = win.hide();
                    }
                }
            }
        }
    }

    Ok(())
}
