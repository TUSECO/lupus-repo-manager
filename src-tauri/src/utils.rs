use std::path::Path;
use std::process::Command;
use std::time::Instant;

/// Root yetkisi gerektiren komutlar için pkexec / sudo hazırlayıcı
pub fn build_root_command(binary: &str) -> Command {
    let is_root = unsafe { libc::geteuid() == 0 };
    if is_root {
        return Command::new(binary);
    }

    // Önce şifresiz sudo hakkı var mı dene
    let sudo_check = Command::new("sudo")
        .arg("-n")
        .arg(binary)
        .arg("--help")
        .output();

    if let Ok(ref out) = sudo_check {
        if out.status.success() {
            let mut cmd = Command::new("sudo");
            cmd.arg("-n").arg(binary);
            return cmd;
        }
    }

    // Polkit / pkexec kullan
    let mut cmd = Command::new("pkexec");
    cmd.arg(binary);
    cmd
}

/// Dizinleri kopyalar (non-root iken /var/lib/luppo/db'yi okumak için)
pub fn copy_dir_all(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> std::io::Result<()> {
    let src = src.as_ref();
    let dst = dst.as_ref();
    std::fs::create_dir_all(dst)?;
    if let Ok(entries) = std::fs::read_dir(src) {
        for entry in entries.flatten() {
            let ty = entry.file_type()?;
            let dest_path = dst.join(entry.file_name());
            if ty.is_dir() {
                copy_dir_all(entry.path(), dest_path)?;
            } else {
                let _ = std::fs::copy(entry.path(), dest_path);
            }
        }
    }
    Ok(())
}

/// URL'ye ping (HTTP HEAD/GET) atıp gecikmeyi (ms) ölçer
pub async fn ping_url(url: &str) -> Result<u64, String> {
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(5))
        .danger_accept_invalid_certs(true)
        .build()
        .map_err(|e| e.to_string())?;

    let start = Instant::now();
    let res = client.head(url).send().await;

    match res {
        Ok(resp) => {
            if resp.status().is_success() || resp.status().is_redirection() {
                Ok(start.elapsed().as_millis() as u64)
            } else {
                // HEAD başarısızsa GET dene (bazı CDN/sunucular HEAD desteklemez)
                let get_res = client.get(url).send().await;
                match get_res {
                    Ok(g_resp) if g_resp.status().is_success() || g_resp.status().is_redirection() => {
                        Ok(start.elapsed().as_millis() as u64)
                    }
                    Ok(g_resp) => Err(format!("HTTP {}", g_resp.status().as_u16())),
                    Err(e) => Err(e.to_string()),
                }
            }
        }
        Err(_) => {
            // GET ile dene
            let get_res = client.get(url).send().await;
            match get_res {
                Ok(g_resp) if g_resp.status().is_success() || g_resp.status().is_redirection() => {
                    Ok(start.elapsed().as_millis() as u64)
                }
                Ok(g_resp) => Err(format!("HTTP {}", g_resp.status().as_u16())),
                Err(e) => Err(e.to_string()),
            }
        }
    }
}

/// Luppo paket isimlerini görsel başlığa çevirir
pub fn format_display_name(name: &str) -> String {
    if name.is_empty() {
        return String::new();
    }
    name.split('-')
        .filter(|w| !w.is_empty())
        .map(|word| {
            let mut chars = word.chars();
            match chars.next() {
                None => String::new(),
                Some(first) => {
                    let upper_first: String = first.to_uppercase().collect();
                    upper_first + chars.as_str()
                }
            }
        })
        .collect::<Vec<String>>()
        .join(" ")
}
