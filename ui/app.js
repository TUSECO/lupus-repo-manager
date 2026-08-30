// LupuS Repo Manager — app.js
// Vanilla JS frontend — Tauri IPC entegrasyonu

const { invoke } = window.__TAURI__.core;
const { listen } = window.__TAURI__.event;

// ─── Uygulama Durumu ─────────────────────────────────────────────────────────
const State = {
  currentPage: 'overview',
  theme: 'dark',
  lang: 'tr',
  translations: {},
  luppoRepos: [],
  flatpakRemotes: [],
  flatpakFilter: 'all',
  allPackages: [],        // paket modalında kullanılır
  confirmCallback: null,
  presetFilter: 'all',   // 'luppo' | 'flatpak' | 'all'
};

// ─── Tauri IPC Yardımcıları ──────────────────────────────────────────────────
async function rpc(cmd, args = {}) {
  try {
    return await invoke(cmd, args);
  } catch (e) {
    console.error(`[rpc:${cmd}]`, e);
    throw e;
  }
}

// ─── i18n ────────────────────────────────────────────────────────────────────
function t(key, params = {}) {
  const map = State.translations[State.lang] || State.translations['tr'] || {};
  let text = map[key] !== undefined ? map[key] : key;
  if (typeof text === 'string' && params && typeof params === 'object') {
    for (const [k, v] of Object.entries(params)) {
      text = text.replace(new RegExp(`\\{${k}\\}`, 'g'), v);
    }
  }
  return text;
}

function applyTranslations() {
  document.querySelectorAll('[data-i18n]').forEach(el => {
    const key = el.getAttribute('data-i18n');
    const text = t(key);
    if (text && text !== key) el.textContent = text;
  });

  document.querySelectorAll('[data-i18n-placeholder]').forEach(el => {
    const key = el.getAttribute('data-i18n-placeholder');
    const text = t(key);
    if (text && text !== key) el.placeholder = text;
  });

  document.querySelectorAll('[data-i18n-title]').forEach(el => {
    const key = el.getAttribute('data-i18n-title');
    const text = t(key);
    if (text && text !== key) el.title = text;
  });

  document.querySelectorAll('[id^="t-"]').forEach(el => {
    const key = el.id.slice(2).replace(/-page$/, '');
    const text = t(key);
    if (text && text !== key) el.textContent = text;
  });

  // Sayfa başlığı
  const titles = {
    'overview':        t('overview'),
    'luppo-repos':     t('luppo_repos'),
    'flatpak-remotes': t('flatpak_remotes'),
    'mirrors':         t('mirrors_bench'),
    'logs':            t('logs'),
    'backup':          t('backup_restore'),
    'settings':        t('settings'),
  };
  const titleEl = document.getElementById('page-title');
  if (titleEl) titleEl.textContent = titles[State.currentPage] || State.currentPage;

  // Tema etiketi
  applyTheme(State.theme);
}

async function loadTranslations() {
  try {
    State.translations = await rpc('get_translations');
    applyTranslations();
  } catch (_) {}
}

// ─── Navigation ──────────────────────────────────────────────────────────────
const App = window.App = {

  navigateTo(pageId) {
    // hide all pages
    document.querySelectorAll('.content-area.page').forEach(p => p.classList.remove('active'));
    document.querySelectorAll('.nav-btn').forEach(b => b.classList.remove('active'));

    const page = document.getElementById(`page-${pageId}`);
    if (page) page.classList.add('active');

    const btn = document.querySelector(`.nav-btn[data-page="${pageId}"]`);
    if (btn) btn.classList.add('active');

    State.currentPage = pageId;

    // Sayfa başlığı
    const titles = {
      'overview':        t('overview'),
      'luppo-repos':     t('luppo_repos'),
      'flatpak-remotes': t('flatpak_remotes'),
      'mirrors':         t('mirrors_bench'),
      'logs':            t('logs'),
      'backup':          t('backup_restore'),
      'settings':        t('settings'),
    };
    const titleEl = document.getElementById('page-title');
    if (titleEl) titleEl.textContent = titles[pageId] || pageId;

    // Sayfa yükleme kancaları
    if (pageId === 'overview')        loadOverview();
    if (pageId === 'luppo-repos')     loadLuppoRepos();
    if (pageId === 'flatpak-remotes') loadFlatpakRemotes();
    if (pageId === 'mirrors')         loadMirrors();
    if (pageId === 'settings')        loadSettings();
  },

  // ── Theme ──────────────────────────────────────────────────────────────────
  toggleTheme() {
    State.theme = State.theme === 'dark' ? 'light' : 'dark';
    applyTheme(State.theme);
  },

  // ── Sync All ───────────────────────────────────────────────────────────────
  async syncAll() {
    const btn = document.getElementById('btn-sync-all');
    btn.disabled = true;
    addLog(t('syncing_all'), 'info');

    // Realtime log dinleyici
    const unlisten = await listen('repo-sync-log', e => {
      addLog(String(e.payload), 'info');
    });

    const complete = await listen('repo-sync-complete', e => {
      const payload = String(e.payload);
      if (payload.includes('success'))
        addLog('✓ ' + payload.replace('_success', ' ' + t('sync_completed')), 'success');
      else
        addLog('✗ ' + t('sync_failed') + ': ' + payload, 'error');
    });

    try {
      const msg = await rpc('sync_all_repos');
      showToast(msg || t('sync_completed'), 'success');
    } catch (e) {
      showToast(String(e), 'error');
    } finally {
      unlisten();
      complete();
      btn.disabled = false;
      // Listeyi yenile
      if (State.currentPage === 'overview') loadOverview();
      if (State.currentPage === 'luppo-repos') loadLuppoRepos();
      if (State.currentPage === 'flatpak-remotes') loadFlatpakRemotes();
    }
  },

  // ─── Overview ────────────────────────────────────────────────────────────────
  // ─── Luppo Repos ─────────────────────────────────────────────────────────────

  showAddLuppoRepoModal() {
    document.getElementById('luppo-name').value = '';
    document.getElementById('luppo-url').value = '';
    document.getElementById('luppo-mirrors').value = '';
    openModal('modal-add-luppo');
  },

  async addLuppoRepo() {
    const name    = document.getElementById('luppo-name').value.trim();
    const url     = document.getElementById('luppo-url').value.trim();
    const mirrors = document.getElementById('luppo-mirrors').value
      .split('\n').map(s => s.trim()).filter(Boolean);

    if (!name || !url) { showToast(t('err_name_url_required'), 'warning'); return; }

    try {
      const msg = await rpc('add_luppo_repo', { name, url, mirrors });
      showToast(msg, 'success');
      addLog(`+ [Luppo] ${name} → ${url}`, 'success');
      closeModal('modal-add-luppo');
      loadLuppoRepos();
      if (State.currentPage === 'overview') loadOverview();
    } catch (e) {
      showToast(String(e), 'error');
      addLog('✗ ' + String(e), 'error');
    }
  },

  showEditLuppoModal(name, url, mirrors) {
    document.getElementById('luppo-edit-name').value = name;
    document.getElementById('luppo-edit-name-display').value = name;
    document.getElementById('luppo-edit-url').value = url;
    document.getElementById('luppo-edit-mirrors').value = mirrors.join('\n');
    openModal('modal-edit-luppo');
  },

  async editLuppoRepo() {
    const name    = document.getElementById('luppo-edit-name').value;
    const newUrl  = document.getElementById('luppo-edit-url').value.trim();
    const mirrors = document.getElementById('luppo-edit-mirrors').value
      .split('\n').map(s => s.trim()).filter(Boolean);

    try {
      const msg = await rpc('edit_luppo_repo', { name, newUrl, mirrors });
      showToast(msg, 'success');
      addLog(`✎ [Luppo] ${name}`, 'success');
      closeModal('modal-edit-luppo');
      loadLuppoRepos();
    } catch (e) {
      showToast(String(e), 'error');
      addLog('✗ ' + String(e), 'error');
    }
  },

  confirmRemoveLuppo(name) {
    State.confirmCallback = async () => {
      try {
        const msg = await rpc('remove_luppo_repo', { name });
        showToast(msg, 'success');
        addLog(`− [Luppo] ${name}`, 'warn');
        loadLuppoRepos();
        if (State.currentPage === 'overview') loadOverview();
      } catch (e) {
        showToast(String(e), 'error');
        addLog('✗ ' + String(e), 'error');
      }
      closeModal('modal-confirm');
    };
    document.getElementById('confirm-title').textContent = t('confirm_delete_title');
    document.getElementById('confirm-msg').textContent = t('confirm_delete_msg', { name });
    document.getElementById('confirm-ok-btn').textContent = t('delete');
    document.getElementById('confirm-ok-btn').className = 'btn btn-danger';
    openModal('modal-confirm');
  },

  async toggleLuppoRepo(name, enabled) {
    try {
      const msg = await rpc('toggle_luppo_repo', { name, enabled });
      showToast(msg, 'success');
      addLog((enabled ? `✓ [Luppo] ${name} ` + t('enabled') : `✗ [Luppo] ${name} ` + t('disabled')), enabled ? 'success' : 'warn');
      loadLuppoRepos();
    } catch (e) {
      showToast(String(e), 'error');
    }
  },

  async viewLuppoPackages(repoName) {
    document.getElementById('packages-modal-title').textContent = `${repoName} — ${t('packages')}`;
    document.getElementById('packages-search').value = '';
    document.getElementById('packages-list').innerHTML = '<div class="empty-state"><div class="spinner"></div></div>';
    document.getElementById('packages-count').textContent = '';
    openModal('modal-packages');

    try {
      const pkgs = await rpc('get_luppo_repo_packages', { repoName });
      State.allPackages = pkgs;
      renderPackages(pkgs);
    } catch (e) {
      document.getElementById('packages-list').innerHTML =
        `<div class="empty-state"><h3>${escHtml(t('err_luppo_db'))}</h3><p>${escHtml(String(e))}</p></div>`;
    }
  },

  filterPackages(query) {
    const q = query.toLowerCase();
    const filtered = q
      ? State.allPackages.filter(p =>
          p.name.toLowerCase().includes(q) ||
          p.summary.toLowerCase().includes(q))
      : State.allPackages;
    renderPackages(filtered);
  },

  async syncLuppoRepos() {
    addLog(t('syncing'), 'info');
    const unlisten = await listen('repo-sync-log', e => addLog(String(e.payload), 'info'));
    try {
      const msg = await rpc('sync_luppo_repos');
      showToast(msg, 'success');
      addLog('✓ ' + t('sync_completed'), 'success');
      loadLuppoRepos();
    } catch (e) {
      showToast(String(e), 'error');
      addLog('✗ ' + String(e), 'error');
    } finally {
      unlisten();
    }
  },

  async pingLuppoRepo(name, url, mirrors) {
    const pingEl = document.getElementById(`ping-luppo-${escId(name)}`);
    if (pingEl) pingEl.innerHTML = '<span class="ping-ms ping-testing">...</span>';

    try {
      const results = await rpc('ping_luppo_repo', { url, mirrors });
      const ms = results[url];
      if (pingEl) {
        if (ms && ms.Ok !== undefined) {
          pingEl.innerHTML = pingBadge(ms.Ok);
        } else {
          pingEl.innerHTML = `<span class="ping-ms ping-offline">${t('offline')}</span>`;
        }
      }
    } catch (_) {
      if (pingEl) pingEl.innerHTML = `<span class="ping-ms ping-offline">${t('offline')}</span>`;
    }
  },

  // ─── Flatpak Remotes ──────────────────────────────────────────────────────────

  filterFlatpak(scope) {
    State.flatpakFilter = scope;
    document.querySelectorAll('[id^="fp-filter-"]').forEach(b => b.classList.remove('btn-primary'));
    document.getElementById(`fp-filter-${scope}`).classList.add('btn-primary');
    renderFlatpakRemotes(State.flatpakRemotes);
  },

  showAddFlatpakRemoteModal() {
    ['fp-name','fp-url','fp-title','fp-comment'].forEach(id => {
      document.getElementById(id).value = '';
    });
    document.getElementById('fp-gpg').checked = true;
    document.getElementById('fp-scope').value = 'system';
    openModal('modal-add-flatpak');
  },

  async addFlatpakRemote() {
    const name    = document.getElementById('fp-name').value.trim();
    const url     = document.getElementById('fp-url').value.trim();
    const scope   = document.getElementById('fp-scope').value;
    const gpgVerify = document.getElementById('fp-gpg').checked;
    const title   = document.getElementById('fp-title').value.trim() || null;
    const comment = document.getElementById('fp-comment').value.trim() || null;

    if (!name || !url) { showToast(t('err_name_url_required'), 'warning'); return; }

    try {
      const msg = await rpc('add_flatpak_remote', { name, url, scope, gpgVerify, title, comment });
      showToast(msg, 'success');
      addLog(`+ [Flatpak] ${name} [${scope}]`, 'success');
      closeModal('modal-add-flatpak');
      loadFlatpakRemotes();
      if (State.currentPage === 'overview') loadOverview();
    } catch (e) {
      showToast(String(e), 'error');
      addLog('✗ ' + String(e), 'error');
    }
  },

  showEditFlatpakModal(name, scope, url, priority) {
    document.getElementById('fp-edit-name').value = name;
    document.getElementById('fp-edit-scope').value = scope;
    document.getElementById('fp-edit-name-display').value = `${name} [${scope}]`;
    document.getElementById('fp-edit-url').value = url || '';
    document.getElementById('fp-edit-priority').value = priority || 1;
    openModal('modal-edit-flatpak');
  },

  async editFlatpakRemote() {
    const name     = document.getElementById('fp-edit-name').value;
    const scope    = document.getElementById('fp-edit-scope').value;
    const url      = document.getElementById('fp-edit-url').value.trim() || null;
    const priority = parseInt(document.getElementById('fp-edit-priority').value) || null;

    try {
      const msg = await rpc('edit_flatpak_remote', { name, scope, priority, url });
      showToast(msg, 'success');
      addLog(`✎ [Flatpak] ${name}`, 'success');
      closeModal('modal-edit-flatpak');
      loadFlatpakRemotes();
    } catch (e) {
      showToast(String(e), 'error');
      addLog('✗ ' + String(e), 'error');
    }
  },

  confirmRemoveFlatpak(name, scope) {
    State.confirmCallback = async () => {
      try {
        const msg = await rpc('remove_flatpak_remote', { name, scope });
        showToast(msg, 'success');
        addLog(`− [Flatpak] ${name} [${scope}]`, 'warn');
        loadFlatpakRemotes();
        if (State.currentPage === 'overview') loadOverview();
      } catch (e) {
        showToast(String(e), 'error');
      }
      closeModal('modal-confirm');
    };
    document.getElementById('confirm-title').textContent = t('confirm_delete_fp_title');
    document.getElementById('confirm-msg').textContent = t('confirm_delete_fp_msg', { name, scope });
    document.getElementById('confirm-ok-btn').textContent = t('delete');
    document.getElementById('confirm-ok-btn').className = 'btn btn-danger';
    openModal('modal-confirm');
  },

  async toggleFlatpakRemote(name, scope, enabled) {
    try {
      const msg = await rpc('toggle_flatpak_remote', { name, scope, enabled });
      showToast(msg, 'success');
      addLog((enabled ? `✓ [Flatpak] ${name} ` + t('enabled') : `✗ [Flatpak] ${name} ` + t('disabled')) + ' [' + scope + ']',
             enabled ? 'success' : 'warn');
      loadFlatpakRemotes();
    } catch (e) {
      showToast(String(e), 'error');
    }
  },

  async viewFlatpakApps(name, scope) {
    document.getElementById('packages-modal-title').textContent = `${name} — ${t('view_apps')}`;
    document.getElementById('packages-search').value = '';
    document.getElementById('packages-list').innerHTML = '<div class="empty-state"><div class="spinner"></div></div>';
    document.getElementById('packages-count').textContent = '';
    openModal('modal-packages');

    try {
      const apps = await rpc('get_flatpak_remote_apps', { remoteName: name, scope });
      State.allPackages = apps.map(a => ({
        name: a.name,
        display_name: a.name,
        version: a.version || '-',
        release: a.branch,
        summary: a.description,
        size: '-',
        installed: false,
        component: a.remote_name,
      }));
      renderPackages(State.allPackages);
    } catch (e) {
      document.getElementById('packages-list').innerHTML =
        `<div class="empty-state"><h3>${escHtml(t('err_flatpak'))}</h3><p>${escHtml(String(e))}</p></div>`;
    }
  },

  async pingFlatpakRemote(name, url) {
    const pingEl = document.getElementById(`ping-fp-${escId(name)}`);
    if (!url || pingEl === null) return;
    if (pingEl) pingEl.innerHTML = '<span class="ping-ms ping-testing">...</span>';

    try {
      const ms = await rpc('ping_flatpak_remote', { url });
      if (pingEl) pingEl.innerHTML = pingBadge(ms);
    } catch (_) {
      if (pingEl) pingEl.innerHTML = `<span class="ping-ms ping-offline">${t('offline')}</span>`;
    }
  },

  // ─── Mirrors page ────────────────────────────────────────────────────────────

  async pingAll() {
    // Tüm Luppo repos ve Flatpak remotes için ping başlat
    const luppoItems = document.querySelectorAll('[id^="ping-luppo-"]');
    const fpItems    = document.querySelectorAll('[id^="ping-fp-"]');

    luppoItems.forEach(el => { el.innerHTML = '<span class="ping-ms ping-testing">...</span>'; });
    fpItems.forEach(el => { el.innerHTML = '<span class="ping-ms ping-testing">...</span>'; });

    const promises = [];

    State.luppoRepos.forEach(repo => {
      promises.push(App.pingLuppoRepo(repo.name, repo.url, repo.mirrors));
    });

    State.flatpakRemotes.forEach(remote => {
      if (remote.url) promises.push(App.pingFlatpakRemote(remote.name, remote.url));
    });

    await Promise.allSettled(promises);
  },

  // ─── Presets ─────────────────────────────────────────────────────────────────

  showPresetModal(filter) {
    State.presetFilter = filter;
    renderPresets(filter);
    openModal('modal-presets');
  },

  async addPreset(preset) {
    try {
      let msg;
      if (preset.repo_type === 'luppo') {
        msg = await rpc('add_luppo_repo', {
          name: preset.name,
          url: preset.url,
          mirrors: preset.mirrors,
        });
        loadLuppoRepos();
      } else {
        msg = await rpc('add_flatpak_remote', {
          name: preset.name,
          url: preset.url,
          scope: preset.default_scope,
          gpgVerify: true,
          title: preset.title,
          comment: preset.description,
        });
        loadFlatpakRemotes();
      }
      showToast(msg, 'success');
      addLog(`+ ${preset.name}`, 'success');
      closeModal('modal-presets');
      if (State.currentPage === 'overview') loadOverview();
    } catch (e) {
      showToast(String(e), 'error');
      addLog('✗ ' + String(e), 'error');
    }
  },

  // ─── Backup/Restore ──────────────────────────────────────────────────────────

  async exportRepos() {
    try {
      const json = await rpc('export_repos_json');
      document.getElementById('backup-preview').textContent = json;

      // Dosya indirme
      const blob = new Blob([json], { type: 'application/json' });
      const url  = URL.createObjectURL(blob);
      const a    = document.createElement('a');
      a.href     = url;
      a.download = `lupus-repos-backup-${new Date().toISOString().slice(0,10)}.json`;
      a.click();
      URL.revokeObjectURL(url);

      showToast(t('export_success'), 'success');
      addLog('✓ ' + t('export_success'), 'success');
    } catch (e) {
      showToast(String(e), 'error');
    }
  },

  importRepos() {
    document.getElementById('file-import').click();
  },

  async handleImportFile(input) {
    const file = input.files[0];
    if (!file) return;

    const text = await file.text();
    document.getElementById('backup-preview').textContent = text;

    State.confirmCallback = async () => {
      try {
        const msg = await rpc('import_repos_json', { jsonContent: text });
        showToast(msg, 'success');
        addLog('✓ ' + t('import_success'), 'success');
        loadOverview();
      } catch (e) {
        showToast(String(e), 'error');
      }
      closeModal('modal-confirm');
    };

    document.getElementById('confirm-title').textContent = t('restore_confirm_title');
    document.getElementById('confirm-msg').textContent = t('restore_confirm_msg', { file: file.name });
    document.getElementById('confirm-ok-btn').textContent = t('restore_btn');
    document.getElementById('confirm-ok-btn').className = 'btn btn-primary';
    openModal('modal-confirm');

    input.value = '';
  },

  // ─── Settings ────────────────────────────────────────────────────────────────

  async saveSettings() {
    const settings = {
      theme:                 document.getElementById('setting-theme').value,
      language:              document.getElementById('setting-language').value,
      default_flatpak_scope: document.getElementById('setting-flatpak-scope').value,
      auto_ping_on_load:     document.getElementById('setting-auto-ping').checked,
    };
    try {
      await rpc('save_settings', { settings });
      State.theme = settings.theme;
      State.lang  = settings.language;
      applyTheme(State.theme);
      applyTranslations();
      showToast(t('settings_saved'), 'success');
      // Mevcut sayfayı yeniden çiz
      App.navigateTo(State.currentPage);
    } catch (e) {
      showToast(String(e), 'error');
    }
  },

  // ─── Logs ────────────────────────────────────────────────────────────────────

  clearLogs() {
    document.getElementById('log-area').innerHTML =
      `<span class="log-entry text-muted">${escHtml(t('logs_cleared'))}</span>`;
  },

  // ─── Confirm ─────────────────────────────────────────────────────────────────

  confirmOk() {
    if (State.confirmCallback) State.confirmCallback();
  },

  // ─── Modal helpers ───────────────────────────────────────────────────────────

  closeModal: (id) => closeModal(id),
};

// ─── Sayfa Yükleme Fonksiyonları ─────────────────────────────────────────────

async function loadOverview() {
  try {
    const info = await rpc('get_system_overview');
    document.getElementById('stat-luppo-repos').textContent   = info.luppo_total_repos;
    document.getElementById('stat-active-repos').textContent  = info.luppo_active_repos;
    document.getElementById('stat-total-packages').textContent = info.luppo_total_packages;
    document.getElementById('stat-flatpak-remotes').textContent = info.flatpak_total_remotes;
  } catch (_) {}

  // Mini Luppo listesi
  try {
    const repos = await rpc('get_luppo_repos');
    State.luppoRepos = repos;
    document.getElementById('overview-luppo-list').innerHTML = renderLuppoTable(repos.slice(0, 5), true);
  } catch (_) {
    document.getElementById('overview-luppo-list').innerHTML =
      `<div class="empty-state"><p>${escHtml(t('err_luppo_db'))}</p></div>`;
  }

  // Mini Flatpak listesi
  try {
    const remotes = await rpc('get_flatpak_remotes');
    State.flatpakRemotes = remotes;
    document.getElementById('overview-flatpak-list').innerHTML = renderFlatpakTable(remotes.slice(0, 5), true);
  } catch (_) {
    document.getElementById('overview-flatpak-list').innerHTML =
      `<div class="empty-state"><p>${escHtml(t('err_flatpak'))}</p></div>`;
  }
}

async function loadLuppoRepos() {
  document.getElementById('luppo-repo-list').innerHTML =
    '<div class="empty-state"><div class="spinner"></div></div>';
  try {
    const repos = await rpc('get_luppo_repos');
    State.luppoRepos = repos;
    document.getElementById('luppo-repo-list').innerHTML = renderLuppoTable(repos, false);
  } catch (e) {
    document.getElementById('luppo-repo-list').innerHTML =
      `<div class="empty-state"><h3>${escHtml(t('err_luppo_db'))}</h3><p>${escHtml(String(e))}</p></div>`;
  }
}

async function loadFlatpakRemotes() {
  document.getElementById('flatpak-remote-list').innerHTML =
    '<div class="empty-state"><div class="spinner"></div></div>';
  try {
    const remotes = await rpc('get_flatpak_remotes');
    State.flatpakRemotes = remotes;
    renderFlatpakRemotes(remotes);
  } catch (e) {
    document.getElementById('flatpak-remote-list').innerHTML =
      `<div class="empty-state"><h3>${escHtml(t('err_flatpak'))}</h3><p>${escHtml(String(e))}</p></div>`;
  }
}

async function loadMirrors() {
  const container = document.getElementById('mirrors-list');
  if (State.luppoRepos.length === 0 && State.flatpakRemotes.length === 0) {
    try {
      State.luppoRepos    = await rpc('get_luppo_repos');
      State.flatpakRemotes = await rpc('get_flatpak_remotes');
    } catch (_) {}
  }

  if (State.luppoRepos.length === 0 && State.flatpakRemotes.length === 0) {
    container.innerHTML = `<div class="empty-state"><h3>${escHtml(t('no_repos_found'))}</h3><p>${escHtml(t('no_repos_hint'))}</p></div>`;
    return;
  }

  let html = '';

  if (State.luppoRepos.length > 0) {
    html += `<div class="repo-row" style="background:var(--sidebar-bg);font-size:11px;font-weight:700;text-transform:uppercase;letter-spacing:.6px;color:var(--text-secondary);">
      <span>${escHtml(t('ping_luppo_header'))}</span><span>${escHtml(t('col_main_url'))}</span><span>${escHtml(t('col_mirrors'))}</span><span>${escHtml(t('col_latency'))}</span><span></span>
    </div>`;

    State.luppoRepos.forEach(repo => {
      const mirrorCount = repo.mirrors.length;
      html += `<div class="repo-row" style="grid-template-columns:1.5fr 2.5fr 100px 120px 80px;gap:12px;">
        <span class="repo-name">${escHtml(repo.name)}</span>
        <span class="repo-url">${escHtml(repo.url)}</span>
        <span class="text-sm text-muted">${mirrorCount} ${t('mirrors_suffix')}</span>
        <span id="ping-luppo-${escId(repo.name)}">—</span>
        <span>
          <button class="btn btn-icon" title="${escAttr(t('action_ping'))}" onclick="App.pingLuppoRepo('${escAttr(repo.name)}','${escAttr(repo.url)}',${JSON.stringify(repo.mirrors)})">
            <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
              <path d="M13 2L3 14h9l-1 8 10-12h-9l1-8z"/>
            </svg>
          </button>
        </span>
      </div>`;
    });
  }

  if (State.flatpakRemotes.length > 0) {
    html += `<div class="repo-row" style="background:var(--sidebar-bg);font-size:11px;font-weight:700;text-transform:uppercase;letter-spacing:.6px;color:var(--text-secondary);">
      <span>${escHtml(t('ping_flatpak_header'))}</span><span>${escHtml(t('col_main_url'))}</span><span>${escHtml(t('col_scope'))}</span><span>${escHtml(t('col_latency'))}</span><span></span>
    </div>`;

    State.flatpakRemotes.forEach(remote => {
      if (!remote.url) return;
      html += `<div class="repo-row" style="grid-template-columns:1.5fr 2.5fr 100px 120px 80px;gap:12px;">
        <span class="repo-name">${escHtml(remote.name)}</span>
        <span class="repo-url">${escHtml(remote.url)}</span>
        <span class="badge ${remote.scope === 'system' ? 'badge-system' : 'badge-user'}">${escHtml(remote.scope === 'system' ? t('system') : t('user'))}</span>
        <span id="ping-fp-${escId(remote.name)}">—</span>
        <span>
          <button class="btn btn-icon" title="${escAttr(t('action_ping'))}" onclick="App.pingFlatpakRemote('${escAttr(remote.name)}','${escAttr(remote.url)}')">
            <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
              <path d="M13 2L3 14h9l-1 8 10-12h-9l1-8z"/>
            </svg>
          </button>
        </span>
      </div>`;
    });
  }

  container.innerHTML = html || `<div class="empty-state"><h3>${escHtml(t('no_repos_found'))}</h3></div>`;
}

async function loadSettings() {
  try {
    const s = await rpc('load_settings');
    document.getElementById('setting-theme').value          = s.theme || 'dark';
    document.getElementById('setting-language').value       = s.language || 'tr';
    document.getElementById('setting-flatpak-scope').value  = s.default_flatpak_scope || 'system';
    document.getElementById('setting-auto-ping').checked    = s.auto_ping_on_load ?? true;
  } catch (_) {}
}

// ─── Render Helpers ───────────────────────────────────────────────────────────

function renderLuppoTable(repos, compact) {
  if (repos.length === 0) {
    return `<div class="empty-state">
      <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
        <path d="M3 7h18M3 12h18M3 17h18"/>
      </svg>
      <h3>${escHtml(t('empty_luppo_title'))}</h3>
      <p>${escHtml(t('empty_luppo_desc'))}</p>
    </div>`;
  }

  const colsCls = compact
    ? '2fr 3fr 80px 100px'
    : '1.8fr 2.8fr 80px 100px 120px';

  let html = `<div class="repo-table-header" style="--table-cols:${colsCls};grid-template-columns:${colsCls}">
    <span>${escHtml(t('col_name'))}</span><span>${escHtml(t('col_url'))}</span><span>${escHtml(t('col_packages'))}</span><span>${escHtml(t('col_status'))}</span>
    ${!compact ? `<span style="text-align:right">${escHtml(t('col_actions'))}</span>` : ''}
  </div>`;

  repos.forEach(repo => {
    const badge = repo.enabled
      ? `<span class="badge badge-enabled">${escHtml(t('badge_enabled'))}</span>`
      : `<span class="badge badge-disabled">${escHtml(t('badge_disabled'))}</span>`;

    const actions = compact ? '' : `
      <div class="row-actions">
        <span id="ping-luppo-${escId(repo.name)}"></span>
        <button class="btn btn-icon" title="${escAttr(t('action_ping'))}" onclick="App.pingLuppoRepo('${escAttr(repo.name)}','${escAttr(repo.url)}',${JSON.stringify(repo.mirrors)})">
          <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <path d="M13 2L3 14h9l-1 8 10-12h-9l1-8z"/>
          </svg>
        </button>
        <button class="btn btn-icon" title="${escAttr(t('action_view_packages'))}" onclick="App.viewLuppoPackages('${escAttr(repo.name)}')">
          <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <path d="M21 16V8a2 2 0 0 0-1-1.73l-7-4a2 2 0 0 0-2 0l-7 4A2 2 0 0 0 3 8v8a2 2 0 0 0 1 1.73l7 4a2 2 0 0 0 2 0l7-4A2 2 0 0 0 21 16z"/>
          </svg>
        </button>
        <button class="btn btn-icon" title="${escAttr(repo.enabled ? t('action_disable') : t('action_enable'))}"
          onclick="App.toggleLuppoRepo('${escAttr(repo.name)}', ${!repo.enabled})">
          <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            ${repo.enabled
              ? '<circle cx="12" cy="12" r="10"/><path d="M10 15l5-3-5-3v6z"/>'
              : '<circle cx="12" cy="12" r="10"/><path d="M9 15V9M15 15V9"/>' }
          </svg>
        </button>
        <button class="btn btn-icon" title="${escAttr(t('action_edit'))}"
          onclick="App.showEditLuppoModal('${escAttr(repo.name)}','${escAttr(repo.url)}',${JSON.stringify(repo.mirrors)})">
          <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <path d="M11 4H4a2 2 0 0 0-2 2v14a2 2 0 0 0 2 2h14a2 2 0 0 0 2-2v-7"/>
            <path d="M18.5 2.5a2.12 2.12 0 0 1 3 3L12 15l-4 1 1-4z"/>
          </svg>
        </button>
        <button class="btn btn-icon" title="${escAttr(t('action_delete'))}" style="color:var(--danger)"
          onclick="App.confirmRemoveLuppo('${escAttr(repo.name)}')">
          <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <polyline points="3 6 5 6 21 6"/><path d="M19 6l-1 14H6L5 6M10 11v6M14 11v6M9 6V4h6v2"/>
          </svg>
        </button>
      </div>`;

    html += `<div class="repo-row" style="--table-cols:${colsCls};grid-template-columns:${colsCls}">
      <span class="repo-name">${escHtml(repo.name)}</span>
      <span class="repo-url" title="${escAttr(repo.url)}">${escHtml(repo.url)}</span>
      <span class="repo-count">${repo.package_count > 0 ? repo.package_count : '—'}</span>
      <span>${badge}</span>
      ${actions}
    </div>`;
  });

  return html;
}

function renderFlatpakRemotes(remotes) {
  const filtered = State.flatpakFilter === 'all'
    ? remotes
    : remotes.filter(r => r.scope === State.flatpakFilter);

  document.getElementById('flatpak-remote-list').innerHTML = renderFlatpakTable(filtered, false);
}

function renderFlatpakTable(remotes, compact) {
  if (remotes.length === 0) {
    return `<div class="empty-state">
      <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
        <circle cx="12" cy="12" r="3"/><path d="M12 1v3M12 20v3M4.22 4.22l2.12 2.12M17.66 17.66l2.12 2.12M1 12h3M20 12h3"/>
      </svg>
      <h3>${escHtml(t('empty_flatpak_title'))}</h3>
      <p>${escHtml(t('empty_flatpak_desc'))}</p>
    </div>`;
  }

  const colsCls = compact
    ? '1.5fr 2.5fr 80px 100px'
    : '1.5fr 2.2fr 80px 90px 100px 140px';

  let html = `<div class="repo-table-header" style="--table-cols:${colsCls};grid-template-columns:${colsCls}">
    <span>${escHtml(t('col_name'))}</span><span>${escHtml(t('col_url'))}</span>
    ${compact ? '' : `<span>${escHtml(t('col_scope'))}</span><span>${escHtml(t('col_priority'))}</span>`}
    <span>${escHtml(t('col_status'))}</span>
    ${!compact ? `<span style="text-align:right">${escHtml(t('col_actions'))}</span>` : ''}
  </div>`;

  remotes.forEach(remote => {
    const badge = remote.enabled
      ? `<span class="badge badge-enabled">${escHtml(t('badge_enabled'))}</span>`
      : `<span class="badge badge-disabled">${escHtml(t('badge_disabled'))}</span>`;

    const scopeBadge = `<span class="badge ${remote.scope === 'system' ? 'badge-system' : 'badge-user'}">${escHtml(remote.scope === 'system' ? t('system') : t('user'))}</span>`;

    const actions = compact ? '' : `
      <div class="row-actions">
        <span id="ping-fp-${escId(remote.name)}"></span>
        <button class="btn btn-icon" title="${escAttr(t('action_ping'))}" onclick="App.pingFlatpakRemote('${escAttr(remote.name)}','${escAttr(remote.url)}')">
          <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <path d="M13 2L3 14h9l-1 8 10-12h-9l1-8z"/>
          </svg>
        </button>
        <button class="btn btn-icon" title="${escAttr(t('action_view_apps'))}"
          onclick="App.viewFlatpakApps('${escAttr(remote.name)}','${escAttr(remote.scope)}')">
          <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <rect x="2" y="3" width="20" height="14" rx="2"/>
            <path d="M8 21h8M12 17v4"/>
          </svg>
        </button>
        <button class="btn btn-icon" title="${escAttr(remote.enabled ? t('action_disable') : t('action_enable'))}"
          onclick="App.toggleFlatpakRemote('${escAttr(remote.name)}','${escAttr(remote.scope)}',${!remote.enabled})">
          <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            ${remote.enabled
              ? '<circle cx="12" cy="12" r="10"/><path d="M9 15V9M15 15V9"/>'
              : '<circle cx="12" cy="12" r="10"/><path d="M10 15l5-3-5-3v6z"/>'}
          </svg>
        </button>
        <button class="btn btn-icon" title="${escAttr(t('action_edit'))}"
          onclick="App.showEditFlatpakModal('${escAttr(remote.name)}','${escAttr(remote.scope)}','${escAttr(remote.url)}',${remote.priority})">
          <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <path d="M11 4H4a2 2 0 0 0-2 2v14a2 2 0 0 0 2 2h14a2 2 0 0 0 2-2v-7"/>
            <path d="M18.5 2.5a2.12 2.12 0 0 1 3 3L12 15l-4 1 1-4z"/>
          </svg>
        </button>
        <button class="btn btn-icon" title="${escAttr(t('action_delete'))}" style="color:var(--danger)"
          onclick="App.confirmRemoveFlatpak('${escAttr(remote.name)}','${escAttr(remote.scope)}')">
          <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <polyline points="3 6 5 6 21 6"/><path d="M19 6l-1 14H6L5 6M10 11v6M14 11v6M9 6V4h6v2"/>
          </svg>
        </button>
      </div>`;

    html += `<div class="repo-row" style="--table-cols:${colsCls};grid-template-columns:${colsCls}">
      <span class="repo-name">${escHtml(remote.name)} ${remote.title && remote.title !== remote.name ? `<span class="text-xs text-muted">(${escHtml(remote.title)})</span>` : ''}</span>
      <span class="repo-url" title="${escAttr(remote.url)}">${escHtml(remote.url || '—')}</span>
      ${compact ? '' : `<span>${scopeBadge}</span><span class="text-sm text-muted">${remote.priority}</span>`}
      <span>${badge}</span>
      ${actions}
    </div>`;
  });

  return html;
}

function renderPackages(pkgs) {
  if (pkgs.length === 0) {
    document.getElementById('packages-list').innerHTML =
      `<div class="empty-state"><h3>${escHtml(t('empty_packages_title'))}</h3></div>`;
    document.getElementById('packages-count').textContent = `0 ${t('packages_suffix')}`;
    return;
  }

  document.getElementById('packages-count').textContent = `${pkgs.length} ${t('packages_suffix')}`;

  let html = `<div class="pkg-row pkg-header">
    <span>${escHtml(t('col_name'))}</span><span>${escHtml(t('col_version'))}</span><span>${escHtml(t('col_size'))}</span><span>${escHtml(t('col_status'))}</span>
  </div>`;

  pkgs.forEach(p => {
    const installedBadge = p.installed
      ? `<span class="badge badge-enabled" style="font-size:10px;">${escHtml(t('badge_installed'))}</span>`
      : '';
    html += `<div class="pkg-row">
      <span>
        <span class="pkg-name">${escHtml(p.display_name || p.name)}</span>
        <span class="text-xs text-muted" style="margin-left:6px;">${escHtml(p.name)}</span>
      </span>
      <span class="pkg-version">${escHtml(p.version)}</span>
      <span class="pkg-size">${escHtml(p.size)}</span>
      <span>${installedBadge}</span>
    </div>`;
  });

  document.getElementById('packages-list').innerHTML = html;
}

async function renderPresets(filter) {
  let presets = [];
  try { presets = await rpc('get_popular_presets'); } catch (_) { return; }

  const filtered = filter === 'all' ? presets : presets.filter(p => p.repo_type === filter);

  document.getElementById('preset-grid').innerHTML = filtered.map(p => `
    <div class="preset-card" onclick="App.addPreset(${JSON.stringify(p).replace(/"/g, '&quot;')})">
      <div class="preset-title">${escHtml(p.title)}</div>
      <div class="preset-desc">${escHtml(p.description)}</div>
      <div class="preset-type">
        <span class="badge ${p.repo_type === 'luppo' ? 'badge-enabled' : 'badge-system'}">
          ${p.repo_type === 'luppo' ? 'Luppo' : 'Flatpak'}
        </span>
        <span class="badge badge-user" style="margin-left:4px;">${p.default_scope}</span>
      </div>
    </div>`).join('');
}

// ─── Ping Badge Helper ────────────────────────────────────────────────────────

function pingBadge(ms) {
  if (ms === null || ms === undefined) return '—';
  if (ms < 100)  return `<span class="ping-ms ping-fast">${ms} ${t('ms')}</span>`;
  if (ms < 300)  return `<span class="ping-ms ping-normal">${ms} ${t('ms')}</span>`;
  return               `<span class="ping-ms ping-slow">${ms} ${t('ms')}</span>`;
}

// ─── Log ─────────────────────────────────────────────────────────────────────

function addLog(msg, type = 'info') {
  const logEl = document.getElementById('log-area');
  const now   = new Date().toLocaleTimeString(State.lang === 'tr' ? 'tr-TR' : 'en-US', { hour: '2-digit', minute: '2-digit', second: '2-digit' });
  const line  = document.createElement('div');
  line.className = `log-entry ${type}`;
  line.textContent = `[${now}] ${msg}`;
  logEl.appendChild(line);
  logEl.scrollTop = logEl.scrollHeight;
}

// ─── Toast ───────────────────────────────────────────────────────────────────

function showToast(msg, type = 'info', duration = 3500) {
  const container = document.getElementById('toast-container');
  const toast = document.createElement('div');
  toast.className = `toast toast-${type}`;

  const icons = {
    success: '✓', error: '✗', warning: '⚠', info: 'ℹ',
  };
  toast.innerHTML = `<span style="font-size:16px">${icons[type] || 'ℹ'}</span><span>${escHtml(msg)}</span>`;
  container.appendChild(toast);

  setTimeout(() => {
    toast.style.opacity = '0';
    toast.style.transform = 'translateX(40px)';
    toast.style.transition = 'opacity 0.3s, transform 0.3s';
    setTimeout(() => toast.remove(), 300);
  }, duration);
}

// ─── Modal ───────────────────────────────────────────────────────────────────

function openModal(id) {
  const el = document.getElementById(id);
  if (el) el.classList.remove('hidden');
}

function closeModal(id) {
  const el = document.getElementById(id);
  if (el) el.classList.add('hidden');

  // Confirm butonunu varsayılana döndür
  if (id === 'modal-confirm') {
    State.confirmCallback = null;
    document.getElementById('confirm-ok-btn').textContent = t('delete');
    document.getElementById('confirm-ok-btn').className = 'btn btn-danger';
  }
}

// Overlay tıklaması ile modalı kapat
document.querySelectorAll('.modal-overlay').forEach(overlay => {
  overlay.addEventListener('click', e => {
    if (e.target === overlay) {
      overlay.classList.add('hidden');
    }
  });
});

// ─── Theme ───────────────────────────────────────────────────────────────────

function applyTheme(theme) {
  const isDark = theme === 'dark' || (theme === 'auto' && window.matchMedia('(prefers-color-scheme: dark)').matches);
  document.documentElement.setAttribute('data-theme', isDark ? 'dark' : 'light');

  const icon  = document.getElementById('theme-icon');
  const label = document.getElementById('theme-label');
  if (isDark) {
    if (icon) icon.innerHTML = '<circle cx="12" cy="12" r="5"/><path d="M12 1v2M12 21v2M4.22 4.22l1.42 1.42M18.36 18.36l1.42 1.42M1 12h2M21 12h2M4.22 19.78l1.42-1.42M18.36 5.64l1.42-1.42"/>';
    if (label) label.textContent = t('theme_light');
  } else {
    if (icon) icon.innerHTML = '<path d="M21 12.79A9 9 0 1 1 11.21 3 7 7 0 0 0 21 12.79z"/>';
    if (label) label.textContent = t('theme_dark');
  }
}

// ─── String Helpers ───────────────────────────────────────────────────────────

function escHtml(str) {
  return String(str || '')
    .replace(/&/g, '&amp;')
    .replace(/</g, '&lt;')
    .replace(/>/g, '&gt;')
    .replace(/"/g, '&quot;');
}

function escAttr(str) {
  return String(str || '').replace(/'/g, "\\'").replace(/"/g, '\\"');
}

function escId(str) {
  return String(str || '').replace(/[^a-z0-9-_]/gi, '-');
}

// ─── Navigation Binding ───────────────────────────────────────────────────────

document.querySelectorAll('.nav-btn[data-page]').forEach(btn => {
  btn.addEventListener('click', () => App.navigateTo(btn.dataset.page));
});

// ─── Init ────────────────────────────────────────────────────────────────────

(async function init() {
  try {
    const settings = await rpc('load_settings');
    State.theme = settings.theme || 'dark';
    State.lang  = settings.language || 'tr';

    await loadTranslations();
    applyTheme(State.theme);

    App.navigateTo('overview');

    // Realtime sync log dinleyici (her zaman aktif)
    listen('repo-sync-log', e => addLog(String(e.payload), 'info'));
    listen('repo-sync-complete', e => {
      const p = String(e.payload);
      addLog(p.includes('success')
        ? '✓ ' + t('sync_completed') + ': ' + p
        : '✗ ' + t('sync_failed') + ': ' + p,
        p.includes('success') ? 'success' : 'error');
    });

    // Auto ping ayarına göre
    if (settings.auto_ping_on_load) {
      // Kısa gecikme sonra ping başlat
      setTimeout(() => {
        if (State.luppoRepos.length > 0 || State.flatpakRemotes.length > 0) {
          App.pingAll();
        }
      }, 3000);
    }
  } catch (e) {
    console.error('Init hatası:', e);
    showToast(String(e), 'error');
  }
})();
