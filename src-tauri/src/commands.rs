use crate::flatpak_backend::{self, FlatpakAppModel, FlatpakRemoteModel};
use crate::i18n;
use crate::luppo_backend::{self, LuppoPackageSummary, LuppoRepoModel};
use crate::settings::{self, AppSettings};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tauri::AppHandle;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemOverview {
    pub luppo_installed: bool,
    pub luppo_total_repos: usize,
    pub luppo_active_repos: usize,
    pub luppo_total_packages: usize,
    pub flatpak_installed: bool,
    pub flatpak_total_remotes: usize,
    pub flatpak_active_remotes: usize,
    pub is_root: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PresetRepo {
    pub repo_type: String, // "luppo" veya "flatpak"
    pub name: String,
    pub title: String,
    pub url: String,
    pub description: String,
    pub default_scope: String,
    pub mirrors: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportData {
    pub version: String,
    pub export_date: String,
    pub luppo_repos: Vec<LuppoRepoModel>,
    pub flatpak_remotes: Vec<FlatpakRemoteModel>,
}

// ─── Genel Bakış ────────────────────────────────────────────────────────────

#[tauri::command]
pub async fn get_system_overview() -> Result<SystemOverview, String> {
    let is_root = unsafe { libc::geteuid() == 0 };

    let luppo_repos = luppo_backend::list_repos().unwrap_or_default();
    let luppo_total_repos = luppo_repos.len();
    let luppo_active_repos = luppo_repos.iter().filter(|r| r.enabled).count();
    let luppo_total_packages = luppo_repos.iter().map(|r| r.package_count).sum();

    let flatpak_installed = flatpak_backend::is_flatpak_installed();
    let flatpak_remotes = flatpak_backend::list_remotes().unwrap_or_default();
    let flatpak_total_remotes = flatpak_remotes.len();
    let flatpak_active_remotes = flatpak_remotes.iter().filter(|r| r.enabled).count();

    Ok(SystemOverview {
        luppo_installed: true,
        luppo_total_repos,
        luppo_active_repos,
        luppo_total_packages,
        flatpak_installed,
        flatpak_total_remotes,
        flatpak_active_remotes,
        is_root,
    })
}

// ─── Luppo Komutları ─────────────────────────────────────────────────────────

#[tauri::command]
pub async fn get_luppo_repos() -> Result<Vec<LuppoRepoModel>, String> {
    luppo_backend::list_repos()
}

#[tauri::command]
pub async fn get_luppo_repo_packages(repo_name: String) -> Result<Vec<LuppoPackageSummary>, String> {
    luppo_backend::get_repo_packages(&repo_name)
}

#[tauri::command]
pub async fn add_luppo_repo(name: String, url: String, mirrors: Vec<String>) -> Result<String, String> {
    luppo_backend::add_repo(&name, &url, mirrors)
}

#[tauri::command]
pub async fn remove_luppo_repo(name: String) -> Result<String, String> {
    luppo_backend::remove_repo(&name)
}

#[tauri::command]
pub async fn toggle_luppo_repo(name: String, enabled: bool) -> Result<String, String> {
    luppo_backend::toggle_repo(&name, enabled)
}

#[tauri::command]
pub async fn edit_luppo_repo(name: String, new_url: String, mirrors: Vec<String>) -> Result<String, String> {
    luppo_backend::edit_repo(&name, &new_url, mirrors)
}

#[tauri::command]
pub async fn sync_luppo_repos(app: AppHandle) -> Result<String, String> {
    luppo_backend::sync_luppo_repos(app).await
}

#[tauri::command]
pub async fn ping_luppo_repo(url: String, mirrors: Vec<String>) -> Result<HashMap<String, Result<u64, String>>, String> {
    Ok(luppo_backend::ping_luppo_repo(url, mirrors).await)
}

// ─── Flatpak Komutları ───────────────────────────────────────────────────────

#[tauri::command]
pub async fn get_flatpak_remotes() -> Result<Vec<FlatpakRemoteModel>, String> {
    flatpak_backend::list_remotes()
}

#[tauri::command]
pub async fn get_flatpak_remote_apps(remote_name: String, scope: String) -> Result<Vec<FlatpakAppModel>, String> {
    flatpak_backend::get_remote_apps(&remote_name, &scope)
}

#[tauri::command]
pub async fn add_flatpak_remote(
    name: String,
    url: String,
    scope: String,
    gpg_verify: bool,
    title: Option<String>,
    comment: Option<String>,
) -> Result<String, String> {
    flatpak_backend::add_remote(&name, &url, &scope, gpg_verify, title, comment)
}

#[tauri::command]
pub async fn remove_flatpak_remote(name: String, scope: String) -> Result<String, String> {
    flatpak_backend::remove_remote(&name, &scope)
}

#[tauri::command]
pub async fn toggle_flatpak_remote(name: String, scope: String, enabled: bool) -> Result<String, String> {
    flatpak_backend::toggle_remote(&name, &scope, enabled)
}

#[tauri::command]
pub async fn edit_flatpak_remote(
    name: String,
    scope: String,
    priority: Option<i32>,
    url: Option<String>,
) -> Result<String, String> {
    flatpak_backend::edit_remote(&name, &scope, priority, url)
}

#[tauri::command]
pub async fn sync_flatpak_remotes(app: AppHandle) -> Result<String, String> {
    flatpak_backend::sync_flatpak_remotes(app).await
}

#[tauri::command]
pub async fn ping_flatpak_remote(url: String) -> Result<u64, String> {
    flatpak_backend::ping_flatpak_remote(url).await
}

// ─── Ortak Senkronizasyon ───────────────────────────────────────────────────

#[tauri::command]
pub async fn sync_all_repos(app: AppHandle) -> Result<String, String> {
    let mut messages = Vec::new();

    // 1. Luppo sync
    match luppo_backend::sync_luppo_repos(app.clone()).await {
        Ok(msg) => messages.push(msg),
        Err(e) => messages.push(format!("Luppo Hatası: {}", e)),
    }

    // 2. Flatpak sync
    if flatpak_backend::is_flatpak_installed() {
        match flatpak_backend::sync_flatpak_remotes(app.clone()).await {
            Ok(msg) => messages.push(msg),
            Err(e) => messages.push(format!("Flatpak Hatası: {}", e)),
        }
    }

    Ok(messages.join(" | "))
}

// ─── Hazır Depo Şablonları (Presets) ─────────────────────────────────────────

#[tauri::command]
pub fn get_popular_presets() -> Vec<PresetRepo> {
    vec![
        PresetRepo {
            repo_type: "luppo".to_string(),
            name: "stable".to_string(),
            title: "LupuS (Stable)".to_string(),
            url: "https://cdn.antolun.com/luppo/repo/stable".to_string(),
            description: "Official Repo for LupuS".to_string(),
            default_scope: "system".to_string(),
            mirrors: vec![
                "https://cdn.antolun.com/luppo/repo/stable/luppo-index.json.xz".to_string(),
            ],
        },
        PresetRepo {
            repo_type: "luppo".to_string(),
            name: "contrib".to_string(),
            title: "LupuS (Contrib)".to_string(),
            url: "https://cdn.antolun.com/luppo/repo/contrib".to_string(),
            description: "Community Repo for LupuS".to_string(),
            default_scope: "system".to_string(),
            mirrors: vec![
                "https://cdn.antolun.com/luppo/repo/contrib/luppo-index.json.xz".to_string(),
            ],
        },
        PresetRepo {
            repo_type: "luppo".to_string(),
            name: "pisistable".to_string(),
            title: "PiSiLinux (Stable)".to_string(),
            url: "https://stable.pisilinux.org/pisi-index.xml.xz".to_string(),
            description: "Official Repo for PiSiLinux".to_string(),
            default_scope: "system".to_string(),
            mirrors: vec![
                "https://mirror.pisilinux.org/stable/pisi-index.xml.xz".to_string(),
                "https://stable2.pisilinux.org/pisi-index.xml.xz".to_string(),
            ],
        },
        PresetRepo {
            repo_type: "luppo".to_string(),
            name: "pisicontrib".to_string(),
            title: "PiSiLinux (Contrib)".to_string(),
            url: "https://contrib.pisilinux.org/pisi-index.xml.xz".to_string(),
            description: "Community Repository for PiSiLinux".to_string(),
            default_scope: "system".to_string(),
            mirrors: vec![
                "https://mirror.pisilinux.org/contrib/pisi-index.xml.xz".to_string(),
            ],
        },
        PresetRepo {
            repo_type: "flatpak".to_string(),
            name: "flathub".to_string(),
            title: "Flathub (Official)".to_string(),
            url: "https://dl.flathub.org/repo/flathub.flatpakrepo".to_string(),
            description: "FlatHub Repo for Flatpak".to_string(),
            default_scope: "system".to_string(),
            mirrors: vec![],
        },
        PresetRepo {
            repo_type: "flatpak".to_string(),
            name: "flathub-beta".to_string(),
            title: "Flathub (Beta)".to_string(),
            url: "https://dl.flathub.org/beta-repo/flathub-beta.flatpakrepo".to_string(),
            description: "FlatHub Beta Repo for Flatpak".to_string(),
            default_scope: "user".to_string(),
            mirrors: vec![],
        },
        PresetRepo {
            repo_type: "flatpak".to_string(),
            name: "gnome-nightly".to_string(),
            title: "GNOME Nightly".to_string(),
            url: "https://nightly.gnome.org/gnome-nightly.flatpakrepo".to_string(),
            description: "GNOME Desktop and Kernel Applications Repo".to_string(),
            default_scope: "user".to_string(),
            mirrors: vec![],
        },
        PresetRepo {
            repo_type: "flatpak".to_string(),
            name: "kdeapps".to_string(),
            title: "KDE Flatpaks".to_string(),
            url: "https://cdn.kde.org/flatpak/".to_string(),
            description: "KDE Official Flatpak Application and Developer Builds".to_string(),
            default_scope: "user".to_string(),
            mirrors: vec![],
        },
    ]
}

// ─── Yedekleme ve Geri Yükleme ───────────────────────────────────────────────

#[tauri::command]
pub async fn export_repos_json() -> Result<String, String> {
    let luppo_repos = luppo_backend::list_repos().unwrap_or_default();
    let flatpak_remotes = flatpak_backend::list_remotes().unwrap_or_default();

    let export = ExportData {
        version: "1.0.0".to_string(),
        export_date: chrono::Local::now().to_rfc3339(),
        luppo_repos,
        flatpak_remotes,
    };

    serde_json::to_string_pretty(&export).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn import_repos_json(json_content: String) -> Result<String, String> {
    let data: ExportData = serde_json::from_str(&json_content)
        .map_err(|e| format!("Geçersiz JSON biçimi: {}", e))?;

    let mut added_luppo = 0;
    let mut added_flatpak = 0;

    for r in data.luppo_repos {
        if let Ok(_) = luppo_backend::add_repo(&r.name, &r.url, r.mirrors) {
            added_luppo += 1;
        }
    }

    for r in data.flatpak_remotes {
        if let Ok(_) = flatpak_backend::add_remote(
            &r.name,
            &r.url,
            &r.scope,
            true,
            Some(r.title),
            Some(r.comment),
        ) {
            added_flatpak += 1;
        }
    }

    Ok(format!(
        "İçe aktarma tamamlandı: {} Luppo deposu, {} Flatpak deposu.",
        added_luppo, added_flatpak
    ))
}

// ─── Ayarlar ve Çeviri ───────────────────────────────────────────────────────

#[tauri::command]
pub fn load_settings() -> AppSettings {
    settings::load_settings()
}

#[tauri::command]
pub fn save_settings(settings: AppSettings) -> Result<(), String> {
    settings::save_settings(&settings)
}

#[tauri::command]
pub fn get_translations() -> HashMap<String, HashMap<String, String>> {
    i18n::get_translations()
}
