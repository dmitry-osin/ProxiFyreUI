use crate::{run_exe, CREATE_NO_WINDOW};
use serde::Serialize;
use std::os::windows::process::CommandExt;
use std::process::Command;

#[derive(Debug, Serialize, Clone)]
pub struct ServiceStatus {
    pub installed: bool,
    pub running: bool,
    pub status: String,
}

#[tauri::command]
pub fn get_service_status() -> ServiceStatus {
    match Command::new("sc")
        .args(["query", "ProxiFyreService"])
        .output()
    {
        Ok(out) => parse_sc_output(&String::from_utf8_lossy(&out.stdout)),
        Err(e) => ServiceStatus {
            installed: false,
            running: false,
            status: format!("Error: {e}"),
        },
    }
}

#[tauri::command]
pub fn start_service() -> Result<(), String> {
    run_sc(&["start", "ProxiFyreService"])
}

#[tauri::command]
pub fn stop_service() -> Result<(), String> {
    run_sc(&["stop", "ProxiFyreService"])
}

#[tauri::command]
pub fn restart_service() -> Result<(), String> {
    let _ = run_sc(&["stop", "ProxiFyreService"]);
    std::thread::sleep(std::time::Duration::from_secs(2));
    run_sc(&["start", "ProxiFyreService"])
}

#[tauri::command]
pub fn install_service(proxifyre_path: String) -> Result<(), String> {
    run_exe(&proxifyre_path, "install")
}

#[tauri::command]
pub fn uninstall_service(proxifyre_path: String) -> Result<(), String> {
    run_exe(&proxifyre_path, "uninstall")
}

fn parse_sc_output(output: &str) -> ServiceStatus {
    if output.contains("FAILED 1060") || output.contains("does not exist") {
        return ServiceStatus {
            installed: false,
            running: false,
            status: "Not Installed".into(),
        };
    }
    if output.contains("RUNNING") {
        return ServiceStatus {
            installed: true,
            running: true,
            status: "Running".into(),
        };
    }
    if output.contains("START_PENDING") {
        return ServiceStatus {
            installed: true,
            running: false,
            status: "Starting...".into(),
        };
    }
    if output.contains("STOP_PENDING") {
        return ServiceStatus {
            installed: true,
            running: true,
            status: "Stopping...".into(),
        };
    }
    if output.contains("STOPPED") {
        return ServiceStatus {
            installed: true,
            running: false,
            status: "Stopped".into(),
        };
    }
    ServiceStatus {
        installed: true,
        running: false,
        status: "Unknown".into(),
    }
}

fn run_sc(args: &[&str]) -> Result<(), String> {
    let out = Command::new("sc")
        .args(args)
        .creation_flags(CREATE_NO_WINDOW)
        .output()
        .map_err(|e| e.to_string())?;
    let stdout = String::from_utf8_lossy(&out.stdout);
    if out.status.success() || stdout.contains("SUCCESS") {
        Ok(())
    } else {
        Err(stdout.trim().to_string())
    }
}
