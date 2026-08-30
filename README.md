# LupuS Repository Manager

[![License: GPL v3](https://img.shields.io/badge/License-GPLv3-blue.svg)](LICENSE)
[![Built with Tauri](https://img.shields.io/badge/Built%20with-Tauri%202.0-24C8D8.svg)](https://tauri.app)
[![Rust](https://img.shields.io/badge/Rust-2021%20Edition-DEA584.svg)](https://www.rust-lang.org)

**LupuS Repository Manager** (`lupus-repo-manager`) is a modern, responsive, and lightweight desktop application designed for managing **Luppo** package repositories and **Flatpak** remotes on LupuS Linux and compatible distributions.

Built with **Tauri 2.0** and **Rust**, it provides a seamless and secure graphical interface with non-root execution and Polkit privilege escalation.

---

## ✨ Features

- **📊 System Overview Dashboard**
  - Instant visibility into active/total Luppo repositories, package counts, and Flatpak remotes.
  - Quick action shortcuts and synchronization triggers.

- **📦 Luppo Repository Management**
  - Add, edit, remove, enable, or disable native Luppo package repositories.
  - Configure primary repo URLs and alternative mirror lists.
  - Explore and search available packages and their details within any repository.

- **📱 Flatpak Remote Management**
  - Full management of System-wide (`system`) and User-level (`user`) Flatpak remotes.
  - Set priorities, configure GPG verification, and add custom title/descriptions.
  - Browse and search applications available from configured Flatpak remotes.

- **⚡ Mirror & Remote Latency Benchmarks**
  - Test latency (ping) against main URLs and all configured mirror servers using concurrent HTTP requests.
  - Automatic categorization of latency (Fast, Normal, Slow, Offline).

- **⭐ Popular Preset Repositories**
  - One-click setup for official LupuS repositories (*Stable*, *Contrib*).
  - Built-in presets for popular Flatpak remotes (*Flathub*, *Flathub Beta*, *GNOME Nightly*, *KDE Flatpaks*).

- **💾 Backup & Restore**
  - Export complete repository and remote configurations into clean JSON files.
  - One-click restore to import backup files across different systems.

- **📝 Live Operation Logs**
  - Real-time stream of output from synchronization and repository operations.

- **🌐 Internationalization (i18n) & Theming**
  - Multi-language support (**English** and **Turkish**) with instant runtime language switching.
  - Dark, Light, and System theme modes.

- **🔒 Secure Privilege Handling**
  - Runs safely with standard user privileges and invokes `pkexec` / `sudo` via Polkit rules only when elevated actions are required.

---

## 🛠️ Tech Stack

- **Backend**: Rust (2021 Edition), [Tauri v2](https://tauri.app), `luppo-core`, `tokio`, `reqwest`, `serde`, `libc`
- **Frontend**: Vanilla JS (ES6+), HTML5, CSS3 with CSS variables for dynamic theming
- **Packaging**: Luppo package format (`.luppo`) via `lopec.xml`

---

## 🚀 Getting Started

### Prerequisites

Make sure the following dependencies are installed on your system:

- **Rust & Cargo** (>= 1.77.2)
- **Tauri CLI**: `cargo install tauri-cli --version "^2.0"`
- **WebKit2GTK** and **GTK3** development libraries
- **Luppo** (`luppo` command and `luppo-core`)
- **Flatpak** (`flatpak` binary)
- **Polkit** (`pkexec`)

On LupuS / Pisi Linux:
```bash
sudo luppo install libwebkit2gtk-devel gtk3-devel flatpak luppo
```

### Building from Source

1. Clone the repository:
   ```bash
   git clone https://github.com/Antolun/lupus-repo-manager.git
   cd lupus-repo-manager
   ```

2. Run in development mode:
   ```bash
   make run
   # or
   cd src-tauri && cargo tauri dev
   ```

3. Build the release binary:
   ```bash
   make build
   # or
   cd src-tauri && cargo build --release
   ```

4. Create a `.luppo` package:
   ```bash
   make package
   # or
   bash build-luppo.sh
   ```

---

## 📁 Project Structure

```text
repo-manager/
├── Makefile                     # Build & run helper targets
├── build-luppo.sh               # Luppo package build script
├── lopec.xml                    # Luppo package specification
├── lupus-repo-manager.desktop   # XDG desktop entry
├── polkit/                      # Polkit privilege rules
│   └── 50-lupus-repo-manager.rules
├── sudoers/                     # Sudoers configuration
│   └── 50-lupus-repo-manager
├── ui/                          # Frontend assets
│   ├── app.js                   # Application state & Tauri IPC
│   ├── index.html               # Main UI template & modals
│   ├── style.css                # Custom theme styling & variables
│   └── assets/                  # Icons and images
└── src-tauri/                   # Rust backend
    ├── Cargo.toml               # Dependencies & metadata
    └── src/
        ├── commands.rs          # Tauri IPC command handlers
        ├── flatpak_backend.rs   # Flatpak CLI integration
        ├── i18n.rs              # Localization maps (TR/EN)
        ├── lib.rs               # Library entrypoint
        ├── luppo_backend.rs     # Luppo DB & CLI backend
        ├── main.rs              # Binary main entrypoint
        ├── settings.rs          # Persistent application settings
        └── utils.rs             # Helper functions & root executor
```

---

## 👥 Authors

- **Solzic0** &lt;solzic0@yandex.com&gt;
- **Antolun** &lt;admin@antolun.com&gt;

---

## 📄 License

This project is licensed under the **GNU General Public License v3.0 (GPL-3.0)**. See the [LICENSE](LICENSE) file for more details.
