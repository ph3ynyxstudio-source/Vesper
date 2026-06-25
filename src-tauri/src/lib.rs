use tauri::{Emitter, Manager};
use tauri_plugin_opener::OpenerExt;

const PROJECTS_ROOT: &str = r"C:\Ph3yNyx.OS\05_⭐VESPΣR";

#[derive(Debug, serde::Serialize)]
struct ProjectDirectory {
    name: String,
    path: String,
    modified_at_epoch_seconds: Option<u64>,
    status: String,
}

#[derive(Debug, serde::Serialize)]
struct ProjectBranch {
    id: String,
    label: String,
    path: Option<String>,
    exists: bool,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
struct CompanionSettings {
    visible: bool,
    shadow: bool,
}

fn is_valid_project_status(status: &str) -> bool {
    matches!(status, "active" | "paused" | "concept" | "archived")
}

fn read_project_status(project_path: &std::path::Path) -> String {
    let metadata_path = project_path.join("vesper.json");
    let Ok(metadata) = std::fs::read_to_string(metadata_path) else {
        return "active".into();
    };
    let Ok(value) = serde_json::from_str::<serde_json::Value>(&metadata) else {
        return "active".into();
    };
    let Some(status) = value.get("status").and_then(serde_json::Value::as_str) else {
        return "active".into();
    };

    match status {
        "pause" => "paused".into(),
        valid_status if is_valid_project_status(valid_status) => valid_status.into(),
        _ => "active".into(),
    }
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

        let path = entry.path();

        projects.push(ProjectDirectory {
            name: entry.file_name().to_string_lossy().into_owned(),
            path: path.to_string_lossy().into_owned(),
            modified_at_epoch_seconds,
            status: read_project_status(&path),
        });
    }

    projects.sort_by(|left, right| left.name.to_lowercase().cmp(&right.name.to_lowercase()));
    Ok(projects)
}

#[tauri::command]
fn open_project_directory(app: tauri::AppHandle, path: &str) -> Result<(), &'static str> {
    let requested_path = canonical_project_path(path)?;

    app.opener()
        .open_path(requested_path.to_string_lossy().as_ref(), None::<&str>)
        .map_err(|_| "open_failed")
}

fn canonical_project_path(path: &str) -> Result<std::path::PathBuf, &'static str> {
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

    Ok(requested_path)
}

fn matching_branch_dir(
    branch_id: &str,
    fallback_label: &str,
    directory_names: &[String],
) -> Option<String> {
    let numbered_match = directory_names
        .iter()
        .map(String::as_str)
        .find(|directory_name| {
            let normalized_name = directory_name.to_lowercase();

            match branch_id {
                "assets" => {
                    normalized_name.starts_with("01_") && normalized_name.contains("asset")
                }
                "docs" => normalized_name.starts_with("02_") && normalized_name.contains("docs"),
                "features" => {
                    normalized_name.starts_with("05_") && normalized_name.contains("features")
                }
                "archives" => {
                    normalized_name.starts_with("99_") && normalized_name.contains("archive")
                }
                _ => false,
            }
        })
        .map(str::to_owned);

    numbered_match.or_else(|| {
        directory_names
            .iter()
            .map(String::as_str)
            .find(|directory_name| directory_name.eq_ignore_ascii_case(fallback_label))
            .map(str::to_owned)
    })
}

#[tauri::command]
fn list_project_genealogy(project_path: &str) -> Result<Vec<ProjectBranch>, &'static str> {
    let project_path = canonical_project_path(project_path)?;
    let branch_specs = [
        ("docs", "Docs"),
        ("assets", "Assets"),
        ("features", "Features"),
        ("archives", "Archives"),
    ];
    let directory_names = std::fs::read_dir(&project_path)
        .map_err(|_| "read_failed")?
        .filter_map(Result::ok)
        .filter_map(|entry| {
            let is_directory = entry.file_type().ok()?.is_dir();

            is_directory.then(|| entry.file_name().to_string_lossy().into_owned())
        })
        .collect::<Vec<_>>();

    Ok(branch_specs
        .iter()
        .map(|(id, label)| {
            let branch_path = matching_branch_dir(
                id,
                label,
                &directory_names,
            )
            .map(|directory_name| project_path.join(directory_name));
            let exists = branch_path.is_some();

            ProjectBranch {
                id: (*id).into(),
                label: (*label).into(),
                path: branch_path.map(|path| path.to_string_lossy().into_owned()),
                exists,
            }
        })
        .collect())
}

#[tauri::command]
fn update_project_status(project_path: &str, status: &str) -> Result<(), &'static str> {
    if !is_valid_project_status(status) {
        return Err("invalid_status");
    }

    let project_path = canonical_project_path(project_path)?;
    let metadata_path = project_path.join("vesper.json");
    let mut metadata = std::fs::read_to_string(&metadata_path)
        .ok()
        .and_then(|value| serde_json::from_str::<serde_json::Value>(&value).ok())
        .and_then(|value| value.as_object().cloned())
        .unwrap_or_default();

    metadata.insert("status".into(), serde_json::Value::String(status.into()));

    let metadata = serde_json::to_string_pretty(&serde_json::Value::Object(metadata))
        .map_err(|_| "serialize_failed")?;

    std::fs::write(metadata_path, metadata).map_err(|_| "write_failed")
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
            list_project_genealogy,
            update_project_status,
            open_project_directory,
            apply_companion_settings
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[cfg(test)]
mod tests {
    use super::matching_branch_dir;

    fn names(values: &[&str]) -> Vec<String> {
        values.iter().map(|value| (*value).into()).collect()
    }

    #[test]
    fn matches_numbered_project_branch_directories() {
        let directories = names(&[
            "01_🌙Lun△rMood_Assets",
            "02_🌙Lun△rMood_Docs",
            "05_🌙Lun△rMood_FEATURES",
            "99_🌙Lun△rMood_Archive",
        ]);

        assert_eq!(
            matching_branch_dir("assets", "Assets", &directories),
            Some("01_🌙Lun△rMood_Assets".into())
        );
        assert_eq!(
            matching_branch_dir("docs", "Docs", &directories),
            Some("02_🌙Lun△rMood_Docs".into())
        );
        assert_eq!(
            matching_branch_dir("features", "Features", &directories),
            Some("05_🌙Lun△rMood_FEATURES".into())
        );
        assert_eq!(
            matching_branch_dir("archives", "Archives", &directories),
            Some("99_🌙Lun△rMood_Archive".into())
        );
    }

    #[test]
    fn matches_singular_asset_directory() {
        let directories = names(&["01_⭐Astr4lForge_Asset"]);

        assert_eq!(
            matching_branch_dir("assets", "Assets", &directories),
            Some("01_⭐Astr4lForge_Asset".into())
        );
    }

    #[test]
    fn falls_back_to_legacy_exact_directory_names() {
        let directories = names(&["Assets", "Docs", "Features", "Archives"]);

        assert_eq!(
            matching_branch_dir("docs", "Docs", &directories),
            Some("Docs".into())
        );
        assert_eq!(
            matching_branch_dir("archives", "Archives", &directories),
            Some("Archives".into())
        );
    }

    #[test]
    fn returns_none_for_missing_branch_directory() {
        let directories = names(&["01_Project_Assets"]);

        assert_eq!(matching_branch_dir("features", "Features", &directories), None);
    }
}
