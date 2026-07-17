use tauri::{
    menu::{Menu, MenuItem},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    Emitter, Manager, WindowEvent,
};
use tauri_plugin_opener::OpenerExt;

const PROJECTS_ROOT: &str = r"C:\Ph3yNyx.OS\05_⭐VESPΣR";
const VSCODE_LUNAR_PATH: &str = r"C:\Ph3yNyx.OS\Devs\Lun4rMood";
const VSCODE_CHRONOS_PATH: &str = r"C:\Ph3yNyx.OS\Devs\Chr0nosV3rs";
const VSCODE_ASTRAL_PATH: &str = r"C:\Ph3yNyx.OS\Devs\Astr4lDesign";
const VSCODE_VESPER_PATH: &str = r"C:\Ph3yNyx.OS\Devs\Vesper";
const VSCODE_PORTFOLIO_PATH: &str = r"C:\Ph3yNyx.OS\Devs\Ph3yNyx.Portfolio";
const SESSION_VESPER_RAW_PATH: &str =
    r"C:\Users\pheyr\AppData\Roaming\com.ph3yn.chronosvers\projects\VespΣr\raw";
const SESSION_LUNAR_RAW_PATH: &str =
    r"C:\Users\pheyr\AppData\Roaming\com.ph3yn.chronosvers\projects\LunarMood\raw";
const SESSION_CHRONOS_RAW_PATH: &str =
    r"C:\Users\pheyr\AppData\Roaming\com.ph3yn.chronosvers\projects\ChronoVers\raw";
const SESSION_ASTRAL_RAW_PATH: &str =
    r"C:\Users\pheyr\AppData\Roaming\com.ph3yn.chronosvers\projects\Astr4lForge\raw";
const SESSION_PORTFOLIO_RAW_PATH: &str =
    r"C:\Users\pheyr\AppData\Roaming\com.ph3yn.chronosvers\projects\Ph3yNyx\raw";
const CHRONOS_PROJECTS_ROOT: &str =
    r"C:\Users\pheyr\AppData\Roaming\com.ph3yn.chronosvers\projects";
const GLOBAL_OBSIDIAN_SHORTCUT_PATH: &str =
    r"C:\Users\pheyr\AppData\Roaming\Microsoft\Windows\Start Menu\Programs\Obsidian.lnk";
const GLOBAL_VSCODE_SHORTCUT_PATH: &str =
    r"C:\Users\pheyr\AppData\Roaming\Microsoft\Windows\Start Menu\Programs\Visual Studio Code.lnk";
const CHRONOS_SHORTCUT_PATH: &str =
    r"C:\Users\pheyr\AppData\Roaming\Microsoft\Windows\Start Menu\Programs\chronosvers.lnk";
const PLUM3_EXE_PATH: &str = r"C:\Program Files\Plum3\plum3-de-nyx.exe";
const GLOBAL_EXPLORER_PATH: &str = r"C:\Ph3yNyx.OS";
const GLOBAL_GITHUB_URL: &str = "https://github.com/ph3ynyxstudio-source";
const GLOBAL_TERMINAL_PATH: &str = r"C:\Ph3yNyx.OS";
const WINDOWS_TERMINAL_EXE: &str = "wt.exe";
const WINDOWS_POWERSHELL_EXE: &str = r"C:\Windows\System32\WindowsPowerShell\v1.0\powershell.exe";
const MAIN_WINDOW_LABEL: &str = "main";
const OPEN_VESPER_MENU_ID: &str = "open_vesper";
const TOGGLE_COMPANION_MENU_ID: &str = "toggle_companion";
const REFRESH_VESPER_MENU_ID: &str = "refresh_vesper";
const QUIT_VESPER_MENU_ID: &str = "quit_vesper";
const TRAY_TOGGLE_COMPANION_EVENT: &str = "tray-toggle-companion";
const TRAY_REFRESH_APP_EVENT: &str = "tray-refresh-app";

#[derive(Debug, serde::Serialize)]
struct ProjectDirectory {
    name: String,
    path: String,
    icon: Option<String>,
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

#[derive(Debug, serde::Serialize)]
struct SessionSnapshot {
    content: String,
    display_date: String,
}

fn is_valid_project_status(status: &str) -> bool {
    matches!(status, "active" | "paused" | "concept" | "archived")
}

fn is_valid_project_icon_id(icon_id: &str) -> bool {
    !icon_id.is_empty()
        && icon_id.len() <= 64
        && icon_id
            .bytes()
            .all(|value| value.is_ascii_lowercase() || value.is_ascii_digit() || value == b'-')
}

fn project_structure_directory_names(project_name: &str) -> [String; 4] {
    [
        format!("01_⏳ {project_name}_Assets"),
        format!("02_⏳ {project_name}_Docs"),
        format!("05_⏳ {project_name}_FEATURES"),
        format!("99_⏳ {project_name}_Archive"),
    ]
}

fn vscode_project_path(project_name: &str) -> Option<&'static str> {
    let normalized_name = project_name.to_lowercase();

    if normalized_name.contains("lun") {
        Some(VSCODE_LUNAR_PATH)
    } else if normalized_name.contains("hr0nos") || normalized_name.contains("chr0nos") {
        Some(VSCODE_CHRONOS_PATH)
    } else if normalized_name.contains("astr4l") {
        Some(VSCODE_ASTRAL_PATH)
    } else if normalized_name.contains("vesp") {
        Some(VSCODE_VESPER_PATH)
    } else if normalized_name.contains("portfolio") || normalized_name.contains("ph3ynyx") {
        Some(VSCODE_PORTFOLIO_PATH)
    } else {
        None
    }
}

fn show_main_window(app: &tauri::AppHandle) -> Result<(), tauri::Error> {
    if let Some(window) = app.get_webview_window(MAIN_WINDOW_LABEL) {
        window.show()?;
        if window.is_minimized()? {
            window.unminimize()?;
        }
        window.set_focus()?;
    }

    Ok(())
}

fn chronosvers_raw_path(project_name: &str) -> Option<&'static str> {
    let normalized_name = project_name.to_lowercase();

    if normalized_name.contains("lun") {
        Some(SESSION_LUNAR_RAW_PATH)
    } else if normalized_name.contains("hr0nos") || normalized_name.contains("chr0nos") {
        Some(SESSION_CHRONOS_RAW_PATH)
    } else if normalized_name.contains("astr4l") {
        Some(SESSION_ASTRAL_RAW_PATH)
    } else if normalized_name.contains("vesp") {
        Some(SESSION_VESPER_RAW_PATH)
    } else if normalized_name.contains("portfolio") || normalized_name.contains("ph3ynyx") {
        Some(SESSION_PORTFOLIO_RAW_PATH)
    } else {
        None
    }
}

fn canonical_chronos_project_path(path: &str) -> Result<std::path::PathBuf, &'static str> {
    let requested_path = std::path::Path::new(path)
        .canonicalize()
        .map_err(|_| "invalid_session_source")?;
    let root = std::path::Path::new(CHRONOS_PROJECTS_ROOT)
        .canonicalize()
        .map_err(|_| "invalid_session_root")?;

    if requested_path.parent() != Some(root.as_path()) || !requested_path.is_dir() {
        return Err("session_source_outside_root");
    }

    Ok(requested_path)
}

fn github_shortcut_path(project_path: &std::path::Path) -> Result<std::path::PathBuf, &'static str> {
    let entries = std::fs::read_dir(project_path).map_err(|_| "read_failed")?;

    for entry in entries {
        let entry = entry.map_err(|_| "read_failed")?;
        let path = entry.path();
        let is_url = path
            .extension()
            .and_then(|extension| extension.to_str())
            .map(|extension| extension.eq_ignore_ascii_case("url"))
            .unwrap_or(false);
        let contains_git = path
            .file_name()
            .and_then(|file_name| file_name.to_str())
            .map(|file_name| file_name.to_lowercase().contains("git"))
            .unwrap_or(false);

        if path.is_file() && is_url && contains_git {
            return Ok(path);
        }
    }

    Err("git_shortcut_missing")
}

fn official_context_path(
    project_path: &std::path::Path,
) -> Result<std::path::PathBuf, &'static str> {
    if let Some(mapped_path) = mapped_official_context_path(project_path) {
        if mapped_path.is_file() {
            return Ok(mapped_path);
        }
    }

    let entries = std::fs::read_dir(project_path).map_err(|_| "read_failed")?;

    for entry in entries {
        let entry = entry.map_err(|_| "read_failed")?;
        let path = entry.path();
        let is_markdown = path
            .extension()
            .and_then(|extension| extension.to_str())
            .map(|extension| extension.eq_ignore_ascii_case("md"))
            .unwrap_or(false);
        let is_context_file = path
            .file_name()
            .and_then(|file_name| file_name.to_str())
            .map(|file_name| {
                let normalized_name = file_name.to_lowercase();
                normalized_name.contains("contexte officiel")
                    || normalized_name.contains("context officiel")
                    || normalized_name.contains("officiel")
            })
            .unwrap_or(false);

        if path.is_file() && is_markdown && is_context_file {
            return Ok(path);
        }
    }

    Err("official_context_missing")
}

fn mapped_official_context_path(project_path: &std::path::Path) -> Option<std::path::PathBuf> {
    let project_name = project_path.file_name()?.to_string_lossy().to_lowercase();
    let file_name = if project_name.contains("lun") {
        Some("🌙Lun△rMood — CONTEXTE OFFICIEL.md")
    } else if project_name.contains("hr0nos") || project_name.contains("chr0nos") {
        Some("⏳↻hr0nosV3rs — CONTEXTE OFFICIEL.md")
    } else if project_name.contains("astr4l") {
        Some("Astr4l Ecosystem - CONTEXTE OFFICIEL.md")
    } else if project_name.contains("vesp") {
        Some("⭐VESPΣR — CONTEXTE OFFICIEL.md")
    } else if project_name.contains("portfolio") || project_name.contains("ph3ynyx") {
        Some("🪟Ph3yNyx.Portfolio — CONTEXTE OFFICIEL.md")
    } else {
        None
    }?;

    Some(project_path.join(file_name))
}

fn github_url_from_shortcut(shortcut_path: &std::path::Path) -> Result<String, &'static str> {
    let file_contents = std::fs::read_to_string(shortcut_path).map_err(|_| "read_failed")?;

    file_contents
        .lines()
        .find_map(|line| line.strip_prefix("URL=").map(str::trim))
        .filter(|url| !url.is_empty())
        .map(str::to_owned)
        .ok_or("git_url_missing")
}

fn latest_file_in_directory(directory_path: &std::path::Path) -> Result<std::path::PathBuf, &'static str> {
    let entries = std::fs::read_dir(directory_path).map_err(|_| "read_failed")?;
    let mut latest: Option<(std::time::SystemTime, std::path::PathBuf)> = None;

    for entry in entries {
        let entry = entry.map_err(|_| "read_failed")?;
        let path = entry.path();
        if !path.is_file() {
            continue;
        }

        let modified_at = entry
            .metadata()
            .map_err(|_| "read_failed")?
            .modified()
            .map_err(|_| "read_failed")?;

        match &latest {
            Some((current_modified_at, _)) if modified_at <= *current_modified_at => {}
            _ => latest = Some((modified_at, path)),
        }
    }

    latest
        .map(|(_, path)| path)
        .ok_or("latest_session_missing")
}

fn session_date_from_file_name(file_path: &std::path::Path) -> Option<String> {
    let file_name = file_path.file_stem()?.to_string_lossy();
    let chars = file_name.chars().collect::<Vec<_>>();

    for start in 0..chars.len() {
        let year = chars.get(start..start + 4)?;
        if !year.iter().all(char::is_ascii_digit) {
            continue;
        }

        let first_separator = *chars.get(start + 4)?;
        if !matches!(first_separator, '-' | '_' | '.' | ' ') {
            continue;
        }

        let month = chars.get(start + 5..start + 7)?;
        if !month.iter().all(char::is_ascii_digit) {
            continue;
        }

        let second_separator = *chars.get(start + 7)?;
        if second_separator != first_separator && !matches!(second_separator, '-' | '_' | '.' | ' ') {
            continue;
        }

        let day = chars.get(start + 8..start + 10)?;
        if !day.iter().all(char::is_ascii_digit) {
            continue;
        }

        let year = year.iter().collect::<String>();
        let month = month.iter().collect::<String>();
        let day = day.iter().collect::<String>();

        return Some(format!("{year}-{month}-{day}"));
    }

    None
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

fn read_project_icon(project_path: &std::path::Path) -> Option<String> {
    let metadata = std::fs::read_to_string(project_path.join("vesper.json")).ok()?;
    let value = serde_json::from_str::<serde_json::Value>(&metadata).ok()?;

    value
        .get("icon")
        .and_then(serde_json::Value::as_str)
        .map(str::to_owned)
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
            icon: read_project_icon(&path),
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

#[tauri::command]
fn open_project_vscode(app: tauri::AppHandle, project_name: &str) -> Result<(), &'static str> {
    let project_path = vscode_project_path(project_name).ok_or("project_not_mapped")?;
    let path = std::path::Path::new(project_path);

    if !path.is_dir() {
        return Err("vscode_project_missing");
    }

    app.opener()
        .open_path(path.to_string_lossy().into_owned(), Some("code"))
        .map_err(|_| "open_failed")
}

#[tauri::command]
fn open_project_github(app: tauri::AppHandle, project_path: &str) -> Result<(), &'static str> {
    let project_path = canonical_project_path(project_path)?;
    let shortcut_path = github_shortcut_path(&project_path)?;
    let github_url = github_url_from_shortcut(&shortcut_path)?;

    app.opener()
        .open_url(github_url, None::<&str>)
        .map_err(|_| "open_failed")
}

#[tauri::command]
fn read_project_official_context(project_path: &str) -> Result<String, &'static str> {
    let project_path = canonical_project_path(project_path)?;
    let context_path = official_context_path(&project_path)?;

    std::fs::read_to_string(context_path).map_err(|_| "read_failed")
}

#[tauri::command]
fn read_latest_project_session_markdown(project_name: &str) -> Result<String, &'static str> {
    Ok(read_latest_project_session_snapshot(project_name)?.content)
}

#[tauri::command]
fn read_latest_project_session_snapshot(project_name: &str) -> Result<SessionSnapshot, &'static str> {
    let raw_path = chronosvers_raw_path(project_name).ok_or("project_not_mapped")?;
    let raw_path = std::path::Path::new(raw_path);

    if !raw_path.is_dir() {
        return Err("raw_directory_missing");
    }

    let latest_file_path = latest_file_in_directory(raw_path)?;
    let content = std::fs::read_to_string(&latest_file_path).map_err(|_| "read_failed")?;
    let display_date =
        session_date_from_file_name(&latest_file_path).unwrap_or_else(|| "Non disponible".into());

    Ok(SessionSnapshot {
        content,
        display_date,
    })
}

#[tauri::command]
fn validate_chronos_project_directory(path: &str) -> Result<String, &'static str> {
    canonical_chronos_project_path(path)
        .map(|path| path.to_string_lossy().into_owned())
}

#[tauri::command]
fn read_latest_session_snapshot_from_directory(
    project_directory: &str,
) -> Result<SessionSnapshot, &'static str> {
    let project_directory = canonical_chronos_project_path(project_directory)?;
    let raw_path = project_directory.join("raw");

    if !raw_path.is_dir() {
        return Err("raw_directory_missing");
    }

    let latest_file_path = latest_file_in_directory(&raw_path)?;
    let content = std::fs::read_to_string(&latest_file_path).map_err(|_| "read_failed")?;
    let display_date =
        session_date_from_file_name(&latest_file_path).unwrap_or_else(|| "Non disponible".into());

    Ok(SessionSnapshot {
        content,
        display_date,
    })
}

#[tauri::command]
fn open_global_access(app: tauri::AppHandle, access_label: &str) -> Result<(), &'static str> {
    match access_label {
        "Obsidian" => app
            .opener()
            .open_path(GLOBAL_OBSIDIAN_SHORTCUT_PATH, None::<&str>)
            .map_err(|_| "open_failed"),
        "GitHub" => app
            .opener()
            .open_url(GLOBAL_GITHUB_URL, None::<&str>)
            .map_err(|_| "open_failed"),
        "Explorateur" => app
            .opener()
            .open_path(GLOBAL_EXPLORER_PATH, None::<&str>)
            .map_err(|_| "open_failed"),
        "VS Code" => app
            .opener()
            .open_path(GLOBAL_VSCODE_SHORTCUT_PATH, None::<&str>)
            .map_err(|_| "open_failed"),
        "Terminal" => std::process::Command::new(WINDOWS_TERMINAL_EXE)
            .args(["-d", GLOBAL_TERMINAL_PATH, WINDOWS_POWERSHELL_EXE, "-NoExit"])
            .spawn()
            .map(|_| ())
            .map_err(|_| "open_failed"),
        _ => Err("unknown_access"),
    }
}

#[tauri::command]
fn open_chronos_app(app: tauri::AppHandle) -> Result<(), &'static str> {
    let shortcut_path = std::path::Path::new(CHRONOS_SHORTCUT_PATH);
    if !shortcut_path.is_file() {
        return Err("chronos_shortcut_missing");
    }

    app.opener()
        .open_path(CHRONOS_SHORTCUT_PATH, None::<&str>)
        .map_err(|_| "open_failed")
}

#[tauri::command]
fn open_plum3_app(app: tauri::AppHandle) -> Result<(), &'static str> {
    let executable_path = std::path::Path::new(PLUM3_EXE_PATH);
    if !executable_path.is_file() {
        return Err("plum3_executable_missing");
    }

    app.opener()
        .open_path(PLUM3_EXE_PATH, None::<&str>)
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
fn create_project_structure(project_path: &str) -> Result<Vec<String>, String> {
    let project_path = canonical_project_path(project_path).map_err(str::to_owned)?;
    let project_name = project_path
        .file_name()
        .and_then(|name| name.to_str())
        .filter(|name| !name.trim().is_empty())
        .ok_or_else(|| "invalid_project_name".to_owned())?;
    let directory_names = project_structure_directory_names(project_name);

    for directory_name in &directory_names {
        if project_path.join(directory_name).exists() {
            return Err(format!("target_exists:{directory_name}"));
        }
    }

    for directory_name in &directory_names {
        std::fs::create_dir(project_path.join(directory_name))
            .map_err(|error| format!("create_failed:{directory_name}:{error}"))?;
    }

    Ok(directory_names.into_iter().collect())
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
fn update_project_icon(project_path: &str, icon_id: &str) -> Result<(), &'static str> {
    if !is_valid_project_icon_id(icon_id) {
        return Err("invalid_icon_id");
    }

    let project_path = canonical_project_path(project_path)?;
    let metadata_path = project_path.join("vesper.json");
    let mut metadata = std::fs::read_to_string(&metadata_path)
        .ok()
        .and_then(|value| serde_json::from_str::<serde_json::Value>(&value).ok())
        .and_then(|value| value.as_object().cloned())
        .unwrap_or_default();

    metadata.insert("icon".into(), serde_json::Value::String(icon_id.into()));

    let metadata = serde_json::to_string_pretty(&serde_json::Value::Object(metadata))
        .map_err(|_| "serialize_failed")?;

    std::fs::write(metadata_path, metadata).map_err(|_| "write_failed")
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
        .setup(|app| {
            let open_item =
                MenuItem::with_id(app, OPEN_VESPER_MENU_ID, "Ouvrir VespΣr", true, None::<&str>)?;
            let toggle_companion_item = MenuItem::with_id(
                app,
                TOGGLE_COMPANION_MENU_ID,
                "Afficher / masquer le compagnon",
                true,
                None::<&str>,
            )?;
            let refresh_item = MenuItem::with_id(
                app,
                REFRESH_VESPER_MENU_ID,
                "Rafraîchir l'app",
                true,
                None::<&str>,
            )?;
            let quit_item =
                MenuItem::with_id(app, QUIT_VESPER_MENU_ID, "Quitter", true, None::<&str>)?;
            let menu = Menu::with_items(
                app,
                &[&open_item, &toggle_companion_item, &refresh_item, &quit_item],
            )?;
            let tray_icon = app.default_window_icon().cloned();

            TrayIconBuilder::with_id("vesper-tray")
                .icon(tray_icon.ok_or("missing_default_icon")?)
                .menu(&menu)
                .show_menu_on_left_click(false)
                .on_menu_event(|app, event| match event.id().as_ref() {
                    OPEN_VESPER_MENU_ID => {
                        let _ = show_main_window(app);
                    }
                    TOGGLE_COMPANION_MENU_ID => {
                        let _ = app.emit_to(MAIN_WINDOW_LABEL, TRAY_TOGGLE_COMPANION_EVENT, ());
                    }
                    REFRESH_VESPER_MENU_ID => {
                        let _ = app.emit_to(MAIN_WINDOW_LABEL, TRAY_REFRESH_APP_EVENT, ());
                    }
                    QUIT_VESPER_MENU_ID => {
                        app.exit(0);
                    }
                    _ => {}
                })
                .on_tray_icon_event(|tray, event| {
                    if let TrayIconEvent::Click {
                        button: MouseButton::Left,
                        button_state: MouseButtonState::Up,
                        ..
                    } = event
                    {
                        let _ = show_main_window(tray.app_handle());
                    }
                })
                .build(app)?;

            Ok(())
        })
        .on_window_event(|window, event| {
            if window.label() == MAIN_WINDOW_LABEL {
                if let WindowEvent::CloseRequested { api, .. } = event {
                    api.prevent_close();
                    let _ = window.hide();
                }
            }
        })
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![
            greet,
            drag_vesperion,
            list_project_directories,
            list_project_genealogy,
            create_project_structure,
            update_project_status,
            update_project_icon,
            open_project_directory,
            open_project_vscode,
            open_project_github,
            open_global_access,
            open_chronos_app,
            open_plum3_app,
            read_project_official_context,
            read_latest_project_session_markdown,
            read_latest_project_session_snapshot,
            validate_chronos_project_directory,
            read_latest_session_snapshot_from_directory,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[cfg(test)]
mod tests {
    use super::{
        is_valid_project_icon_id, matching_branch_dir, project_structure_directory_names,
        session_date_from_file_name,
    };
    use std::path::Path;

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
    fn builds_the_normalized_project_structure_names() {
        assert_eq!(
            project_structure_directory_names("Chr0nos"),
            [
                "01_⏳ Chr0nos_Assets",
                "02_⏳ Chr0nos_Docs",
                "05_⏳ Chr0nos_FEATURES",
                "99_⏳ Chr0nos_Archive",
            ]
        );
    }

    #[test]
    fn validates_project_icon_identifiers() {
        assert!(is_valid_project_icon_id("crystal-ball"));
        assert!(is_valid_project_icon_id("robot2"));
        assert!(!is_valid_project_icon_id(""));
        assert!(!is_valid_project_icon_id("Crystal Ball"));
        assert!(!is_valid_project_icon_id("../robot"));
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

    #[test]
    fn extracts_session_date_from_file_name() {
        assert_eq!(
            session_date_from_file_name(Path::new("2026-06-24 - Fin de session.md")),
            Some("2026-06-24".into())
        );
        assert_eq!(
            session_date_from_file_name(Path::new("Journal_2026_07_03_raw.md")),
            Some("2026-07-03".into())
        );
    }
}
