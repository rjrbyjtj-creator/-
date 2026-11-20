#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use serde::Serialize;
use tauri::{
    utils::config::WebviewUrl,
    webview::WebviewWindowBuilder,
    Manager,
};
use tauri_plugin_updater::UpdaterExt;

#[derive(Serialize)]
struct UpdateInfo {
    version: String,
    current_version: String,
    notes: Option<String>,
    date: Option<String>,
}

#[tauri::command]
async fn check_for_update(app: tauri::AppHandle) -> Result<Option<UpdateInfo>, String> {
    let update = app
        .updater()
        .map_err(|error| error.to_string())?
        .check()
        .await
        .map_err(|error| error.to_string())?;

    Ok(update.map(|item| UpdateInfo {
        version: item.version,
        current_version: item.current_version,
        notes: item.body,
        date: item.date.map(|value| value.to_string()),
    }))
}

#[tauri::command]
fn get_app_version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}

#[tauri::command]
async fn install_update(app: tauri::AppHandle) -> Result<(), String> {
    let update = app
        .updater()
        .map_err(|error| error.to_string())?
        .check()
        .await
        .map_err(|error| error.to_string())?
        .ok_or_else(|| "در حال حاضر به‌روزرسانی جدیدی وجود ندارد.".to_string())?;

    update
        .download_and_install(|_, _| {}, || {})
        .await
        .map_err(|error| error.to_string())?;

    app.restart();
}

#[tauri::command]

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_updater::Builder::new().build())
        .invoke_handler(tauri::generate_handler![
            check_for_update,
            install_update,
            get_app_version
        ])
        .setup(|app| {
            WebviewWindowBuilder::new(app, "main", WebviewUrl::App("index.html".into()))
                .title("مدار | روبو")
                .inner_size(1180.0, 820.0)
                .min_inner_size(760.0, 620.0)
                .center()
                .resizable(true)
                .on_navigation(|url| {
                    let host = url.host_str().unwrap_or_default();

                    let is_local_app = matches!(host, "tauri.localhost" | "localhost")
                        && matches!(url.scheme(), "http" | "https" | "tauri");

                    let is_madar_site = url.scheme() == "https"
                        && host == "rjrbyjtj-creator.github.io";

                    is_local_app || is_madar_site
                })
                .on_page_load(move |window, payload| {
                    if payload.event() == PageLoadEvent::Finished
                        && payload.url().scheme() == "https"
                        && payload.url().host_str() == Some("rjrbyjtj-creator.github.io")
                    {
                        let _ = window.eval(updater_button);
                    }
                })
                .build()?;

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running Madar Robo desktop application");
}
