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
pub struct AppData {
    #[serde(default)]
    pub candidate_profile: Option<CandidateProfile>,
    #[serde(default)]
    pub resume_source: String,
    #[serde(default)]
    pub work_history: Vec<WorkHistoryEntry>,
    #[serde(default)]
    pub prospects: Vec<Prospect>,
    #[serde(default)]
    pub search_criteria: Option<SearchCriteria>,
    #[serde(default)]
    pub split_width: Option<f64>,
}

fn storage_file_path() -> PathBuf {
    // 1. Current working directory
    let local = PathBuf::from("tailorbird_data.json");
    if local.exists() {
        return local;
    }
    // 2. Next to running binary
    if let Ok(exe_path) = std::env::current_exe() {
        if let Some(exe_dir) = exe_path.parent() {
            let exe_local = exe_dir.join("tailorbird_data.json");
            if exe_local.exists() {
                return exe_local;
            }
        }
    }
    local
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
                        "[Tailorbird Storage] Loaded data successfully: {} history entries, {} prospects.",
                        data.work_history.len(),
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
        "[Tailorbird Storage] Saving data to {:?}: {} history entries, {} prospects.",
        path,
        data.work_history.len(),
        data.prospects.len()
    );
    let json = serde_json::to_string_pretty(data).map_err(|e| e.to_string())?;
    fs::write(&path, json).map_err(|e| e.to_string())?;
    Ok(())
}
