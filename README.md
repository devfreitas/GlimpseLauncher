<div align="center">

# ⚡ Glimpse Launcher

**An ultralight, blazing-fast desktop launcher for Windows 11 — built entirely in Rust.**

[![Rust](https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![Windows 11](https://img.shields.io/badge/Windows_11-0078D4?style=for-the-badge&logo=windows11&logoColor=white)](https://www.microsoft.com/windows)
[![License: MIT](https://img.shields.io/badge/License-MIT-22c55e?style=for-the-badge)](LICENSE)
[![Version](https://img.shields.io/badge/v0.6.0-8b5cf6?style=for-the-badge&label=version)](https://github.com/devfreitas/GlimpseLauncher/releases)

<br />

<p align="center">
  <em>
    Launch any app in under <strong>50ms</strong>. Fuzzy search powered by the same engine behind the
    <a href="https://helix-editor.com/">Helix editor</a>. Zero bloat. Zero Electron.
  </em>
</p>

<br />

<p align="center">
  <a href="#-quick-start">Quick Start</a> •
  <a href="#-features">Features</a> •
  <a href="#%EF%B8%8F-usage">Usage</a> •
  <a href="#%EF%B8%8F-architecture">Architecture</a> •
  <a href="#-roadmap">Roadmap</a>
</p>

</div>

<br />

---

<br />

## ✨ Features

### 🔍 Search

- ⚡ **Ultra-fast fuzzy search** — Powered by `nucleo-matcher`, the same engine used by the Helix editor
- 📦 **UWP & Win32 app indexing** — Discovers both classic desktop and modern Store apps automatically
- 👁️ **File system watcher** — Real-time index updates via `notify` when apps are installed or removed
- 💾 **Persistent index cache** — Lightning-fast startup with serialized index via `bincode`

### 🧰 Productivity

- 🔢 **Inline calculator** — Type any math expression to get instant results (powered by `evalexpr`)
- 🌐 **Web search** — Prefix with `g ` to search Google directly from the launcher
- 🖥️ **Terminal commands** — Prefix with `> ` to execute shell commands without leaving the launcher
- ⚙️ **Quick settings** — Type `> config` to open the settings panel

### 🎨 User Interface

- 🌗 **Light & Dark themes** — Follows your system preference with a Windows 11 Fluent Design aesthetic
- 🪟 **Transparent, borderless window** — Frameless, always-on-top, with native transparency
- 🖱️ **Draggable positioning** — Click and drag the launcher window wherever you want
- 🧊 **Fluent Design** — Modern rounded corners, subtle shadows, and smooth animations

### ⚙️ System

- 🧬 **Singleton architecture** — IPC-based guard prevents multiple instances via `interprocess`
- 🚀 **Auto-start with Windows** — Optional registry-based auto-launch on login
- 🗂️ **System tray** — Background operation with quick-access tray icon and context menu
- 🦀 **Custom allocator** — Uses `mimalloc` for reduced memory footprint and faster allocations

<br />

## 🚀 Quick Start

### Option A: Download Release

1. Head to [**Releases**](https://github.com/devfreitas/GlimpseLauncher/releases)
2. Download the latest `.exe` installer
3. Run & launch with **Alt + S**

### Option B: Build from Source

```bash
# Clone the repository
git clone https://github.com/devfreitas/GlimpseLauncher.git
cd GlimpseLauncher

# Build in release mode (optimized)
cargo build --release

# Run
./target/release/glimpse_launcher.exe
```

> [!NOTE]
> Building from source requires the [Rust toolchain](https://rustup.rs/) and a Windows 11 development environment with the Windows SDK.

<br />

## ⌨️ Usage

### Hotkeys

| Hotkey | Action |
|:---|:---|
| `Alt + S` | Toggle launcher visibility |
| `↑` `↓` | Navigate through results |
| `Enter` | Launch selected app / execute command |
| `Escape` | Hide the launcher |

### Commands

| Prefix / Input | Action | Example |
|:---|:---|:---|
| *(just type)* | Fuzzy search apps | `fire` → Firefox |
| `g <query>` | Search Google | `g rust async` |
| `> <command>` | Execute terminal command | `> ping 8.8.8.8` |
| `> config` | Open settings panel | `> config` |
| *math expression* | Inline calculator | `2^10 + 3 * 7` → `1045` |

<br />

## 🏗️ Architecture

Glimpse follows a clean **modular architecture** with clear separation of concerns:

```
┌──────────────────────────────────────────────┐
│                   main.rs                    │
│          Entrypoint & orchestration           │
├──────────┬──────────────┬────────────────────┤
│  core/   │     os/      │       ui/          │
│          │              │                    │
│ indexer  │   hotkey     │   launcher UI      │
│ search   │   window     │   (egui / eframe)  │
│ config   │              │                    │
└──────────┴──────────────┴────────────────────┘
```

| Module | Responsibility |
|:---|:---|
| **`core/`** | App indexing (UWP + Win32), fuzzy search engine, configuration management |
| **`os/`** | Global hotkey registration, native window manipulation (Win32 API) |
| **`ui/`** | Full launcher interface rendered with `egui` via `eframe` |
| **`constants.rs`** | Shared constants and application-wide defaults |

<br />

## 🛠️ Tech Stack

| Crate | Purpose |
|:---|:---|
| [`eframe`](https://crates.io/crates/eframe) | GUI framework (egui backend for native rendering) |
| [`nucleo-matcher`](https://crates.io/crates/nucleo-matcher) | High-performance fuzzy matching engine |
| [`windows`](https://crates.io/crates/windows) | Official Microsoft Win32 API bindings |
| [`winapi`](https://crates.io/crates/winapi) | Additional low-level Windows API access |
| [`tray-icon`](https://crates.io/crates/tray-icon) | Cross-platform system tray support |
| [`interprocess`](https://crates.io/crates/interprocess) | IPC for singleton instance enforcement |
| [`notify`](https://crates.io/crates/notify) | File system event watcher for live re-indexing |
| [`bincode`](https://crates.io/crates/bincode) | Fast binary serialization for index cache |
| [`evalexpr`](https://crates.io/crates/evalexpr) | Math expression evaluator for inline calculator |
| [`mimalloc`](https://crates.io/crates/mimalloc) | High-performance memory allocator by Microsoft |
| [`serde`](https://crates.io/crates/serde) / [`toml`](https://crates.io/crates/toml) | Configuration serialization & deserialization |
| [`egui-phosphor`](https://crates.io/crates/egui-phosphor) | Phosphor icon set for the UI |
| [`anyhow`](https://crates.io/crates/anyhow) / [`thiserror`](https://crates.io/crates/thiserror) | Ergonomic error handling |

<br />

## 📦 Project Structure

```
glimpse_launcher/
├── src/
│   ├── main.rs              # Entrypoint, IPC guard, tray & event loop
│   ├── constants.rs          # App-wide constants & defaults
│   ├── core/
│   │   ├── mod.rs            # Module declarations
│   │   ├── indexer.rs        # UWP & Win32 app discovery + caching
│   │   ├── search.rs         # Fuzzy search via nucleo-matcher
│   │   └── config.rs         # User settings (TOML persistence)
│   ├── os/
│   │   ├── mod.rs            # Module declarations
│   │   ├── hotkey.rs         # Global hotkey (Alt+S) registration
│   │   └── window.rs         # Win32 window management & positioning
│   ├── ui/
│   │   ├── mod.rs            # Module declarations
│   │   └── ui.rs             # Full launcher UI (egui/eframe)
│   └── bin/
│       └── test_apps.rs      # Dev utility for testing app indexing
├── public/
│   ├── icon.png              # App icon (high-res)
│   └── icone.ico             # App icon (Windows ICO format)
├── installer/
│   └── Teste.iss             # Inno Setup installer script
├── build.rs                  # Build script (winres icon embedding)
├── Cargo.toml                # Dependencies & build configuration
├── Cargo.lock                # Reproducible dependency tree
├── LICENSE                   # MIT License
└── README.md                 # You are here
```

<br />

## 🗺️ Roadmap

- [ ] 🔌 Plugin system — Extend functionality with community plugins
- [ ] 🌍 Internationalization (i18n) — Multi-language UI support
- [ ] ⌨️ Configurable hotkey — Let users choose their own toggle shortcut
- [ ] 🔄 Auto-update — Built-in update checker with seamless upgrades
- [ ] 📦 MSIX packaging — Native Windows Store distribution
- [ ] 📋 Clipboard history — Search and paste from clipboard history

<br />

## 🤝 Contributing

Contributions are welcome! Whether it's bug reports, feature requests, or pull requests — all input is valued.

Please read [**CONTRIBUTING.md**](CONTRIBUTING.md) for guidelines on how to get started.

```bash
# Fork → Clone → Branch → Code → PR
git checkout -b feature/amazing-feature
cargo test
cargo clippy
git commit -m "feat: add amazing feature"
git push origin feature/amazing-feature
```

<br />

## 📄 License

This project is licensed under the **MIT License** — see the [LICENSE](LICENSE) file for details.

<br />

<div align="center">

## 👤 Author

**DevFreitas**

[![GitHub](https://img.shields.io/badge/GitHub-100000?style=for-the-badge&logo=github&logoColor=white)](https://github.com/devfreitas)

---

<sub>Built with 🦀 Rust and ❤️ by DevFreitas</sub>

</div>
