use std::fmt::Display;

use serde::Serialize;
use tauri::{Emitter, Manager};

use crate::theory::BASS_CLEFF_RANGE;

mod iter;
mod mei;
mod pozzoli;
mod simple;
mod theory;

#[cfg(test)]
mod test_utils;

#[derive(Debug)]
enum Error {
    TauriFailed(String),
    XmlSerialization(String),
}
impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::TauriFailed(msg) => write!(f, "Tauri failed {}", msg),
            Error::XmlSerialization(msg) => write!(f, "XML serialization failed: {}", msg),
        }
    }
}

#[derive(Clone, Serialize)]
struct MeiChangedEvent {
    mei: String,
}
fn send_mei(app: tauri::AppHandle, m: mei::Mei) -> Result<(), Error> {
    let xml = m
        .to_xml()
        .map_err(|e| Error::XmlSerialization(e.to_string()))?;
    app.emit("meiChanged", MeiChangedEvent { mei: xml })
        .map_err(|e| Error::TauriFailed(e.to_string()))?;
    Ok(())
}
#[tauri::command]
async fn refresh(app: tauri::AppHandle, exercise: &str) -> Result<(), String> {
    let drill = match exercise {
        "Simple" => {
            simple::quarters_in_quadruple("C2-C4 on Bass Clef".to_owned(), 100, BASS_CLEFF_RANGE)
        }
        _ => pozzoli::series1_time2(),
    };
    send_mei(app, drill).map_err(|e| e.to_string())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            #[cfg(debug_assertions)]
            {
                let window = app.get_webview_window("main").unwrap();
                window.open_devtools();
            }
            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![refresh])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
