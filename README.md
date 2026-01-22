# RepoCard Studio KZ

**Generate polished social cards, README snippets, and press kits for your GitHub repos — in 60 seconds.**

Created by **LAZYFROG (KZ)** — [kindware.dev](https://kindware.dev)

[![CI](https://github.com/Brutus1066/Repocard-Studio-KZ/actions/workflows/ci.yml/badge.svg)](https://github.com/Brutus1066/Repocard-Studio-KZ/actions/workflows/ci.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![GitHub release](https://img.shields.io/github/v/release/Brutus1066/Repocard-Studio-KZ)](https://github.com/Brutus1066/Repocard-Studio-KZ/releases)

---

## 🚀 60-Second Quickstart

1. **Download** → Grab the latest installer from [Releases](https://github.com/Brutus1066/Repocard-Studio-KZ/releases)
2. **Paste URL** → Enter any public GitHub repo URL
3. **Export** → Click "Export Share Kit" and get everything you need

That's it. No account. No cloud. No tracking.

---

## 📸 Screenshots

<p align="center">
  <img src="docs/screenshots/screenshot-1.png" alt="RepoCard Studio - Empty State" width="800"/>
</p>
<p align="center"><em>Clean interface ready to fetch any GitHub repository</em></p>

<p align="center">
  <img src="docs/screenshots/screenshot-2.png" alt="RepoCard Studio - Minimal Template" width="800"/>
</p>
<p align="center"><em>Minimal template with light, professional styling</em></p>

<p align="center">
  <img src="docs/screenshots/screenshot-3.png" alt="RepoCard Studio - Gradient Template" width="800"/>
</p>
<p align="center"><em>Modern gradient template with customizable colors</em></p>

---

## ✨ Features

- **3 Beautiful Templates** — Modern (dark), Minimal (light), Gradient (customizable)
- **Social Card Export** — SVG + PNG at optimal 1200×630 resolution
- **README Snippets** — Copy-paste markdown sections
- **Release Notes Draft** — Auto-generated from recent commits
- **Press Kit** — Complete folder with all assets organized

---

## 🔒 No Cloud. No Tracking.

RepoCard Studio is a **local-first desktop app**:

| Promise | Status |
|---------|--------|
| No telemetry | ✅ |
| No analytics | ✅ |
| No account required | ✅ |
| No data leaves your machine | ✅ |
| Works offline (after initial fetch) | ✅ |

Your repos, your files, your control. See [SECURITY.md](SECURITY.md) for details.

---

## 🤔 Why This Exists

Promoting your open source project shouldn't require design skills or expensive tools. 

I built RepoCard Studio because every time I released something, I spent more time making social cards than writing code. Now it takes 60 seconds.

**Who it's for:**
- Solo developers releasing side projects
- Open source maintainers who want better visibility
- Anyone who wants a polished GitHub presence without the hassle

---

## 📥 Download

| Platform | Download |
|----------|----------|
| Windows (NSIS Installer) | [Releases Page](https://github.com/Brutus1066/Repocard-Studio-KZ/releases/latest) |
| Windows (MSI Installer) | [Releases Page](https://github.com/Brutus1066/Repocard-Studio-KZ/releases/latest) |

> macOS and Linux builds coming if there's demand. Open an issue!

---

## 📦 Export Structure

```
share-kit/
├── repo-card.svg           # Editable vector
├── repo-card.png           # Social-ready (1200×630)
├── README-snippet.md       # Copy-paste markdown
├── release-notes-draft.md  # From recent commits
└── press-kit/
    └── overview.md         # Press kit document
```

---

## 🛠️ Development

### Prerequisites

- [Node.js 18+](https://nodejs.org/)
- [Rust](https://www.rust-lang.org/tools/install) (latest stable)
- [Tauri Prerequisites](https://tauri.app/start/prerequisites/)

### Setup

```bash
git clone https://github.com/Brutus1066/Repocard-Studio-KZ.git
cd Repocard-Studio-KZ
npm install
npm run tauri dev
```

### Testing

```bash
npm test              # Frontend tests
npm run lint          # TypeScript check
cargo test --manifest-path src-tauri/Cargo.toml  # Rust tests
```

### Build

```bash
npm run tauri build
# Output: src-tauri/target/release/bundle/
```

---

## 🎨 Share Your Card!

Made something cool? Share it!

→ [Post in GitHub Discussions](https://github.com/Brutus1066/Repocard-Studio-KZ/discussions) with the tag **#show-your-card**

---

## 🏷️ Attribution

Generated assets include optional attribution:

> *Generated with RepoCard Studio — LAZYFROG (KZ) — kindware.dev*

You can disable this in the UI, but keeping it helps free tools stay visible. 🙏

---

## 🤝 Contributing

Contributions welcome! See [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

---

## 📄 License

MIT License — See [LICENSE](LICENSE) for details.

---

## 🛠️ Tech Stack

- **Framework**: [Tauri 2.x](https://tauri.app/)
- **Frontend**: React 19 + TypeScript + Vite
- **Backend**: Rust
- **SVG Rendering**: resvg

---

<p align="center">
  <strong>Built with ❤️ by <a href="https://kindware.dev">LAZYFROG (KZ)</a></strong><br>
  <em>Free tools for developers who ship.</em>
</p>
