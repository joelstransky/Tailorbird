mod autofill;
mod prospect;
mod resume;

use std::sync::{Arc, Mutex};
use tao::{
    dpi::{LogicalPosition, LogicalSize, Position, Size},
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoopBuilder, EventLoopProxy},
    window::WindowBuilder,
};
use wry::{
    NewWindowResponse, PageLoadEvent, Rect, WebView, WebViewBuilder, WebViewBuilderExtWindows,
    WebViewExtWindows,
};

use crate::autofill::{generate_autofill_script, generate_context_menu_script, CandidateProfile};
use crate::prospect::{load_stored_data, save_stored_data, AppData, Prospect, SearchCriteria};
use crate::resume::{load_resume_text, parse_work_history, WorkHistoryEntry};

const LEFT_PANE_WIDTH: f64 = 420.0;
const TOOLBAR_HEIGHT: f64 = 78.0;
const DEFAULT_WINDOW_WIDTH: f64 = 1280.0;
const DEFAULT_WINDOW_HEIGHT: f64 = 750.0;

// Embedded HTML assets for zero-dependency standalone execution
const LEFT_PANE_HTML: &str = include_str!("assets/left_pane.html");
const TOOLBAR_HTML: &str = include_str!("assets/toolbar.html");
const MOCK_JOB_HTML: &str = include_str!("assets/mock_job_page.html");
const HITLIST_HTML: &str = include_str!("assets/hitlist.html");
const SETTINGS_HTML: &str = include_str!("assets/settings.html");

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
                if (parts.length > 1) {
                    company = parts[parts.length - 1].split('|')[0].trim();
                }
            } else if (title.includes(' | ')) {
                const parts = title.split(' | ');
                if (parts.length > 1) {
                    company = parts[parts.length - 1].trim();
                }
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

#[derive(Debug)]
enum AppEvent {
    CreateTab { url: String, activate: bool },
    SwitchTab { id: usize },
    CloseTab { id: usize },
    TabTitleChanged { id: usize, title: String },
    TabPageLoaded { id: usize, url: String },
    NavigateActiveTab { url: String },
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
}

struct BrowserTab {
    id: usize,
    title: String,
    url: String,
    webview: WebView,
}

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
    #[serde(rename = "LOAD_HIT_LIST")]
    LoadHitList,
    #[serde(rename = "SAVE_HIT_LIST")]
    SaveHitList {
        #[serde(rename = "hitList")]
        hit_list: Vec<crate::prospect::HitListTarget>,
    },
    #[serde(rename = "OPEN_NEW_TAB")]
    OpenNewTab {
        url: String,
    },
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

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("============================================================");
    println!("  Tailorbird — Dual-Pane Specialized Job-Search Browser");
    println!("============================================================");

    let mut event_loop_builder = EventLoopBuilder::<AppEvent>::with_user_event();
    let event_loop = event_loop_builder.build();
    let proxy: EventLoopProxy<AppEvent> = event_loop.create_proxy();

    let window = WindowBuilder::new()
        .with_title("Tailorbird — Specialized Job Search Browser")
        .with_inner_size(LogicalSize::new(DEFAULT_WINDOW_WIDTH, DEFAULT_WINDOW_HEIGHT))
        .with_min_inner_size(LogicalSize::new(760.0, 480.0))
        .with_visible(true)
        .build(&event_loop)?;

    window.set_focus();
    println!("[Tailorbird] Native parent window created successfully.");

    // Load persisted app data to restore split width if available
    let initial_app_data = load_stored_data();
    let initial_left_width = initial_app_data.split_width.unwrap_or(LEFT_PANE_WIDTH);
    let initial_data_json = serde_json::to_string(&initial_app_data).unwrap_or_else(|_| "{}".to_string());
    let init_script = format!("window.__INITIAL_DATA__ = {};", initial_data_json);
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

    let left_for_ipc = left_wv_holder.clone();
    let left_width_for_ipc = left_width_holder.clone();
    let tabs_for_left = tabs_holder.clone();
    let active_id_for_left = active_tab_id_holder.clone();
    let proxy_for_left = proxy.clone();
    let toolbar_for_left_ipc = toolbar_wv_holder.clone();

    let initial_primary_color = initial_app_data.primary_color.clone().unwrap_or_else(|| "#818CF8".to_string());
    let tb_init_script = format!("window.__INITIAL_PRIMARY_COLOR__ = {};", serde_json::to_string(&initial_primary_color).unwrap_or_default());

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

    let coordinator_for_ipc = layout_coordinator.clone();

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
        .with_html(LEFT_PANE_HTML)
        .with_ipc_handler({
            let left_holder = left_for_ipc.clone();
            let tabs_holder = tabs_for_left.clone();
            let active_id_holder = active_id_for_left.clone();
            let proxy_ipc = proxy_for_left.clone();
            let coord = coordinator_for_ipc.clone();
            let left_w_holder = left_width_for_ipc.clone();
            let toolbar_for_left = toolbar_for_left_ipc.clone();

            move |req| {
                let body = req.body();
                match serde_json::from_str::<IpcMessage>(body) {
                    Ok(IpcMessage::Autofill { data }) => {
                        println!("[Tailorbird Host] Triggering AUTOFILL for: {}", data.full_name);
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
                        println!("[Tailorbird Host] Triggering page scraper on active tab...");
                        if let Ok(guard) = tabs_holder.lock() {
                            let active_id = *active_id_holder.lock().unwrap();
                            if let Some(tab) = guard.iter().find(|t| t.id == active_id) {
                                let _ = tab.webview.evaluate_script(SCRAPE_PAGE_SCRIPT);
                            }
                        }
                    }
                    Ok(IpcMessage::SetLeftWidth { width }) => {
                        println!("[Tailorbird Host] SetLeftWidth received: {:.1}", width);
                        coord(Some(width));
                    }
                    Ok(IpcMessage::SetPrimaryColor { color }) => {
                        println!("[Tailorbird Host] SetPrimaryColor received: {}", color);
                        let mut data = load_stored_data();
                        data.primary_color = Some(color.clone());
                        if let Err(e) = save_stored_data(&data) {
                            eprintln!("[Tailorbird Host] Error saving primary color: {}", e);
                        }
                        if let Ok(guard) = toolbar_for_left.lock() {
                            if let Some(ref tb) = *guard {
                                let js = format!("if (window.setPrimaryColor) {{ window.setPrimaryColor({}); }}", serde_json::to_string(&color).unwrap_or_default());
                                let _ = tb.evaluate_script(&js);
                            }
                        }
                    }
                    Ok(IpcMessage::SaveData {
                        candidate_profile,
                        resume_source,
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
    let shared_env = left_webview.environment();

    let initial_right_width = (DEFAULT_WINDOW_WIDTH - initial_left_width).max(0.0);

    // 2. Initialize Browser Toolbar (Tabs Strip, Back, Forward, Reload, Home, Omnibar)
    let toolbar_webview = WebViewBuilder::new()
        .with_environment(shared_env.clone())
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
                match serde_json::from_str::<IpcMessage>(body) {
                    Ok(IpcMessage::Back) => {
                        let _ = proxy_tb.send_event(AppEvent::Back);
                    }
                    Ok(IpcMessage::Forward) => {
                        let _ = proxy_tb.send_event(AppEvent::Forward);
                    }
                    Ok(IpcMessage::Reload) => {
                        let _ = proxy_tb.send_event(AppEvent::Reload);
                    }
                    Ok(IpcMessage::Home) => {
                        let _ = proxy_tb.send_event(AppEvent::Home);
                    }
                    Ok(IpcMessage::Navigate { url }) => {
                        let _ = proxy_tb.send_event(AppEvent::NavigateActiveTab { url });
                    }
                    Ok(IpcMessage::SwitchTab { id }) => {
                        let _ = proxy_tb.send_event(AppEvent::SwitchTab { id });
                    }
                    Ok(IpcMessage::CloseTab { id }) => {
                        let _ = proxy_tb.send_event(AppEvent::CloseTab { id });
                    }
                    Ok(IpcMessage::NewTab { url }) => {
                        let target_url = url.unwrap_or_else(|| "local://mock".to_string());
                        let _ = proxy_tb.send_event(AppEvent::CreateTab {
                            url: target_url,
                            activate: true,
                        });
                    }
                    _ => {}
                }
            }
        })
        .build_as_child(&window)?;

    // 3. Tab WebView Factory Closure
    let make_tab_webview = {
        let shared_env = shared_env.clone();
        let initial_context_script = initial_context_script.clone();
        let proxy = proxy.clone();
        let left_holder = left_for_ipc.clone();
        let toolbar_holder_tab = toolbar_wv_holder.clone();

        move |win: &tao::window::Window, tab_id: usize, url: &str, bounds: Rect, visible: bool| -> Result<WebView, Box<dyn std::error::Error>> {
            let is_mock = url == "local://mock";
            let is_hitlist = url == "local://hitlist";
            let is_settings = url == "local://settings";
            let proxy_title = proxy.clone();
            let proxy_load = proxy.clone();
            let proxy_new_win = proxy.clone();
            let proxy_ipc = proxy.clone();
            let left_h = left_holder.clone();
            let toolbar_h = toolbar_holder_tab.clone();

            let builder = WebViewBuilder::new()
                .with_environment(shared_env.clone())
                .with_bounds(bounds)
                .with_visible(visible)
                .with_devtools(true)
                .with_initialization_script(&initial_context_script)
                .with_new_window_req_handler({
                    let proxy_new_win = proxy_new_win.clone();
                    move |target_url, _features| {
                        println!("[Tailorbird Tab #{}] New window requested for URL: {}", tab_id, target_url);
                        if !target_url.is_empty() && target_url != "about:blank" {
                            let _ = proxy_new_win.send_event(AppEvent::CreateTab {
                                url: target_url,
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
                    let proxy_ipc = proxy_ipc.clone();
                    move |req| {
                        let body = req.body();
                        match serde_json::from_str::<IpcMessage>(body) {
                            Ok(IpcMessage::RecordPageData { data }) => {
                                println!("[Tailorbird Host] Scraped listing: {} at {}", data.job_title, data.company);
                                if let Ok(guard) = left_h.lock() {
                                    if let Some(ref left_wv) = *guard {
                                        let js = format!("if (window.addProspectRow) {{ window.addProspectRow({}); }}", serde_json::to_string(&data).unwrap_or_default());
                                        let _ = left_wv.evaluate_script(&js);
                                    }
                                }
                            }
                            Ok(IpcMessage::SaveHitList { hit_list }) => {
                                let mut data = load_stored_data();
                                data.hit_list = hit_list;
                                if let Err(e) = save_stored_data(&data) {
                                    eprintln!("[Tailorbird Host] Error saving hit list: {}", e);
                                } else {
                                    println!("[Tailorbird Host] Successfully saved {} hit list targets.", data.hit_list.len());
                                }
                            }
                            Ok(IpcMessage::OpenNewTab { url }) => {
                                let _ = proxy_ipc.send_event(AppEvent::CreateTab {
                                    url,
                                    activate: true,
                                });
                            }
                            Ok(IpcMessage::SetPrimaryColor { color }) => {
                                println!("[Tailorbird Tab IPC] SetPrimaryColor: {}", color);
                                let mut data = load_stored_data();
                                data.primary_color = Some(color.clone());
                                if let Err(e) = save_stored_data(&data) {
                                    eprintln!("[Tailorbird Host] Error saving primary color: {}", e);
                                }
                                if let Ok(guard) = toolbar_h.lock() {
                                    if let Some(ref tb) = *guard {
                                        let js = format!("if (window.setPrimaryColor) {{ window.setPrimaryColor({}); }}", serde_json::to_string(&color).unwrap_or_default());
                                        let _ = tb.evaluate_script(&js);
                                    }
                                }
                                if let Ok(guard) = left_h.lock() {
                                    if let Some(ref left_wv) = *guard {
                                        let js = format!("if (window.applyPrimaryColor) {{ window.applyPrimaryColor({}, false); }}", serde_json::to_string(&color).unwrap_or_default());
                                        let _ = left_wv.evaluate_script(&js);
                                    }
                                }
                            }
                            _ => {}
                        }
                    }
                });

            let webview = if is_mock {
                builder.with_html(MOCK_JOB_HTML).build_as_child(win)?
            } else if is_hitlist {
                builder.with_html(HITLIST_HTML).build_as_child(win)?
            } else if is_settings {
                builder.with_html(SETTINGS_HTML).build_as_child(win)?
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

    // 4. Create Initial Tab (Mock Job Page)
    let initial_target_height = (DEFAULT_WINDOW_HEIGHT - TOOLBAR_HEIGHT).max(0.0);
    let initial_bounds = Rect {
        position: Position::Logical(LogicalPosition::new(initial_left_width, TOOLBAR_HEIGHT)),
        size: Size::Logical(LogicalSize::new(initial_right_width, initial_target_height)),
    };

    let initial_tab_wv = make_tab_webview(&window, 1, "local://mock", initial_bounds, true)?;
    let initial_tab = BrowserTab {
        id: 1,
        title: "Tailorbird Mock Job Listing".to_string(),
        url: "local://mock".to_string(),
        webview: initial_tab_wv,
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
    sync_url(&toolbar_wv_holder, "local://mock");

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

                    // If requesting settings and already open, switch to existing tab
                    let already_open_id = if formatted_url == "local://settings" {
                        let tabs = tabs_holder.lock().unwrap();
                        tabs.iter().find(|t| t.url == "local://settings").map(|t| t.id)
                    } else {
                        None
                    };

                    if let Some(existing_id) = already_open_id {
                        let mut tabs = tabs_holder.lock().unwrap();
                        for t in tabs.iter_mut() {
                            if t.id == existing_id {
                                let _ = t.webview.set_bounds(bounds);
                                let _ = t.webview.set_visible(true);
                                let _ = t.webview.focus();
                            } else {
                                let _ = t.webview.set_visible(false);
                            }
                        }
                        *active_tab_id_holder.lock().unwrap() = existing_id;
                        sync_url(&toolbar_wv_holder, "local://settings");
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
                                let initial_title = if formatted_url == "local://mock" {
                                    "Tailorbird Mock Job Listing".to_string()
                                } else if formatted_url == "local://hitlist" {
                                    "🎯 Hit List".to_string()
                                } else if formatted_url == "local://settings" {
                                    "⚙️ Settings".to_string()
                                } else {
                                    "New Tab".to_string()
                                };

                                let new_tab = BrowserTab {
                                    id: new_id,
                                    title: initial_title,
                                    url: formatted_url.clone(),
                                    webview: wv,
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
                            let _ = t.webview.load_html(MOCK_JOB_HTML);
                            t.url = "local://mock".to_string();
                            t.title = "Tailorbird Mock Job Listing".to_string();
                            t.id
                        } else {
                            1
                        };
                        sync_url(&toolbar_wv_holder, "local://mock");
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
                        tab.url = url.clone();
                        if id == cur_active {
                            sync_url(&toolbar_wv_holder, &url);
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
                        if formatted == "local://mock" {
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
                        } else if formatted == "local://hitlist" {
                            let _ = tab.webview.load_html(HITLIST_HTML);
                            tab.url = "local://hitlist".to_string();
                            tab.title = "🎯 Hit List".to_string();
                            let stored = load_stored_data();
                            let data_json = serde_json::to_string(&stored.hit_list).unwrap_or_else(|_| "[]".to_string());
                            let js = format!("if (window.setHitListData) {{ window.setHitListData({}); }}", data_json);
                            let _ = tab.webview.evaluate_script(&js);
                        } else if formatted == "local://settings" {
                            let _ = tab.webview.load_html(SETTINGS_HTML);
                            tab.url = "local://settings".to_string();
                            tab.title = "⚙️ Settings".to_string();
                            let stored = load_stored_data();
                            let current_color = stored.primary_color.unwrap_or_else(|| "#818CF8".to_string());
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
                    let tabs = tabs_holder.lock().unwrap();
                    if let Some(tab) = tabs.iter().find(|t| t.id == cur_active) {
                        let _ = tab.webview.reload();
                    }
                }
                AppEvent::Home => {
                    let cur_active = *active_tab_id_holder.lock().unwrap();
                    let mut tabs = tabs_holder.lock().unwrap();
                    if let Some(tab) = tabs.iter_mut().find(|t| t.id == cur_active) {
                        let _ = tab.webview.load_html(MOCK_JOB_HTML);
                        tab.url = "local://mock".to_string();
                        tab.title = "Tailorbird Mock Job Listing".to_string();
                        sync_url(&toolbar_wv_holder, "local://mock");
                    }
                    sync_tabs(&toolbar_wv_holder, &tabs, cur_active);
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
                let logical_size = physical_size.to_logical::<f64>(scale_factor);
                if let Ok(mut sz) = window_size_holder.lock() {
                    *sz = (logical_size.width, logical_size.height);
                }
                coordinator_for_loop(None);
            }
            _ => {}
        }
    });
}

fn format_input_to_url(input: &str) -> String {
    let trimmed = input.trim();
    if trimmed.is_empty() || trimmed == "local://mock" {
        return "local://mock".to_string();
    }
    if trimmed == "local://hitlist" || trimmed.eq_ignore_ascii_case("hitlist") {
        return "local://hitlist".to_string();
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
