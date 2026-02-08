#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod libreoffice;

use libreoffice::LibreOfficeManager;
use std::path::PathBuf;
use tauri::{api::dialog, State};
use tokio::sync::Mutex;

struct AppState {
    lo_manager: Mutex<LibreOfficeManager>,
}

#[derive(serde::Serialize, Clone)]
struct ConversionProgress {
    file_path: String,
    progress: u32,
    status: String,
    error: Option<String>,
}

#[tauri::command]
async fn select_files() -> Result<Vec<String>, String> {
    let paths = dialog::blocking::FileDialogBuilder::new()
        .add_filter("Word Documents", &["docx"])
        .set_title("Select DOCX Files")
        .pick_files();

    match paths {
        Some(paths) => Ok(paths.iter().map(|p| p.to_string_lossy().to_string()).collect()),
        None => Ok(vec![]),
    }
}

#[tauri::command]
async fn select_output_directory() -> Result<String, String> {
    let path = dialog::blocking::FileDialogBuilder::new()
        .set_title("Select Output Directory")
        .pick_folder();

    match path {
        Some(path) => Ok(path.to_string_lossy().to_string()),
        None => Err("No directory selected".to_string()),
    }
}

#[tauri::command]
async fn get_dropped_file_path(name: String, _size: u64) -> Result<String, String> {
    // For drag and drop, we'll use the file picker as a fallback
    // since getting the exact path from drag events requires additional handling
    let paths = dialog::blocking::FileDialogBuilder::new()
        .add_filter("Word Documents", &["docx"])
        .set_title("Select DOCX Files")
        .pick_files();

    match paths {
        Some(paths) => {
            // Filter for the file with matching name
            if let Some(path) = paths.iter().find(|p| {
                p.file_name()
                    .map(|f| f.to_string_lossy() == name)
                    .unwrap_or(false)
            }) {
                Ok(path.to_string_lossy().to_string())
            } else if let Some(first) = paths.first() {
                Ok(first.to_string_lossy().to_string())
            } else {
                Err("No file selected".to_string())
            }
        }
        None => Err("No file selected".to_string()),
    }
}

#[tauri::command]
async fn convert_batch(
    files: Vec<String>,
    output_dir: Option<String>,
    state: State<'_, AppState>,
    window: tauri::Window,
) -> Result<(), String> {
    let lo_manager = state.lo_manager.lock().await;
    let total_files = files.len();

    for (index, file_path) in files.iter().enumerate() {
        let progress = ((index as f32 / total_files as f32) * 100.0) as u32;
        
        // Emit progress update
        let _ = window.emit(
            "conversion-progress",
            ConversionProgress {
                file_path: file_path.clone(),
                progress,
                status: "converting".to_string(),
                error: None,
            },
        );

        // Perform conversion
        let output_path = if let Some(ref dir) = output_dir {
            let path_buf = PathBuf::from(file_path);
            let file_name = path_buf
                .file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or("output");
            format!("{}/{}.pdf", dir, file_name)
        } else {
            let path = PathBuf::from(file_path);
            let parent = path.parent().and_then(|p| p.to_str()).unwrap_or(".");
            let file_name = path
                .file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or("output");
            format!("{}/{}.pdf", parent, file_name)
        };

        match lo_manager.convert_file(file_path, &output_path).await {
            Ok(_) => {
                let _ = window.emit(
                    "conversion-progress",
                    ConversionProgress {
                        file_path: file_path.clone(),
                        progress: 100,
                        status: "completed".to_string(),
                        error: None,
                    },
                );
            }
            Err(e) => {
                let _ = window.emit(
                    "conversion-progress",
                    ConversionProgress {
                        file_path: file_path.clone(),
                        progress: 0,
                        status: "error".to_string(),
                        error: Some(e.to_string()),
                    },
                );
            }
        }
    }

    Ok(())
}

#[tauri::command]
async fn check_for_updates() -> Result<Option<String>, String> {
    // Check for updates from GitHub releases
    // This is a placeholder - replace with your actual update checking logic
    match reqwest::get("https://api.github.com/repos/YOUR_USERNAME/docx2pdf-converter/releases/latest").await {
        Ok(response) => {
            if let Ok(json) = response.json::<serde_json::Value>().await {
                if let Some(version) = json.get("tag_name").and_then(|v| v.as_str()) {
                    // Compare with current version
                    let current_version = env!("CARGO_PKG_VERSION");
                    if version != format!("v{}", current_version) {
                        return Ok(Some(version.to_string()));
                    }
                }
            }
            Ok(None)
        }
        Err(_) => {
            // Silently fail if no network
            Ok(None)
        }
    }
}

fn main() {
    tauri::Builder::default()
        .manage(AppState {
            lo_manager: Mutex::new(LibreOfficeManager::new()),
        })
        .invoke_handler(tauri::generate_handler![
            select_files,
            select_output_directory,
            get_dropped_file_path,
            convert_batch,
            check_for_updates,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
