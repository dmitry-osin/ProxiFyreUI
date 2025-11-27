# ProxiFyre UI

A modern desktop application for managing the [ProxiFyre](https://github.com/wiresock/proxifyre) SOCKS5 proxifier — configure proxy rules, manage application exclusions, and control the Windows service, all from a clean dark/light-themed GUI.

> **Windows only.** ProxiFyre itself is a Windows-native proxifier; this UI targets the same platform.

---

## Features

- **Proxy rules** — add, edit, and delete SOCKS5 proxy entries with named rules, per-app routing, TCP/UDP protocol selection, and optional authentication
- **Application picker** — select apps from running processes or browse for `.exe` files directly
- **Exclusions** — maintain a bypass list so chosen apps skip all proxy rules
- **Service management** — install, uninstall, start, stop, and restart the ProxiFyre Windows service
- **System tray** — minimize to tray, show/hide window, and control the service from the tray menu
- **Autostart** — optional launch with Windows via the current-user Run registry key
- **Start minimized** — optionally start hidden in the tray
- **Dark / Light theme** — persisted across sessions
- **Admin elevation** — detects non-elevated sessions and offers a one-click RunAs prompt

---

## Tech Stack

| Layer | Technology |
|---|---|
| Desktop shell | [Tauri 2](https://tauri.app) |
| Frontend | [SvelteKit 2](https://kit.svelte.dev) + [Svelte 5](https://svelte.dev) (Runes) |
| Language | TypeScript + Rust |
| Bundler | Vite |
| Package manager | pnpm |
| Installers | NSIS · MSI (Windows) |

---

## Getting Started

### Prerequisites

- [Rust](https://rustup.rs) (stable, `x86_64-pc-windows-msvc` target)
- [Node.js](https://nodejs.org) ≥ 20
- [pnpm](https://pnpm.io) ≥ 9
- [Tauri prerequisites](https://tauri.app/start/prerequisites/) for Windows (Visual Studio Build Tools / MSVC)

### Development

```powershell
pnpm install
pnpm tauri dev
```

### Production build

```powershell
pnpm tauri build
```

Installers are written to `src-tauri/target/release/bundle/`.

### Icon generation

If you replace `src-tauri/icons/icon.png` (minimum **1024 × 1024 px**), regenerate all required icon formats with:

```powershell
pnpm tauri icon src-tauri/icons/icon.png
```

---

## Configuration

On first launch go to **Settings** and provide:

1. **ProxiFyre Executable** — path to `ProxiFyre.exe`
2. **Configuration File** — path to `app-config.json` (create a default one with the button if the file doesn't exist yet)

All UI settings are stored in the Tauri app-config directory (`%APPDATA%\pro.osin.tools.proxifyre-ui\`).

---

## CI / CD

GitHub Actions workflow (`.github/workflows/release.yml`):

- **`check-build`** — runs `cargo check` + `svelte-check` + frontend build on every push/PR to `main`
- **`build-windows`** — triggered on `v*.*.*` tags; builds NSIS + MSI installers and publishes a GitHub Release

---

## License

MIT © 2025 [Dmitry Osin](mailto:d@osin.pro)
