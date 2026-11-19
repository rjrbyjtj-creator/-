#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{
    utils::config::WebviewUrl,
    webview::WebviewWindowBuilder,
};

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            // Keep the splash page local, then allow navigation only to the
            // official Madar GitHub Pages host. This keeps login and Robo
            // inside the desktop window without opening arbitrary websites.
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
                .build()?;

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running Madar Robo desktop application");
}
