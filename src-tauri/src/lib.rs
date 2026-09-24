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
const CHRONOS_PROJECT_DIRECTORY_NAMES: [&str; 5] =
    ["archives", "monthly", "quarterly", "raw", "weekly"];

#[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
enum ChronosProjectDirectoryState {
    Absent,
    Complete,
    Incomplete,
    NotDirectory,
}

#[derive(Debug, serde::Serialize)]
struct NewProjectPreparationReport {
    valid: bool,
    can_create: bool,
    project_name: String,
    project_path: String,
    project_exists: bool,
    create_vesper_structure: bool,
    project_structure_paths: Vec<String>,
    project_structure_collisions: Vec<String>,
    create_or_associate_chronos: bool,
    chronos_path: String,
    chronos_directory_paths: Vec<String>,
    chronos_state: ChronosProjectDirectoryState,
    missing_chronos_directories: Vec<String>,
    targets: Vec<DirectoryOperationReport>,
    blockers: Vec<String>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
enum DirectoryOperation {
    Create,
    Associate,
    Verify,
    Skip,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
enum DirectoryOperationResult {
    Planned,
    Created,
    AlreadyExistingValid,
    Ignored,
    Blocked,
    Failed,
    Verified,
}

#[derive(Clone, Debug, serde::Serialize)]
struct DirectoryOperationReport {
    path: String,
    operation: DirectoryOperation,
    result: DirectoryOperationResult,
    message: Option<String>,
}

#[derive(Debug, serde::Serialize)]
struct NewProjectCreationReport {
    success: bool,
    partial: bool,
    project_name: String,
    project_path: String,
    chronos_path: String,
    operations: Vec<DirectoryOperationReport>,
    errors: Vec<String>,
}

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

fn validate_windows_project_name(project_name: &str) -> Result<(), &'static str> {
    if project_name.trim().is_empty() {
        return Err("project_name_empty");
    }

    if project_name.encode_utf16().count() > 255 {
        return Err("project_name_too_long");
    }

    if project_name.ends_with([' ', '.']) {
        return Err("project_name_invalid_ending");
    }

    if project_name.starts_with('.') {
        return Err("project_name_reserved_for_technical_directory");
    }

    if project_name.chars().any(|character| {
        character <= '\u{1f}'
            || matches!(
                character,
                '<' | '>' | ':' | '"' | '/' | '\\' | '|' | '?' | '*'
            )
    }) {
        return Err("project_name_invalid_character");
    }

    let mut components = std::path::Path::new(project_name).components();
    if !matches!(components.next(), Some(std::path::Component::Normal(_)))
        || components.next().is_some()
    {
        return Err("project_name_not_single_component");
    }

    let reserved_stem = project_name
        .split('.')
        .next()
        .unwrap_or_default()
        .to_ascii_uppercase();
    let is_reserved = matches!(reserved_stem.as_str(), "CON" | "PRN" | "AUX" | "NUL")
        || reserved_stem.strip_prefix("COM").is_some_and(|suffix| {
            matches!(
                suffix,
                "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9" | "¹" | "²" | "³"
            )
        })
        || reserved_stem.strip_prefix("LPT").is_some_and(|suffix| {
            matches!(
                suffix,
                "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9" | "¹" | "²" | "³"
            )
        });

    if is_reserved {
        return Err("project_name_reserved_by_windows");
    }

    Ok(())
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

fn path_text(path: &std::path::Path) -> String {
    let path = path.to_string_lossy();

    if let Some(path) = path.strip_prefix(r"\\?\UNC\") {
        return format!(r"\\{path}");
    }
    if let Some(path) = path.strip_prefix(r"\\?\") {
        return path.into();
    }

    path.into_owned()
}

fn operation_report(
    path: &std::path::Path,
    operation: DirectoryOperation,
    result: DirectoryOperationResult,
    message: Option<String>,
) -> DirectoryOperationReport {
    DirectoryOperationReport {
        path: path_text(path),
        operation,
        result,
        message,
    }
}

fn confined_child_path(
    root: &std::path::Path,
    child_name: &str,
) -> Result<std::path::PathBuf, String> {
    let root = root
        .canonicalize()
        .map_err(|error| format!("root_unavailable:{error}"))?;
    if !root.is_dir() {
        return Err("root_not_directory".into());
    }

    let candidate = root.join(child_name);
    if candidate.parent() != Some(root.as_path()) {
        return Err("path_outside_root".into());
    }

    if candidate.exists() {
        let metadata = std::fs::symlink_metadata(&candidate)
            .map_err(|error| format!("target_metadata_failed:{error}"))?;
        if metadata.file_type().is_symlink() {
            return Err("target_is_symlink".into());
        }

        let canonical_candidate = candidate
            .canonicalize()
            .map_err(|error| format!("target_unavailable:{error}"))?;
        if canonical_candidate.parent() != Some(root.as_path()) {
            return Err("path_outside_root".into());
        }
    }

    Ok(candidate)
}

fn is_verified_direct_child_directory(candidate: &std::path::Path, root: &std::path::Path) -> bool {
    let Ok(root) = root.canonicalize() else {
        return false;
    };
    let Ok(metadata) = std::fs::symlink_metadata(candidate) else {
        return false;
    };
    if metadata.file_type().is_symlink() || !metadata.is_dir() {
        return false;
    }
    let Ok(candidate) = candidate.canonicalize() else {
        return false;
    };

    candidate.parent() == Some(root.as_path())
}

fn inspect_chronos_project_directory(
    chronos_path: &std::path::Path,
) -> (ChronosProjectDirectoryState, Vec<String>) {
    if !chronos_path.exists() {
        return (ChronosProjectDirectoryState::Absent, Vec::new());
    }
    if !chronos_path.is_dir() {
        return (ChronosProjectDirectoryState::NotDirectory, Vec::new());
    }

    let missing = CHRONOS_PROJECT_DIRECTORY_NAMES
        .iter()
        .filter(|directory_name| !chronos_path.join(directory_name).is_dir())
        .map(|directory_name| (*directory_name).to_owned())
        .collect::<Vec<_>>();

    if missing.is_empty() {
        (ChronosProjectDirectoryState::Complete, missing)
    } else {
        (ChronosProjectDirectoryState::Incomplete, missing)
    }
}

fn invalid_preparation_report(
    project_name: &str,
    create_vesper_structure: bool,
    create_or_associate_chronos: bool,
    blockers: Vec<String>,
) -> NewProjectPreparationReport {
    NewProjectPreparationReport {
        valid: false,
        can_create: false,
        project_name: project_name.into(),
        project_path: String::new(),
        project_exists: false,
        create_vesper_structure,
        project_structure_paths: Vec::new(),
        project_structure_collisions: Vec::new(),
        create_or_associate_chronos,
        chronos_path: String::new(),
        chronos_directory_paths: Vec::new(),
        chronos_state: ChronosProjectDirectoryState::NotDirectory,
        missing_chronos_directories: Vec::new(),
        targets: Vec::new(),
        blockers,
    }
}

fn prepare_new_project_in_roots(
    projects_root: &std::path::Path,
    chronos_root: &std::path::Path,
    project_name: &str,
    create_vesper_structure: bool,
    create_or_associate_chronos: bool,
) -> NewProjectPreparationReport {
    if let Err(error) = validate_windows_project_name(project_name) {
        return invalid_preparation_report(
            project_name,
            create_vesper_structure,
            create_or_associate_chronos,
            vec![error.into()],
        );
    }

    let project_path = match confined_child_path(projects_root, project_name) {
        Ok(path) => path,
        Err(error) => {
            return invalid_preparation_report(
                project_name,
                create_vesper_structure,
                create_or_associate_chronos,
                vec![format!("vesper_path_invalid:{error}")],
            );
        }
    };
    let chronos_path = match confined_child_path(chronos_root, project_name) {
        Ok(path) => path,
        Err(error) => {
            return invalid_preparation_report(
                project_name,
                create_vesper_structure,
                create_or_associate_chronos,
                vec![format!("chronos_path_invalid:{error}")],
            );
        }
    };

    let mut blockers = Vec::new();
    let mut targets = Vec::new();
    let project_exists = project_path.exists();
    targets.push(operation_report(
        &project_path,
        DirectoryOperation::Create,
        if project_exists {
            DirectoryOperationResult::Blocked
        } else {
            DirectoryOperationResult::Planned
        },
        project_exists.then(|| "Le projet VespΣr existe déjà.".into()),
    ));
    if project_exists {
        blockers.push("vesper_project_exists".into());
    }

    let structure_names = project_structure_directory_names(project_name);
    let project_structure_paths = structure_names
        .iter()
        .map(|directory_name| project_path.join(directory_name))
        .collect::<Vec<_>>();
    let mut project_structure_collisions = Vec::new();

    for (directory_name, path) in structure_names.iter().zip(&project_structure_paths) {
        let invalid_name = validate_windows_project_name(directory_name).err();
        let collision = path.exists();
        if collision {
            project_structure_collisions.push(path_text(path));
        }

        let result = if !create_vesper_structure {
            DirectoryOperationResult::Ignored
        } else if invalid_name.is_some() || collision {
            DirectoryOperationResult::Blocked
        } else {
            DirectoryOperationResult::Planned
        };
        let message = if !create_vesper_structure {
            Some("Structure VespΣr non demandée.".into())
        } else if let Some(error) = invalid_name {
            blockers.push(format!(
                "invalid_structure_directory:{directory_name}:{error}"
            ));
            Some(format!("Nom de sous-dossier invalide : {error}."))
        } else if collision {
            blockers.push(format!("vesper_structure_collision:{directory_name}"));
            Some("La cible existe déjà.".into())
        } else {
            None
        };

        targets.push(operation_report(
            path,
            if create_vesper_structure {
                DirectoryOperation::Create
            } else {
                DirectoryOperation::Skip
            },
            result,
            message,
        ));
    }

    let (chronos_state, missing_chronos_directories) =
        inspect_chronos_project_directory(&chronos_path);
    let chronos_directory_paths = CHRONOS_PROJECT_DIRECTORY_NAMES
        .iter()
        .map(|directory_name| chronos_path.join(directory_name))
        .collect::<Vec<_>>();

    if !create_or_associate_chronos {
        targets.push(operation_report(
            &chronos_path,
            DirectoryOperation::Skip,
            DirectoryOperationResult::Ignored,
            Some("Création ou association Chr0 non demandée.".into()),
        ));
        for path in &chronos_directory_paths {
            targets.push(operation_report(
                path,
                DirectoryOperation::Skip,
                DirectoryOperationResult::Ignored,
                Some("Création ou association Chr0 non demandée.".into()),
            ));
        }
    } else {
        match chronos_state {
            ChronosProjectDirectoryState::Absent => {
                targets.push(operation_report(
                    &chronos_path,
                    DirectoryOperation::Create,
                    DirectoryOperationResult::Planned,
                    None,
                ));
                for path in &chronos_directory_paths {
                    targets.push(operation_report(
                        path,
                        DirectoryOperation::Create,
                        DirectoryOperationResult::Planned,
                        None,
                    ));
                }
            }
            ChronosProjectDirectoryState::Complete => {
                targets.push(operation_report(
                    &chronos_path,
                    DirectoryOperation::Associate,
                    DirectoryOperationResult::AlreadyExistingValid,
                    Some("Le dossier Chr0 complet sera associé sans modification.".into()),
                ));
                for path in &chronos_directory_paths {
                    targets.push(operation_report(
                        path,
                        DirectoryOperation::Verify,
                        DirectoryOperationResult::AlreadyExistingValid,
                        None,
                    ));
                }
            }
            ChronosProjectDirectoryState::Incomplete => {
                blockers.push("chronos_project_incomplete".into());
                targets.push(operation_report(
                    &chronos_path,
                    DirectoryOperation::Associate,
                    DirectoryOperationResult::Blocked,
                    Some("Le dossier Chr0 existe mais sa structure est incomplète.".into()),
                ));
                for (directory_name, path) in CHRONOS_PROJECT_DIRECTORY_NAMES
                    .iter()
                    .zip(&chronos_directory_paths)
                {
                    let missing = missing_chronos_directories
                        .iter()
                        .any(|value| value == directory_name);
                    targets.push(operation_report(
                        path,
                        DirectoryOperation::Verify,
                        if missing {
                            DirectoryOperationResult::Blocked
                        } else {
                            DirectoryOperationResult::AlreadyExistingValid
                        },
                        missing.then(|| "Sous-dossier Chr0 manquant.".into()),
                    ));
                }
            }
            ChronosProjectDirectoryState::NotDirectory => {
                blockers.push("chronos_project_not_directory".into());
                targets.push(operation_report(
                    &chronos_path,
                    DirectoryOperation::Associate,
                    DirectoryOperationResult::Blocked,
                    Some("La cible Chr0 existe mais n’est pas un dossier.".into()),
                ));
                for path in &chronos_directory_paths {
                    targets.push(operation_report(
                        path,
                        DirectoryOperation::Verify,
                        DirectoryOperationResult::Blocked,
                        Some("Vérification Chr0 impossible.".into()),
                    ));
                }
            }
        }
    }

    NewProjectPreparationReport {
        valid: true,
        can_create: blockers.is_empty(),
        project_name: project_name.into(),
        project_path: path_text(&project_path),
        project_exists,
        create_vesper_structure,
        project_structure_paths: project_structure_paths
            .iter()
            .map(|path| path_text(path))
            .collect(),
        project_structure_collisions,
        create_or_associate_chronos,
        chronos_path: path_text(&chronos_path),
        chronos_directory_paths: chronos_directory_paths
            .iter()
            .map(|path| path_text(path))
            .collect(),
        chronos_state,
        missing_chronos_directories,
        targets,
        blockers,
    }
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

fn github_shortcut_path(
    project_path: &std::path::Path,
) -> Result<std::path::PathBuf, &'static str> {
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

fn latest_file_in_directory(
    directory_path: &std::path::Path,
) -> Result<std::path::PathBuf, &'static str> {
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

    latest.map(|(_, path)| path).ok_or("latest_session_missing")
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
        if second_separator != first_separator && !matches!(second_separator, '-' | '_' | '.' | ' ')
        {
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

fn is_visible_project_directory_name(file_name: &std::ffi::OsStr) -> bool {
    !file_name.to_string_lossy().starts_with('.')
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

        let file_name = entry.file_name();
        if !is_visible_project_directory_name(&file_name) {
            continue;
        }

        let metadata = entry.metadata().ok();
        let modified_at_epoch_seconds =
            metadata
                .and_then(|value| value.modified().ok())
                .and_then(|value| {
                    value
                        .duration_since(std::time::UNIX_EPOCH)
                        .ok()
                        .map(|duration| duration.as_secs())
                });

        let path = entry.path();

        projects.push(ProjectDirectory {
            name: file_name.to_string_lossy().into_owned(),
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
fn open_project_vscode(
    app: tauri::AppHandle,
    project_name: &str,
    project_path: &str,
) -> Result<(), &'static str> {
    let fallback_path;
    let path = if let Some(mapped_path) = vscode_project_path(project_name) {
        std::path::Path::new(mapped_path)
    } else {
        fallback_path = canonical_project_path(project_path)?;
        fallback_path.as_path()
    };

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
fn read_latest_project_session_snapshot(
    project_name: &str,
) -> Result<SessionSnapshot, &'static str> {
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
    canonical_chronos_project_path(path).map(|path| path.to_string_lossy().into_owned())
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
            .args([
                "-d",
                GLOBAL_TERMINAL_PATH,
                WINDOWS_POWERSHELL_EXE,
                "-NoExit",
            ])
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
                "assets" => normalized_name.starts_with("01_") && normalized_name.contains("asset"),
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
            let branch_path = matching_branch_dir(id, label, &directory_names)
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

fn create_project_structure_at(project_path: &std::path::Path) -> Result<Vec<String>, String> {
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

fn blocked_creation_report(
    preparation: NewProjectPreparationReport,
    extra_error: Option<&str>,
) -> NewProjectCreationReport {
    let mut errors = preparation.blockers;
    if let Some(error) = extra_error {
        errors.push(error.into());
    }

    let operations = preparation
        .targets
        .into_iter()
        .map(|mut target| {
            if target.result == DirectoryOperationResult::Planned {
                target.result = DirectoryOperationResult::Blocked;
                target.message = Some("Création non autorisée par la validation finale.".into());
            }
            target
        })
        .collect();

    NewProjectCreationReport {
        success: false,
        partial: false,
        project_name: preparation.project_name,
        project_path: preparation.project_path,
        chronos_path: preparation.chronos_path,
        operations,
        errors,
    }
}

fn create_new_project_in_roots(
    projects_root: &std::path::Path,
    chronos_root: &std::path::Path,
    project_name: &str,
    create_vesper_structure: bool,
    create_or_associate_chronos: bool,
    confirmed: bool,
) -> NewProjectCreationReport {
    create_new_project_in_roots_with_hook(
        projects_root,
        chronos_root,
        project_name,
        create_vesper_structure,
        create_or_associate_chronos,
        confirmed,
        |_| {},
    )
}

fn create_new_project_in_roots_with_hook<BeforeChronos>(
    projects_root: &std::path::Path,
    chronos_root: &std::path::Path,
    project_name: &str,
    create_vesper_structure: bool,
    create_or_associate_chronos: bool,
    confirmed: bool,
    before_chronos: BeforeChronos,
) -> NewProjectCreationReport
where
    BeforeChronos: FnOnce(&std::path::Path),
{
    let preparation = prepare_new_project_in_roots(
        projects_root,
        chronos_root,
        project_name,
        create_vesper_structure,
        create_or_associate_chronos,
    );
    if !confirmed {
        return blocked_creation_report(preparation, Some("confirmation_required"));
    }
    if !preparation.can_create {
        return blocked_creation_report(preparation, None);
    }

    let project_path = std::path::PathBuf::from(&preparation.project_path);
    let chronos_path = std::path::PathBuf::from(&preparation.chronos_path);
    let structure_paths = preparation
        .project_structure_paths
        .iter()
        .map(std::path::PathBuf::from)
        .collect::<Vec<_>>();
    let chronos_directory_paths = preparation
        .chronos_directory_paths
        .iter()
        .map(std::path::PathBuf::from)
        .collect::<Vec<_>>();
    let chronos_state = preparation.chronos_state;
    let mut operations = preparation
        .targets
        .into_iter()
        .filter(|target| target.result == DirectoryOperationResult::Ignored)
        .collect::<Vec<_>>();
    let mut errors = Vec::new();

    match std::fs::create_dir(&project_path) {
        Ok(()) => operations.push(operation_report(
            &project_path,
            DirectoryOperation::Create,
            DirectoryOperationResult::Created,
            None,
        )),
        Err(error) => {
            errors.push(format!("vesper_project_create_failed:{error}"));
            operations.push(operation_report(
                &project_path,
                DirectoryOperation::Create,
                DirectoryOperationResult::Failed,
                Some(error.to_string()),
            ));
        }
    }

    let project_verified = is_verified_direct_child_directory(&project_path, projects_root);
    operations.push(operation_report(
        &project_path,
        DirectoryOperation::Verify,
        if project_verified {
            DirectoryOperationResult::Verified
        } else {
            DirectoryOperationResult::Failed
        },
        (!project_verified).then(|| "Le dossier projet VespΣr n’a pas été vérifié.".into()),
    ));
    if !project_verified {
        errors.push("vesper_project_verification_failed".into());
    }

    if create_vesper_structure {
        if project_verified {
            let structure_result = create_project_structure_at(&project_path);
            if let Err(error) = &structure_result {
                errors.push(format!("vesper_structure_create_failed:{error}"));
            }

            for path in &structure_paths {
                let verified = path.is_dir();
                let created = structure_result.is_ok() || verified;
                operations.push(operation_report(
                    path,
                    DirectoryOperation::Create,
                    if created {
                        DirectoryOperationResult::Created
                    } else {
                        DirectoryOperationResult::Failed
                    },
                    (!created)
                        .then(|| structure_result.as_ref().err().cloned())
                        .flatten(),
                ));
                operations.push(operation_report(
                    path,
                    DirectoryOperation::Verify,
                    if verified {
                        DirectoryOperationResult::Verified
                    } else {
                        DirectoryOperationResult::Failed
                    },
                    (!verified).then(|| "Le sous-dossier VespΣr n’a pas été vérifié.".into()),
                ));
                if !verified {
                    errors.push(format!(
                        "vesper_structure_verification_failed:{}",
                        path_text(path)
                    ));
                }
            }
        } else {
            for path in &structure_paths {
                operations.push(operation_report(
                    path,
                    DirectoryOperation::Create,
                    DirectoryOperationResult::Blocked,
                    Some("Le dossier principal VespΣr n’est pas valide.".into()),
                ));
            }
        }
    }

    if create_or_associate_chronos {
        match chronos_state {
            ChronosProjectDirectoryState::Complete => {
                operations.push(operation_report(
                    &chronos_path,
                    DirectoryOperation::Associate,
                    DirectoryOperationResult::AlreadyExistingValid,
                    Some("Dossier Chr0 associé sans modification.".into()),
                ));
                for path in &chronos_directory_paths {
                    operations.push(operation_report(
                        path,
                        DirectoryOperation::Verify,
                        if path.is_dir() {
                            DirectoryOperationResult::Verified
                        } else {
                            DirectoryOperationResult::Failed
                        },
                        (!path.is_dir()).then(|| "Sous-dossier Chr0 non vérifié.".into()),
                    ));
                    if !path.is_dir() {
                        errors.push(format!("chronos_verification_failed:{}", path_text(path)));
                    }
                }
            }
            ChronosProjectDirectoryState::Absent => {
                before_chronos(&chronos_path);
                match std::fs::create_dir(&chronos_path) {
                    Ok(()) => operations.push(operation_report(
                        &chronos_path,
                        DirectoryOperation::Create,
                        DirectoryOperationResult::Created,
                        None,
                    )),
                    Err(error) => {
                        errors.push(format!("chronos_project_create_failed:{error}"));
                        operations.push(operation_report(
                            &chronos_path,
                            DirectoryOperation::Create,
                            DirectoryOperationResult::Failed,
                            Some(error.to_string()),
                        ));
                    }
                }

                let chronos_verified =
                    is_verified_direct_child_directory(&chronos_path, chronos_root);
                operations.push(operation_report(
                    &chronos_path,
                    DirectoryOperation::Verify,
                    if chronos_verified {
                        DirectoryOperationResult::Verified
                    } else {
                        DirectoryOperationResult::Failed
                    },
                    (!chronos_verified)
                        .then(|| "Le dossier projet Chr0 n’a pas été vérifié.".into()),
                ));

                if chronos_verified {
                    let (_, missing_after_race) = inspect_chronos_project_directory(&chronos_path);
                    let directory_was_created_by_this_command =
                        operations.iter().any(|operation| {
                            operation.path == path_text(&chronos_path)
                                && operation.operation == DirectoryOperation::Create
                                && operation.result == DirectoryOperationResult::Created
                        });

                    if directory_was_created_by_this_command {
                        for path in &chronos_directory_paths {
                            match std::fs::create_dir(path) {
                                Ok(()) => operations.push(operation_report(
                                    path,
                                    DirectoryOperation::Create,
                                    DirectoryOperationResult::Created,
                                    None,
                                )),
                                Err(error) => {
                                    errors.push(format!(
                                        "chronos_directory_create_failed:{}:{error}",
                                        path_text(path)
                                    ));
                                    operations.push(operation_report(
                                        path,
                                        DirectoryOperation::Create,
                                        if path.is_dir() {
                                            DirectoryOperationResult::AlreadyExistingValid
                                        } else {
                                            DirectoryOperationResult::Failed
                                        },
                                        Some(error.to_string()),
                                    ));
                                }
                            }

                            let verified = path.is_dir();
                            operations.push(operation_report(
                                path,
                                DirectoryOperation::Verify,
                                if verified {
                                    DirectoryOperationResult::Verified
                                } else {
                                    DirectoryOperationResult::Failed
                                },
                                (!verified)
                                    .then(|| "Le sous-dossier Chr0 n’a pas été vérifié.".into()),
                            ));
                            if !verified {
                                errors.push(format!(
                                    "chronos_directory_verification_failed:{}",
                                    path_text(path)
                                ));
                            }
                        }
                    } else if missing_after_race.is_empty() {
                        operations.push(operation_report(
                            &chronos_path,
                            DirectoryOperation::Associate,
                            DirectoryOperationResult::AlreadyExistingValid,
                            Some("Dossier Chr0 apparu entre les validations et associé sans modification.".into()),
                        ));
                    } else {
                        errors.push(format!(
                            "chronos_project_became_incomplete:{}",
                            missing_after_race.join(",")
                        ));
                        for path in &chronos_directory_paths {
                            operations.push(operation_report(
                                path,
                                DirectoryOperation::Verify,
                                if path.is_dir() {
                                    DirectoryOperationResult::AlreadyExistingValid
                                } else {
                                    DirectoryOperationResult::Blocked
                                },
                                (!path.is_dir()).then(|| {
                                    "Sous-dossier Chr0 manquant; aucune complétion automatique."
                                        .into()
                                }),
                            ));
                        }
                    }
                } else {
                    errors.push("chronos_project_verification_failed".into());
                    for path in &chronos_directory_paths {
                        operations.push(operation_report(
                            path,
                            DirectoryOperation::Create,
                            DirectoryOperationResult::Blocked,
                            Some("Le dossier principal Chr0 n’est pas valide.".into()),
                        ));
                    }
                }
            }
            ChronosProjectDirectoryState::Incomplete
            | ChronosProjectDirectoryState::NotDirectory => {
                errors.push("chronos_state_changed_after_validation".into());
            }
        }
    }

    let structure_verified =
        !create_vesper_structure || structure_paths.iter().all(|path| path.is_dir());
    let chronos_verified = !create_or_associate_chronos
        || (is_verified_direct_child_directory(&chronos_path, chronos_root)
            && chronos_directory_paths.iter().all(|path| path.is_dir()));
    let success = errors.is_empty() && project_verified && structure_verified && chronos_verified;
    let partial = !success
        && operations.iter().any(|operation| {
            matches!(
                operation.result,
                DirectoryOperationResult::Created
                    | DirectoryOperationResult::AlreadyExistingValid
                    | DirectoryOperationResult::Verified
            )
        });

    NewProjectCreationReport {
        success,
        partial,
        project_name: project_name.into(),
        project_path: path_text(&project_path),
        chronos_path: path_text(&chronos_path),
        operations,
        errors,
    }
}

#[tauri::command]
fn prepare_new_project(
    project_name: &str,
    create_vesper_structure: bool,
    create_or_associate_chronos: bool,
) -> NewProjectPreparationReport {
    prepare_new_project_in_roots(
        std::path::Path::new(PROJECTS_ROOT),
        std::path::Path::new(CHRONOS_PROJECTS_ROOT),
        project_name,
        create_vesper_structure,
        create_or_associate_chronos,
    )
}

#[tauri::command]
fn create_new_project(
    project_name: &str,
    create_vesper_structure: bool,
    create_or_associate_chronos: bool,
    confirmed: bool,
) -> NewProjectCreationReport {
    create_new_project_in_roots(
        std::path::Path::new(PROJECTS_ROOT),
        std::path::Path::new(CHRONOS_PROJECTS_ROOT),
        project_name,
        create_vesper_structure,
        create_or_associate_chronos,
        confirmed,
    )
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
            let open_item = MenuItem::with_id(
                app,
                OPEN_VESPER_MENU_ID,
                "Ouvrir VespΣr",
                true,
                None::<&str>,
            )?;
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
                &[
                    &open_item,
                    &toggle_companion_item,
                    &refresh_item,
                    &quit_item,
                ],
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
            prepare_new_project,
            create_new_project,
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
        create_new_project_in_roots, create_new_project_in_roots_with_hook,
        is_valid_project_icon_id, is_visible_project_directory_name, matching_branch_dir,
        prepare_new_project_in_roots, project_structure_directory_names,
        session_date_from_file_name, validate_windows_project_name, vscode_project_path,
        ChronosProjectDirectoryState, DirectoryOperation, DirectoryOperationResult,
        CHRONOS_PROJECT_DIRECTORY_NAMES, VSCODE_VESPER_PATH,
    };
    use std::{
        path::{Path, PathBuf},
        sync::atomic::{AtomicUsize, Ordering},
    };

    static TEMPORARY_ROOT_COUNTER: AtomicUsize = AtomicUsize::new(0);

    struct TemporaryRoots {
        base: PathBuf,
        vesper: PathBuf,
        chronos: PathBuf,
    }

    impl TemporaryRoots {
        fn new(label: &str) -> Self {
            let counter = TEMPORARY_ROOT_COUNTER.fetch_add(1, Ordering::Relaxed);
            let base = std::env::temp_dir().join(format!(
                "vesper-new-project-{label}-{}-{counter}",
                std::process::id()
            ));
            let vesper = base.join("vesper");
            let chronos = base.join("chronos");

            std::fs::create_dir_all(&vesper).expect("temporary VespΣr root should be created");
            std::fs::create_dir_all(&chronos).expect("temporary Chr0 root should be created");

            Self {
                base,
                vesper,
                chronos,
            }
        }
    }

    impl Drop for TemporaryRoots {
        fn drop(&mut self) {
            let _ = std::fs::remove_dir_all(&self.base);
        }
    }

    fn create_complete_chronos_project(project_path: &Path) {
        std::fs::create_dir(project_path).expect("temporary Chr0 project should be created");
        for directory_name in CHRONOS_PROJECT_DIRECTORY_NAMES {
            std::fs::create_dir(project_path.join(directory_name))
                .expect("temporary Chr0 directory should be created");
        }
    }

    fn tree_entries(root: &Path) -> Vec<String> {
        fn visit(root: &Path, directory: &Path, entries: &mut Vec<String>) {
            let mut children = std::fs::read_dir(directory)
                .expect("temporary directory should be readable")
                .collect::<Result<Vec<_>, _>>()
                .expect("temporary entries should be readable");
            children.sort_by_key(std::fs::DirEntry::file_name);

            for child in children {
                let path = child.path();
                entries.push(
                    path.strip_prefix(root)
                        .expect("entry should stay under its temporary root")
                        .to_string_lossy()
                        .into_owned(),
                );
                if path.is_dir() {
                    visit(root, &path, entries);
                }
            }
        }

        let mut entries = Vec::new();
        visit(root, root, &mut entries);
        entries
    }

    fn names(values: &[&str]) -> Vec<String> {
        values.iter().map(|value| (*value).into()).collect()
    }

    #[test]
    fn hides_dot_prefixed_technical_project_directories() {
        assert!(!is_visible_project_directory_name(std::ffi::OsStr::new(
            ".tmp.driveupload"
        )));
        assert!(!is_visible_project_directory_name(std::ffi::OsStr::new(
            ".technique"
        )));
        assert!(is_visible_project_directory_name(std::ffi::OsStr::new(
            "🎮MonJeu"
        )));
    }

    #[test]
    fn preserves_known_vscode_mappings_and_leaves_unknown_projects_unmapped() {
        assert_eq!(vscode_project_path("⭐VESPΣR"), Some(VSCODE_VESPER_PATH));
        assert_eq!(vscode_project_path("🎮MonJeu"), None);
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
    fn defines_the_exact_chronos_project_directories() {
        assert_eq!(
            CHRONOS_PROJECT_DIRECTORY_NAMES,
            ["archives", "monthly", "quarterly", "raw", "weekly"]
        );
    }

    #[test]
    fn prepares_a_valid_project_on_two_temporary_roots() {
        let roots = TemporaryRoots::new("prepare-valid");

        let report =
            prepare_new_project_in_roots(&roots.vesper, &roots.chronos, "🎮MonJeu", true, true);

        assert!(report.valid);
        assert!(report.can_create);
        assert!(!report.project_exists);
        assert_eq!(report.chronos_state, ChronosProjectDirectoryState::Absent);
        assert!(report.blockers.is_empty());
        assert_eq!(report.project_structure_paths.len(), 4);
        assert_eq!(report.chronos_directory_paths.len(), 5);
    }

    #[test]
    fn blocks_preparation_for_an_invalid_project_name() {
        let roots = TemporaryRoots::new("prepare-invalid-name");

        let report = prepare_new_project_in_roots(
            &roots.vesper,
            &roots.chronos,
            "Projet/Enfant",
            true,
            true,
        );

        assert!(!report.valid);
        assert!(!report.can_create);
        assert_eq!(report.blockers, ["project_name_invalid_character"]);
        assert!(tree_entries(&roots.vesper).is_empty());
        assert!(tree_entries(&roots.chronos).is_empty());
    }

    #[test]
    fn blocks_preparation_when_the_vesper_project_exists() {
        let roots = TemporaryRoots::new("prepare-existing-project");
        std::fs::create_dir(roots.vesper.join("MonProjet"))
            .expect("temporary VespΣr project should be created");

        let report =
            prepare_new_project_in_roots(&roots.vesper, &roots.chronos, "MonProjet", true, false);

        assert!(report.valid);
        assert!(!report.can_create);
        assert!(report.project_exists);
        assert!(report
            .blockers
            .iter()
            .any(|blocker| blocker == "vesper_project_exists"));
    }

    #[test]
    fn reports_each_existing_vesper_structure_collision() {
        let roots = TemporaryRoots::new("prepare-structure-collision");
        let project_path = roots.vesper.join("MonProjet");
        std::fs::create_dir(&project_path).expect("temporary VespΣr project should be created");
        let collision_name = project_structure_directory_names("MonProjet")[1].clone();
        let collision_path = project_path.join(&collision_name);
        std::fs::create_dir(&collision_path).expect("temporary collision should be created");

        let report =
            prepare_new_project_in_roots(&roots.vesper, &roots.chronos, "MonProjet", true, false);

        assert!(!report.can_create);
        assert_eq!(
            report.project_structure_collisions,
            [report.project_structure_paths[1].clone()]
        );
        assert!(report
            .blockers
            .iter()
            .any(|blocker| blocker == &format!("vesper_structure_collision:{collision_name}")));
    }

    #[test]
    fn reports_an_absent_chronos_project_as_creatable() {
        let roots = TemporaryRoots::new("prepare-chronos-absent");

        let report =
            prepare_new_project_in_roots(&roots.vesper, &roots.chronos, "MonProjet", false, true);

        assert!(report.can_create);
        assert_eq!(report.chronos_state, ChronosProjectDirectoryState::Absent);
        assert!(report.missing_chronos_directories.is_empty());
        assert!(report.targets.iter().any(|target| {
            target.path == report.chronos_path
                && target.operation == DirectoryOperation::Create
                && target.result == DirectoryOperationResult::Planned
        }));
    }

    #[test]
    fn accepts_a_complete_chronos_project_with_extra_entries() {
        let roots = TemporaryRoots::new("prepare-chronos-complete");
        let chronos_project = roots.chronos.join("MonProjet");
        create_complete_chronos_project(&chronos_project);
        std::fs::write(chronos_project.join("notes.txt"), "extra")
            .expect("extra Chr0 file should be created");
        std::fs::create_dir(chronos_project.join("extra"))
            .expect("extra Chr0 directory should be created");

        let report =
            prepare_new_project_in_roots(&roots.vesper, &roots.chronos, "MonProjet", true, true);

        assert!(report.can_create);
        assert_eq!(report.chronos_state, ChronosProjectDirectoryState::Complete);
        assert!(report.missing_chronos_directories.is_empty());
        assert!(report.targets.iter().any(|target| {
            target.path == report.chronos_path
                && target.operation == DirectoryOperation::Associate
                && target.result == DirectoryOperationResult::AlreadyExistingValid
        }));
    }

    #[test]
    fn blocks_an_incomplete_chronos_project_and_lists_missing_directories() {
        let roots = TemporaryRoots::new("prepare-chronos-incomplete");
        let chronos_project = roots.chronos.join("MonProjet");
        std::fs::create_dir(&chronos_project).expect("temporary Chr0 project should be created");
        std::fs::create_dir(chronos_project.join("archives"))
            .expect("temporary archives directory should be created");
        std::fs::create_dir(chronos_project.join("raw"))
            .expect("temporary raw directory should be created");

        let report =
            prepare_new_project_in_roots(&roots.vesper, &roots.chronos, "MonProjet", true, true);

        assert!(!report.can_create);
        assert_eq!(
            report.chronos_state,
            ChronosProjectDirectoryState::Incomplete
        );
        assert_eq!(
            report.missing_chronos_directories,
            ["monthly", "quarterly", "weekly"]
        );
        assert!(report
            .blockers
            .iter()
            .any(|blocker| blocker == "chronos_project_incomplete"));
        assert!(!chronos_project.join("monthly").exists());
    }

    #[test]
    fn preparation_returns_the_exact_generated_paths() {
        let roots = TemporaryRoots::new("prepare-paths");

        let report =
            prepare_new_project_in_roots(&roots.vesper, &roots.chronos, "⌛Projet", true, true);

        let project_path = roots
            .vesper
            .canonicalize()
            .expect("temporary VespΣr root should canonicalize")
            .join("⌛Projet");
        assert_eq!(report.project_path, super::path_text(&project_path));
        assert_eq!(
            report.project_structure_paths,
            project_structure_directory_names("⌛Projet")
                .iter()
                .map(|name| super::path_text(&project_path.join(name)))
                .collect::<Vec<_>>()
        );

        let chronos_path = roots
            .chronos
            .canonicalize()
            .expect("temporary Chr0 root should canonicalize")
            .join("⌛Projet");
        assert_eq!(report.chronos_path, super::path_text(&chronos_path));
        assert_eq!(
            report.chronos_directory_paths,
            CHRONOS_PROJECT_DIRECTORY_NAMES
                .iter()
                .map(|name| super::path_text(&chronos_path.join(name)))
                .collect::<Vec<_>>()
        );
    }

    #[test]
    fn preparation_never_writes_to_either_root() {
        let roots = TemporaryRoots::new("prepare-read-only");
        std::fs::write(roots.vesper.join("marker.txt"), "vesper")
            .expect("VespΣr marker should be created");
        std::fs::write(roots.chronos.join("marker.txt"), "chronos")
            .expect("Chr0 marker should be created");
        let vesper_before = tree_entries(&roots.vesper);
        let chronos_before = tree_entries(&roots.chronos);

        let report =
            prepare_new_project_in_roots(&roots.vesper, &roots.chronos, "MonProjet", true, true);

        assert!(report.can_create);
        assert_eq!(tree_entries(&roots.vesper), vesper_before);
        assert_eq!(tree_entries(&roots.chronos), chronos_before);
        assert_eq!(
            std::fs::read_to_string(roots.vesper.join("marker.txt"))
                .expect("VespΣr marker should remain readable"),
            "vesper"
        );
        assert_eq!(
            std::fs::read_to_string(roots.chronos.join("marker.txt"))
                .expect("Chr0 marker should remain readable"),
            "chronos"
        );
    }

    #[test]
    fn creates_and_verifies_a_complete_new_project() {
        let roots = TemporaryRoots::new("create-complete");

        let report = create_new_project_in_roots(
            &roots.vesper,
            &roots.chronos,
            "MonProjet",
            true,
            true,
            true,
        );

        assert!(report.success, "creation errors: {:?}", report.errors);
        assert!(!report.partial);
        let project_path = roots.vesper.join("MonProjet");
        assert!(project_path.is_dir());
        for directory_name in project_structure_directory_names("MonProjet") {
            assert!(project_path.join(directory_name).is_dir());
        }
        let chronos_path = roots.chronos.join("MonProjet");
        assert!(chronos_path.is_dir());
        for directory_name in CHRONOS_PROJECT_DIRECTORY_NAMES {
            assert!(chronos_path.join(directory_name).is_dir());
        }
        assert!(report
            .operations
            .iter()
            .any(|operation| operation.result == DirectoryOperationResult::Created));
        assert!(report
            .operations
            .iter()
            .any(|operation| operation.result == DirectoryOperationResult::Verified));
    }

    #[test]
    fn associates_a_complete_chronos_project_without_modifying_it() {
        let roots = TemporaryRoots::new("create-associate-complete");
        let chronos_project = roots.chronos.join("MonProjet");
        create_complete_chronos_project(&chronos_project);
        std::fs::write(chronos_project.join("notes.txt"), "à conserver")
            .expect("extra Chr0 file should be created");
        let chronos_before = tree_entries(&chronos_project);

        let report = create_new_project_in_roots(
            &roots.vesper,
            &roots.chronos,
            "MonProjet",
            true,
            true,
            true,
        );

        assert!(report.success, "creation errors: {:?}", report.errors);
        assert_eq!(tree_entries(&chronos_project), chronos_before);
        assert_eq!(
            std::fs::read_to_string(chronos_project.join("notes.txt"))
                .expect("extra Chr0 file should remain readable"),
            "à conserver"
        );
        assert!(report.operations.iter().any(|operation| {
            operation.path == report.chronos_path
                && operation.operation == DirectoryOperation::Associate
                && operation.result == DirectoryOperationResult::AlreadyExistingValid
        }));
    }

    #[test]
    fn creation_revalidates_and_never_modifies_an_existing_vesper_project() {
        let roots = TemporaryRoots::new("create-revalidate-existing");
        let project_path = roots.vesper.join("MonProjet");
        std::fs::create_dir(&project_path).expect("temporary VespΣr project should be created");
        std::fs::write(project_path.join("marker.txt"), "inchangé")
            .expect("VespΣr marker should be created");
        let project_before = tree_entries(&project_path);

        let report = create_new_project_in_roots(
            &roots.vesper,
            &roots.chronos,
            "MonProjet",
            true,
            true,
            true,
        );

        assert!(!report.success);
        assert!(!report.partial);
        assert!(report
            .errors
            .iter()
            .any(|error| error == "vesper_project_exists"));
        assert_eq!(tree_entries(&project_path), project_before);
        assert_eq!(
            std::fs::read_to_string(project_path.join("marker.txt"))
                .expect("VespΣr marker should remain readable"),
            "inchangé"
        );
        assert!(tree_entries(&roots.chronos).is_empty());
    }

    #[test]
    fn reports_partial_success_without_rolling_back_vesper() {
        let roots = TemporaryRoots::new("create-partial");
        let chronos_collision = roots.chronos.join("MonProjet");

        let report = create_new_project_in_roots_with_hook(
            &roots.vesper,
            &roots.chronos,
            "MonProjet",
            true,
            true,
            true,
            |chronos_path| {
                std::fs::write(chronos_path, "collision")
                    .expect("Chr0 collision should be injected");
            },
        );

        assert!(!report.success);
        assert!(report.partial);
        let project_path = roots.vesper.join("MonProjet");
        assert!(project_path.is_dir());
        for directory_name in project_structure_directory_names("MonProjet") {
            assert!(project_path.join(directory_name).is_dir());
        }
        assert!(chronos_collision.is_file());
        assert!(report
            .errors
            .iter()
            .any(|error| error.starts_with("chronos_project_create_failed:")));
        assert!(report.operations.iter().any(|operation| {
            operation.path == report.chronos_path
                && operation.operation == DirectoryOperation::Create
                && operation.result == DirectoryOperationResult::Failed
        }));
    }

    #[test]
    fn refuses_creation_without_explicit_confirmation() {
        let roots = TemporaryRoots::new("create-unconfirmed");

        let report = create_new_project_in_roots(
            &roots.vesper,
            &roots.chronos,
            "MonProjet",
            true,
            true,
            false,
        );

        assert!(!report.success);
        assert!(!report.partial);
        assert!(report
            .errors
            .iter()
            .any(|error| error == "confirmation_required"));
        assert!(tree_entries(&roots.vesper).is_empty());
        assert!(tree_entries(&roots.chronos).is_empty());
    }

    #[test]
    fn accepts_valid_windows_project_names_as_single_components() {
        for project_name in ["🎮MonJeu", "MonJeu", "⌛Chr0nosV3rs", "Projet.v2"] {
            assert_eq!(validate_windows_project_name(project_name), Ok(()));
        }
    }

    #[test]
    fn rejects_empty_or_structural_project_names() {
        assert_eq!(
            validate_windows_project_name("   "),
            Err("project_name_empty")
        );
        assert_eq!(
            validate_windows_project_name("."),
            Err("project_name_invalid_ending")
        );
        assert_eq!(
            validate_windows_project_name(".."),
            Err("project_name_invalid_ending")
        );
        assert_eq!(
            validate_windows_project_name(".tmp.driveupload"),
            Err("project_name_reserved_for_technical_directory")
        );
    }

    #[test]
    fn rejects_windows_invalid_characters_and_endings() {
        for project_name in [
            "Projet/Enfant",
            "Projet\\Enfant",
            "C:Projet",
            "Projet?",
            "Projet*",
            "Projet|Test",
            "Projet<Version>",
            "Projet\"Test",
        ] {
            assert_eq!(
                validate_windows_project_name(project_name),
                Err("project_name_invalid_character")
            );
        }

        assert_eq!(
            validate_windows_project_name("Projet "),
            Err("project_name_invalid_ending")
        );
        assert_eq!(
            validate_windows_project_name("Projet."),
            Err("project_name_invalid_ending")
        );
    }

    #[test]
    fn rejects_windows_reserved_device_names() {
        for project_name in [
            "CON",
            "con.txt",
            "PRN",
            "AUX",
            "NUL",
            "COM1",
            "com9.log",
            "COM¹",
            "LPT1",
            "lpt9.txt",
            "LPT³.log",
        ] {
            assert_eq!(
                validate_windows_project_name(project_name),
                Err("project_name_reserved_by_windows")
            );
        }

        assert_eq!(validate_windows_project_name("COM10"), Ok(()));
        assert_eq!(validate_windows_project_name("LPT10"), Ok(()));
    }

    #[test]
    fn rejects_project_names_longer_than_one_windows_component() {
        let project_name = "a".repeat(256);

        assert_eq!(
            validate_windows_project_name(&project_name),
            Err("project_name_too_long")
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

        assert_eq!(
            matching_branch_dir("features", "Features", &directories),
            None
        );
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
