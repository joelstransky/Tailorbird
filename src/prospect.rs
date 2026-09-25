use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

use crate::autofill::CandidateProfile;
use crate::resume::WorkHistoryEntry;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Prospect {
    pub id: String,
    pub company: String,
    pub job_title: String,
    pub url: String,
    pub status: String,
    pub date_added: String,
    #[serde(default)]
    pub notes: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SearchCriteria {
    #[serde(default)]
    pub title: String,
    #[serde(default)]
    pub location: String,
    #[serde(default)]
    pub greenhouse: bool,
    #[serde(default)]
    pub lever: bool,
    #[serde(default)]
    pub linkedin: bool,
    #[serde(default)]
    pub indeed: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SpecialField {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub label: String,
    #[serde(default)]
    pub content: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct HitListContact {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub title: String,
    #[serde(default)]
    pub linkedin_url: String,
    #[serde(default)]
    pub email: String,
    #[serde(default)]
    pub twitter_url: String,
    #[serde(default)]
    pub github_url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct HitListTarget {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub company_name: String,
    #[serde(default)]
    pub website_url: String,
    #[serde(default)]
    pub industry: String,
    #[serde(default)]
    pub status: String,
    #[serde(default)]
    pub contact: HitListContact,
    #[serde(default)]
    pub value_angle: String,
    #[serde(default)]
    pub draft_message: String,
    #[serde(default)]
    pub notes: String,
    #[serde(default)]
    pub date_added: String,
    #[serde(default)]
    pub last_contacted: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct AppData {
    #[serde(default)]
    pub candidate_profile: Option<CandidateProfile>,
    #[serde(default)]
    pub resume_source: String,
    #[serde(default)]
    pub cover_letter_source: String,
    #[serde(default)]
    pub work_history: Vec<WorkHistoryEntry>,
    #[serde(default)]
    pub special_fields: Vec<SpecialField>,
    #[serde(default)]
    pub prospects: Vec<Prospect>,
    #[serde(default)]
    pub search_criteria: Option<SearchCriteria>,
    #[serde(default)]
    pub split_width: Option<f64>,
    #[serde(default)]
    pub drawer_states: Option<std::collections::HashMap<String, bool>>,
    #[serde(default)]
    pub hit_list: Vec<HitListTarget>,
    #[serde(default)]
    pub primary_color: Option<String>,
}

fn storage_file_path() -> PathBuf {
    // 1. In debug/development mode only, check current working directory for local testing
    #[cfg(debug_assertions)]
    {
        let local = PathBuf::from("tailorbird_data.json");
        if local.exists() {
            return local;
        }
    }

    // 2. Portable mode: check next to running binary
    if let Ok(exe_path) = std::env::current_exe() {
        if let Some(exe_dir) = exe_path.parent() {
            let exe_local = exe_dir.join("tailorbird_data.json");
            if exe_local.exists() {
                return exe_local;
            }
        }
    }

    // 3. Platform-specific user data directories (for installed applications):
    // Windows: %APPDATA%\Tailorbird
    #[cfg(target_os = "windows")]
    if let Ok(appdata) = std::env::var("APPDATA") {
        let app_dir = PathBuf::from(appdata).join("Tailorbird");
        let _ = fs::create_dir_all(&app_dir);
        return app_dir.join("tailorbird_data.json");
    }

    // macOS: ~/Library/Application Support/Tailorbird
    #[cfg(target_os = "macos")]
    if let Ok(home) = std::env::var("HOME") {
        let app_dir = PathBuf::from(home)
            .join("Library")
            .join("Application Support")
            .join("Tailorbird");
        let _ = fs::create_dir_all(&app_dir);
        return app_dir.join("tailorbird_data.json");
    }

    // Linux (Arch, Debian, etc.): $XDG_CONFIG_HOME/tailorbird or ~/.config/tailorbird
    #[cfg(target_os = "linux")]
    {
        let config_dir = std::env::var("XDG_CONFIG_HOME")
            .map(PathBuf::from)
            .unwrap_or_else(|_| {
                let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
                PathBuf::from(home).join(".config")
            });
        let app_dir = config_dir.join("tailorbird");
        let _ = fs::create_dir_all(&app_dir);
        return app_dir.join("tailorbird_data.json");
    }

    PathBuf::from("tailorbird_data.json")
}

pub fn load_stored_data() -> AppData {
    let path = storage_file_path();
    let display_path = path.canonicalize().unwrap_or_else(|_| path.clone());
    println!("[Tailorbird Storage] Checking app data file at: {:?}", display_path);

    if path.exists() {
        match fs::read_to_string(&path) {
            Ok(content) => match serde_json::from_str::<AppData>(&content) {
                Ok(data) => {
                    println!(
                        "[Tailorbird Storage] Loaded data successfully: {} history entries, {} special fields, {} prospects.",
                        data.work_history.len(),
                        data.special_fields.len(),
                        data.prospects.len()
                    );
                    return data;
                }
                Err(err) => eprintln!("[Tailorbird Storage] Error parsing JSON from {:?}: {}", path, err),
            },
            Err(err) => eprintln!("[Tailorbird Storage] Error reading file {:?}: {}", path, err),
        }
    } else {
        println!("[Tailorbird Storage] No data file found at {:?}. Will create on save.", display_path);
    }
    AppData::default()
}

pub fn save_stored_data(data: &AppData) -> Result<(), String> {
    let path = storage_file_path();
    println!(
        "[Tailorbird Storage] Saving data to {:?}: {} history entries, {} special fields, {} prospects.",
        path,
        data.work_history.len(),
        data.special_fields.len(),
        data.prospects.len()
    );
    let json = serde_json::to_string_pretty(data).map_err(|e| e.to_string())?;
    fs::write(&path, json).map_err(|e| e.to_string())?;
    Ok(())
}
