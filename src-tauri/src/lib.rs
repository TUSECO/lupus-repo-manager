mod commands;
mod flatpak_backend;
mod i18n;
mod luppo_backend;
mod settings;
mod utils;

use tauri::Manager;

pub fn run() {
    glib::set_application_name("LupuS Repo Manager");

    tauri::Builder::default()
        .plugin(tauri_plugin_single_instance::init(|app, _args, _cwd| {
            if let Some(win) = app.get_webview_window("main") {
                let _ = win.show();
                let _ = win.set_focus();
            }
        }))
        .plugin(
            tauri_plugin_log::Builder::new()
                .level(log::LevelFilter::Info)
                .level_for("reqwest", log::LevelFilter::Warn)
                .level_for("hyper", log::LevelFilter::Warn)
                .build(),
        )
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_fs::init())
        .setup(|_app| {
            log::info!("LupuS Repo Manager başlatıldı");
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::get_system_overview,
            commands::get_luppo_repos,
            commands::get_luppo_repo_packages,
            commands::add_luppo_repo,
            commands::remove_luppo_repo,
            commands::toggle_luppo_repo,
            commands::edit_luppo_repo,
            commands::sync_luppo_repos,
            commands::ping_luppo_repo,
            commands::get_flatpak_remotes,
            commands::get_flatpak_remote_apps,
            commands::add_flatpak_remote,
            commands::remove_flatpak_remote,
            commands::toggle_flatpak_remote,
            commands::edit_flatpak_remote,
            commands::sync_flatpak_remotes,
            commands::ping_flatpak_remote,
            commands::sync_all_repos,
            commands::get_popular_presets,
            commands::export_repos_json,
            commands::import_repos_json,
            commands::load_settings,
            commands::save_settings,
            commands::get_translations,
        ])
        .run(tauri::generate_context!())
        .expect("LupuS Repo Manager başlatılamadı");
}
