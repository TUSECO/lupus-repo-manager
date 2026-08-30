use crate::utils::{build_root_command, ping_url};
use serde::{Deserialize, Serialize};
use std::io::{BufRead, BufReader};
use std::process::{Command, Stdio};
use tauri::Emitter;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FlatpakRemoteModel {
    pub name: String,
    pub title: String,
    pub url: String,
    pub collection_id: String,
    pub priority: i32,
    pub scope: String, // "system" veya "user"
    pub enabled: bool,
    pub comment: String,
    pub description: String,
    pub homepage: String,
    pub icon: String,
    pub app_count: usize,
    pub ping_ms: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FlatpakAppModel {
    pub name: String,
    pub app_id: String,
    pub version: String,
    pub branch: String,
    pub description: String,
    pub remote_name: String,
    pub scope: String,
}

pub fn is_flatpak_installed() -> bool {
    Command::new("flatpak")
        .arg("--version")
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
}

pub fn list_remotes() -> Result<Vec<FlatpakRemoteModel>, String> {
    if !is_flatpak_installed() {
        return Ok(Vec::new());
    }

    let mut remotes = Vec::new();

    // 1. Sistem düzeyindeki depoları tara
    if let Ok(sys_remotes) = parse_flatpak_remotes_for_scope("system") {
        remotes.extend(sys_remotes);
    }

    // 2. Kullanıcı düzeyindeki depoları tara
    if let Ok(user_remotes) = parse_flatpak_remotes_for_scope("user") {
        remotes.extend(user_remotes);
    }

    remotes.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));
    Ok(remotes)
}

fn parse_flatpak_remotes_for_scope(scope: &str) -> Result<Vec<FlatpakRemoteModel>, String> {
    let mut cmd = Command::new("flatpak");
    cmd.arg("remotes")
        .arg("--show-disabled")
        .arg("--columns=name:f,title:f,url:f,collection:f,priority:f,options:f,comment:f,description:f,homepage:f,icon:f");

    if scope == "user" {
        cmd.arg("--user");
    } else {
        cmd.arg("--system");
    }

    let output = cmd.output().map_err(|e| e.to_string())?;
    if !output.status.success() {
        return Ok(Vec::new());
    }

    let text = String::from_utf8_lossy(&output.stdout);
    let mut list = Vec::new();

    for line in text.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        let parts: Vec<&str> = line.split('\t').collect();
        if parts.is_empty() {
            continue;
        }

        let name = parts.first().unwrap_or(&"").trim().to_string();
        let title = parts.get(1).unwrap_or(&"").trim().to_string();
        let url = parts.get(2).unwrap_or(&"").trim().to_string();
        let collection = parts.get(3).unwrap_or(&"-").trim().to_string();
        let priority_str = parts.get(4).unwrap_or(&"1").trim();
        let priority = priority_str.parse::<i32>().unwrap_or(1);
        let options = parts.get(5).unwrap_or(&"").to_lowercase();
        let comment = parts.get(6).unwrap_or(&"").trim().to_string();
        let description = parts.get(7).unwrap_or(&"").trim().to_string();
        let homepage = parts.get(8).unwrap_or(&"").trim().to_string();
        let icon = parts.get(9).unwrap_or(&"").trim().to_string();

        let enabled = !options.contains("disabled");

        let clean_title = if title == "-" || title.is_empty() {
            name.clone()
        } else {
            title
        };

        list.push(FlatpakRemoteModel {
            name,
            title: clean_title,
            url: if url == "-" { String::new() } else { url },
            collection_id: if collection == "-" { String::new() } else { collection },
            priority,
            scope: scope.to_string(),
            enabled,
            comment: if comment == "-" { String::new() } else { comment },
            description: if description == "-" { String::new() } else { description },
            homepage: if homepage == "-" { String::new() } else { homepage },
            icon: if icon == "-" { String::new() } else { icon },
            app_count: 0,
            ping_ms: None,
        });
    }

    Ok(list)
}

pub fn get_remote_apps(remote_name: &str, scope: &str) -> Result<Vec<FlatpakAppModel>, String> {
    if !is_flatpak_installed() {
        return Ok(Vec::new());
    }

    let mut cmd = Command::new("flatpak");
    cmd.arg("remote-ls")
        .arg(remote_name)
        .arg("--app")
        .arg("--columns=name,application,version,branch,description");

    if scope == "user" {
        cmd.arg("--user");
    } else {
        cmd.arg("--system");
    }

    let output = cmd.output().map_err(|e| e.to_string())?;
    if !output.status.success() {
        return Ok(Vec::new());
    }

    let text = String::from_utf8_lossy(&output.stdout);
    let mut apps = Vec::new();

    for line in text.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        let parts: Vec<&str> = line.split('\t').collect();
        if parts.len() < 2 {
            continue;
        }

        let name = parts.first().unwrap_or(&"").trim().to_string();
        let app_id = parts.get(1).unwrap_or(&"").trim().to_string();
        let version = parts.get(2).unwrap_or(&"-").trim().to_string();
        let branch = parts.get(3).unwrap_or(&"stable").trim().to_string();
        let description = parts.get(4).unwrap_or(&"").trim().to_string();

        apps.push(FlatpakAppModel {
            name,
            app_id,
            version: if version == "-" { String::new() } else { version },
            branch,
            description,
            remote_name: remote_name.to_string(),
            scope: scope.to_string(),
        });
    }

    apps.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));
    Ok(apps)
}

pub fn add_remote(
    name: &str,
    url_or_path: &str,
    scope: &str,
    gpg_verify: bool,
    title: Option<String>,
    comment: Option<String>,
) -> Result<String, String> {
    if name.trim().is_empty() {
        return Err("Depo adı boş olamaz".to_string());
    }
    if url_or_path.trim().is_empty() {
        return Err("Depo adresi boş olamaz".to_string());
    }

    let is_system = scope == "system";
    let mut cmd = if is_system {
        build_root_command("flatpak")
    } else {
        Command::new("flatpak")
    };

    cmd.arg("remote-add");
    if is_system {
        cmd.arg("--system");
    } else {
        cmd.arg("--user");
    }

    cmd.arg("--if-not-exists");

    if !gpg_verify {
        cmd.arg("--no-gpg-verify");
    }

    if let Some(ref t) = title {
        if !t.trim().is_empty() {
            cmd.arg(format!("--title={}", t));
        }
    }

    if let Some(ref c) = comment {
        if !c.trim().is_empty() {
            cmd.arg(format!("--comment={}", c));
        }
    }

    cmd.arg(name).arg(url_or_path);

    let output = cmd.output().map_err(|e| format!("Komut çalıştırılamadı: {}", e))?;
    if output.status.success() {
        Ok(format!("'{}' Flatpak deposu başarıyla eklendi.", name))
    } else {
        let err = String::from_utf8_lossy(&output.stderr).to_string();
        Err(if err.is_empty() {
            String::from_utf8_lossy(&output.stdout).to_string()
        } else {
            err
        })
    }
}

pub fn remove_remote(name: &str, scope: &str) -> Result<String, String> {
    let is_system = scope == "system";
    let mut cmd = if is_system {
        build_root_command("flatpak")
    } else {
        Command::new("flatpak")
    };

    cmd.arg("remote-delete");
    if is_system {
        cmd.arg("--system");
    } else {
        cmd.arg("--user");
    }
    cmd.arg(name);

    let output = cmd.output().map_err(|e| format!("Komut çalıştırılamadı: {}", e))?;
    if output.status.success() {
        Ok(format!("'{}' Flatpak deposu silindi.", name))
    } else {
        let err = String::from_utf8_lossy(&output.stderr).to_string();
        Err(if err.is_empty() {
            String::from_utf8_lossy(&output.stdout).to_string()
        } else {
            err
        })
    }
}

pub fn toggle_remote(name: &str, scope: &str, enable: bool) -> Result<String, String> {
    let is_system = scope == "system";
    let mut cmd = if is_system {
        build_root_command("flatpak")
    } else {
        Command::new("flatpak")
    };

    cmd.arg("remote-modify");
    if is_system {
        cmd.arg("--system");
    } else {
        cmd.arg("--user");
    }

    if enable {
        cmd.arg("--enable");
    } else {
        cmd.arg("--disable");
    }
    cmd.arg(name);

    let output = cmd.output().map_err(|e| format!("Komut çalıştırılamadı: {}", e))?;
    if output.status.success() {
        let state = if enable { "etkinleştirildi" } else { "devre dışı bırakıldı" };
        Ok(format!("'{}' Flatpak deposu {}.", name, state))
    } else {
        let err = String::from_utf8_lossy(&output.stderr).to_string();
        Err(if err.is_empty() {
            String::from_utf8_lossy(&output.stdout).to_string()
        } else {
            err
        })
    }
}

pub fn edit_remote(
    name: &str,
    scope: &str,
    priority: Option<i32>,
    url: Option<String>,
) -> Result<String, String> {
    let is_system = scope == "system";
    let mut cmd = if is_system {
        build_root_command("flatpak")
    } else {
        Command::new("flatpak")
    };

    cmd.arg("remote-modify");
    if is_system {
        cmd.arg("--system");
    } else {
        cmd.arg("--user");
    }

    if let Some(prio) = priority {
        cmd.arg(format!("--prio={}", prio));
    }

    if let Some(ref u) = url {
        if !u.trim().is_empty() {
            cmd.arg(format!("--url={}", u));
        }
    }

    cmd.arg(name);

    let output = cmd.output().map_err(|e| format!("Komut çalıştırılamadı: {}", e))?;
    if output.status.success() {
        Ok(format!("'{}' Flatpak deposu güncellendi.", name))
    } else {
        let err = String::from_utf8_lossy(&output.stderr).to_string();
        Err(if err.is_empty() {
            String::from_utf8_lossy(&output.stdout).to_string()
        } else {
            err
        })
    }
}

pub async fn sync_flatpak_remotes(app: tauri::AppHandle) -> Result<String, String> {
    let mut cmd = Command::new("flatpak");
    cmd.arg("update")
        .arg("--appstream")
        .arg("-y")
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());

    let mut child = cmd.spawn().map_err(|e| format!("Flatpak başlatılamadı: {}", e))?;

    let stdout = child.stdout.take();
    let stderr = child.stderr.take();

    let app_handle_out = app.clone();
    let stdout_handle = tokio::spawn(async move {
        if let Some(out) = stdout {
            let reader = BufReader::new(out);
            for line in reader.lines().flatten() {
                let _ = app_handle_out.emit("repo-sync-log", format!("[Flatpak] {}", line));
            }
        }
    });

    let app_handle_err = app.clone();
    let stderr_handle = tokio::spawn(async move {
        if let Some(err) = stderr {
            let reader = BufReader::new(err);
            for line in reader.lines().flatten() {
                let _ = app_handle_err.emit("repo-sync-log", format!("[Flatpak HATA] {}", line));
            }
        }
    });

    let status = child.wait().map_err(|e| format!("İşlem hatası: {}", e))?;
    let _ = stdout_handle.await;
    let _ = stderr_handle.await;

    if status.success() {
        let _ = app.emit("repo-sync-complete", "flatpak_success");
        Ok("Flatpak AppStream meta verileri güncellendi.".to_string())
    } else {
        let _ = app.emit("repo-sync-complete", "flatpak_failed");
        Err("Flatpak meta verileri güncellenirken hata oluştu.".to_string())
    }
}

pub async fn ping_flatpak_remote(url: String) -> Result<u64, String> {
    if url.is_empty() {
        return Err("URL adresi yok".to_string());
    }
    ping_url(&url).await
}
