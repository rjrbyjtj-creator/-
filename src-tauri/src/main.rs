#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use serde::Serialize;
use tauri::{
    utils::config::WebviewUrl,
    webview::{PageLoadEvent, WebviewWindowBuilder},
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
async fn open_updater_window(app: tauri::AppHandle) -> Result<(), String> {
    if let Some(window) = app.get_webview_window("updater") {
        window.show().map_err(|error| error.to_string())?;
        window.set_focus().map_err(|error| error.to_string())?;
        return Ok(());
    }

    WebviewWindowBuilder::new(&app, "updater", WebviewUrl::App("updater.html".into()))
        .title("به‌روزرسانی مدار")
        .inner_size(560.0, 590.0)
        .min_inner_size(500.0, 520.0)
        .resizable(false)
        .center()
        .build()
        .map_err(|error| error.to_string())?;

    Ok(())
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_updater::Builder::new().build())
        .invoke_handler(tauri::generate_handler![
            check_for_update,
            install_update,
            open_updater_window
        ])
        .setup(|app| {
            // Keep the splash page local, then allow navigation only to the
            // official Madar GitHub Pages host. Add a small in-app updater
            // entry point to every official Madar page.
            let updater_button = r#"
              (() => {
                if (window.location.hostname !== 'rjrbyjtj-creator.github.io') return;
                if (document.getElementById('madar-desktop-update')) return;
                const button = document.createElement('button');
                button.id = 'madar-desktop-update';
                button.type = 'button';
                button.textContent = '↻  به‌روزرسانی برنامه';
                button.setAttribute('aria-label', 'بررسی به‌روزرسانی برنامه مدار');
                button.style.cssText = 'position:fixed;z-index:2147483000;left:20px;bottom:20px;padding:11px 16px;border:1px solid rgba(205,174,94,.48);border-radius:12px;background:#111a2d;color:#f2d58b;font:600 12px Tahoma, sans-serif;box-shadow:0 8px 28px rgba(0,0,0,.24);cursor:pointer;direction:rtl';
                button.addEventListener('click', async () => {
                  try {
                    await window.__TAURI__.core.invoke('open_updater_window');
                  } catch (error) {
                    button.textContent = 'خطا در باز کردن به‌روزرسانی';
                    console.error(error);
                  }
                });
                document.body.appendChild(button);
              })();
            "#;

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
