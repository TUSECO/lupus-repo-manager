use crate::utils::{build_root_command, copy_dir_all, format_display_name, ping_url};
use luppo_core::config::Config;
use luppo_core::database::LuppoDatabase;
use luppo_core::repo::RepositoryEntry;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::io::{BufRead, BufReader};
use std::process::Stdio;
use tauri::Emitter;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LuppoRepoModel {
    pub name: String,
    pub url: String,
    pub mirrors: Vec<String>,
    pub enabled: bool,
    pub package_count: usize,
    pub ping_ms: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LuppoPackageSummary {
    pub name: String,
    pub display_name: String,
    pub version: String,
    pub release: String,
    pub summary: String,
    pub component: String,
    pub repo_name: String,
    pub installed: bool,
    pub size: String,
}

pub fn open_luppo_db() -> Result<LuppoDatabase, String> {
    let config = Config::load(None);
    let original_db_path = config.directories.lib_dir.join("db");

    if !original_db_path.exists() {
        return Err("Luppo veritabanı (/var/lib/luppo/db) bulunamadı".to_string());
    }

    let is_root = unsafe { libc::geteuid() == 0 };
    let db_path = if !is_root {
        let uid = unsafe { libc::geteuid() };
        let temp_db = std::env::temp_dir().join(format!("luppo-db-repomanager-{}", uid));
        let _ = copy_dir_all(&original_db_path, &temp_db);
        temp_db
    } else {
        original_db_path
    };

    LuppoDatabase::open(db_path).map_err(|e| format!("Luppo veritabanı açılamadı: {}", e))
}

pub fn list_repos() -> Result<Vec<LuppoRepoModel>, String> {
    let db = open_luppo_db()?;
    let entries = db.list_repos().map_err(|e| e.to_string())?;

    // Paket sayımlarını repo adına göre grupla
    let mut repo_counts: HashMap<String, usize> = HashMap::new();
    if let Ok(packages) = db.list_available_packages() {
        for pkg in packages {
            *repo_counts.entry(pkg.repo_name.clone()).or_insert(0) += 1;
        }
    }

    let mut result = Vec::new();
    for entry in entries {
        let count = repo_counts.get(&entry.name).copied().unwrap_or(0);
        result.push(LuppoRepoModel {
            name: entry.name.clone(),
            url: entry.url.clone(),
            mirrors: entry.mirrors.clone(),
            enabled: entry.enabled,
            package_count: count,
            ping_ms: None,
        });
    }

    // İsme göre sırala
    result.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));
    Ok(result)
}

pub fn get_repo_packages(repo_name: &str) -> Result<Vec<LuppoPackageSummary>, String> {
    let db = open_luppo_db()?;
    let installed_names = db.list_installed_package_names().unwrap_or_default();
    let all_packages = db.list_available_packages().map_err(|e| e.to_string())?;

    let mut result = Vec::new();
    for pkg in all_packages {
        if pkg.repo_name.eq_ignore_ascii_case(repo_name) {
            let is_installed = installed_names.contains(&pkg.name);
            let display = format_display_name(&pkg.name);
            let version = pkg.latest_version();
            let release = if pkg.release != 0 {
                pkg.release.to_string()
            } else {
                pkg.history
                    .updates
                    .first()
                    .map(|u| u.release.to_string())
                    .unwrap_or_else(|| "1".to_string())
            };
            let summary = pkg.get_summary();
            let component = if pkg.partof.is_empty() {
                "base".to_string()
            } else {
                pkg.partof.clone()
            };
            let size = format_bytes(pkg.installed_size);

            result.push(LuppoPackageSummary {
                name: pkg.name,
                display_name: display,
                version,
                release,
                summary,
                component,
                repo_name: pkg.repo_name,
                installed: is_installed,
                size,
            });
        }
    }

    result.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));
    Ok(result)
}

fn format_bytes(bytes: u64) -> String {
    const KB: u64 = 1024;
    const MB: u64 = KB * 1024;
    const GB: u64 = MB * 1024;

    if bytes >= GB {
        format!("{:.1} GB", bytes as f64 / GB as f64)
    } else if bytes >= MB {
        format!("{:.1} MB", bytes as f64 / MB as f64)
    } else if bytes >= KB {
        format!("{:.1} KB", bytes as f64 / KB as f64)
    } else {
        format!("{} B", bytes)
    }
}

pub fn add_repo(name: &str, url: &str, mirrors: Vec<String>) -> Result<String, String> {
    if name.trim().is_empty() {
        return Err("Depo adı boş olamaz".to_string());
    }
    if url.trim().is_empty() {
        return Err("Depo URL adresi boş olamaz".to_string());
    }

    // pkexec/sudo ile luppo add-repo çalıştır
    let mut cmd = build_root_command("luppo");
    cmd.arg("add-repo").arg(name).arg(url);

    let output = cmd.output().map_err(|e| format!("Komut çalıştırılamadı: {}", e))?;
    if output.status.success() {
        // Eğer mirrors tanımlandıysa, doğrudan veritabanında mirrors'ı güncelle
        if !mirrors.is_empty() {
            let _ = update_repo_mirrors_internal(name, mirrors);
        }
        Ok(format!("'{}' deposu başarıyla eklendi.", name))
    } else {
        let err = String::from_utf8_lossy(&output.stderr).to_string();
        Err(if err.is_empty() {
            String::from_utf8_lossy(&output.stdout).to_string()
        } else {
            err
        })
    }
}

pub fn remove_repo(name: &str) -> Result<String, String> {
    let mut cmd = build_root_command("luppo");
    cmd.arg("remove-repo").arg(name);

    let output = cmd.output().map_err(|e| format!("Komut çalıştırılamadı: {}", e))?;
    if output.status.success() {
        Ok(format!("'{}' deposu sistemden kaldırıldı.", name))
    } else {
        let err = String::from_utf8_lossy(&output.stderr).to_string();
        Err(if err.is_empty() {
            String::from_utf8_lossy(&output.stdout).to_string()
        } else {
            err
        })
    }
}

pub fn toggle_repo(name: &str, enable: bool) -> Result<String, String> {
    let mut cmd = build_root_command("luppo");
    if enable {
        cmd.arg("enable-repo").arg(name);
    } else {
        cmd.arg("disable-repo").arg(name);
    }

    let output = cmd.output().map_err(|e| format!("Komut çalıştırılamadı: {}", e))?;
    if output.status.success() {
        let state = if enable { "etkinleştirildi" } else { "devre dışı bırakıldı" };
        Ok(format!("'{}' deposu {}.", name, state))
    } else {
        let err = String::from_utf8_lossy(&output.stderr).to_string();
        Err(if err.is_empty() {
            String::from_utf8_lossy(&output.stdout).to_string()
        } else {
            err
        })
    }
}

pub fn edit_repo(name: &str, new_url: &str, mirrors: Vec<String>) -> Result<String, String> {
    // Önce db'deki kaydı güncelle
    let is_root = unsafe { libc::geteuid() == 0 };
    if is_root {
        if let Ok(db) = open_luppo_db() {
            let repos = db.list_repos().unwrap_or_default();
            if let Some(existing) = repos.into_iter().find(|r| r.name == name) {
                let updated = RepositoryEntry {
                    name: name.to_string(),
                    url: new_url.to_string(),
                    mirrors,
                    enabled: existing.enabled,
                };
                db.insert_repo(name, updated).map_err(|e| e.to_string())?;
                return Ok(format!("'{}' deposu güncellendi.", name));
            }
        }
    }

    // Root değilse pkexec ile kaldır ve yeniden ekle
    let _ = remove_repo(name);
    add_repo(name, new_url, mirrors)?;
    Ok(format!("'{}' deposu başarıyla güncellendi.", name))
}

fn update_repo_mirrors_internal(name: &str, mirrors: Vec<String>) -> Result<(), String> {
    if let Ok(db) = open_luppo_db() {
        if let Ok(repos) = db.list_repos() {
            if let Some(mut repo) = repos.into_iter().find(|r| r.name == name) {
                repo.mirrors = mirrors;
                let _ = db.insert_repo(name, repo);
            }
        }
    }
    Ok(())
}

pub async fn sync_luppo_repos(app: tauri::AppHandle) -> Result<String, String> {
    let mut cmd = build_root_command("luppo");
    cmd.arg("update-repo")
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());

    let mut child = cmd.spawn().map_err(|e| format!("Luppo başlatılamadı: {}", e))?;

    let stdout = child.stdout.take();
    let stderr = child.stderr.take();

    let app_handle_out = app.clone();
    let stdout_handle = tokio::spawn(async move {
        if let Some(out) = stdout {
            let reader = BufReader::new(out);
            for line in reader.lines().flatten() {
                let _ = app_handle_out.emit("repo-sync-log", format!("[Luppo] {}", line));
            }
        }
    });

    let app_handle_err = app.clone();
    let stderr_handle = tokio::spawn(async move {
        if let Some(err) = stderr {
            let reader = BufReader::new(err);
            for line in reader.lines().flatten() {
                let _ = app_handle_err.emit("repo-sync-log", format!("[Luppo HATA] {}", line));
            }
        }
    });

    let status = child.wait().map_err(|e| format!("İşlem hatası: {}", e))?;
    let _ = stdout_handle.await;
    let _ = stderr_handle.await;

    if status.success() {
        let _ = app.emit("repo-sync-complete", "luppo_success");
        Ok("Luppo depoları başarıyla senkronize edildi.".to_string())
    } else {
        let _ = app.emit("repo-sync-complete", "luppo_failed");
        Err("Luppo depoları güncellenirken hata oluştu.".to_string())
    }
}

pub async fn ping_luppo_repo(url: String, mirrors: Vec<String>) -> HashMap<String, Result<u64, String>> {
    let mut results = HashMap::new();

    // Ana URL ping
    let main_res = ping_url(&url).await;
    results.insert(url, main_res);

    // Mirrors ping
    for mirror in mirrors {
        let res = ping_url(&mirror).await;
        results.insert(mirror, res);
    }

    results
}
