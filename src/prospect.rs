use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

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
pub struct AppData {
    pub resume_source: String,
    pub work_history: Vec<WorkHistoryEntry>,
    pub prospects: Vec<Prospect>,
    #[serde(default)]
    pub split_width: Option<f64>,
}

fn storage_file_path() -> PathBuf {
    // Save in the current working directory for easy access & portability
    PathBuf::from("tailorbird_data.json")
}

pub fn load_stored_data() -> AppData {
    let path = storage_file_path();
    if path.exists() {
        if let Ok(content) = fs::read_to_string(&path) {
            if let Ok(data) = serde_json::from_str::<AppData>(&content) {
                return data;
            }
        }
    }
    AppData::default()
}

pub fn save_stored_data(data: &AppData) -> Result<(), String> {
    let path = storage_file_path();
    let json = serde_json::to_string_pretty(data).map_err(|e| e.to_string())?;
    fs::write(&path, json).map_err(|e| e.to_string())?;
    Ok(())
}
