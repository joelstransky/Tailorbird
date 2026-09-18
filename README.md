# 🐦 Tailorbird

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/rust-1.75%2B-orange.svg)](https://www.rust-lang.org/)
[![GitHub Pages](https://img.shields.io/badge/homepage-online-blue.svg)](https://joelstransky.github.io/Tailorbird/)

> **The Dual-Pane Job Application Browser & Command Center**  
> Built with Rust, Wry, and Tao. 100% local, blazing fast, zero telemetry.

🌐 **Website & Live Demo Preview**: [joelstransky.github.io/Tailorbird](https://joelstransky.github.io/Tailorbird/)

---

## 🌟 Overview

Applying for jobs today often requires jumping between multiple browser tabs, LinkedIn/Indeed searches, disparate ATS systems (Greenhouse, Lever, Ashby, Workday), and tracking spreadsheets—all while repetitive forms ask for the exact same contact information, work history, and portfolio links.

**Tailorbird** solves this by unifying your candidate profile, resume parser, prospect pipeline, and job search builder directly alongside a live, native browser webview inside a single lightweight desktop window.

```
+----------------------------------------------------------------------------------------------------+
| Tailorbird Native Window (1280x750)                                                                |
|                                                                                                    |
| +-------------------------+ +--------------------------------------------------------------------+ |
| | Control Panel (Resizable| | Browser Navigation Toolbar (Flex width, 44px height)               | |
| |  Photoshop-Style Dock)  | | [ ◀ ] [ ▶ ] [ 🔄 ] [ 🏠 ]  [ 🔒 / 🔍 Omnibar              ] [ Go ]  | |
| |                         | +--------------------------------------------------------------------+ |
| | [▼ Candidate Profile]   | | Target Job Application Pane (Flex width, remaining height)        | |
| | - Autofill Action       | | - Live Job Postings / ATS Forms / Built-in Test Portal             | |
| |                         | | - Reactive DOM Scraper & Autofill Targets                          | |
| | [▼ Resume & History]    | | - Real-time Unfilled Inputs Visual Highlighting (Warm Amber)       | |
| | - Linked PDF / GDoc     | | - Persistent Autofill Live Countdown Toast                         | |
| | - Import & Scan         | |                                                                    | |
| | - Work History CRUD     | |                                                                    | |
| |                         | |                                                                    | |
| | [▼ Prospect Sheet]      | |                                                                    | |
| | - Record Current Page   | |                                                                    | |
| | - Status Pipeline Table | |                                                                    | |
| | - Inline Row Edit / CSV | |                                                                    | |
| |                         | |                                                                    | |
| | [▶ Job Finder]          | |                                                                    | |
| | - Boolean Synthesizer   | |                                                                    | |
| +-------------------------+ +--------------------------------------------------------------------+ |
+----------------------------------------------------------------------------------------------------+
```

---

## ✨ Key Features

### ⚡ Framework-Resilient ATS Autofill
- Accurately identifies and fills fields across **Greenhouse**, **Lever**, **Ashby**, and custom enterprise portals.
- Supports native text inputs, textareas, multi-option `<select>` dropdowns, pronoun selectors, and radio button groups.
- Successfully autofilled inputs receive a subtle green confirmation pulse.

### ⚠️ Visual Highlighting of Unfilled Fields
- Unfilled inputs are visually highlighted with a distinct dashed amber border and warm glow so required questions are never missed.
- **Persistent Live Feedback Toast**: Stays on screen while unfilled fields remain, counting down in real time as each field is filled, before concluding with a green completion badge.

### 📄 Intelligent Resume & Work History Scanner
- Link any local resume file (`.pdf`, `.txt`, `.md`, `.json`) or public Google Docs URL.
- Heuristic parser intelligently isolates dates, job titles, and employers without splitting lines on periods in company names or hyphens in bullet descriptions.
- Full manual inline CRUD to add, modify, or delete past positions.

### 📝 Special Fields & Snippet Manager (Tabbed View)
- Segmented tab switcher inside the Work History panel: **Work History** and **Special Fields**.
- Add, edit, and remove custom paragraphs for questions that frequently come up during applications (e.g. *"What makes you want to work here?"*, elevator pitches, cover snippets).
- Dedicated **📋 Copy** button on each snippet with live confirmation feedback (`✓ Copied!`) for quick pasting.
- 100% persisted in local storage across sessions.

### 📊 Integrated Prospect Sheet & Pipeline Tracker
- **One-Click Recording (`➕ Record Current Page`)**: Scrapes the current page's job title, company name, and URL.
- **Status Stages**: Track application lifecycle (`Interested`, `Screening`, `Interviewing`, `Responded`, `Applied`, `Offer`, `Rejected`).
- **Inline Row Editing**: Toggle `✏️` / `💾` to edit rows directly with keyboard shortcuts (**Enter** to save, **Esc** to cancel).
- **Export to CSV**: Export your tracked pipeline for backups or spreadsheet workflows.

### 🔍 Targeted Job Finder (Boolean Search Synthesizer)
- Construct boolean queries for Greenhouse, Lever, LinkedIn, Indeed, and Google Jobs with title, location, and remote filters.
- Opens synthesized search results directly in the browser pane without leaving Tailorbird.

### 🔒 Privacy-First & 100% Local
- Your profile, resume, and application history never leave your computer.
- Persisted locally in `tailorbird_data.json` (gitignored). Zero third-party telemetry, zero external trackers.

---

## 🚀 Quick Start

### Prerequisites
- [Rust & Cargo](https://rustup.rs/) (version 1.75 or later recommended)
- Windows 10/11, macOS, or modern Linux desktop

### Installation & Running

```bash
# 1. Clone the repository
git clone https://github.com/joelstransky/Tailorbird.git
cd Tailorbird

# 2. Run in development mode
cargo run

# 3. Or build an optimized release binary
cargo build --release
```

The compiled standalone executable will be located in `target/release/tailorbird` (or `tailorbird.exe` on Windows).

---

## ⌨️ Shortcuts & Navigation

| Control | Action |
| :--- | :--- |
| **Middle Divider** | Click and drag left/right to resize the Control Panel |
| **Enter (Omnibar)** | Navigate to URL or run Google Search |
| **Enter (Prospect Edit)** | Save row changes |
| **Esc (Prospect Edit)** | Cancel row editing |
| **📄 Test Form** | Load built-in offline ATS test portal (`local://mock`) |

---

## 🛠️ Tech Stack

- **Core**: [Rust](https://www.rust-lang.org/)
- **Window Management**: [Tao](https://github.com/tauri-apps/tao)
- **Webview Engine**: [Wry](https://github.com/tauri-apps/wry)
- **PDF Extraction**: `pdf-extract`
- **Serialization**: `serde` / `serde_json`

---

## 📄 License

Distributed under the MIT License. See `LICENSE` for details.
