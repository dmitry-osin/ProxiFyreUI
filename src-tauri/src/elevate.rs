use crate::CREATE_NO_WINDOW;
use std::os::windows::process::CommandExt;
use std::process::Command;
use tauri::AppHandle;

#[tauri::command]
pub fn check_admin() -> bool {
    Command::new("net")
        .args(["session"])
        .creation_flags(CREATE_NO_WINDOW)
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .status()
        .map(|s| s.success())
        .unwrap_or(false)
}

#[tauri::command]
pub fn relaunch_as_admin(app: AppHandle) -> Result<(), String> {
    let exe = std::env::current_exe().map_err(|e| e.to_string())?;
    let exe_str = exe.to_string_lossy().to_string();
    Command::new("powershell")
        .args([
            "-WindowStyle",
            "Hidden",
            "-Command",
            &format!("Start-Process '{}' -Verb RunAs", exe_str),
        ])
        .spawn()
        .map_err(|e| e.to_string())?;
    app.exit(0);
    Ok(())
}
