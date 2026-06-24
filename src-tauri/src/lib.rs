use tauri::{Emitter, Manager};
use tauri_plugin_opener::OpenerExt;

const PROJECTS_ROOT: &str = r"C:\Ph3yNyx.OS\05_⭐VESPΣR";

#[derive(Debug, serde::Serialize)]
struct ProjectDirectory {
    name: String,
    path: String,
    modified_at_epoch_seconds: Option<u64>,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
struct CompanionSettings {
    visible: bool,
    shadow: bool,
}

#[tauri::command]
fn list_project_directories() -> Result<Vec<ProjectDirectory>, String> {
    let root = std::path::Path::new(PROJECTS_ROOT);
    let entries = std::fs::read_dir(root).map_err(|error| error.to_string())?;
    let mut projects = Vec::new();

    for entry in entries {
        let entry = entry.map_err(|error| error.to_string())?;
        let file_type = entry.file_type().map_err(|error| error.to_string())?;

        if !file_type.is_dir() {
            continue;
        }

        let metadata = entry.metadata().ok();
        let modified_at_epoch_seconds = metadata
            .and_then(|value| value.modified().ok())
            .and_then(|value| {
                value
                    .duration_since(std::time::UNIX_EPOCH)
                    .ok()
                    .map(|duration| duration.as_secs())
            });

        projects.push(ProjectDirectory {
            name: entry.file_name().to_string_lossy().into_owned(),
            path: entry.path().to_string_lossy().into_owned(),
            modified_at_epoch_seconds,
        });
    }

    projects.sort_by(|left, right| left.name.to_lowercase().cmp(&right.name.to_lowercase()));
    Ok(projects)
}

#[tauri::command]
fn open_project_directory(app: tauri::AppHandle, path: &str) -> Result<(), &'static str> {
    let requested_path = std::path::Path::new(path);
    let root = std::path::Path::new(PROJECTS_ROOT);
    let requested_path = requested_path.canonicalize().map_err(|_| "invalid_path")?;
    let root = root.canonicalize().map_err(|_| "invalid_root")?;

    if !requested_path.starts_with(&root) {
        return Err("path_outside_projects_root");
    }

    if !requested_path.is_dir() {
        return Err("not_a_directory");
    }

    app.opener()
        .open_path(requested_path.to_string_lossy().as_ref(), None::<&str>)
        .map_err(|_| "open_failed")
}

#[tauri::command]
fn apply_companion_settings(
    app: tauri::AppHandle,
    settings: CompanionSettings,
) -> Result<(), &'static str> {
    let window = app
        .get_webview_window("vesperion")
        .ok_or("vesperion_window_missing")?;

    if settings.visible {
        window.show().map_err(|_| "show_failed")?;
    }

    app.emit_to("vesperion", "vesperion-settings", settings.clone())
        .map_err(|_| "emit_failed")?;

    if !settings.visible {
        window.hide().map_err(|_| "hide_failed")?;
    }

    Ok(())
}

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
async fn drag_vesperion(window: tauri::WebviewWindow) -> Result<(), String> {
    #[cfg(target_os = "windows")]
    {
        use std::{thread, time::Duration};
        use tauri::{PhysicalPosition, Position};
        use windows_sys::Win32::{
            Foundation::POINT,
            UI::{
                Input::KeyboardAndMouse::{GetAsyncKeyState, VK_LBUTTON},
                WindowsAndMessaging::GetCursorPos,
            },
        };

        let window_position = window.outer_position().map_err(|error| error.to_string())?;
        let mut initial_cursor = POINT { x: 0, y: 0 };

        if unsafe { GetCursorPos(&mut initial_cursor) } == 0 {
            return Err("Unable to read the desktop cursor position".into());
        }

        let cursor_offset_x = initial_cursor.x - window_position.x;
        let cursor_offset_y = initial_cursor.y - window_position.y;

        return tauri::async_runtime::spawn_blocking(move || {
            while unsafe { GetAsyncKeyState(VK_LBUTTON as i32) } < 0 {
                let mut cursor = POINT { x: 0, y: 0 };
                if unsafe { GetCursorPos(&mut cursor) } != 0 {
                    window
                        .set_position(Position::Physical(PhysicalPosition::new(
                            cursor.x - cursor_offset_x,
                            cursor.y - cursor_offset_y,
                        )))
                        .map_err(|error| error.to_string())?;
                }

                thread::sleep(Duration::from_millis(8));
            }

            Ok(())
        })
        .await
        .map_err(|error| error.to_string())?;
    }

    #[cfg(not(target_os = "windows"))]
    window.start_dragging().map_err(|error| error.to_string())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            greet,
            drag_vesperion,
            list_project_directories,
            open_project_directory,
            apply_companion_settings
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
