mod autofill;
mod prospect;
mod resume;

use std::sync::{Arc, Mutex};
use tao::{
    dpi::{LogicalPosition, LogicalSize, Position, Size},
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
use wry::{PageLoadEvent, Rect, WebView, WebViewBuilder, WebViewBuilderExtWindows, WebViewExtWindows};

use crate::autofill::{generate_autofill_script, CandidateProfile};
use crate::prospect::{load_stored_data, save_stored_data, AppData, Prospect};
use crate::resume::{load_resume_text, parse_work_history, WorkHistoryEntry};

const LEFT_PANE_WIDTH: f64 = 420.0;
const TOOLBAR_HEIGHT: f64 = 44.0;
const DEFAULT_WINDOW_WIDTH: f64 = 1280.0;
const DEFAULT_WINDOW_HEIGHT: f64 = 750.0;

// Embedded HTML assets for zero-dependency standalone execution
const LEFT_PANE_HTML: &str = include_str!("assets/left_pane.html");
const TOOLBAR_HTML: &str = include_str!("assets/toolbar.html");
const MOCK_JOB_HTML: &str = include_str!("assets/mock_job_page.html");

const SCRAPE_PAGE_SCRIPT: &str = r#"(function() {
    try {
        const url = window.location.href;
        const title = document.title || '';

        const ogTitle = document.querySelector('meta[property="og:title"]')?.content;
        const ogSite = document.querySelector('meta[property="og:site_name"]')?.content;

        const ghTitle = document.querySelector('.app-title')?.innerText?.trim();
        const ghCompany = document.querySelector('.company-name')?.innerText?.trim();

        const leverTitle = document.querySelector('.posting-headline h2')?.innerText?.trim();
        const ashbyTitle = document.querySelector('h1')?.innerText?.trim();
        const h1 = document.querySelector('h1')?.innerText?.trim();

        let jobTitle = ghTitle || leverTitle || ashbyTitle || h1 || ogTitle || title;
        let company = ghCompany || ogSite || '';

        if (!company) {
            if (title.includes(' at ')) {
                company = title.split(' at ')[1].split(/[-–|]/)[0].trim();
            } else if (title.includes(' - ')) {
                const parts = title.split(' - ');
                if (parts.length > 1) company = parts[1].trim();
            } else {
                const host = window.location.hostname.replace('www.', '');
                company = host.split('.')[0];
                if (company) company = company.charAt(0).toUpperCase() + company.slice(1);
            }
        }

        if (jobTitle.includes(' at ')) {
            jobTitle = jobTitle.split(' at ')[0].trim();
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

#[derive(Debug, serde::Deserialize)]
#[serde(tag = "action")]
enum IpcMessage {
    #[serde(rename = "AUTOFILL")]
    Autofill { data: CandidateProfile },
    #[serde(rename = "NAVIGATE")]
    Navigate { url: String },
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
    #[serde(rename = "IMPORT_RESUME")]
    ImportResume { source: String },
    #[serde(rename = "SCRAPE_CURRENT_PAGE")]
    ScrapeCurrentPage,
    #[serde(rename = "RECORD_PAGE_DATA")]
    RecordPageData { data: ScrapedData },
    #[serde(rename = "SAVE_DATA")]
    SaveData {
        #[serde(rename = "resumeSource")]
        resume_source: String,
        #[serde(rename = "workHistory")]
        work_history: Vec<WorkHistoryEntry>,
        prospects: Vec<Prospect>,
    },
    #[serde(rename = "LOAD_DATA")]
    LoadData,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
struct ScrapedData {
    url: String,
    job_title: String,
    company: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("============================================================");
    println!("  Tailorbird — Dual-Pane Specialized Job-Search Browser");
    println!("============================================================");

    let event_loop = EventLoop::new();

    let window = WindowBuilder::new()
        .with_title("Tailorbird — Specialized Job Search Browser")
        .with_inner_size(LogicalSize::new(DEFAULT_WINDOW_WIDTH, DEFAULT_WINDOW_HEIGHT))
        .with_min_inner_size(LogicalSize::new(760.0, 480.0))
        .with_visible(true)
        .build(&event_loop)?;

    window.set_focus();
    println!("[Tailorbird] Native parent window created successfully.");

    // Shared references across IPC handlers and event loop
    let left_wv_holder: Arc<Mutex<Option<WebView>>> = Arc::new(Mutex::new(None));
    let toolbar_wv_holder: Arc<Mutex<Option<WebView>>> = Arc::new(Mutex::new(None));
    let target_wv_holder: Arc<Mutex<Option<WebView>>> = Arc::new(Mutex::new(None));

    let left_for_ipc = left_wv_holder.clone();
    let toolbar_for_ipc = toolbar_wv_holder.clone();
    let target_for_ipc = target_wv_holder.clone();

    // Helper to perform navigation on the target webview and update the toolbar UI
    let do_navigate = {
        let target_holder = target_for_ipc.clone();
        let toolbar_holder = toolbar_for_ipc.clone();

        move |input: &str| {
            let formatted_url = format_input_to_url(input);
            println!("[Tailorbird Host] Navigating to: {} (from input: '{}')", formatted_url, input);

            if let Ok(guard) = target_holder.lock() {
                if let Some(ref target_wv) = *guard {
                    if formatted_url == "local://mock" {
                        let _ = target_wv.load_html(MOCK_JOB_HTML);
                    } else {
                        let _ = target_wv.load_url(&formatted_url);
                    }
                }
            }

            // Sync URL to Toolbar
            if let Ok(guard) = toolbar_holder.lock() {
                if let Some(ref tb_wv) = *guard {
                    let js = format!("if (window.setUrl) {{ window.setUrl({}); }}", serde_json::to_string(&formatted_url).unwrap_or_default());
                    let _ = tb_wv.evaluate_script(&js);
                }
            }
        }
    };

    // 1. Initialize Left Pane (Control Panel: Profile, Resume/Work History, Prospects, Finder)
    let left_webview = WebViewBuilder::new()
        .with_bounds(Rect {
            position: Position::Logical(LogicalPosition::new(0.0, 0.0)),
            size: Size::Logical(LogicalSize::new(LEFT_PANE_WIDTH, DEFAULT_WINDOW_HEIGHT)),
        })
        .with_devtools(true)
        .with_html(LEFT_PANE_HTML)
        .with_ipc_handler({
            let target_holder = target_for_ipc.clone();
            let left_holder = left_for_ipc.clone();
            let do_nav = do_navigate.clone();

            move |req| {
                let body = req.body();
                match serde_json::from_str::<IpcMessage>(body) {
                    Ok(IpcMessage::Autofill { data }) => {
                        println!("[Tailorbird Host] Triggering AUTOFILL for: {}", data.full_name);
                        let injection_script = generate_autofill_script(&data);

                        if let Ok(guard) = target_holder.lock() {
                            if let Some(ref target_wv) = *guard {
                                if let Err(e) = target_wv.evaluate_script(&injection_script) {
                                    eprintln!("[Tailorbird Host] Script injection error: {:?}", e);
                                } else {
                                    println!("[Tailorbird Host] Autofill script injected into target pane.");
                                }
                            }
                        }
                    }
                    Ok(IpcMessage::Navigate { url }) => {
                        do_nav(&url);
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
                                        let js = format!("alert('Failed to read resume: {}');", err.replace('\'', "\\'"));
                                        let _ = left_wv.evaluate_script(&js);
                                    }
                                }
                            }
                        }
                    }
                    Ok(IpcMessage::ScrapeCurrentPage) => {
                        println!("[Tailorbird Host] Triggering page scraper on target webview...");
                        if let Ok(guard) = target_holder.lock() {
                            if let Some(ref target_wv) = *guard {
                                let _ = target_wv.evaluate_script(SCRAPE_PAGE_SCRIPT);
                            }
                        }
                    }
                    Ok(IpcMessage::SaveData { resume_source, work_history, prospects }) => {
                        let data = AppData {
                            resume_source,
                            work_history,
                            prospects,
                        };
                        if let Err(e) = save_stored_data(&data) {
                            eprintln!("[Tailorbird Host] Error saving app data: {}", e);
                        }
                    }
                    Ok(IpcMessage::LoadData) => {
                        let data = load_stored_data();
                        if let Ok(guard) = left_holder.lock() {
                            if let Some(ref left_wv) = *guard {
                                if !data.resume_source.is_empty() {
                                    let js = format!("if (window.setResumePath) {{ window.setResumePath({}); }}", serde_json::to_string(&data.resume_source).unwrap_or_default());
                                    let _ = left_wv.evaluate_script(&js);
                                }
                                if !data.work_history.is_empty() {
                                    let js = format!("if (window.setWorkHistory) {{ window.setWorkHistory({}); }}", serde_json::to_string(&data.work_history).unwrap_or_default());
                                    let _ = left_wv.evaluate_script(&js);
                                }
                                if !data.prospects.is_empty() {
                                    let js = format!("if (window.setProspects) {{ window.setProspects({}); }}", serde_json::to_string(&data.prospects).unwrap_or_default());
                                    let _ = left_wv.evaluate_script(&js);
                                }
                            }
                        }
                    }
                    _ => {}
                }
            }
        })
        .build_as_child(&window)?;

    // Share WebView2 environment on Windows across all webviews
    let shared_env = left_webview.environment();

    let initial_right_width = (DEFAULT_WINDOW_WIDTH - LEFT_PANE_WIDTH).max(0.0);

    // 2. Initialize Browser Toolbar (Back, Forward, Reload, Home, Omnibar)
    let toolbar_webview = WebViewBuilder::new()
        .with_environment(shared_env.clone())
        .with_bounds(Rect {
            position: Position::Logical(LogicalPosition::new(LEFT_PANE_WIDTH, 0.0)),
            size: Size::Logical(LogicalSize::new(initial_right_width, TOOLBAR_HEIGHT)),
        })
        .with_devtools(true)
        .with_html(TOOLBAR_HTML)
        .with_ipc_handler({
            let target_holder = target_for_ipc.clone();
            let do_nav = do_navigate.clone();

            move |req| {
                let body = req.body();
                match serde_json::from_str::<IpcMessage>(body) {
                    Ok(IpcMessage::Back) => {
                        if let Ok(guard) = target_holder.lock() {
                            if let Some(ref target_wv) = *guard {
                                let _ = target_wv.go_back();
                            }
                        }
                    }
                    Ok(IpcMessage::Forward) => {
                        if let Ok(guard) = target_holder.lock() {
                            if let Some(ref target_wv) = *guard {
                                let _ = target_wv.go_forward();
                            }
                        }
                    }
                    Ok(IpcMessage::Reload) => {
                        if let Ok(guard) = target_holder.lock() {
                            if let Some(ref target_wv) = *guard {
                                let _ = target_wv.reload();
                            }
                        }
                    }
                    Ok(IpcMessage::Home) => {
                        do_nav("local://mock");
                    }
                    Ok(IpcMessage::Navigate { url }) => {
                        do_nav(&url);
                    }
                    _ => {}
                }
            }
        })
        .build_as_child(&window)?;

    // 3. Initialize Right Target Pane (The job site / mock application form)
    let initial_target_height = (DEFAULT_WINDOW_HEIGHT - TOOLBAR_HEIGHT).max(0.0);
    let target_webview = WebViewBuilder::new()
        .with_environment(shared_env)
        .with_bounds(Rect {
            position: Position::Logical(LogicalPosition::new(LEFT_PANE_WIDTH, TOOLBAR_HEIGHT)),
            size: Size::Logical(LogicalSize::new(initial_right_width, initial_target_height)),
        })
        .with_devtools(true)
        .with_html(MOCK_JOB_HTML)
        .with_ipc_handler({
            let left_holder = left_for_ipc.clone();
            move |req| {
                let body = req.body();
                if let Ok(IpcMessage::RecordPageData { data }) = serde_json::from_str::<IpcMessage>(body) {
                    println!("[Tailorbird Host] Scraped listing: {} at {}", data.job_title, data.company);
                    if let Ok(guard) = left_holder.lock() {
                        if let Some(ref left_wv) = *guard {
                            let js = format!("if (window.addProspectRow) {{ window.addProspectRow({}); }}", serde_json::to_string(&data).unwrap_or_default());
                            let _ = left_wv.evaluate_script(&js);
                        }
                    }
                }
            }
        })
        .with_on_page_load_handler({
            let toolbar_holder = toolbar_wv_holder.clone();
            move |event, url| {
                if let PageLoadEvent::Finished = event {
                    if let Ok(guard) = toolbar_holder.lock() {
                        if let Some(ref tb_wv) = *guard {
                            let js = format!("if (window.setUrl) {{ window.setUrl({}); }}", serde_json::to_string(&url).unwrap_or_default());
                            let _ = tb_wv.evaluate_script(&js);
                        }
                    }
                }
            }
        })
        .build_as_child(&window)?;

    // Store holders
    *left_wv_holder.lock().unwrap() = Some(left_webview);
    *toolbar_wv_holder.lock().unwrap() = Some(toolbar_webview);
    *target_wv_holder.lock().unwrap() = Some(target_webview);

    println!("[Tailorbird] All panes and browser toolbar ready! Event loop running.");

    // Event loop management
    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::WindowEvent {
                event: WindowEvent::Resized(physical_size),
                ..
            } => {
                let scale_factor = window.scale_factor();
                let logical_size = physical_size.to_logical::<f64>(scale_factor);

                let right_width = (logical_size.width - LEFT_PANE_WIDTH).max(0.0);
                let height = logical_size.height;
                let target_height = (height - TOOLBAR_HEIGHT).max(0.0);

                // 1. Update Left Sidebar: fixed width, full window height
                if let Ok(guard) = left_wv_holder.lock() {
                    if let Some(ref left_wv) = *guard {
                        let _ = left_wv.set_bounds(Rect {
                            position: Position::Logical(LogicalPosition::new(0.0, 0.0)),
                            size: Size::Logical(LogicalSize::new(LEFT_PANE_WIDTH, height)),
                        });
                    }
                }

                // 2. Update Browser Toolbar: spans right width, fixed 44px height at top
                if let Ok(guard) = toolbar_wv_holder.lock() {
                    if let Some(ref tb_wv) = *guard {
                        let _ = tb_wv.set_bounds(Rect {
                            position: Position::Logical(LogicalPosition::new(LEFT_PANE_WIDTH, 0.0)),
                            size: Size::Logical(LogicalSize::new(right_width, TOOLBAR_HEIGHT)),
                        });
                    }
                }

                // 3. Update Target Pane: spans right width, fills height beneath toolbar
                if let Ok(guard) = target_wv_holder.lock() {
                    if let Some(ref target_wv) = *guard {
                        let _ = target_wv.set_bounds(Rect {
                            position: Position::Logical(LogicalPosition::new(LEFT_PANE_WIDTH, TOOLBAR_HEIGHT)),
                            size: Size::Logical(LogicalSize::new(right_width, target_height)),
                        });
                    }
                }
            }

            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => {
                println!("[Tailorbird] Close requested. Exiting.");
                *control_flow = ControlFlow::Exit;
            }

            _ => ()
        }
    });
}

fn format_input_to_url(input: &str) -> String {
    let trimmed = input.trim();
    if trimmed.is_empty() || trimmed == "local://mock" {
        return "local://mock".to_string();
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
