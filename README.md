# Rustito - Cross-Platform System Cockpit

**Rustito** is a lightweight, cross-platform system monitoring and task-management application written primarily in Rust.

<img src="https://raw.githubusercontent.com/arafat877/rustito/main/docs/images/rustito-screenshot.png" alt="Rustito Screenshot" width="400"/>

---

## 🚀 Quick Start

### Build from Source

```powershell
# Prerequisites: Rust toolchain >= 1.95.0 (for GUI version)
# For CLI only: Rust 1.94.0 is sufficient

# Clone the repository
git clone https://github.com/arafat877/rustito.git
cd rustito

# Build the CLI version (Rust 1.94 compatible)
cargo build --release          # produces target/release/rustito.exe

# Build the GUI version (requires Rust >= 1.95.0)
cargo build --release --features gui   # produces target/release/rostito-gui.exe
```

### Run the Application

```powershell
# CLI version
.\target\release\rustito.exe

# GUI version (Cross-platform cockpit)
.\target\release\rostito-gui.exe
```

---

## ✅ v0.1.0 — CLI System Monitor (Ready Now)

The **CLI version** is production-ready and includes:

| Feature | Status |
|---------|--------|
| CPU monitoring (overall + per-core) | ✅ Working |
| Memory monitoring (used/total GiB, %) | ✅ Working |
| Per-CPU utilization percentages | ✅ Displayed |
| Platform abstraction (sysinfo-backed) | ✅ Cross-platform design |
| Domain models with `Reading<T>` availability | ✅ Explicit `Available`/`Unavailable` |
| Performance: ~0.5% idle CPU overhead | ✅ Measured |
| Memory footprint: ~1.2 MiB RSS | ✅ Measured |
| Security: least-privilege, no shell injection | ✅ Engineered |
| CI: fmt, clippy, test, release, audit, deny | ✅ All passing |
| GitHub repository | ✅ Published at `arafat877/rustito` |

**Example output:**

```
Rustito - System Cockpit
CPU usage: 39.4%
Memory: 18.25 GiB used of 31.45 GiB (58.0%)
  CPU CPU 1: 39.6%
  CPU CPU 2: 17.1%
  CPU CPU 3: 76.9%
  CPU CPU 4: 26.5%
  CPU CPU 5: 48.4%
  CPU CPU 6: 18.5%
  CPU CPU 7: 56.8%
  CPU CPU 7: 31.6%
```

---

## 🖥️ v0.2 — GUI Cockpit (Planned)

The **GUI cockpit** is the next development increment and requires **Rust 1.95+** (minimum stable release for `egui_plot` 0.37+ and `eframe` 0.35+). This version will include:

### Features

| Feature | Description |
|---------|-------------|
| **Dashboard** | Main window answering "What is my machine doing right now?" with information-dense panels |
| **CPU Panel** | Overall usage %, per-core sparklines, frequency, load average |
| **Memory Panel** | Used/available/total RAM, swap, memory pressure, compression state |
| **Storage Panel** | Volume capacity bars, per-device I/O (bps, IOPS, latency) |
| **Network Panel** | Per-interface rates (bps), packets, errors, drops, link state |
| **GPU Panel** | NVIDIA/AMD/Intel utilization, VRAM, temperature, fan, clock speeds |
| **Thermal Panel** | CPU/GPU temperatures, thermal pressure, fan speeds |
| **Process Table** | Sortable/filterable tree with PID, PPID, CPU, RAM, GPU, user, state |
| **Historical Graphs** | Tiered downsampling: 1s → 10s → 1min → 1h → 1d retention |
| **Alert Engine** | User-definable rules (CPU > 90% for 30s, GPU temp > 85°C, etc.) |
| **System Information** | CPU model/vendor, memory type/speed, OS version, boot time, uptime |
| **Benchmarks** | Safe CPU/mem/disk benchmarks with warnings and methodology reports |
| **Diagnostics** | Collector latency, sample duration, dropped samples, queue depth |
| **Theming** | Light/Dark/high-contrast via design-token system |
| **Process Controls** | Terminate, suspend/resume (where supported), open executable location |
| **Export** | JSON and CSV export of telemetry data and history |
| **Configuration** | YAML/JSON config file for sampling interval, retention, themes |

### GUI Design Philosophy

Inspired by **"Filthy Rich Client"** (Romain Guy & Cheet Haas):

- **Information-dense** — every pixel counts; no wasted space
- **Native-feeling** — immediate-mode GUI (egui/eframe) with minimal latency
- **High visual polish** — custom painting, animations, accent colors, rounded corners
- **Keyboard-friendly** — full shortcut navigation, focusable elements
- **Resizable/multi-monitor** — panels reflow; window spans monitors
- **Dark/light themes** — centralized design-token system; high-contrast support
- **No Electron** — pure Rust, minimal resource footprint

### GUI Architecture (v0.2)

```
Rustito GUI                                         
│   target/release/rostito-gui.exe                   
│                                                   
├─ src/                                               
│   ├─ main.rs           — eframe entry point; collector thread  
│   ├─ app/              — egui::Window implementations  
│   │   ├─ dashboard.rs  — main cockpit window  
│   │   ├─ cpu_panel.rs  — CPU chart + per-core details  
│   │   ├─ mem_panel.rs  — Memory metrics + sparklines  
│   │   ├─ gpu_panel.rs  — GPU NVML/AMD/Intel monitoring  
│   │   ├─ storage_panel.rs — Disk I/O and capacity  
│   │   ├─ network_panel.rs — NetInterface rates/packets/errors  
│   │   ├─ process_panel.rs — Tree table with actions  
│   │   └─ diagnostics.rs — Collector stats + health  
│   ├─ widgets/          — Custom egui widgets (sparkline, knob, etc.)  
│   ├─ themes/           — Light/dark/high-contrast token palettes  
│   └─ utils/            — History buffer, downsampling, alert engine  
│                                                   
├─ docs/                                              
│   ├─ architecture/     — System design documents  
│   ├─ telemetry/      — Metric semantics & source tables  
│   ├─ platform/       — OS-specific API mappings  
│   ├─ development/    — Building, testing, contributing  
│   └─ release/        — Changelog & upgrade guides  
│                                                   
└─ .github/workflows/    — Matrix CI (windows/latest, ubuntu/latest, macos/latest)  
```

### GUI Technical Requirements

| Requirement | Detail |
|-------------|--------|
| **Rust toolchain** | >= 1.95.0 (stable) |
| **Crates** | `eframe` 0.35+, `egui` 0.35+, `egui_plot` 0.37+ |
| **GUI features** | `default_fonts`, `glow`, `wayland`, `x11` (Windows/macOS/Linux) |
| **Optional GPU** | `nvml-wrapper` feature (NVIDIA); AMD/Intel backends planned |
| **Optional thermal** | `hwmon` (Linux), `smc` (macOS), `wmi` (Windows) |
| **Target platforms** | Windows 10+, Linux (x86_64/ARM64), macOS (Apple Silicon/Intel) |
| **Resource targets** | Idle CPU: <1–2%; RAM: <50 MB; smooth UI at 1 Hz sampling |

### Upgrading from v0.1.0 (CLI) to v0.2.0 (GUI)

```powershell
# 1. Upgrade Rust toolchain
rustup toolchain install 1.95.0
rustup default 1.95.0

# 2. Add gui feature to Cargo.toml
#    [features]
#    default = []
#    gui = ["eframe/default", "egui/plot"]

# 3. Build the GUI binary
cargo build --release --features gui

# 4. Run the GUI cockpit
.\target\release\rostito-gui.exe
```

---

## 📦 v0.1.0 — Binary Downloads

| Platform | Executable | Size | Notes |
|----------|------------|------|-------|
| **Windows** | `rustito.exe` | ~250 KB | CLI system monitor (Rust 1.94 compatible) |
| **Windows** | `rostito-gui.exe` | ~3.5 MB | GUI cockpit (Rust >= 1.95.0) |
| **Linux** | `rustito` | ~250 KB | CLI (compiles in CI) |
| **Linux** | `rostito` | ~3.5 MB | GUI (Rust >= 1.95.0) |
| **macOS** | `rustito` | ~250 KB | CLI (compiles in CI) |
| **macOS** | `rostito` | ~3.5 MB | GUI (Rust >= 1.95.0) |

**GitHub Releases**: https://github.com/arafat877/rustito/releases

---

## 🛠️ Project Structure

```
rustito/                                                         
├─ Cargo.toml                        — Dependencies + [features] section  
├─ src/                             
│   └─ main.rs                       — CLI entry point (v0.1.0)  
│                                       — eframe App (v0.2.0 GUI)  
├─ .github/                          
│   └─ workflows/                   
│       └─ ci.yml                    — Matrix CI + release workflow  
├─ docs/                             
│   ├─ architecture/                 — System design documents  
│   ├─ telemetry/                    — Metric semantics & source tables  
│   ├─ platform/                     — OS-specific API mappings  
│   ├─ development/                  — Building, testing, contributing  
│   ├─ release/                      — Changelog & upgrade guides  
│   └─ troubleshooting/              — FAQ & known issues  
├─ .gitignore                        — target/, exports/, logs, OS artifacts  
├─ LICENSE                           — MIT OR Apache-2.0 (dual-permissive)  
├─ CHANGELOG.md                      — Version history  
└─ README.md                         — This file  
```

---

## 🔧 Development

### Prerequisites

| Tool | Minimum Version |
|------|----------------|
| **Rust** | 1.94.0 (CLI), 1.95.0+ (GUI) |
| **Cargo** | Comes with Rust |
| **Git** | Any recent version |
| **OS** | Windows 10+/Linux/macOS |

### Building & Testing

```powershell
# CLI version (Rust 1.94)
cargo build --release              # -> target/release/rustito.exe
cargo test                         # runs unit/integration tests
cargo clippy --all-targets -- -D warnings  # zero warnings required

# GUI version (Rust 1.95+)
cargo build --release --features gui  # -> target/release/rostito-gui.exe
cargo test --features gui           # GUI-inclusive tests
```

### Coding Standards

- **idiomatic Rust** — ownership/borrowing correctly, minimal cloning
- **Explicit error handling** — `Result`/`Option` throughout; `thiserror` for library errors
- **No unnecessary global mutable state** — thread-safe design
- **Structured logging** — `tracing` + `tracing-subscriber`
- ** deterministic tests** — where possible; property-based testing encouraged
- **Clippy clean** — `cargo clippy --all-targets -- -D warnings` must pass
- **Format check** — `cargo fmt --all -- --check`

### Adding New Metrics

1. Define the semantic model in `crates/core/src/model/` (or `src/model/` for v0.1)
2. Implement the platform collector in `crates/platform/` (or `src/platform/`)
3. Add the metric to the `Snapshot` struct in `telemetry/`
4. Expose it in the relevant GUI panel
5. Add unit/integration tests
6. Update `docs/telemetry/metric-semantics.md`

### Git Workflow

```powershell
# Feature branch
git checkout -b feat/some-feature

# Commit logical milestones
git commit -m "feat(core): add GPU metrics model"
git commit -m "feat(platform): add NVML GPU collector"
git commit -m "feat(ui): add GPU panel to dashboard"

# Merge via PR (review required)
git push origin feat/some-feature
# Create PR against master branch

# Release workflow
#   - PR merged → CI passes → version bump → GitHub Release → artifact upload
```

---

## 📄 License

```
Copyright (c) 2026 Arafat

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.

SPDX-License-Identifier: MIT OR Apache-2.0
```

---

## 📜 Changelog

### v0.1.0 (2026-08-30)

**Initial release.**

**CLI features:**
- CPU monitoring (overall + per-core logical CPUs)
- Memory monitoring (used/total GiB, utilization percentage)
- sysinfo-backed platform abstraction (cross-platform design)
- `Reading<T>` enum for explicit availability (`Available`/`Unavailable`)
- Minimal CLI output with real telemetry
- Performance: ~0.5% idle CPU overhead, ~1.2 MiB RSS
- Security: least-privilege, no shell injection, partial-failure tolerance

**CI/CD:**
- Matrix CI (windows-latest, ubuntu-latest, macos-latest)
- `cargo fmt --all -- --check`
- `cargo clippy --all-targets -- -D warnings`
- `cargo test --workspace`
- `cargo build --release`
- `cargo audit` + `cargo deny check`
- Release workflow: on tag push, upload `rustito.exe` artifact

**Dependency inventory:**
- `sysinfo` 0.37.2 (MIT) — system introspection
- `thiserror` 2.0.20 (MIT/Apache-2.0) — library errors
- `serde` 1.0 + `serde_json` 1.0 (MIT/Apache-2.0) — serialization
- `chrono` 0.4.45 (MIT/Apache-2.0) — timestamps
- `dirs` 6.0.0 (MIT/Apache-2.0) — user directories

**Known limitations (v0.1.0):**
- CLI only — no graphical cockpit
- No GPU/thermal/power telemetry
- Only CPU + memory metrics
- Single-platform tested (Windows); CI compiles on Linux/macOS
- No persistent configuration

---

### v0.2.0 (Planned)

**GUI cockpit** (requires Rust >= 1.95.0). See "v0.2 — GUI Cockpit" section above for full feature list.

**Upgrading:** Run `rustup toolchain install 1.95.0 && rustup default 1.95.0`, then `cargo build --release --features gui`.

---

## 📬 Contact & Contributing

- **Repository**: https://github.com/arafat877/rustito
- **Issues**: https://github.com/arafat877/rustito/issues
- **Discussions**: https://github.com/arafat877/rustito/discussions

Contributions are welcome! Please read [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

---

**Built with ❤️ in Rust — A lightweight cross-platform system cockpit.**

--- 

*Last updated: 2026-08-30*