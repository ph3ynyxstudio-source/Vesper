use tauri_plugin_opener::OpenerExt;

#[derive(Debug, PartialEq, Eq)]
enum KnownDestination {
    Url(&'static str),
    Path(&'static str),
}

fn resolve_known_destination(destination_id: &str) -> Result<KnownDestination, &'static str> {
    match destination_id {
        "lunarmood_github" => Ok(KnownDestination::Url(
            "https://github.com/ph3ynyxstudio-source/Lun4rMood",
        )),
        "lunarmood_root" => Ok(KnownDestination::Path(r"C:\Ph3yNyx.OS\Devs\Lun4rMood")),
        "lunarmood_assets" => Ok(KnownDestination::Path(
            r"C:\Ph3yNyx.OS\02_🌙Lun△rMood\Lun△rMood Asset",
        )),
        "lunarmood_docs" => Ok(KnownDestination::Path(
            r"C:\Ph3yNyx.OS\02_🌙Lun△rMood\Lun△rMood Docs",
        )),
        "lunarmood_sessions" => Ok(KnownDestination::Path(
            r"C:\Ph3yNyx.OS\01_⏳↻hr0nosV3rs\AppData\chronosvers\data\projects\LunarMood",
        )),
        _ => Err("unknown_destination"),
    }
}

#[tauri::command]
fn open_known_destination(app: tauri::AppHandle, destination_id: &str) -> Result<(), &'static str> {
    let destination = resolve_known_destination(destination_id)?;
    let result = match destination {
        KnownDestination::Url(url) => app.opener().open_url(url, None::<&str>),
        KnownDestination::Path(path) => app.opener().open_path(path, None::<&str>),
    };

    result.map_err(|_| "open_failed")
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
            open_known_destination
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[cfg(test)]
mod tests {
    use super::{resolve_known_destination, KnownDestination};

    #[test]
    fn resolves_every_lunarmood_destination() {
        assert_eq!(
            resolve_known_destination("lunarmood_github"),
            Ok(KnownDestination::Url(
                "https://github.com/ph3ynyxstudio-source/Lun4rMood"
            ))
        );
        assert_eq!(
            resolve_known_destination("lunarmood_root"),
            Ok(KnownDestination::Path(r"C:\Ph3yNyx.OS\Devs\Lun4rMood"))
        );
        assert_eq!(
            resolve_known_destination("lunarmood_assets"),
            Ok(KnownDestination::Path(
                r"C:\Ph3yNyx.OS\02_🌙Lun△rMood\Lun△rMood Asset"
            ))
        );
        assert_eq!(
            resolve_known_destination("lunarmood_docs"),
            Ok(KnownDestination::Path(
                r"C:\Ph3yNyx.OS\02_🌙Lun△rMood\Lun△rMood Docs"
            ))
        );
        assert_eq!(
            resolve_known_destination("lunarmood_sessions"),
            Ok(KnownDestination::Path(
                r"C:\Ph3yNyx.OS\01_⏳↻hr0nosV3rs\AppData\chronosvers\data\projects\LunarMood"
            ))
        );
    }

    #[test]
    fn rejects_unknown_and_path_like_values() {
        assert_eq!(
            resolve_known_destination("unknown"),
            Err("unknown_destination")
        );
        assert_eq!(
            resolve_known_destination(r"C:\Windows\System32"),
            Err("unknown_destination")
        );
        assert_eq!(
            resolve_known_destination("https://example.com"),
            Err("unknown_destination")
        );
    }
}
