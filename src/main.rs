#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod autofill;
mod prospect;
mod resume;

use std::sync::{Arc, Mutex};
use tao::{
    dpi::{LogicalPosition, LogicalSize, Position, Size},
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoopBuilder, EventLoopProxy},
    window::{Icon, WindowBuilder},
};

#[cfg(target_os = "windows")]
use tao::platform::windows::WindowExtWindows;
use wry::{NewWindowResponse, PageLoadEvent, Rect, WebView, WebViewBuilder};
#[cfg(target_os = "windows")]
use wry::{WebViewBuilderExtWindows, WebViewExtWindows};

use crate::autofill::{generate_autofill_script, generate_context_menu_script, CandidateProfile};
use crate::prospect::{load_stored_data, save_stored_data, AppData, Prospect, SearchCriteria};
use crate::resume::{load_resume_text, parse_work_history, WorkHistoryEntry};
use base64::Engine;

const LEFT_PANE_WIDTH: f64 = 420.0;
const TOOLBAR_HEIGHT: f64 = 78.0;
const DEFAULT_WINDOW_WIDTH: f64 = 1280.0;
const DEFAULT_WINDOW_HEIGHT: f64 = 750.0;

// Embedded icon bytes for window title bar and taskbar
const ICON_RGBA: &[u8] = include_bytes!("assets/icon_32.rgba");

// Embedded HTML assets for zero-dependency standalone execution
const LEFT_PANE_HTML: &str = include_str!("assets/left_pane.html");
const TOOLBAR_HTML: &str = include_str!("assets/toolbar.html");
const MOCK_JOB_HTML: &str = include_str!("assets/mock_job_page.html");
const HITLIST_HTML: &str = include_str!("assets/hitlist.html");
const SETTINGS_HTML: &str = include_str!("assets/settings.html");
const WELCOME_HTML: &str = include_str!("assets/welcome.html");

fn prepare_settings_html(primary_color: &str) -> String {
    let color_json = serde_json::to_string(primary_color).unwrap_or_else(|_| "\"#818CF8\"".to_string());
    SETTINGS_HTML.replace(
        "/*__INITIAL_COLOR__*/",
        &format!("window.__INITIAL_ACCENT_COLOR__ = {};", color_json),
    )
}

fn prepare_hitlist_html(targets: &[crate::prospect::HitListTarget]) -> String {
    let data_json = serde_json::to_string(targets).unwrap_or_else(|_| "[]".to_string());
    HITLIST_HTML.replace(
        "/*__INITIAL_DATA__*/",
        &format!("window.__INITIAL_HITLIST_DATA__ = {};", data_json),
    )
}

#[cfg(target_os = "macos")]
fn setup_macos_menu() {
    use muda::{Menu, PredefinedMenuItem, Submenu};
    let app_menu = Submenu::new("Tailorbird", true);
    let _ = app_menu.append(&PredefinedMenuItem::about(None, None));
    let _ = app_menu.append(&PredefinedMenuItem::separator());
    let _ = app_menu.append(&PredefinedMenuItem::hide(None));
    let _ = app_menu.append(&PredefinedMenuItem::hide_others(None));
    let _ = app_menu.append(&PredefinedMenuItem::show_all(None));
    let _ = app_menu.append(&PredefinedMenuItem::separator());
    let _ = app_menu.append(&PredefinedMenuItem::quit(None));

    let edit_menu = Submenu::new("Edit", true);
    let _ = edit_menu.append(&PredefinedMenuItem::undo(None));
    let _ = edit_menu.append(&PredefinedMenuItem::redo(None));
    let _ = edit_menu.append(&PredefinedMenuItem::separator());
    let _ = edit_menu.append(&PredefinedMenuItem::cut(None));
    let _ = edit_menu.append(&PredefinedMenuItem::copy(None));
    let _ = edit_menu.append(&PredefinedMenuItem::paste(None));
    let _ = edit_menu.append(&PredefinedMenuItem::select_all(None));

    let window_menu = Submenu::new("Window", true);
    let _ = window_menu.append(&PredefinedMenuItem::minimize(None));
    let _ = window_menu.append(&PredefinedMenuItem::close_window(None));

    let menu = Menu::new();
    let _ = menu.append(&app_menu);
    let _ = menu.append(&edit_menu);
    let _ = menu.append(&window_menu);
    let _ = menu.init_for_nsapp();
}

fn prepare_left_pane_html(primary_color: &str) -> String {
    let color_json = serde_json::to_string(primary_color).unwrap_or_else(|_| "\"#818CF8\"".to_string());
    LEFT_PANE_HTML
        .replace("/*__PRIMARY_COLOR__*/ #818CF8", primary_color)
        .replace(
            "let currentPrimaryColor = (window.__INITIAL_PRIMARY_COLOR__)",
            &format!("window.__INITIAL_PRIMARY_COLOR__ = {}; let currentPrimaryColor = (window.__INITIAL_PRIMARY_COLOR__)", color_json),
        )
}

const SCRAPE_PAGE_SCRIPT: &str = r#"(function() {
    try {
        const url = window.location.href;
        const title = document.title || '';
        const hostname = window.location.hostname.toLowerCase();

        const ogTitle = document.querySelector('meta[property="og:title"]')?.content || '';
        const ogSite = document.querySelector('meta[property="og:site_name"]')?.content || '';

        let jobTitle = '';
        let company = '';

        // 1. Check Schema.org / JSON-LD structured data first (highest accuracy)
        try {
            const scripts = document.querySelectorAll('script[type="application/ld+json"]');
            for (const s of scripts) {
                try {
                    const parsed = JSON.parse(s.innerText || s.textContent || '{}');
                    const items = Array.isArray(parsed) ? parsed : (parsed['@graph'] || [parsed]);
                    for (const item of items) {
                        if (item && (item['@type'] === 'JobPosting' || item['@type'] === 'jobPosting')) {
                            if (!jobTitle && item.title) jobTitle = item.title.trim();
                            if (!company && item.hiringOrganization && item.hiringOrganization.name) {
                                company = item.hiringOrganization.name.trim();
                            }
                        }
                    }
                } catch(e) {}
            }
        } catch(e) {}

        // 2. LinkedIn-specific extraction
        if (hostname.includes('linkedin.com')) {
            // Selectors for company on LinkedIn (public & authenticated)
            if (!company) {
                const liCompanyElem = document.querySelector(
                    'a.topcard__org-name-link, ' +
                    '.topcard__flavor--black-link, ' +
                    'a[data-tracking-control-name="public_jobs_topcard-org-name"], ' +
                    '.job-details-jobs-unified-top-card__company-name a, ' +
                    '.job-details-jobs-unified-top-card__company-name, ' +
                    '.jobs-unified-top-card__company-name a, ' +
                    '.jobs-unified-top-card__company-name, ' +
                    '.sub-nav-cta__optional-url, ' +
                    '.topcard__flavor-row a[href*="/company/"], ' +
                    'a[href*="linkedin.com/company/"], ' +
                    '.jobs-company__box .jobs-company__name'
                );
                if (liCompanyElem) {
                    const txt = liCompanyElem.innerText?.trim();
                    if (txt && txt.toLowerCase() !== 'linkedin') {
                        company = txt;
                    }
                }
            }

            // Selectors for job title on LinkedIn
            if (!jobTitle) {
                const liTitleElem = document.querySelector(
                    'h1.top-card-layout__title, ' +
                    'h1.topcard__title, ' +
                    'h1.job-details-jobs-unified-top-card__job-title, ' +
                    '.jobs-unified-top-card__job-title, ' +
                    'h2.top-card-layout__title'
                );
                if (liTitleElem) {
                    jobTitle = liTitleElem.innerText?.trim();
                }
            }

            // Title parsing for LinkedIn: "Company hiring Job Title in Location | LinkedIn"
            const candidateTitle = ogTitle || title;
            const liHiringMatch = candidateTitle.match(/^(.+?)\s+hiring\s+(.+?)(?:\s+in\s+[^|]+)?(?:\s*\|\s*LinkedIn)?$/i);
            if (liHiringMatch) {
                if (!company) company = liHiringMatch[1].trim();
                if (!jobTitle) jobTitle = liHiringMatch[2].trim();
            }

            // Title parsing: "Job Title at Company | LinkedIn"
            const liAtMatch = candidateTitle.match(/^(.+?)\s+at\s+([^|–-]+)/i);
            if (liAtMatch) {
                if (!jobTitle) jobTitle = liAtMatch[1].trim();
                if (!company) company = liAtMatch[2].trim();
            }

            // Fallback from canonical URL: /jobs/view/job-title-at-company-12345
            if (!company || !jobTitle) {
                const canonical = document.querySelector('link[rel="canonical"]')?.href || url;
                const matchSlug = canonical.match(/\/jobs\/view\/([a-zA-Z0-9-]+)-at-([a-zA-Z0-9-]+)-\d+/);
                if (matchSlug) {
                    if (!company) {
                        company = matchSlug[2].split('-').map(w => w.charAt(0).toUpperCase() + w.slice(1)).join(' ');
                    }
                    if (!jobTitle) {
                        jobTitle = matchSlug[1].split('-').map(w => w.charAt(0).toUpperCase() + w.slice(1)).join(' ');
                    }
                }
            }
        }

        // 3. General ATS & DOM element fallbacks
        const ghTitle = document.querySelector('.app-title')?.innerText?.trim();
        const ghCompany = document.querySelector('.company-name')?.innerText?.trim();
        const leverTitle = document.querySelector('.posting-headline h2')?.innerText?.trim();
        const ashbyTitle = document.querySelector('h1')?.innerText?.trim();
        const badgeCompany = document.querySelector('.company-badge')?.innerText?.trim();
        const compElem = document.querySelector('.company-name, .company, [class*="company-name"], [class*="companyName"]')?.innerText?.trim();
        const h1 = document.querySelector('h1')?.innerText?.trim();

        if (!jobTitle) {
            jobTitle = ghTitle || leverTitle || ashbyTitle || h1 || ogTitle || title;
        }

        if (!company) {
            company = badgeCompany || ghCompany || compElem || '';
        }

        // Filter out job board platform names from company
        const jobBoardPlatforms = ['linkedin', 'indeed', 'glassdoor', 'ziprecruiter', 'dice', 'monster', 'careerbuilder', 'simplyhired', 'builtin'];
        if (company && jobBoardPlatforms.includes(company.toLowerCase().trim())) {
            company = '';
        }

        // If company still empty, check ogSite (only if not a job board platform!)
        if (!company && ogSite && !jobBoardPlatforms.includes(ogSite.toLowerCase().trim())) {
            company = ogSite.trim();
        }

        // 4. Title string splitting heuristics
        if (!company) {
            if (title.includes(' at ')) {
                company = title.split(' at ')[1].split(/[-–|]/)[0].trim();
            } else if (title.includes(' - ')) {
                const parts = title.split(' - ');
                if (parts.length > 1) {
                    if (parts[1].trim().toLowerCase() === jobTitle.toLowerCase()) {
                        company = parts[0].trim();
                    } else if (parts[0].trim().toLowerCase() === jobTitle.toLowerCase()) {
                        company = parts[1].trim();
                    } else {
                        company = parts[0].trim();
                    }
                }
            } else if (title.includes(' | ')) {
                const parts = title.split(' | ');
                if (parts.length > 1) {
                    const first = parts[0].trim();
                    const last = parts[parts.length - 1].trim();
                    if (!jobBoardPlatforms.includes(last.toLowerCase())) {
                        company = last;
                    } else if (!jobBoardPlatforms.includes(first.toLowerCase())) {
                        company = first;
                    }
                }
            }
        }

        // Final sanitation
        if (company && jobBoardPlatforms.includes(company.toLowerCase().trim())) {
            company = '';
        }

        if (jobTitle.includes(' at ')) {
            jobTitle = jobTitle.split(' at ')[0].trim();
        }
        if (jobTitle.includes(' | ')) {
            const p = jobTitle.split(' | ');
            if (jobBoardPlatforms.includes(p[p.length - 1].trim().toLowerCase())) {
                jobTitle = p[0].trim();
            }
        }

        if (window.ipc && typeof window.ipc.postMessage === 'function') {
            window.ipc.postMessage(JSON.stringify({
                action: "RECORD_PAGE_DATA",
                data: {
                    url: url,
                    jobTitle: jobTitle.substring(0, 120),
                    company: company.substring(0, 80)
                }
            }));
        }
    } catch (e) {
        console.error("Tailorbird scrape error:", e);
    }
})();"#;

const WINDOW_HOOKS_SCRIPT: &str = r#"(function() {
    if (window.__TAILORBIRD_WINDOW_HOOKS_INSTALLED__) return;
    window.__TAILORBIRD_WINDOW_HOOKS_INSTALLED__ = true;

    function unwrapUrl(dest) {
        if (!dest) return '';
        if (dest.includes('linkedin.com/safety/go') && dest.includes('url=')) {
            try {
                const parsed = new URL(dest, window.location.href);
                const real = parsed.searchParams.get('url');
                if (real) return real;
            } catch(e) {}
        }
        return dest;
    }

    function openInTailorbird(destUrl) {
        if (!destUrl || destUrl === 'about:blank') return;
        const clean = unwrapUrl(destUrl);
        // Do not open empty safety/go placeholders without url parameter
        if (clean.includes('linkedin.com/safety/go') && !clean.includes('url=')) {
            return;
        }
        if (window.ipc && typeof window.ipc.postMessage === 'function') {
            window.ipc.postMessage(JSON.stringify({
                action: "OPEN_NEW_TAB",
                url: clean
            }));
        }
    }

    // Intercept window.open so async handlers (e.g. LinkedIn's Apply button popup blocker bypass)
    // can set win.location.href or win.location.replace without failing or losing the destination URL
    const originalWindowOpen = window.open;
    window.open = function(url, target, features) {
        const initialUrl = url ? String(url).trim() : '';

        if (initialUrl && initialUrl !== 'about:blank') {
            openInTailorbird(initialUrl);
        }

        const winProxy = {
            closed: false,
            name: target || '',
            opener: window,
            focus: function() {},
            blur: function() {},
            close: function() { this.closed = true; },
            postMessage: function() {},
            location: {
                _href: initialUrl,
                set href(newVal) {
                    this._href = newVal;
                    openInTailorbird(newVal);
                },
                get href() {
                    return this._href;
                },
                replace: function(newVal) {
                    this.href = newVal;
                },
                assign: function(newVal) {
                    this.href = newVal;
                },
                toString: function() {
                    return this._href;
                }
            },
            document: {
                write: function() {},
                writeln: function() {},
                close: function() {}
            }
        };

        return winProxy;
    };

    // Auto-unwrap safety/go on any link clicks
    document.addEventListener('click', function(e) {
        const a = e.target.closest('a');
        if (a && a.href && a.href.includes('linkedin.com/safety/go') && a.href.includes('url=')) {
            const clean = unwrapUrl(a.href);
            if (clean && clean !== a.href) {
                a.href = clean;
            }
        }
    }, true);
})();"#;

const GLOBAL_SHORTCUTS_SCRIPT: &str = r#"(function() {
    if (window.__TAILORBIRD_SHORTCUTS_INSTALLED__) return;
    window.__TAILORBIRD_SHORTCUTS_INSTALLED__ = true;

    window.addEventListener('keydown', function(e) {
        const isMac = navigator.platform.toUpperCase().indexOf('MAC') >= 0;
        const cmdOrCtrl = isMac ? e.metaKey : e.ctrlKey;
        const key = e.key ? e.key.toLowerCase() : '';

        // Never intercept standard clipboard and text editing shortcuts
        if (cmdOrCtrl && !e.shiftKey && (key === 'c' || key === 'v' || key === 'x' || key === 'a' || key === 'z')) {
            return;
        }

        // 1. New Tab: Cmd+T / Ctrl+T
        if (cmdOrCtrl && !e.shiftKey && !e.altKey && key === 't') {
            e.preventDefault();
            sendShortcutIpc({ action: 'NEW_TAB' });
            return;
        }

        // 2. Close Tab: Cmd+W / Ctrl+W
        if (cmdOrCtrl && !e.shiftKey && !e.altKey && key === 'w') {
            e.preventDefault();
            sendShortcutIpc({ action: 'CLOSE_ACTIVE_TAB' });
            return;
        }

        // 3. Reload: Cmd+R / Ctrl+R / F5
        if ((cmdOrCtrl && !e.shiftKey && !e.altKey && key === 'r') || e.key === 'F5') {
            e.preventDefault();
            sendShortcutIpc({ action: 'RELOAD' });
            return;
        }

        // 4. Focus Omnibar: Cmd+L / Ctrl+L
        if (cmdOrCtrl && !e.shiftKey && !e.altKey && key === 'l') {
            e.preventDefault();
            sendShortcutIpc({ action: 'FOCUS_OMNIBAR' });
            return;
        }

        // 5. Back: Cmd+[ / Alt+Left
        if ((isMac && e.metaKey && e.key === '[') || (!isMac && e.altKey && e.key === 'ArrowLeft')) {
            e.preventDefault();
            sendShortcutIpc({ action: 'BACK' });
            return;
        }

        // 6. Forward: Cmd+] / Alt+Right
        if ((isMac && e.metaKey && e.key === ']') || (!isMac && e.altKey && e.key === 'ArrowRight')) {
            e.preventDefault();
            sendShortcutIpc({ action: 'FORWARD' });
            return;
        }

        // 7. Tab Cycling: Ctrl+Tab (next) / Ctrl+Shift+Tab (prev)
        if (e.ctrlKey && e.key === 'Tab') {
            e.preventDefault();
            sendShortcutIpc({ action: 'CYCLE_TAB', direction: e.shiftKey ? -1 : 1 });
            return;
        }

        // 8. Jump to Tab 1-9: Cmd+1..9 / Ctrl+1..9
        if (cmdOrCtrl && !e.shiftKey && !e.altKey && e.key >= '1' && e.key <= '9') {
            e.preventDefault();
            const index = parseInt(e.key, 10);
            sendShortcutIpc({ action: 'SWITCH_TAB_BY_INDEX', index: index });
            return;
        }

        // 9. Scrape / Record Prospect: Cmd+Shift+S / Ctrl+Shift+S
        if (cmdOrCtrl && e.shiftKey && key === 's') {
            e.preventDefault();
            sendShortcutIpc({ action: 'SCRAPE_CURRENT_PAGE' });
            return;
        }
    }, true);

    function sendShortcutIpc(msg) {
        if (window.ipc && typeof window.ipc.postMessage === 'function') {
            window.ipc.postMessage(JSON.stringify(msg));
        }
    }
})();"#;

fn decode_percent(s: &str) -> String {
    let mut bytes = Vec::new();
    let mut chars = s.bytes();
    while let Some(b) = chars.next() {
        if b == b'%' {
            let h1 = chars.next();
            let h2 = chars.next();
            if let (Some(h1), Some(h2)) = (h1, h2) {
                let hex_str = [h1, h2];
                if let Ok(hex_val) = std::str::from_utf8(&hex_str) {
                    if let Ok(byte_val) = u8::from_str_radix(hex_val, 16) {
                        bytes.push(byte_val);
                        continue;
                    }
                }
                bytes.push(b'%');
                bytes.push(h1);
                bytes.push(h2);
            } else {
                bytes.push(b'%');
                if let Some(h) = h1 { bytes.push(h); }
            }
        } else if b == b'+' {
            bytes.push(b' ');
        } else {
            bytes.push(b);
        }
    }
    String::from_utf8_lossy(&bytes).to_string()
}

fn unwrap_safety_url(url: &str) -> String {
    if url.contains("linkedin.com/safety/go") && url.contains("url=") {
        if let Some(pos) = url.find("url=") {
            let rest = &url[pos + 4..];
            let end = rest.find('&').unwrap_or(rest.len());
            let encoded = &rest[..end];
            let decoded = decode_percent(encoded);
            if decoded.starts_with("http://") || decoded.starts_with("https://") {
                return decoded;
            }
        }
    }
    url.to_string()
}

#[derive(Debug)]
enum AppEvent {
    CreateTab { url: String, activate: bool },
    SwitchTab { id: usize },
    CloseTab { id: usize },
    CloseActiveTab,
    FocusOmnibar,
    FocusActiveTab,
    CycleTab { direction: i32 },
    SwitchTabByIndex { index: usize },
    TabTitleChanged { id: usize, title: String },
    TabPageLoaded { id: usize, url: String },
    NavigateActiveTab { url: String },
    RunSearch { query: String },
    SetLeftWidth { width: f64 },
    SetPrimaryColor { color: String },
    ScrapeCurrentPage,
    Back,
    Forward,
    Reload,
    Home,
}

#[derive(Debug, Clone, serde::Serialize)]
struct TabInfo {
    id: usize,
    title: String,
    url: String,
    #[serde(rename = "isSearch")]
    is_search: bool,
}

struct BrowserTab {
    id: usize,
    title: String,
    url: String,
    webview: WebView,
    is_search: bool,
}

#[allow(clippy::large_enum_variant)]
#[derive(Debug, serde::Deserialize)]
#[serde(tag = "action")]
enum IpcMessage {
    #[serde(rename = "AUTOFILL")]
    Autofill { data: CandidateProfile },
    #[serde(rename = "NAVIGATE")]
    Navigate { url: String },
    #[serde(rename = "RUN_SEARCH")]
    RunSearch { query: String },
    #[serde(rename = "BACK")]
    Back,
    #[serde(rename = "FORWARD")]
    Forward,
    #[serde(rename = "RELOAD")]
    Reload,
    #[serde(rename = "HOME")]
    Home,
    #[serde(rename = "PICK_RESUME_FILE")]
    PickResumeFile,
    #[serde(rename = "PICK_COVER_LETTER_FILE")]
    PickCoverLetterFile,
    #[serde(rename = "IMPORT_RESUME")]
    ImportResume { source: String },
    #[serde(rename = "SCRAPE_CURRENT_PAGE")]
    ScrapeCurrentPage,
    #[serde(rename = "RECORD_PAGE_DATA")]
    RecordPageData { data: ScrapedData },
    #[serde(rename = "START_RESIZE")]
    StartResize,
    #[serde(rename = "SET_LEFT_WIDTH")]
    SetLeftWidth { width: f64 },
    #[serde(rename = "SWITCH_TAB")]
    SwitchTab { id: usize },
    #[serde(rename = "CLOSE_TAB")]
    CloseTab { id: usize },
    #[serde(rename = "NEW_TAB")]
    NewTab {
        #[serde(default)]
        url: Option<String>,
    },
    #[serde(rename = "SAVE_DATA")]
    SaveData {
        #[serde(rename = "candidateProfile")]
        candidate_profile: Option<CandidateProfile>,
        #[serde(rename = "resumeSource")]
        resume_source: String,
        #[serde(default, rename = "coverLetterSource")]
        cover_letter_source: String,
        #[serde(rename = "workHistory")]
        work_history: Vec<WorkHistoryEntry>,
        #[serde(default, rename = "specialFields")]
        special_fields: Vec<crate::prospect::SpecialField>,
        prospects: Vec<Prospect>,
        #[serde(rename = "searchCriteria")]
        search_criteria: Option<SearchCriteria>,
        #[serde(rename = "splitWidth")]
        split_width: Option<f64>,
        #[serde(default, rename = "drawerStates")]
        drawer_states: Option<std::collections::HashMap<String, bool>>,
        #[serde(default, rename = "primaryColor")]
        primary_color: Option<String>,
    },
    #[serde(rename = "SET_PRIMARY_COLOR")]
    SetPrimaryColor {
        color: String,
    },
    #[serde(rename = "LOAD_DATA")]
    LoadData,
    #[serde(rename = "LOAD_HIT_LIST", alias = "LOAD_OUTREACH_ROSTER")]
    LoadHitList,
    #[serde(rename = "SAVE_HIT_LIST", alias = "SAVE_OUTREACH_ROSTER")]
    SaveHitList {
        #[serde(rename = "hitList", alias = "outreachRoster")]
        hit_list: Vec<crate::prospect::HitListTarget>,
    },
    #[serde(rename = "OPEN_NEW_TAB")]
    OpenNewTab {
        url: String,
    },
    #[serde(rename = "EXPORT_PROSPECTS_CSV")]
    ExportProspectsCsv {
        csv: String,
    },
    #[serde(rename = "CLOSE_ACTIVE_TAB")]
    CloseActiveTab,
    #[serde(rename = "FOCUS_OMNIBAR")]
    FocusOmnibar,
    #[serde(rename = "FOCUS_ACTIVE_TAB")]
    FocusActiveTab,
    #[serde(rename = "CYCLE_TAB")]
    CycleTab { direction: i32 },
    #[serde(rename = "SWITCH_TAB_BY_INDEX")]
    SwitchTabByIndex { index: usize },
}

fn handle_common_shortcut_ipc(msg: &IpcMessage, proxy: &EventLoopProxy<AppEvent>) -> bool {
    match msg {
        IpcMessage::Back => {
            let _ = proxy.send_event(AppEvent::Back);
            true
        }
        IpcMessage::Forward => {
            let _ = proxy.send_event(AppEvent::Forward);
            true
        }
        IpcMessage::Reload => {
            let _ = proxy.send_event(AppEvent::Reload);
            true
        }
        IpcMessage::Home => {
            let _ = proxy.send_event(AppEvent::Home);
            true
        }
        IpcMessage::ScrapeCurrentPage => {
            let _ = proxy.send_event(AppEvent::ScrapeCurrentPage);
            true
        }
        IpcMessage::NewTab { url } => {
            let target_url = url
                .as_ref()
                .filter(|u| !u.trim().is_empty() && u.trim() != "about:blank")
                .cloned()
                .unwrap_or_else(|| "local://welcome".to_string());
            let _ = proxy.send_event(AppEvent::CreateTab {
                url: target_url,
                activate: true,
            });
            true
        }
        IpcMessage::CloseActiveTab => {
            let _ = proxy.send_event(AppEvent::CloseActiveTab);
            true
        }
        IpcMessage::FocusOmnibar => {
            let _ = proxy.send_event(AppEvent::FocusOmnibar);
            true
        }
        IpcMessage::FocusActiveTab => {
            let _ = proxy.send_event(AppEvent::FocusActiveTab);
            true
        }
        IpcMessage::CycleTab { direction } => {
            let _ = proxy.send_event(AppEvent::CycleTab { direction: *direction });
            true
        }
        IpcMessage::SwitchTabByIndex { index } => {
            let _ = proxy.send_event(AppEvent::SwitchTabByIndex { index: *index });
            true
        }
        _ => false,
    }
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
struct ScrapedData {
    url: String,
    job_title: String,
    company: String,
}

fn sync_tabs(toolbar_holder: &Arc<Mutex<Option<WebView>>>, tabs: &[BrowserTab], active_id: usize) {
    let info_list: Vec<TabInfo> = tabs
        .iter()
        .map(|t| TabInfo {
            id: t.id,
            title: t.title.clone(),
            url: t.url.clone(),
            is_search: t.is_search,
        })
        .collect();
    if let Ok(guard) = toolbar_holder.lock() {
        if let Some(ref tb) = *guard {
            let js = format!(
                "if (window.setTabs) {{ window.setTabs({}, {}); }}",
                serde_json::to_string(&info_list).unwrap_or_else(|_| "[]".to_string()),
                active_id
            );
            let _ = tb.evaluate_script(&js);
        }
    }
}

fn sync_url(toolbar_holder: &Arc<Mutex<Option<WebView>>>, url: &str) {
    if let Ok(guard) = toolbar_holder.lock() {
        if let Some(ref tb) = *guard {
            let js = format!(
                "if (window.setUrl) {{ window.setUrl({}); }}",
                serde_json::to_string(url).unwrap_or_default()
            );
            let _ = tb.evaluate_script(&js);
        }
    }
}

#[allow(clippy::arc_with_non_send_sync)]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("============================================================");
    println!("  Tailorbird — Dual-Pane Specialized Job-Search Browser");
    println!("============================================================");

    #[cfg(target_os = "macos")]
    setup_macos_menu();

    let mut event_loop_builder = EventLoopBuilder::<AppEvent>::with_user_event();
    let event_loop = event_loop_builder.build();
    let proxy: EventLoopProxy<AppEvent> = event_loop.create_proxy();

    let mut win_builder = WindowBuilder::new()
        .with_title("Tailorbird — Specialized Job Search Browser")
        .with_inner_size(LogicalSize::new(DEFAULT_WINDOW_WIDTH, DEFAULT_WINDOW_HEIGHT))
        .with_min_inner_size(LogicalSize::new(760.0, 480.0))
        .with_visible(true);

    if let Ok(icon) = Icon::from_rgba(ICON_RGBA.to_vec(), 32, 32) {
        win_builder = win_builder.with_window_icon(Some(icon));
    }

    let window = win_builder.build(&event_loop)?;

    window.set_focus();
    println!("[Tailorbird] Native parent window created successfully.");

    #[cfg(target_os = "windows")]
    let hwnd_raw = window.hwnd();

    // Load persisted app data to restore split width if available
    let initial_app_data = load_stored_data();
    let initial_left_width = initial_app_data.split_width.unwrap_or(LEFT_PANE_WIDTH);
    let initial_data_json = serde_json::to_string(&initial_app_data).unwrap_or_else(|_| "{}".to_string());
    let init_script = format!("window.__INITIAL_DATA__ = {};\n{}", initial_data_json, GLOBAL_SHORTCUTS_SCRIPT);
    let initial_context_script = generate_context_menu_script(
        initial_app_data.candidate_profile.as_ref(),
        &initial_app_data.special_fields,
    );

    // Shared references
    let left_wv_holder: Arc<Mutex<Option<WebView>>> = Arc::new(Mutex::new(None));
    let toolbar_wv_holder: Arc<Mutex<Option<WebView>>> = Arc::new(Mutex::new(None));
    let tabs_holder: Arc<Mutex<Vec<BrowserTab>>> = Arc::new(Mutex::new(Vec::new()));
    let active_tab_id_holder = Arc::new(Mutex::new(1usize));
    let next_tab_id_holder = Arc::new(Mutex::new(2usize));

    let left_width_holder = Arc::new(Mutex::new(initial_left_width));
    let window_size_holder = Arc::new(Mutex::new((DEFAULT_WINDOW_WIDTH, DEFAULT_WINDOW_HEIGHT)));
    let scale_factor_holder = Arc::new(Mutex::new(window.scale_factor()));

    let left_for_ipc = left_wv_holder.clone();
    let left_width_for_ipc = left_width_holder.clone();
    let tabs_for_left = tabs_holder.clone();
    let active_id_for_left = active_tab_id_holder.clone();
    let proxy_for_left = proxy.clone();
    let toolbar_for_left_ipc = toolbar_wv_holder.clone();
    #[cfg(target_os = "windows")]
    let scale_factor_for_ipc = scale_factor_holder.clone();

    let initial_primary_color = initial_app_data.primary_color.clone().unwrap_or_else(|| "#818CF8".to_string());
    let tb_init_script = format!(
        "window.__INITIAL_PRIMARY_COLOR__ = {};\n{}",
        serde_json::to_string(&initial_primary_color).unwrap_or_default(),
        GLOBAL_SHORTCUTS_SCRIPT
    );
    let prepared_left_html = prepare_left_pane_html(&initial_primary_color);

    // Reusable layout coordinator to update all webview bounds seamlessly
    let layout_coordinator = Arc::new({
        let left_h = left_wv_holder.clone();
        let toolbar_h = toolbar_wv_holder.clone();
        let tabs_h = tabs_holder.clone();
        let active_id_h = active_tab_id_holder.clone();
        let left_w_h = left_width_holder.clone();
        let win_sz_h = window_size_holder.clone();

        move |new_width: Option<f64>| {
            let (win_width, win_height) = *win_sz_h.lock().unwrap();
            let mut cur_lw = *left_w_h.lock().unwrap();
            if let Some(w) = new_width {
                cur_lw = w;
            }

            let min_w = 220.0;
            let max_w = (win_width - 250.0).max(min_w);
            let clamped_lw = cur_lw.clamp(min_w, max_w);
            *left_w_h.lock().unwrap() = clamped_lw;

            let right_width = (win_width - clamped_lw).max(0.0);
            let target_height = (win_height - TOOLBAR_HEIGHT).max(0.0);

            if let Ok(guard) = left_h.lock() {
                if let Some(ref wv) = *guard {
                    let _ = wv.set_bounds(Rect {
                        position: Position::Logical(LogicalPosition::new(0.0, 0.0)),
                        size: Size::Logical(LogicalSize::new(clamped_lw, win_height)),
                    });
                }
            }
            if let Ok(guard) = toolbar_h.lock() {
                if let Some(ref wv) = *guard {
                    let _ = wv.set_bounds(Rect {
                        position: Position::Logical(LogicalPosition::new(clamped_lw, 0.0)),
                        size: Size::Logical(LogicalSize::new(right_width, TOOLBAR_HEIGHT)),
                    });
                }
            }
            if let Ok(guard) = tabs_h.lock() {
                let active_id = *active_id_h.lock().unwrap();
                if let Some(tab) = guard.iter().find(|t| t.id == active_id) {
                    let _ = tab.webview.set_bounds(Rect {
                        position: Position::Logical(LogicalPosition::new(clamped_lw, TOOLBAR_HEIGHT)),
                        size: Size::Logical(LogicalSize::new(right_width, target_height)),
                    });
                }
            }
        }
    });

    // 1. Initialize Left Pane (Control Panel: Profile, Resume/Work History, Prospects, Finder)
    let left_webview = WebViewBuilder::new()
        .with_bounds(Rect {
            position: Position::Logical(LogicalPosition::new(0.0, 0.0)),
            size: Size::Logical(LogicalSize::new(initial_left_width, DEFAULT_WINDOW_HEIGHT)),
        })
        .with_initialization_script(&init_script)
        .with_devtools(true)
        .with_new_window_req_handler({
            let proxy_left = proxy.clone();
            move |url, _features| {
                println!("[Tailorbird Left Pane] New window requested for URL: {}", url);
                if !url.is_empty() && url != "about:blank" {
                    let _ = proxy_left.send_event(AppEvent::CreateTab {
                        url,
                        activate: true,
                    });
                }
                NewWindowResponse::Deny
            }
        })
        .with_html(&prepared_left_html)
        .with_ipc_handler({
            let left_holder = left_for_ipc.clone();
            let tabs_holder = tabs_for_left.clone();
            let active_id_holder = active_id_for_left.clone();
            let proxy_ipc = proxy_for_left.clone();
            let left_w_holder = left_width_for_ipc.clone();
            let toolbar_for_left = toolbar_for_left_ipc.clone();
            #[cfg(target_os = "windows")]
            let win_sz_for_ipc = window_size_holder.clone();
            #[cfg(target_os = "windows")]
            let sf_for_ipc = scale_factor_for_ipc.clone();
            #[cfg(target_os = "windows")]
            let hwnd_for_ipc = hwnd_raw;

            move |req| {
                let body = req.body();
                if let Ok(msg) = serde_json::from_str::<IpcMessage>(body) {
                    if handle_common_shortcut_ipc(&msg, &proxy_ipc) {
                        return;
                    }
                }
                match serde_json::from_str::<IpcMessage>(body) {
                    Ok(IpcMessage::Autofill { mut data }) => {
                        println!("[Tailorbird Host] Triggering AUTOFILL for: {}", data.full_name);

                        // Attach Resume file bytes if local path exists
                        if !data.resume_path.trim().is_empty() {
                            let p = std::path::Path::new(data.resume_path.trim());
                            if p.exists() && p.is_file() {
                                if let Ok(bytes) = std::fs::read(p) {
                                    let file_name = p.file_name().and_then(|s| s.to_str()).unwrap_or("Resume.pdf").to_string();
                                    let mime_type = match p.extension().and_then(|s| s.to_str()).unwrap_or("").to_lowercase().as_str() {
                                        "pdf" => "application/pdf",
                                        "docx" => "application/vnd.openxmlformats-officedocument.wordprocessingml.document",
                                        "doc" => "application/msword",
                                        "txt" => "text/plain",
                                        "md" => "text/markdown",
                                        _ => "application/octet-stream",
                                    }.to_string();
                                    let base64_data = base64::prelude::BASE64_STANDARD.encode(&bytes);
                                    data.resume_file = Some(crate::autofill::FilePayload {
                                        file_name,
                                        mime_type,
                                        base64_data,
                                    });
                                    println!("[Tailorbird Host] Loaded resume attachment: {} ({} bytes)", p.display(), bytes.len());
                                }
                            }
                        }

                        // Attach Cover Letter file bytes if local path exists
                        if !data.cover_letter_path.trim().is_empty() {
                            let p = std::path::Path::new(data.cover_letter_path.trim());
                            if p.exists() && p.is_file() {
                                if let Ok(bytes) = std::fs::read(p) {
                                    let file_name = p.file_name().and_then(|s| s.to_str()).unwrap_or("Cover_Letter.pdf").to_string();
                                    let mime_type = match p.extension().and_then(|s| s.to_str()).unwrap_or("").to_lowercase().as_str() {
                                        "pdf" => "application/pdf",
                                        "docx" => "application/vnd.openxmlformats-officedocument.wordprocessingml.document",
                                        "doc" => "application/msword",
                                        "txt" => "text/plain",
                                        "md" => "text/markdown",
                                        _ => "application/octet-stream",
                                    }.to_string();
                                    let base64_data = base64::prelude::BASE64_STANDARD.encode(&bytes);
                                    data.cover_letter_file = Some(crate::autofill::FilePayload {
                                        file_name,
                                        mime_type,
                                        base64_data,
                                    });
                                    println!("[Tailorbird Host] Loaded cover letter attachment: {} ({} bytes)", p.display(), bytes.len());
                                }
                            }
                        }

                        let injection_script = generate_autofill_script(&data);

                        if let Ok(guard) = tabs_holder.lock() {
                            let active_id = *active_id_holder.lock().unwrap();
                            if let Some(tab) = guard.iter().find(|t| t.id == active_id) {
                                if let Err(e) = tab.webview.evaluate_script(&injection_script) {
                                    eprintln!("[Tailorbird Host] Script injection error: {:?}", e);
                                } else {
                                    println!("[Tailorbird Host] Autofill script injected into active tab pane.");
                                }
                            }
                        }
                    }
                    Ok(IpcMessage::Navigate { url }) => {
                        let _ = proxy_ipc.send_event(AppEvent::NavigateActiveTab { url });
                    }
                    Ok(IpcMessage::RunSearch { query }) => {
                        let _ = proxy_ipc.send_event(AppEvent::RunSearch { query });
                    }
                    Ok(IpcMessage::OpenNewTab { url }) => {
                        let _ = proxy_ipc.send_event(AppEvent::CreateTab {
                            url,
                            activate: true,
                        });
                    }
                    Ok(IpcMessage::PickResumeFile) => {
                        if let Some(file) = rfd::FileDialog::new()
                            .set_title("Select Resume File")
                            .add_filter("Resume / CV Files", &["pdf", "txt", "md", "json", "docx"])
                            .pick_file()
                        {
                            let path_str = file.to_string_lossy().to_string();
                            println!("[Tailorbird Host] Selected resume file: {}", path_str);
                            if let Ok(guard) = left_holder.lock() {
                                if let Some(ref left_wv) = *guard {
                                    let js = format!("if (window.setResumePath) {{ window.setResumePath({}); }}", serde_json::to_string(&path_str).unwrap_or_default());
                                    let _ = left_wv.evaluate_script(&js);
                                }
                            }
                        }
                    }
                    Ok(IpcMessage::PickCoverLetterFile) => {
                        if let Some(file) = rfd::FileDialog::new()
                            .set_title("Select Cover Letter File")
                            .add_filter("Cover Letter Files", &["pdf", "docx", "doc", "txt", "md"])
                            .pick_file()
                        {
                            let path_str = file.to_string_lossy().to_string();
                            println!("[Tailorbird Host] Selected cover letter file: {}", path_str);
                            if let Ok(guard) = left_holder.lock() {
                                if let Some(ref left_wv) = *guard {
                                    let js = format!("if (window.setCoverLetterPath) {{ window.setCoverLetterPath({}); }}", serde_json::to_string(&path_str).unwrap_or_default());
                                    let _ = left_wv.evaluate_script(&js);
                                }
                            }
                        }
                    }
                    Ok(IpcMessage::ImportResume { source }) => {
                        println!("[Tailorbird Host] Importing and scanning resume from: {}", source);
                        match load_resume_text(&source) {
                            Ok(text) => {
                                let entries = parse_work_history(&text);
                                println!("[Tailorbird Host] Successfully parsed {} work history entries.", entries.len());
                                if let Ok(guard) = left_holder.lock() {
                                    if let Some(ref left_wv) = *guard {
                                        let json_entries = serde_json::to_string(&entries).unwrap_or_else(|_| "[]".to_string());
                                        let js = format!("if (window.setWorkHistory) {{ window.setWorkHistory({}); }}", json_entries);
                                        let _ = left_wv.evaluate_script(&js);
                                    }
                                }
                            }
                            Err(err) => {
                                eprintln!("[Tailorbird Host] Error reading resume: {}", err);
                                if let Ok(guard) = left_holder.lock() {
                                    if let Some(ref left_wv) = *guard {
                                        let js = format!(
                                            "if (window.showErrorAlert) {{ window.showErrorAlert({}, 'Failed to Read Resume'); }} else {{ alert({}); }}",
                                            serde_json::to_string(&err).unwrap_or_default(),
                                            serde_json::to_string(&err).unwrap_or_default()
                                        );
                                        let _ = left_wv.evaluate_script(&js);
                                    }
                                }
                            }
                        }
                    }
                    Ok(IpcMessage::ScrapeCurrentPage) => {
                        let _ = proxy_ipc.send_event(AppEvent::ScrapeCurrentPage);
                    }
                    Ok(IpcMessage::ExportProspectsCsv { csv }) => {
                        std::thread::spawn(move || {
                            if let Some(path) = rfd::FileDialog::new()
                                .set_title("Export Prospects as CSV")
                                .set_file_name("tailorbird_prospects.csv")
                                .add_filter("CSV Files", &["csv"])
                                .save_file()
                            {
                                if let Err(e) = std::fs::write(&path, &csv) {
                                    eprintln!("[Tailorbird Host] Failed to save CSV file: {}", e);
                                } else {
                                    println!("[Tailorbird Host] Successfully exported prospects to {:?}", path);
                                }
                            }
                        });
                    }
                    Ok(IpcMessage::StartResize) => {
                        #[cfg(target_os = "windows")]
                        {
                            let proxy_resize = proxy_ipc.clone();
                            let win_sz_h = win_sz_for_ipc.clone();
                            let left_w_h = left_w_holder.clone();
                            let sf_h = sf_for_ipc.clone();
                            let hwnd_isize = hwnd_for_ipc;

                            std::thread::spawn(move || {
                                use windows_sys::Win32::Foundation::POINT;
                                use windows_sys::Win32::Graphics::Gdi::ScreenToClient;
                                use windows_sys::Win32::UI::WindowsAndMessaging::GetCursorPos;
                                use windows_sys::Win32::UI::Input::KeyboardAndMouse::{GetAsyncKeyState, VK_LBUTTON};

                                // 1. Record starting logical width and initial physical cursor position
                                let start_lw = *left_w_h.lock().unwrap();
                                let mut start_pt = POINT { x: 0, y: 0 };
                                unsafe {
                                    GetCursorPos(&mut start_pt);
                                    ScreenToClient(hwnd_isize as _, &mut start_pt);
                                }

                                while (unsafe { GetAsyncKeyState(VK_LBUTTON as i32) } as u16 & 0x8000) != 0 {
                                    let mut pt = POINT { x: 0, y: 0 };
                                    unsafe {
                                        GetCursorPos(&mut pt);
                                        ScreenToClient(hwnd_isize as _, &mut pt);
                                    }

                                    let scale_factor = sf_h.lock().map(|g| *g).unwrap_or(1.0).max(0.1);
                                    let delta_physical = pt.x - start_pt.x;
                                    let delta_logical = (delta_physical as f64) / scale_factor;

                                    let (win_width, _) = *win_sz_h.lock().unwrap();
                                    let min_w = 220.0;
                                    let max_w = (win_width - 250.0).max(min_w);
                                    let new_w = (start_lw + delta_logical).clamp(min_w, max_w);

                                    let _ = proxy_resize.send_event(AppEvent::SetLeftWidth { width: new_w });
                                    std::thread::sleep(std::time::Duration::from_millis(16));
                                }

                                let cur_w = *left_w_h.lock().unwrap();
                                let mut data = load_stored_data();
                                data.split_width = Some(cur_w);
                                let _ = save_stored_data(&data);
                            });
                        }
                    }
                    Ok(IpcMessage::SetLeftWidth { width }) => {
                        let _ = proxy_ipc.send_event(AppEvent::SetLeftWidth { width });
                    }
                    Ok(IpcMessage::SetPrimaryColor { color }) => {
                        let _ = proxy_ipc.send_event(AppEvent::SetPrimaryColor { color });
                    }
                    Ok(IpcMessage::SaveData {
                        candidate_profile,
                        resume_source,
                        cover_letter_source,
                        work_history,
                        special_fields,
                        prospects,
                        search_criteria,
                        split_width,
                        drawer_states,
                        primary_color,
                    }) => {
                        let cur_w = split_width.unwrap_or_else(|| *left_w_holder.lock().unwrap());
                        let existing = load_stored_data();
                        let chosen_color = primary_color.or(existing.primary_color);
                        let data = AppData {
                            candidate_profile,
                            resume_source,
                            cover_letter_source,
                            work_history,
                            special_fields,
                            prospects,
                            search_criteria,
                            split_width: Some(cur_w),
                            drawer_states,
                            hit_list: existing.hit_list,
                            primary_color: chosen_color.clone(),
                        };
                        if let Err(e) = save_stored_data(&data) {
                            eprintln!("[Tailorbird Host] Error saving app data: {}", e);
                        }
                        if let Some(ref col) = chosen_color {
                            if let Ok(guard) = toolbar_for_left.lock() {
                                if let Some(ref tb) = *guard {
                                    let js = format!("if (window.setPrimaryColor) {{ window.setPrimaryColor({}); }}", serde_json::to_string(col).unwrap_or_default());
                                    let _ = tb.evaluate_script(&js);
                                }
                            }
                        }

                        // Push updated profile & special_fields to active tab context menu
                        if let Ok(guard) = tabs_holder.lock() {
                            let active_id = *active_id_holder.lock().unwrap();
                            if let Some(tab) = guard.iter().find(|t| t.id == active_id) {
                                let context_data = serde_json::json!({
                                    "candidateProfile": data.candidate_profile,
                                    "specialFields": data.special_fields,
                                });
                                let js = format!("if (window.__UPDATE_TAILORBIRD_CONTEXT_MENU__) {{ window.__UPDATE_TAILORBIRD_CONTEXT_MENU__({}); }}", context_data);
                                let _ = tab.webview.evaluate_script(&js);
                            }
                        }
                    }
                    Ok(IpcMessage::LoadData) => {
                        let data = load_stored_data();
                        let json_str = serde_json::to_string(&data).unwrap_or_else(|_| "{}".to_string());
                        if let Ok(guard) = left_holder.lock() {
                            if let Some(ref left_wv) = *guard {
                                let js = format!("if (window.setAllData) {{ window.setAllData({}); }}", json_str);
                                let _ = left_wv.evaluate_script(&js);
                            }
                        }
                    }
                    _ => {}
                }
            }
        })
        .build_as_child(&window)?;

    // Share WebView2 environment on Windows across all webviews
    #[cfg(target_os = "windows")]
    let shared_env = left_webview.environment();

    let initial_right_width = (DEFAULT_WINDOW_WIDTH - initial_left_width).max(0.0);

    // 2. Initialize Browser Toolbar (Tabs Strip, Back, Forward, Reload, Home, Omnibar)
    let toolbar_builder = WebViewBuilder::new();
    #[cfg(target_os = "windows")]
    let toolbar_builder = toolbar_builder.with_environment(shared_env.clone());
    let toolbar_webview = toolbar_builder
        .with_bounds(Rect {
            position: Position::Logical(LogicalPosition::new(initial_left_width, 0.0)),
            size: Size::Logical(LogicalSize::new(initial_right_width, TOOLBAR_HEIGHT)),
        })
        .with_devtools(true)
        .with_initialization_script(&tb_init_script)
        .with_html(TOOLBAR_HTML)
        .with_ipc_handler({
            let proxy_tb = proxy.clone();
            move |req| {
                let body = req.body();
                if let Ok(msg) = serde_json::from_str::<IpcMessage>(body) {
                    if handle_common_shortcut_ipc(&msg, &proxy_tb) {
                        return;
                    }
                }
                match serde_json::from_str::<IpcMessage>(body) {
                    Ok(IpcMessage::Navigate { url }) => {
                        let _ = proxy_tb.send_event(AppEvent::NavigateActiveTab { url });
                    }
                    Ok(IpcMessage::RunSearch { query }) => {
                        let _ = proxy_tb.send_event(AppEvent::RunSearch { query });
                    }
                    Ok(IpcMessage::SwitchTab { id }) => {
                        let _ = proxy_tb.send_event(AppEvent::SwitchTab { id });
                    }
                    Ok(IpcMessage::CloseTab { id }) => {
                        let _ = proxy_tb.send_event(AppEvent::CloseTab { id });
                    }
                    _ => {}
                }
            }
        })
        .build_as_child(&window)?;

    // 3. Tab WebView Factory Closure
    let make_tab_webview = {
        #[cfg(target_os = "windows")]
        let shared_env = shared_env.clone();
        let tab_init_script = format!("{}\n{}\n{}", initial_context_script, WINDOW_HOOKS_SCRIPT, GLOBAL_SHORTCUTS_SCRIPT);
        let proxy = proxy.clone();
        let left_holder = left_for_ipc.clone();
        let toolbar_holder = toolbar_wv_holder.clone();
        let tabs_factory_holder = tabs_holder.clone();

        move |win: &tao::window::Window, tab_id: usize, url: &str, bounds: Rect, visible: bool| -> Result<WebView, Box<dyn std::error::Error>> {
            let is_mock = url == "local://mock";
            let is_welcome = url == "local://welcome";
            let is_hitlist = url == "local://hitlist" || url == "local://roster";
            let is_settings = url == "local://settings";
            let proxy_title = proxy.clone();
            let proxy_load = proxy.clone();
            let proxy_new_win = proxy.clone();
            let proxy_ipc = proxy.clone();
            let left_h = left_holder.clone();
            let toolbar_h = toolbar_holder.clone();
            let tabs_h = tabs_factory_holder.clone();

            let builder = WebViewBuilder::new();
            #[cfg(target_os = "windows")]
            let builder = builder.with_environment(shared_env.clone());
            let builder = builder
                .with_bounds(bounds)
                .with_visible(visible)
                .with_devtools(true)
                .with_initialization_script(&tab_init_script)
                .with_new_window_req_handler({
                    let proxy_new_win = proxy_new_win.clone();
                    move |target_url, _features| {
                        println!("[Tailorbird Tab #{}] New window requested for URL: {}", tab_id, target_url);

                        // If it's LinkedIn's placeholder safety/go without destination url, suppress it
                        if target_url.contains("linkedin.com/safety/go") && !target_url.contains("url=") {
                            println!("[Tailorbird Tab #{}] Suppressed empty LinkedIn safety/go placeholder: {}", tab_id, target_url);
                            return NewWindowResponse::Deny;
                        }

                        let actual_url = unwrap_safety_url(&target_url);

                        if !actual_url.is_empty() && actual_url != "about:blank" {
                            let _ = proxy_new_win.send_event(AppEvent::CreateTab {
                                url: actual_url,
                                activate: true,
                            });
                        }
                        NewWindowResponse::Deny
                    }
                })
                .with_document_title_changed_handler({
                    let proxy_title = proxy_title.clone();
                    move |title| {
                        let _ = proxy_title.send_event(AppEvent::TabTitleChanged {
                            id: tab_id,
                            title,
                        });
                    }
                })
                .with_on_page_load_handler({
                    let proxy_load = proxy_load.clone();
                    move |event, page_url| {
                        if let PageLoadEvent::Finished = event {
                            let _ = proxy_load.send_event(AppEvent::TabPageLoaded {
                                id: tab_id,
                                url: page_url,
                            });
                        }
                    }
                })
                .with_ipc_handler({
                    let left_h = left_h.clone();
                    let toolbar_h = toolbar_h.clone();
                    let proxy_ipc = proxy_ipc.clone();
                    move |req| {
                        let body = req.body();
                        if let Ok(msg) = serde_json::from_str::<IpcMessage>(body) {
                            if handle_common_shortcut_ipc(&msg, &proxy_ipc) {
                                return;
                            }
                        }
                        match serde_json::from_str::<IpcMessage>(body) {
                            Ok(IpcMessage::RecordPageData { data }) => {
                                println!("[Tailorbird Host] Scraped listing: {} at {}", data.job_title, data.company);
                                if let Ok(guard) = left_h.lock() {
                                    if let Some(ref left_wv) = *guard {
                                        let js = format!("if (window.addProspectRow) {{ window.addProspectRow({}); }}", serde_json::to_string(&data).unwrap_or_default());
                                        let _ = left_wv.evaluate_script(&js);
                                    }
                                }
                                if let Ok(guard) = toolbar_h.lock() {
                                    if let Some(ref tb_wv) = *guard {
                                        let js = format!("if (window.showRecordSuccess) {{ window.showRecordSuccess({}); }}", serde_json::to_string(&data.company).unwrap_or_default());
                                        let _ = tb_wv.evaluate_script(&js);
                                    }
                                }
                            }
                            Ok(IpcMessage::SaveHitList { hit_list }) => {
                                let mut data = load_stored_data();
                                data.hit_list = hit_list;
                                if let Err(e) = save_stored_data(&data) {
                                    eprintln!("[Tailorbird Host] Error saving outreach roster: {}", e);
                                } else {
                                    println!("[Tailorbird Host] Successfully saved {} outreach roster targets.", data.hit_list.len());
                                    let count = data.hit_list.len();
                                    if let Ok(guard) = left_h.lock() {
                                        if let Some(ref left_wv) = *guard {
                                            let js = format!("if (window.updateHitListCount) {{ window.updateHitListCount({}); }}", count);
                                            let _ = left_wv.evaluate_script(&js);
                                        }
                                    }
                                }
                            }
                            Ok(IpcMessage::LoadHitList) => {
                                let stored = load_stored_data();
                                let data_json = serde_json::to_string(&stored.hit_list).unwrap_or_else(|_| "[]".to_string());
                                let js = format!("if (window.setHitListData) {{ window.setHitListData({}); }}", data_json);
                                if let Ok(guard) = tabs_h.lock() {
                                    if let Some(tab) = guard.iter().find(|t| t.id == tab_id) {
                                        let _ = tab.webview.evaluate_script(&js);
                                    }
                                }
                            }
                            Ok(IpcMessage::OpenNewTab { url }) => {
                                let _ = proxy_ipc.send_event(AppEvent::CreateTab {
                                    url,
                                    activate: true,
                                });
                            }
                            Ok(IpcMessage::SetPrimaryColor { color }) => {
                                let _ = proxy_ipc.send_event(AppEvent::SetPrimaryColor { color });
                            }
                            _ => {}
                        }
                    }
                });

            let webview = if is_mock {
                builder.with_html(MOCK_JOB_HTML).build_as_child(win)?
            } else if is_welcome {
                builder.with_html(WELCOME_HTML).build_as_child(win)?
            } else if is_hitlist {
                let stored = load_stored_data();
                let html = prepare_hitlist_html(&stored.hit_list);
                builder.with_html(&html).build_as_child(win)?
            } else if is_settings {
                let stored = load_stored_data();
                let current_color = stored.primary_color.unwrap_or_else(|| "#818CF8".to_string());
                let html = prepare_settings_html(&current_color);
                builder.with_html(&html).build_as_child(win)?
            } else {
                builder.with_url(url).build_as_child(win)?
            };

            if is_mock {
                let stored = load_stored_data();
                let context_data = serde_json::json!({
                    "candidateProfile": stored.candidate_profile,
                    "specialFields": stored.special_fields,
                });
                let js = format!("if (window.__UPDATE_TAILORBIRD_CONTEXT_MENU__) {{ window.__UPDATE_TAILORBIRD_CONTEXT_MENU__({}); }}", context_data);
                let _ = webview.evaluate_script(&js);
            } else if is_hitlist {
                let stored = load_stored_data();
                let data_json = serde_json::to_string(&stored.hit_list).unwrap_or_else(|_| "[]".to_string());
                let js = format!("if (window.setHitListData) {{ window.setHitListData({}); }}", data_json);
                let _ = webview.evaluate_script(&js);
            } else if is_settings {
                let stored = load_stored_data();
                let current_color = stored.primary_color.unwrap_or_else(|| "#818CF8".to_string());
                let js = format!("if (window.setInitialAccentColor) {{ window.setInitialAccentColor({}); }}", serde_json::to_string(&current_color).unwrap_or_default());
                let _ = webview.evaluate_script(&js);
            }

            Ok(webview)
        }
    };

    // 4. Create Initial Tab (Welcome Page)
    let initial_target_height = (DEFAULT_WINDOW_HEIGHT - TOOLBAR_HEIGHT).max(0.0);
    let initial_bounds = Rect {
        position: Position::Logical(LogicalPosition::new(initial_left_width, TOOLBAR_HEIGHT)),
        size: Size::Logical(LogicalSize::new(initial_right_width, initial_target_height)),
    };

    let initial_tab_wv = make_tab_webview(&window, 1, "local://welcome", initial_bounds, true)?;
    let initial_tab = BrowserTab {
        id: 1,
        title: "Welcome".to_string(),
        url: "local://welcome".to_string(),
        webview: initial_tab_wv,
        is_search: false,
    };

    tabs_holder.lock().unwrap().push(initial_tab);
    *left_wv_holder.lock().unwrap() = Some(left_webview);
    *toolbar_wv_holder.lock().unwrap() = Some(toolbar_webview);

    if let Ok(guard) = left_wv_holder.lock() {
        if let Some(ref wv) = *guard {
            let js = format!("if (window.setAllData) {{ window.setAllData({}); }}", initial_data_json);
            let _ = wv.evaluate_script(&js);
        }
    }

    // Sync initial tabs & omnibar to toolbar
    sync_tabs(&toolbar_wv_holder, &tabs_holder.lock().unwrap(), 1);
    sync_url(&toolbar_wv_holder, "local://welcome");

    println!("[Tailorbird] All panes, multi-tab manager, and browser toolbar ready! Event loop running.");

    let coordinator_for_loop = layout_coordinator.clone();

    // Event loop management
    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::UserEvent(app_event) => match app_event {
                AppEvent::CreateTab { url, activate } => {
                    let formatted_url = format_input_to_url(&url);
                    let (win_width, win_height) = *window_size_holder.lock().unwrap();
                    let cur_lw = *left_width_holder.lock().unwrap();
                    let right_width = (win_width - cur_lw).max(0.0);
                    let target_height = (win_height - TOOLBAR_HEIGHT).max(0.0);

                    let bounds = Rect {
                        position: Position::Logical(LogicalPosition::new(cur_lw, TOOLBAR_HEIGHT)),
                        size: Size::Logical(LogicalSize::new(right_width, target_height)),
                    };

                    // Only deduplicate singleton local utility pages (Outreach Roster and Settings).
                    // The test form and standard web pages are never deduplicated so that
                    // the new tab (+) button always creates a new tab unconditionally.
                    let is_singleton_page = formatted_url == "local://hitlist"
                        || formatted_url == "local://roster"
                        || formatted_url == "local://settings";
                    let already_open_id = if is_singleton_page {
                        let tabs = tabs_holder.lock().unwrap();
                        tabs.iter().find(|t| {
                            ((formatted_url == "local://hitlist" || formatted_url == "local://roster") && (t.url == "local://hitlist" || t.url == "local://roster" || t.title.contains("Roster")))
                                || (formatted_url == "local://settings" && (t.url == "local://settings" || t.title.contains("Settings")))
                        }).map(|t| t.id)
                    } else {
                        None
                    };

                    if let Some(existing_id) = already_open_id {
                        let mut tabs = tabs_holder.lock().unwrap();
                        let stored = load_stored_data();
                        let current_color = stored.primary_color.unwrap_or_else(|| "#818CF8".to_string());
                        let js_accent = format!(
                            "if (window.setInitialAccentColor) {{ window.setInitialAccentColor({}); }} if (window.setPrimaryColor) {{ window.setPrimaryColor({}); }}",
                            serde_json::to_string(&current_color).unwrap_or_default(),
                            serde_json::to_string(&current_color).unwrap_or_default()
                        );
                        let hitlist_js = if formatted_url == "local://hitlist" || formatted_url == "local://roster" {
                            let data_json = serde_json::to_string(&stored.hit_list).unwrap_or_else(|_| "[]".to_string());
                            format!("if (window.setHitListData) {{ window.setHitListData({}); }}", data_json)
                        } else {
                            String::new()
                        };

                        for t in tabs.iter_mut() {
                            if t.id == existing_id {
                                t.url = formatted_url.clone();
                                let _ = t.webview.set_bounds(bounds);
                                let _ = t.webview.set_visible(true);
                                let _ = t.webview.evaluate_script(&js_accent);
                                if !hitlist_js.is_empty() {
                                    let _ = t.webview.evaluate_script(&hitlist_js);
                                }
                                let _ = t.webview.focus();
                            } else {
                                let _ = t.webview.set_visible(false);
                            }
                        }
                        *active_tab_id_holder.lock().unwrap() = existing_id;
                        sync_url(&toolbar_wv_holder, &formatted_url);
                        sync_tabs(&toolbar_wv_holder, &tabs, existing_id);
                    } else {
                        let new_id = {
                            let mut nid = next_tab_id_holder.lock().unwrap();
                            let id = *nid;
                            *nid += 1;
                            id
                        };

                        println!("[Tailorbird Host] Creating tab #{} with URL: {}", new_id, formatted_url);

                        match make_tab_webview(&window, new_id, &formatted_url, bounds, activate) {
                            Ok(wv) => {
                                let initial_title = if formatted_url == "local://welcome" {
                                    "Welcome".to_string()
                                } else if formatted_url == "local://mock" {
                                    "Tailorbird Test Bench".to_string()
                                } else if formatted_url == "local://hitlist" || formatted_url == "local://roster" {
                                    "Outreach Roster".to_string()
                                } else if formatted_url == "local://settings" {
                                    "Settings".to_string()
                                } else {
                                    "New Tab".to_string()
                                };

                                let new_tab = BrowserTab {
                                    id: new_id,
                                    title: initial_title,
                                    url: formatted_url.clone(),
                                    webview: wv,
                                    is_search: false,
                                };

                                let mut tabs = tabs_holder.lock().unwrap();
                                tabs.push(new_tab);

                                if activate {
                                    for t in tabs.iter_mut() {
                                        if t.id != new_id {
                                            let _ = t.webview.set_visible(false);
                                        }
                                    }
                                    *active_tab_id_holder.lock().unwrap() = new_id;
                                    sync_url(&toolbar_wv_holder, &formatted_url);
                                }

                                let cur_active = *active_tab_id_holder.lock().unwrap();
                                sync_tabs(&toolbar_wv_holder, &tabs, cur_active);
                            }
                            Err(e) => {
                                eprintln!("[Tailorbird Host] Error creating tab #{}: {:?}", new_id, e);
                            }
                        }
                    }
                }
                AppEvent::SwitchTab { id } => {
                    let (win_width, win_height) = *window_size_holder.lock().unwrap();
                    let cur_lw = *left_width_holder.lock().unwrap();
                    let right_width = (win_width - cur_lw).max(0.0);
                    let target_height = (win_height - TOOLBAR_HEIGHT).max(0.0);
                    let bounds = Rect {
                        position: Position::Logical(LogicalPosition::new(cur_lw, TOOLBAR_HEIGHT)),
                        size: Size::Logical(LogicalSize::new(right_width, target_height)),
                    };

                    let mut tabs = tabs_holder.lock().unwrap();
                    let mut active_url = String::new();
                    for t in tabs.iter_mut() {
                        if t.id == id {
                            let _ = t.webview.set_bounds(bounds);
                            let _ = t.webview.set_visible(true);
                            let _ = t.webview.focus();
                            active_url = t.url.clone();
                        } else {
                            let _ = t.webview.set_visible(false);
                        }
                    }
                    *active_tab_id_holder.lock().unwrap() = id;
                    sync_url(&toolbar_wv_holder, &active_url);
                    sync_tabs(&toolbar_wv_holder, &tabs, id);
                }
                AppEvent::CloseTab { id } => {
                    let mut tabs = tabs_holder.lock().unwrap();
                    if tabs.len() <= 1 {
                        let single_id = if let Some(t) = tabs.first_mut() {
                            let _ = t.webview.load_html(WELCOME_HTML);
                            t.url = "local://welcome".to_string();
                            t.title = "Welcome".to_string();
                            t.is_search = false;
                            t.id
                        } else {
                            1
                        };
                        sync_url(&toolbar_wv_holder, "local://welcome");
                        sync_tabs(&toolbar_wv_holder, &tabs, single_id);
                        return;
                    }

                    let cur_active = *active_tab_id_holder.lock().unwrap();
                    let pos = tabs.iter().position(|t| t.id == id);
                    if let Some(idx) = pos {
                        let _ = tabs[idx].webview.set_visible(false);
                        tabs.remove(idx);

                        if cur_active == id {
                            let new_idx = idx.min(tabs.len() - 1);
                            let next_id = tabs[new_idx].id;
                            let (win_width, win_height) = *window_size_holder.lock().unwrap();
                            let cur_lw = *left_width_holder.lock().unwrap();
                            let right_width = (win_width - cur_lw).max(0.0);
                            let target_height = (win_height - TOOLBAR_HEIGHT).max(0.0);
                            let bounds = Rect {
                                position: Position::Logical(LogicalPosition::new(cur_lw, TOOLBAR_HEIGHT)),
                                size: Size::Logical(LogicalSize::new(right_width, target_height)),
                            };

                            let active_url = tabs[new_idx].url.clone();
                            let _ = tabs[new_idx].webview.set_bounds(bounds);
                            let _ = tabs[new_idx].webview.set_visible(true);
                            let _ = tabs[new_idx].webview.focus();

                            *active_tab_id_holder.lock().unwrap() = next_id;
                            sync_url(&toolbar_wv_holder, &active_url);
                            sync_tabs(&toolbar_wv_holder, &tabs, next_id);
                        } else {
                            sync_tabs(&toolbar_wv_holder, &tabs, cur_active);
                        }
                    }
                }
                AppEvent::TabTitleChanged { id, title } => {
                    if !title.is_empty() {
                        let mut tabs = tabs_holder.lock().unwrap();
                        if let Some(tab) = tabs.iter_mut().find(|t| t.id == id) {
                            tab.title = title;
                        }
                        let cur_active = *active_tab_id_holder.lock().unwrap();
                        sync_tabs(&toolbar_wv_holder, &tabs, cur_active);
                    }
                }
                AppEvent::TabPageLoaded { id, url } => {
                    let mut tabs = tabs_holder.lock().unwrap();
                    let cur_active = *active_tab_id_holder.lock().unwrap();
                    if let Some(tab) = tabs.iter_mut().find(|t| t.id == id) {
                        // Auto-unwrap safety/go URL if present
                        let clean_url = unwrap_safety_url(&url);
                        if clean_url != url {
                            println!("[Tailorbird Tab #{}] Auto-redirecting safety/go to: {}", id, clean_url);
                            let js_redir = format!("window.location.replace({});", serde_json::to_string(&clean_url).unwrap_or_default());
                            let _ = tab.webview.evaluate_script(&js_redir);
                            return;
                        }

                        let is_blank_or_data = url.is_empty() || url == "about:blank" || url.starts_with("data:");
                        let is_currently_local = tab.url.starts_with("local://");

                        // Never overwrite an existing URL with about:blank, data:..., or empty string
                        if !is_blank_or_data {
                            tab.url = url.clone();
                        }

                        if id == cur_active {
                            let omnibar_url = if is_blank_or_data && is_currently_local {
                                &tab.url
                            } else {
                                &url
                            };
                            if !omnibar_url.is_empty() && omnibar_url != "about:blank" {
                                sync_url(&toolbar_wv_holder, omnibar_url);
                            }
                            let stored = load_stored_data();
                            let context_data = serde_json::json!({
                                "candidateProfile": stored.candidate_profile,
                                "specialFields": stored.special_fields,
                            });
                            let js = format!("if (window.__UPDATE_TAILORBIRD_CONTEXT_MENU__) {{ window.__UPDATE_TAILORBIRD_CONTEXT_MENU__({}); }}", context_data);
                            let _ = tab.webview.evaluate_script(&js);
                        }
                    }
                    sync_tabs(&toolbar_wv_holder, &tabs, cur_active);
                }
                AppEvent::NavigateActiveTab { url } => {
                    let formatted = format_input_to_url(&url);
                    let cur_active = *active_tab_id_holder.lock().unwrap();
                    let mut tabs = tabs_holder.lock().unwrap();
                    if let Some(tab) = tabs.iter_mut().find(|t| t.id == cur_active) {
                        if formatted.starts_with("local://") || (!formatted.contains("google.com/search") && !formatted.contains("bing.com/search") && !formatted.contains("duckduckgo.com")) {
                            tab.is_search = false;
                        }
                        if formatted == "local://welcome" {
                            let _ = tab.webview.load_html(WELCOME_HTML);
                            tab.url = "local://welcome".to_string();
                            tab.title = "Welcome".to_string();
                        } else if formatted == "local://mock" {
                            let _ = tab.webview.load_html(MOCK_JOB_HTML);
                            tab.url = "local://mock".to_string();
                            tab.title = "Tailorbird Mock Job Listing".to_string();
                            let stored = load_stored_data();
                            let context_data = serde_json::json!({
                                "candidateProfile": stored.candidate_profile,
                                "specialFields": stored.special_fields,
                            });
                            let js = format!("if (window.__UPDATE_TAILORBIRD_CONTEXT_MENU__) {{ window.__UPDATE_TAILORBIRD_CONTEXT_MENU__({}); }}", context_data);
                            let _ = tab.webview.evaluate_script(&js);
                        } else if formatted == "local://hitlist" || formatted == "local://roster" {
                            let stored = load_stored_data();
                            let html = prepare_hitlist_html(&stored.hit_list);
                            let _ = tab.webview.load_html(&html);
                            tab.url = "local://roster".to_string();
                            tab.title = "Outreach Roster".to_string();
                            let data_json = serde_json::to_string(&stored.hit_list).unwrap_or_else(|_| "[]".to_string());
                            let js = format!("if (window.setHitListData) {{ window.setHitListData({}); }}", data_json);
                            let _ = tab.webview.evaluate_script(&js);
                        } else if formatted == "local://settings" {
                            let stored = load_stored_data();
                            let current_color = stored.primary_color.unwrap_or_else(|| "#818CF8".to_string());
                            let html = prepare_settings_html(&current_color);
                            let _ = tab.webview.load_html(&html);
                            tab.url = "local://settings".to_string();
                            tab.title = "Settings".to_string();
                            let js = format!("if (window.setInitialAccentColor) {{ window.setInitialAccentColor({}); }}", serde_json::to_string(&current_color).unwrap_or_default());
                            let _ = tab.webview.evaluate_script(&js);
                        } else {
                            let _ = tab.webview.load_url(&formatted);
                            tab.url = formatted.clone();
                        }
                        sync_url(&toolbar_wv_holder, &formatted);
                    }
                    sync_tabs(&toolbar_wv_holder, &tabs, cur_active);
                }
                AppEvent::RunSearch { query } => {
                    let search_url = if query.trim().is_empty() {
                        "https://www.google.com".to_string()
                    } else {
                        format_search_url(&query)
                    };

                    let (win_width, win_height) = *window_size_holder.lock().unwrap();
                    let cur_lw = *left_width_holder.lock().unwrap();
                    let right_width = (win_width - cur_lw).max(0.0);
                    let target_height = (win_height - TOOLBAR_HEIGHT).max(0.0);
                    let bounds = Rect {
                        position: Position::Logical(LogicalPosition::new(cur_lw, TOOLBAR_HEIGHT)),
                        size: Size::Logical(LogicalSize::new(right_width, target_height)),
                    };

                    let mut tabs = tabs_holder.lock().unwrap();
                    let existing_search_id = tabs.iter().find(|t| {
                        t.is_search
                            || t.url.contains("google.com/search")
                            || t.title == "Job Search"
                    }).map(|t| t.id);

                    if let Some(search_id) = existing_search_id {
                        println!("[Tailorbird Host] Reusing existing search tab #{}, navigating to: {}", search_id, search_url);
                        for t in tabs.iter_mut() {
                            if t.id == search_id {
                                t.is_search = true;
                                t.url = search_url.clone();
                                t.title = "Job Search".to_string();
                                let _ = t.webview.load_url(&search_url);
                                let _ = t.webview.set_bounds(bounds);
                                let _ = t.webview.set_visible(true);
                                let _ = t.webview.focus();
                            } else {
                                let _ = t.webview.set_visible(false);
                            }
                        }
                        *active_tab_id_holder.lock().unwrap() = search_id;
                        sync_url(&toolbar_wv_holder, &search_url);
                        sync_tabs(&toolbar_wv_holder, &tabs, search_id);
                    } else {
                        let new_id = {
                            let mut nid = next_tab_id_holder.lock().unwrap();
                            let id = *nid;
                            *nid += 1;
                            id
                        };
                        println!("[Tailorbird Host] Launching new search tab #{} with URL: {}", new_id, search_url);

                        match make_tab_webview(&window, new_id, &search_url, bounds, true) {
                            Ok(wv) => {
                                let new_tab = BrowserTab {
                                    id: new_id,
                                    title: "Job Search".to_string(),
                                    url: search_url.clone(),
                                    webview: wv,
                                    is_search: true,
                                };

                                for t in tabs.iter_mut() {
                                    let _ = t.webview.set_visible(false);
                                }
                                tabs.push(new_tab);
                                *active_tab_id_holder.lock().unwrap() = new_id;
                                sync_url(&toolbar_wv_holder, &search_url);
                                sync_tabs(&toolbar_wv_holder, &tabs, new_id);
                            }
                            Err(e) => {
                                eprintln!("[Tailorbird Host] Error creating search tab #{}: {:?}", new_id, e);
                            }
                        }
                    }
                }
                AppEvent::SetLeftWidth { width } => {
                    coordinator_for_loop(Some(width));
                }
                AppEvent::SetPrimaryColor { color } => {
                    println!("[Tailorbird EventLoop] SetPrimaryColor applied: {}", color);
                    let mut data = load_stored_data();
                    data.primary_color = Some(color.clone());
                    if let Err(e) = save_stored_data(&data) {
                        eprintln!("[Tailorbird Host] Error saving primary color: {}", e);
                    }

                    // 1. Update Left Control Panel
                    if let Ok(guard) = left_wv_holder.lock() {
                        if let Some(ref left_wv) = *guard {
                            let js = format!(
                                r#"
                                (function() {{
                                    try {{
                                        if (window.applyPrimaryColor) {{
                                            window.applyPrimaryColor({col}, false);
                                        }} else {{
                                            document.documentElement.style.setProperty('--accent', {col});
                                        }}
                                    }} catch (e) {{
                                        console.error('applyPrimaryColor error:', e);
                                    }}
                                }})();
                                "#,
                                col = serde_json::to_string(&color).unwrap_or_default()
                            );
                            if let Err(e) = left_wv.evaluate_script(&js) {
                                eprintln!("[Tailorbird Host] Failed to evaluate applyPrimaryColor on left_wv: {:?}", e);
                            } else {
                                println!("[Tailorbird Host] Successfully dispatched primary color to left_wv: {}", color);
                            }
                        }
                    }

                    // 2. Update Browser Toolbar
                    if let Ok(guard) = toolbar_wv_holder.lock() {
                        if let Some(ref tb) = *guard {
                            let js = format!("if (window.setPrimaryColor) {{ window.setPrimaryColor({}); }}", serde_json::to_string(&color).unwrap_or_default());
                            let _ = tb.evaluate_script(&js);
                        }
                    }

                    // 3. Update all open tabs (settings, hitlist, etc.)
                    if let Ok(guard) = tabs_holder.lock() {
                        let js_settings = format!("if (window.setInitialAccentColor) {{ window.setInitialAccentColor({}); }}", serde_json::to_string(&color).unwrap_or_default());
                        let js_tab = format!("if (window.setPrimaryColor) {{ window.setPrimaryColor({}); }}", serde_json::to_string(&color).unwrap_or_default());
                        for tab in guard.iter() {
                            let _ = tab.webview.evaluate_script(&js_settings);
                            let _ = tab.webview.evaluate_script(&js_tab);
                        }
                    }
                }
                AppEvent::Back => {
                    let cur_active = *active_tab_id_holder.lock().unwrap();
                    let tabs = tabs_holder.lock().unwrap();
                    if let Some(tab) = tabs.iter().find(|t| t.id == cur_active) {
                        let _ = tab.webview.go_back();
                    }
                }
                AppEvent::Forward => {
                    let cur_active = *active_tab_id_holder.lock().unwrap();
                    let tabs = tabs_holder.lock().unwrap();
                    if let Some(tab) = tabs.iter().find(|t| t.id == cur_active) {
                        let _ = tab.webview.go_forward();
                    }
                }
                AppEvent::Reload => {
                    let cur_active = *active_tab_id_holder.lock().unwrap();
                    let mut tabs = tabs_holder.lock().unwrap();
                    if let Some(tab) = tabs.iter_mut().find(|t| t.id == cur_active) {
                        if tab.url == "local://hitlist" || tab.url == "local://roster" {
                            let stored = load_stored_data();
                            let html = prepare_hitlist_html(&stored.hit_list);
                            let _ = tab.webview.load_html(&html);
                            let data_json = serde_json::to_string(&stored.hit_list).unwrap_or_else(|_| "[]".to_string());
                            let js = format!("if (window.setHitListData) {{ window.setHitListData({}); }}", data_json);
                            let _ = tab.webview.evaluate_script(&js);
                        } else if tab.url == "local://settings" {
                            let stored = load_stored_data();
                            let current_color = stored.primary_color.unwrap_or_else(|| "#818CF8".to_string());
                            let html = prepare_settings_html(&current_color);
                            let _ = tab.webview.load_html(&html);
                            let js = format!("if (window.setInitialAccentColor) {{ window.setInitialAccentColor({}); }}", serde_json::to_string(&current_color).unwrap_or_default());
                            let _ = tab.webview.evaluate_script(&js);
                        } else if tab.url == "local://mock" {
                            let _ = tab.webview.load_html(MOCK_JOB_HTML);
                            let stored = load_stored_data();
                            let context_data = serde_json::json!({
                                "candidateProfile": stored.candidate_profile,
                                "specialFields": stored.special_fields,
                            });
                            let js = format!("if (window.__UPDATE_TAILORBIRD_CONTEXT_MENU__) {{ window.__UPDATE_TAILORBIRD_CONTEXT_MENU__({}); }}", context_data);
                            let _ = tab.webview.evaluate_script(&js);
                        } else if tab.url == "local://welcome" {
                            let _ = tab.webview.load_html(WELCOME_HTML);
                        } else {
                            let _ = tab.webview.reload();
                        }
                    }
                }
                AppEvent::Home => {
                    let cur_active = *active_tab_id_holder.lock().unwrap();
                    let mut tabs = tabs_holder.lock().unwrap();
                    if let Some(tab) = tabs.iter_mut().find(|t| t.id == cur_active) {
                        let _ = tab.webview.load_html(WELCOME_HTML);
                        tab.url = "local://welcome".to_string();
                        tab.title = "Welcome".to_string();
                        sync_url(&toolbar_wv_holder, "local://welcome");
                    }
                    sync_tabs(&toolbar_wv_holder, &tabs, cur_active);
                }
                AppEvent::ScrapeCurrentPage => {
                    println!("[Tailorbird Host] Triggering page scraper on active tab...");
                    if let Ok(guard) = tabs_holder.lock() {
                        let active_id = *active_tab_id_holder.lock().unwrap();
                        if let Some(tab) = guard.iter().find(|t| t.id == active_id) {
                            let _ = tab.webview.evaluate_script(SCRAPE_PAGE_SCRIPT);
                        }
                    }
                }
                AppEvent::CloseActiveTab => {
                    let cur_active = *active_tab_id_holder.lock().unwrap();
                    let _ = proxy.send_event(AppEvent::CloseTab { id: cur_active });
                }
                AppEvent::FocusOmnibar => {
                    if let Ok(guard) = toolbar_wv_holder.lock() {
                        if let Some(ref tb) = *guard {
                            let _ = tb.focus();
                            let _ = tb.evaluate_script("if (window.focusOmnibar) { window.focusOmnibar(); }");
                        }
                    }
                }
                AppEvent::FocusActiveTab => {
                    let cur_active = *active_tab_id_holder.lock().unwrap();
                    if let Ok(guard) = tabs_holder.lock() {
                        if let Some(tab) = guard.iter().find(|t| t.id == cur_active) {
                            let _ = tab.webview.focus();
                        }
                    }
                }
                AppEvent::CycleTab { direction } => {
                    let tabs = tabs_holder.lock().unwrap();
                    if tabs.len() > 1 {
                        let cur_active = *active_tab_id_holder.lock().unwrap();
                        if let Some(pos) = tabs.iter().position(|t| t.id == cur_active) {
                            let count = tabs.len() as i32;
                            let next_pos = ((pos as i32 + direction).rem_euclid(count)) as usize;
                            let next_id = tabs[next_pos].id;
                            drop(tabs);
                            let _ = proxy.send_event(AppEvent::SwitchTab { id: next_id });
                        }
                    }
                }
                AppEvent::SwitchTabByIndex { index } => {
                    let tabs = tabs_holder.lock().unwrap();
                    if index >= 1 && index <= tabs.len() {
                        let target_id = tabs[index - 1].id;
                        drop(tabs);
                        let _ = proxy.send_event(AppEvent::SwitchTab { id: target_id });
                    }
                }
            },
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => {
                println!("[Tailorbird] Window close requested, exiting application cleanly.");
                *control_flow = ControlFlow::Exit;
            }
            Event::WindowEvent {
                event: WindowEvent::Resized(physical_size),
                ..
            } => {
                let scale_factor = window.scale_factor();
                if let Ok(mut sf) = scale_factor_holder.lock() {
                    *sf = scale_factor;
                }
                let logical_size = physical_size.to_logical::<f64>(scale_factor);
                if let Ok(mut sz) = window_size_holder.lock() {
                    *sz = (logical_size.width, logical_size.height);
                }
                coordinator_for_loop(None);
            }
            Event::WindowEvent {
                event: WindowEvent::ScaleFactorChanged { scale_factor, .. },
                ..
            } => {
                if let Ok(mut sf) = scale_factor_holder.lock() {
                    *sf = scale_factor;
                }
            }
            _ => {}
        }
    });
}

fn format_input_to_url(input: &str) -> String {
    let trimmed = input.trim();
    if trimmed.is_empty() || trimmed == "about:blank" || trimmed == "local://welcome" || trimmed.eq_ignore_ascii_case("welcome") {
        return "local://welcome".to_string();
    }
    if trimmed == "local://mock" || trimmed.eq_ignore_ascii_case("mock") || trimmed.eq_ignore_ascii_case("test") {
        return "local://mock".to_string();
    }
    if trimmed == "local://roster" || trimmed == "local://hitlist" || trimmed.eq_ignore_ascii_case("roster") || trimmed.eq_ignore_ascii_case("hitlist") {
        return "local://roster".to_string();
    }
    if trimmed == "local://settings" || trimmed.eq_ignore_ascii_case("settings") {
        return "local://settings".to_string();
    }

    // Explicit protocol schemes
    if trimmed.starts_with("http://") || trimmed.starts_with("https://") || trimmed.starts_with("file://") {
        return trimmed.to_string();
    }

    // Whitespace indicates a multi-word search phrase
    if trimmed.contains(char::is_whitespace) {
        return format_search_url(trimmed);
    }

    // Check for localhost or local IP addresses
    if trimmed.starts_with("localhost") || trimmed.starts_with("127.0.0.1") {
        return format!("http://{}", trimmed);
    }

    // Check for domain-like structure: e.g. "google.com", "boards.greenhouse.io/github", "site.org:8080"
    let host_part = match trimmed.split_once('/') {
        Some((host, _)) => host,
        None => trimmed,
    };

    if host_part.contains('.') && !host_part.starts_with('.') && !host_part.ends_with('.') {
        if let Some(tld) = host_part.rsplit('.').next() {
            let tld_clean = tld.split(':').next().unwrap_or(tld);
            if tld_clean.len() >= 2 && tld_clean.chars().all(|c| c.is_alphanumeric()) {
                return format!("https://{}", trimmed);
            }
        }
    }

    // Fall back to search phrase
    format_search_url(trimmed)
}

fn format_search_url(query: &str) -> String {
    let mut encoded = String::new();
    for b in query.bytes() {
        match b {
            b'a'..=b'z' | b'A'..=b'Z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                encoded.push(b as char);
            }
            b' ' => encoded.push('+'),
            _ => encoded.push_str(&format!("%{:02X}", b)),
        }
    }
    format!("https://www.google.com/search?q={}", encoded)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_input_to_url_with_scheme() {
        assert_eq!(format_input_to_url("https://github.com"), "https://github.com");
        assert_eq!(format_input_to_url("http://example.com"), "http://example.com");
        assert_eq!(format_input_to_url("file:///path/to/file"), "file:///path/to/file");
        assert_eq!(format_input_to_url("local://roster"), "local://roster");
        assert_eq!(format_input_to_url("local://settings"), "local://settings");
    }

    #[test]
    fn test_format_input_to_url_domain_without_scheme() {
        assert_eq!(format_input_to_url("google.com"), "https://google.com");
        assert_eq!(format_input_to_url("boards.greenhouse.io/company"), "https://boards.greenhouse.io/company");
    }

    #[test]
    fn test_format_input_to_url_localhost() {
        assert_eq!(format_input_to_url("localhost:3000"), "http://localhost:3000");
        assert_eq!(format_input_to_url("127.0.0.1:8080"), "http://127.0.0.1:8080");
    }

    #[test]
    fn test_format_input_to_url_search_query() {
        let result = format_input_to_url("senior rust engineer remote");
        assert!(result.starts_with("https://www.google.com/search?q="));
        assert!(result.contains("senior+rust+engineer+remote"));
    }

    #[test]
    fn test_decode_percent() {
        assert_eq!(decode_percent("Hello%20World"), "Hello World");
        assert_eq!(decode_percent("%2Fpath%2Fto"), "/path/to");
        assert_eq!(decode_percent("normal_text"), "normal_text");
        assert_eq!(decode_percent("broken%2"), "broken%2");
    }

    #[test]
    fn test_ipc_message_deserialization_navigate() {
        let json = r#"{"action": "NAVIGATE", "url": "https://google.com"}"#;
        let msg: IpcMessage = serde_json::from_str(json).expect("NAVIGATE should deserialize");
        match msg {
            IpcMessage::Navigate { url } => assert_eq!(url, "https://google.com"),
            _ => panic!("Expected Navigate variant"),
        }
    }

    #[test]
    fn test_ipc_message_deserialization_save_data() {
        let json = r##"{
            "action": "SAVE_DATA",
            "candidateProfile": {
                "fullName": "Alice Smith"
            },
            "resumeSource": "",
            "workHistory": [],
            "specialFields": [],
            "prospects": [],
            "searchCriteria": {
                "title": "Frontend Engineer"
            },
            "splitWidth": 450.0,
            "primaryColor": "#3B82F6"
        }"##;

        let msg: IpcMessage = serde_json::from_str(json).expect("SAVE_DATA should deserialize");
        match msg {
            IpcMessage::SaveData { candidate_profile, search_criteria, primary_color, .. } => {
                let prof = candidate_profile.expect("Profile should be present");
                assert_eq!(prof.full_name, "Alice Smith");
                let crit = search_criteria.expect("Search criteria should be present");
                assert_eq!(crit.title, "Frontend Engineer");
                assert_eq!(primary_color, Some("#3B82F6".to_string()));
            }
            _ => panic!("Expected SaveData variant"),
        }
    }

    #[test]
    fn test_ipc_message_deserialization_hit_list() {
        let json_load = r#"{"action": "LOAD_HIT_LIST"}"#;
        let msg_load: IpcMessage = serde_json::from_str(json_load).expect("LOAD_HIT_LIST should deserialize");
        match msg_load {
            IpcMessage::LoadHitList => {}
            _ => panic!("Expected LoadHitList variant"),
        }

        let json_save = r#"{
            "action": "SAVE_HIT_LIST",
            "hitList": [
                {
                    "id": "t-1",
                    "companyName": "Vercel",
                    "websiteUrl": "https://vercel.com",
                    "status": "Targeted"
                }
            ]
        }"#;
        let msg_save: IpcMessage = serde_json::from_str(json_save).expect("SAVE_HIT_LIST should deserialize");
        match msg_save {
            IpcMessage::SaveHitList { hit_list } => {
                assert_eq!(hit_list.len(), 1);
                assert_eq!(hit_list[0].company_name, "Vercel");
            }
            _ => panic!("Expected SaveHitList variant"),
        }
    }

    #[test]
    fn test_prepare_settings_html() {
        let html = prepare_settings_html("#10B981");
        assert!(html.contains("window.__INITIAL_ACCENT_COLOR__ = \"#10B981\""));
    }

    #[test]
    fn test_prepare_hitlist_html() {
        let target = crate::prospect::HitListTarget {
            id: "test-1".to_string(),
            company_name: "Stripe".to_string(),
            ..Default::default()
        };
        let html = prepare_hitlist_html(&[target]);
        assert!(html.contains("window.__INITIAL_HITLIST_DATA__ = ["));
        assert!(html.contains("\"companyName\":\"Stripe\""));
    }
}
