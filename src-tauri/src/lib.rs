// ProxiFyre UI — Tauri backend
// Manages ProxiFyre SOCKS5 proxifier configuration and Windows service lifecycle.
// Author: Dmitry Osin <d@osin.pro>

mod configuration;
mod elevate;
mod service;
mod tray;

#[cfg(not(target_os = "windows"))]
compile_error!("ProxiFyre UI only supports Windows.");

use crate::configuration::{
    load_app_settings, load_proxifyre_config, save_app_settings, save_proxifyre_config,
};
use crate::elevate::{check_admin, relaunch_as_admin};
use crate::service::{
    get_service_status, install_service, restart_service, start_service, stop_service,
    uninstall_service,
};
use crate::tray::setup_tray;
use std::os::windows::process::CommandExt;
use std::process::Command;
use tauri::Manager;

const CREATE_NO_WINDOW: u32 = 0x08000000;

fn run_exe(path: &str, arg: &str) -> Result<(), String> {
    let out = Command::new(path)
        .arg(arg)
        .creation_flags(CREATE_NO_WINDOW)
        .output()
        .map_err(|e| e.to_string())?;
    if out.status.success() {
        Ok(())
    } else {
        Err(String::from_utf8_lossy(&out.stderr).trim().to_string())
    }
}

#[tauri::command]
fn list_processes() -> Vec<String> {
    let out = match Command::new("powershell")
        .args([
            "-NoProfile",
            "-NonInteractive",
            "-Command",
            "[Console]::OutputEncoding = [System.Text.Encoding]::UTF8; \
             Get-Process | Select-Object -ExpandProperty Name | \
             Sort-Object -Unique | ForEach-Object { $_ + '.exe' }",
        ])
        .creation_flags(CREATE_NO_WINDOW)
        .output()
    {
        Ok(o) => o,
        Err(_) => return Vec::new(),
    };

    let stdout = String::from_utf8_lossy(&out.stdout);
    let mut seen = std::collections::HashSet::new();
    let mut names: Vec<String> = Vec::new();

    for line in stdout.lines() {
        let name = line.trim();
        if !name.is_empty() && seen.insert(name.to_lowercase()) {
            names.push(name.to_string());
        }
    }

    names.sort_by(|a, b| a.to_lowercase().cmp(&b.to_lowercase()));
    names
}

fn apply_autostart(enabled: bool) {
    let key = r"HKCU\Software\Microsoft\Windows\CurrentVersion\Run";
    if enabled {
        if let Ok(exe) = std::env::current_exe() {
            let _ = Command::new("reg")
                .args([
                    "add",
                    key,
                    "/v",
                    "ProxiFyreUI",
                    "/t",
                    "REG_SZ",
                    "/d",
                    &exe.to_string_lossy().to_string(),
                    "/f",
                ])
                .creation_flags(CREATE_NO_WINDOW)
                .stdout(std::process::Stdio::null())
                .stderr(std::process::Stdio::null())
                .status();
        }
    } else {
        let _ = Command::new("reg")
            .args(["delete", key, "/v", "ProxiFyreUI", "/f"])
            .creation_flags(CREATE_NO_WINDOW)
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null())
            .status();
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_single_instance::init(|app, _argv, _cwd| {
            if let Some(win) = app.get_webview_window("main") {
                let _ = win.show();
                let _ = win.set_focus();
            }
        }))
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![
            load_app_settings,
            save_app_settings,
            load_proxifyre_config,
            save_proxifyre_config,
            get_service_status,
            start_service,
            stop_service,
            restart_service,
            install_service,
            uninstall_service,
            check_admin,
            relaunch_as_admin,
            list_processes,
        ])
        .setup(setup_tray)
        .on_window_event(|window, event| {
            if let tauri::WindowEvent::CloseRequested { api, .. } = event {
                api.prevent_close();
                window.hide().unwrap_or_default();
            }
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
