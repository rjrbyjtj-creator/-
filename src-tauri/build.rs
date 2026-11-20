fn main() {
    tauri_build::try_build(
        tauri_build::Attributes::new().app_manifest(
            tauri_build::AppManifest::new().commands(&[
                "check_for_update",
                "install_update",
                "get_app_version",
            ]),
        ),
    )
    .expect("failed to build Tauri application");
}
