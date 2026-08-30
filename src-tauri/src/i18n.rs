use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(dead_code)]
pub struct TranslationMap {
    pub tr: HashMap<String, String>,
    pub en: HashMap<String, String>,
}

pub fn get_translations() -> HashMap<String, HashMap<String, String>> {
    let mut root = HashMap::new();

    let mut tr = HashMap::new();
    // Navigation & App
    tr.insert("app_title".into(), "LupuS Depo Yöneticisi".into());
    tr.insert("app_subtitle".into(), "Luppo & Flatpak".into());
    tr.insert("nav_general".into(), "Genel".into());
    tr.insert("nav_luppo".into(), "Luppo".into());
    tr.insert("nav_flatpak".into(), "Flatpak".into());
    tr.insert("nav_tools".into(), "Araçlar".into());
    tr.insert("overview".into(), "Genel Bakış".into());
    tr.insert("luppo_repos".into(), "Luppo Depoları".into());
    tr.insert("flatpak_remotes".into(), "Flatpak Depoları".into());
    tr.insert("mirrors_bench".into(), "Yansımalar & Hız Testi".into());
    tr.insert("logs".into(), "İşlem Günlüğü".into());
    tr.insert("backup_restore".into(), "Yedekle & Geri Yükle".into());
    tr.insert("settings".into(), "Ayarlar".into());
    tr.insert("theme_light".into(), "Açık Tema".into());
    tr.insert("theme_dark".into(), "Koyu Tema".into());
    tr.insert("theme_auto".into(), "Sistem Teması".into());

    // Stats & Topbar
    tr.insert("total_repos".into(), "Toplam Depo".into());
    tr.insert("active_repos".into(), "Aktif Depolar".into());
    tr.insert("total_packages".into(), "Toplam Paket".into());
    tr.insert("flatpak_apps".into(), "Flatpak Depoları".into());
    tr.insert("sync_all".into(), "Tüm Depoları Güncelle".into());
    tr.insert("syncing".into(), "Güncelleniyor...".into());
    tr.insert("syncing_all".into(), "Tüm depolar senkronize ediliyor...".into());
    tr.insert("sync_completed".into(), "Depolar başarıyla güncellendi.".into());
    tr.insert("sync_failed".into(), "Depo güncellemesi sırasında hata oluştu.".into());
    tr.insert("manage_all".into(), "Tümünü Yönet →".into());
    tr.insert("preset_repos".into(), "Hazır Depolar".into());

    // Common Actions
    tr.insert("add_repo".into(), "Yeni Depo Ekle".into());
    tr.insert("add_remote".into(), "Yeni Flatpak Deposu Ekle".into());
    tr.insert("repo_name".into(), "Depo Adı".into());
    tr.insert("repo_url".into(), "Depo URL Adresi".into());
    tr.insert("mirrors".into(), "Yansıma Sunucuları".into());
    tr.insert("status".into(), "Durum".into());
    tr.insert("packages".into(), "Paketler".into());
    tr.insert("actions".into(), "İşlemler".into());
    tr.insert("enabled".into(), "Etkin".into());
    tr.insert("disabled".into(), "Devre Dışı".into());
    tr.insert("save".into(), "Kaydet".into());
    tr.insert("cancel".into(), "İptal".into());
    tr.insert("delete".into(), "Sil".into());
    tr.insert("edit".into(), "Düzenle".into());
    tr.insert("close".into(), "Kapat".into());
    tr.insert("add".into(), "Ekle".into());
    tr.insert("ping".into(), "Ping Testi".into());
    tr.insert("view_packages".into(), "Paketleri İncele".into());
    tr.insert("view_apps".into(), "Uygulamaları İncele".into());
    tr.insert("filter".into(), "Filtrele:".into());
    tr.insert("all".into(), "Tümü".into());
    tr.insert("system".into(), "Sistem".into());
    tr.insert("user".into(), "Kullanıcı".into());
    tr.insert("system_scope".into(), "Sistem Geneli (Root)".into());
    tr.insert("user_scope".into(), "Kullanıcı (User)".into());
    tr.insert("priority".into(), "Öncelik".into());
    tr.insert("comment".into(), "Açıklama".into());
    tr.insert("homepage".into(), "Ana Sayfa".into());
    tr.insert("popular_presets".into(), "Popüler Hazır Depolar".into());
    tr.insert("preset_hint".into(), "Depoya tıklayarak sisteminize ekleyebilirsiniz.".into());

    // Table Columns
    tr.insert("col_name".into(), "Ad".into());
    tr.insert("col_url".into(), "URL".into());
    tr.insert("col_packages".into(), "Paket".into());
    tr.insert("col_status".into(), "Durum".into());
    tr.insert("col_actions".into(), "İşlemler".into());
    tr.insert("col_scope".into(), "Kapsam".into());
    tr.insert("col_priority".into(), "Öncelik".into());
    tr.insert("col_version".into(), "Sürüm".into());
    tr.insert("col_size".into(), "Boyut".into());
    tr.insert("col_main_url".into(), "Ana URL".into());
    tr.insert("col_mirrors".into(), "Yansımalar".into());
    tr.insert("col_latency".into(), "Gecikme".into());
    tr.insert("ping_luppo_header".into(), "Luppo Deposu".into());
    tr.insert("ping_flatpak_header".into(), "Flatpak Deposu".into());

    // Badges & Counters
    tr.insert("badge_enabled".into(), "● Etkin".into());
    tr.insert("badge_disabled".into(), "○ Pasif".into());
    tr.insert("badge_installed".into(), "Kurulu".into());
    tr.insert("mirrors_suffix".into(), "yansıma".into());
    tr.insert("packages_suffix".into(), "paket".into());

    // Mirrors Page
    tr.insert("test_all".into(), "Tümünü Test Et".into());
    tr.insert("mirrors_desc".into(), "Her deponun ana URL'sine ve yansıma sunucularına HTTP HEAD isteği göndererek gecikme sürelerini ölçer.".into());
    tr.insert("no_repos_found".into(), "Depo bulunamadı".into());
    tr.insert("no_repos_hint".into(), "Önce Luppo veya Flatpak deposu ekleyin.".into());
    tr.insert("ms".into(), "ms".into());
    tr.insert("fast".into(), "Hızlı".into());
    tr.insert("normal".into(), "Normal".into());
    tr.insert("slow".into(), "Yavaş".into());
    tr.insert("offline".into(), "Erişilemiyor".into());

    // Logs Page
    tr.insert("clear_logs".into(), "Günlüğü Temizle".into());
    tr.insert("no_logs_yet".into(), "— Henüz işlem yapılmadı. Senkronizasyon veya depo işlemleri burada görünecek. —".into());
    tr.insert("logs_cleared".into(), "— Günlük temizlendi. —".into());

    // Backup & Restore
    tr.insert("backup_json_title".into(), "Yedekle (JSON)".into());
    tr.insert("backup_json_desc".into(), "Tüm depolar JSON dosyasına aktarılır.".into());
    tr.insert("backup_btn".into(), "Yedekle".into());
    tr.insert("restore_json_title".into(), "Geri Yükle (JSON)".into());
    tr.insert("restore_json_desc".into(), "Daha önce alınan yedeği sisteme uygular.".into());
    tr.insert("restore_btn".into(), "Geri Yükle".into());
    tr.insert("backup_preview_title".into(), "Yedek İçeriği Önizleme".into());
    tr.insert("export_success".into(), "Depolar başarıyla JSON dosyasına yedeklendi.".into());
    tr.insert("import_success".into(), "Depolar başarıyla içe aktarıldı.".into());
    tr.insert("restore_confirm_title".into(), "Yedekten Geri Yükle".into());
    tr.insert("restore_confirm_msg".into(), "\"{file}\" dosyasından tüm depolar geri yüklensin mi?".into());

    // Settings
    tr.insert("setting_theme".into(), "Tema".into());
    tr.insert("setting_lang".into(), "Dil".into());
    tr.insert("setting_flatpak_scope".into(), "Varsayılan Flatpak Kapsamı".into());
    tr.insert("setting_auto_ping".into(), "Uygulama açılışında Ping testi otomatik başlasın".into());
    tr.insert("settings_saved".into(), "Ayarlar başarıyla kaydedildi.".into());

    // Modals & Forms
    tr.insert("add_luppo_modal_title".into(), "Luppo Deposu Ekle".into());
    tr.insert("edit_luppo_modal_title".into(), "Luppo Deposunu Düzenle".into());
    tr.insert("add_flatpak_modal_title".into(), "Flatpak Deposu Ekle".into());
    tr.insert("edit_flatpak_modal_title".into(), "Flatpak Deposunu Düzenle".into());
    tr.insert("packages_modal_title".into(), "Paketler".into());
    tr.insert("confirm_title".into(), "Emin misiniz?".into());
    tr.insert("confirm_delete_title".into(), "Depoyu Sil".into());
    tr.insert("confirm_delete_fp_title".into(), "Flatpak Deposunu Sil".into());
    tr.insert("confirm_delete_msg".into(), "\"{name}\" deposunu sistemden kaldırmak istediğinize emin misiniz?".into());
    tr.insert("confirm_delete_fp_msg".into(), "\"{name}\" [{scope}] deposunu kaldırmak istediğinize emin misiniz?".into());
    tr.insert("field_repo_name".into(), "Depo Adı *".into());
    tr.insert("field_repo_name_no_req".into(), "Depo Adı".into());
    tr.insert("placeholder_repo_name".into(), "örn: stable".into());
    tr.insert("placeholder_fp_name".into(), "örn: flathub".into());
    tr.insert("field_repo_url".into(), "Depo URL Adresi *".into());
    tr.insert("field_url_no_req".into(), "URL Adresi".into());
    tr.insert("field_url_req".into(), "URL Adresi *".into());
    tr.insert("placeholder_repo_url".into(), "https://...".into());
    tr.insert("field_mirrors".into(), "Yansıma URL'leri".into());
    tr.insert("field_mirrors_hint".into(), "(her satıra bir adres)".into());
    tr.insert("field_scope".into(), "Kapsam".into());
    tr.insert("field_fp_url".into(), "URL veya .flatpakrepo Adresi *".into());
    tr.insert("placeholder_fp_url".into(), "https://dl.flathub.org/repo/flathub.flatpakrepo".into());
    tr.insert("field_fp_title".into(), "Başlık (İsteğe Bağlı)".into());
    tr.insert("placeholder_fp_title".into(), "örn: Flathub".into());
    tr.insert("field_fp_comment".into(), "Yorum / Açıklama (İsteğe Bağlı)".into());
    tr.insert("placeholder_fp_comment".into(), "Kısa bir açıklama".into());
    tr.insert("field_fp_gpg".into(), "GPG doğrulamasını etkinleştir".into());
    tr.insert("field_fp_priority".into(), "Öncelik".into());
    tr.insert("placeholder_search_pkgs".into(), "Paket ara...".into());

    // Empty States & Errors
    tr.insert("empty_luppo_title".into(), "Luppo deposu yok".into());
    tr.insert("empty_luppo_desc".into(), "Yeni bir depo ekleyin veya hazır şablonlardan seçin.".into());
    tr.insert("empty_flatpak_title".into(), "Flatpak deposu yok".into());
    tr.insert("empty_flatpak_desc".into(), "Flatpak kurulu değil veya hiç depo tanımlanmamış.".into());
    tr.insert("empty_packages_title".into(), "Paket bulunamadı".into());
    tr.insert("err_name_url_required".into(), "Depo adı ve URL adresi zorunludur.".into());
    tr.insert("err_luppo_db".into(), "Luppo veritabanına erişilemedi.".into());
    tr.insert("err_flatpak".into(), "Flatpak'e erişilemedi.".into());

    let mut en = HashMap::new();
    // Navigation & App
    en.insert("app_title".into(), "LupuS Repository Manager".into());
    en.insert("app_subtitle".into(), "Luppo & Flatpak".into());
    en.insert("nav_general".into(), "General".into());
    en.insert("nav_luppo".into(), "Luppo".into());
    en.insert("nav_flatpak".into(), "Flatpak".into());
    en.insert("nav_tools".into(), "Tools".into());
    en.insert("overview".into(), "Overview".into());
    en.insert("luppo_repos".into(), "Luppo Repositories".into());
    en.insert("flatpak_remotes".into(), "Flatpak Remotes".into());
    en.insert("mirrors_bench".into(), "Mirrors & Speed Test".into());
    en.insert("logs".into(), "Operation Logs".into());
    en.insert("backup_restore".into(), "Backup & Restore".into());
    en.insert("settings".into(), "Settings".into());
    en.insert("theme_light".into(), "Light Theme".into());
    en.insert("theme_dark".into(), "Dark Theme".into());
    en.insert("theme_auto".into(), "System Theme".into());

    // Stats & Topbar
    en.insert("total_repos".into(), "Total Repositories".into());
    en.insert("active_repos".into(), "Active Repositories".into());
    en.insert("total_packages".into(), "Total Packages".into());
    en.insert("flatpak_apps".into(), "Flatpak Remotes".into());
    en.insert("sync_all".into(), "Update All Repositories".into());
    en.insert("syncing".into(), "Syncing...".into());
    en.insert("syncing_all".into(), "Synchronizing all repositories...".into());
    en.insert("sync_completed".into(), "Repositories synchronized successfully.".into());
    en.insert("sync_failed".into(), "Failed to synchronize repositories.".into());
    en.insert("manage_all".into(), "Manage All →".into());
    en.insert("preset_repos".into(), "Preset Repositories".into());

    // Common Actions
    en.insert("add_repo".into(), "Add New Repository".into());
    en.insert("add_remote".into(), "Add Flatpak Remote".into());
    en.insert("repo_name".into(), "Repository Name".into());
    en.insert("repo_url".into(), "Repository URL".into());
    en.insert("mirrors".into(), "Mirrors".into());
    en.insert("status".into(), "Status".into());
    en.insert("packages".into(), "Packages".into());
    en.insert("actions".into(), "Actions".into());
    en.insert("enabled".into(), "Enabled".into());
    en.insert("disabled".into(), "Disabled".into());
    en.insert("save".into(), "Save".into());
    en.insert("cancel".into(), "Cancel".into());
    en.insert("delete".into(), "Delete".into());
    en.insert("edit".into(), "Edit".into());
    en.insert("close".into(), "Close".into());
    en.insert("add".into(), "Add".into());
    en.insert("ping".into(), "Ping Test".into());
    en.insert("view_packages".into(), "Explore Packages".into());
    en.insert("view_apps".into(), "Explore Apps".into());
    en.insert("filter".into(), "Filter:".into());
    en.insert("all".into(), "All".into());
    en.insert("system".into(), "System".into());
    en.insert("user".into(), "User".into());
    en.insert("system_scope".into(), "System-wide (Root)".into());
    en.insert("user_scope".into(), "User Level".into());
    en.insert("priority".into(), "Priority".into());
    en.insert("comment".into(), "Comment".into());
    en.insert("homepage".into(), "Homepage".into());
    en.insert("popular_presets".into(), "Popular Preset Repositories".into());
    en.insert("preset_hint".into(), "Click on a repository to add it to your system.".into());

    // Table Columns
    en.insert("col_name".into(), "Name".into());
    en.insert("col_url".into(), "URL".into());
    en.insert("col_packages".into(), "Packages".into());
    en.insert("col_status".into(), "Status".into());
    en.insert("col_actions".into(), "Actions".into());
    en.insert("col_scope".into(), "Scope".into());
    en.insert("col_priority".into(), "Priority".into());
    en.insert("col_version".into(), "Version".into());
    en.insert("col_size".into(), "Size".into());
    en.insert("col_main_url".into(), "Main URL".into());
    en.insert("col_mirrors".into(), "Mirrors".into());
    en.insert("col_latency".into(), "Latency".into());
    en.insert("ping_luppo_header".into(), "Luppo Repository".into());
    en.insert("ping_flatpak_header".into(), "Flatpak Remote".into());

    // Badges & Counters
    en.insert("badge_enabled".into(), "● Enabled".into());
    en.insert("badge_disabled".into(), "○ Disabled".into());
    en.insert("badge_installed".into(), "Installed".into());
    en.insert("mirrors_suffix".into(), "mirrors".into());
    en.insert("packages_suffix".into(), "packages".into());

    // Mirrors Page
    en.insert("test_all".into(), "Test All".into());
    en.insert("mirrors_desc".into(), "Measures latency by sending HTTP HEAD requests to the main URL and mirrors of each repository.".into());
    en.insert("no_repos_found".into(), "No repositories found".into());
    en.insert("no_repos_hint".into(), "Add a Luppo or Flatpak repository first.".into());
    en.insert("ms".into(), "ms".into());
    en.insert("fast".into(), "Fast".into());
    en.insert("normal".into(), "Normal".into());
    en.insert("slow".into(), "Slow".into());
    en.insert("offline".into(), "Offline".into());

    // Logs Page
    en.insert("clear_logs".into(), "Clear Logs".into());
    en.insert("no_logs_yet".into(), "— No operations yet. Sync or repo operations will appear here. —".into());
    en.insert("logs_cleared".into(), "— Logs cleared. —".into());

    // Backup & Restore
    en.insert("backup_json_title".into(), "Backup (JSON)".into());
    en.insert("backup_json_desc".into(), "All repositories will be exported to a JSON file.".into());
    en.insert("backup_btn".into(), "Backup".into());
    en.insert("restore_json_title".into(), "Restore (JSON)".into());
    en.insert("restore_json_desc".into(), "Applies a previously taken backup to the system.".into());
    en.insert("restore_btn".into(), "Restore".into());
    en.insert("backup_preview_title".into(), "Backup Content Preview".into());
    en.insert("export_success".into(), "Repositories exported successfully.".into());
    en.insert("import_success".into(), "Repositories imported successfully.".into());
    en.insert("restore_confirm_title".into(), "Restore from Backup".into());
    en.insert("restore_confirm_msg".into(), "Do you want to restore all repositories from \"{file}\"?".into());

    // Settings
    en.insert("setting_theme".into(), "Theme".into());
    en.insert("setting_lang".into(), "Language".into());
    en.insert("setting_flatpak_scope".into(), "Default Flatpak Scope".into());
    en.insert("setting_auto_ping".into(), "Automatically run Ping test on app launch".into());
    en.insert("settings_saved".into(), "Settings saved successfully.".into());

    // Modals & Forms
    en.insert("add_luppo_modal_title".into(), "Add Luppo Repository".into());
    en.insert("edit_luppo_modal_title".into(), "Edit Luppo Repository".into());
    en.insert("add_flatpak_modal_title".into(), "Add Flatpak Remote".into());
    en.insert("edit_flatpak_modal_title".into(), "Edit Flatpak Remote".into());
    en.insert("packages_modal_title".into(), "Packages".into());
    en.insert("confirm_title".into(), "Are you sure?".into());
    en.insert("confirm_delete_title".into(), "Delete Repository".into());
    en.insert("confirm_delete_fp_title".into(), "Delete Flatpak Remote".into());
    en.insert("confirm_delete_msg".into(), "Are you sure you want to remove repository \"{name}\" from the system?".into());
    en.insert("confirm_delete_fp_msg".into(), "Are you sure you want to remove remote \"{name}\" [{scope}]?".into());
    en.insert("field_repo_name".into(), "Repository Name *".into());
    en.insert("field_repo_name_no_req".into(), "Repository Name".into());
    en.insert("placeholder_repo_name".into(), "e.g., stable".into());
    en.insert("placeholder_fp_name".into(), "e.g., flathub".into());
    en.insert("field_repo_url".into(), "Repository URL *".into());
    en.insert("field_url_no_req".into(), "URL Address".into());
    en.insert("field_url_req".into(), "URL Address *".into());
    en.insert("placeholder_repo_url".into(), "https://...".into());
    en.insert("field_mirrors".into(), "Mirror URLs".into());
    en.insert("field_mirrors_hint".into(), "(one per line)".into());
    en.insert("field_scope".into(), "Scope".into());
    en.insert("field_fp_url".into(), "URL or .flatpakrepo Address *".into());
    en.insert("placeholder_fp_url".into(), "https://dl.flathub.org/repo/flathub.flatpakrepo".into());
    en.insert("field_fp_title".into(), "Title (Optional)".into());
    en.insert("placeholder_fp_title".into(), "e.g., Flathub".into());
    en.insert("field_fp_comment".into(), "Comment / Description (Optional)".into());
    en.insert("placeholder_fp_comment".into(), "A short description".into());
    en.insert("field_fp_gpg".into(), "Enable GPG verification".into());
    en.insert("field_fp_priority".into(), "Priority".into());
    en.insert("placeholder_search_pkgs".into(), "Search packages...".into());

    // Empty States & Errors
    en.insert("empty_luppo_title".into(), "No Luppo repositories".into());
    en.insert("empty_luppo_desc".into(), "Add a new repository or choose from preset templates.".into());
    en.insert("empty_flatpak_title".into(), "No Flatpak remotes".into());
    en.insert("empty_flatpak_desc".into(), "Flatpak is not installed or no remotes are defined.".into());
    en.insert("empty_packages_title".into(), "No packages found".into());
    en.insert("err_name_url_required".into(), "Repository name and URL are required.".into());
    en.insert("err_luppo_db".into(), "Could not access Luppo database.".into());
    en.insert("err_flatpak".into(), "Could not access Flatpak.".into());

    root.insert("tr".into(), tr);
    root.insert("en".into(), en);
    root
}
