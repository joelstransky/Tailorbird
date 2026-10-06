use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

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
    #[serde(default, alias = "outreach_roster", alias = "outreachRoster", alias = "roster")]
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
        if let Ok(xdg) = std::env::var("XDG_CONFIG_HOME") {
            let app_dir = PathBuf::from(xdg).join("tailorbird");
            let _ = fs::create_dir_all(&app_dir);
            return app_dir.join("tailorbird_data.json");
        } else if let Ok(home) = std::env::var("HOME") {
            let app_dir = PathBuf::from(home).join(".config").join("tailorbird");
            let _ = fs::create_dir_all(&app_dir);
            return app_dir.join("tailorbird_data.json");
        }
    }

    PathBuf::from("tailorbird_data.json")
}

pub fn load_stored_data_from_path(path: &Path) -> AppData {
    let display_path = path.canonicalize().unwrap_or_else(|_| path.to_path_buf());
    println!("[Tailorbird Storage] Checking app data file at: {:?}", display_path);

    if path.exists() {
        match fs::read_to_string(path) {
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

pub fn save_stored_data_to_path(data: &AppData, path: &Path) -> Result<(), String> {
    println!(
        "[Tailorbird Storage] Saving data to {:?}: {} history entries, {} special fields, {} prospects.",
        path,
        data.work_history.len(),
        data.special_fields.len(),
        data.prospects.len()
    );
    let json = serde_json::to_string_pretty(data).map_err(|e| e.to_string())?;
    fs::write(path, json).map_err(|e| e.to_string())?;
    Ok(())
}

pub fn load_stored_data() -> AppData {
    load_stored_data_from_path(&storage_file_path())
}

pub fn save_stored_data(data: &AppData) -> Result<(), String> {
    save_stored_data_to_path(data, &storage_file_path())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_app_data_default_is_empty() {
        let app_data = AppData::default();
        assert!(app_data.candidate_profile.is_none());
        assert!(app_data.prospects.is_empty());
        assert!(app_data.hit_list.is_empty());
        assert!(app_data.work_history.is_empty());
        assert!(app_data.special_fields.is_empty());
    }

    #[test]
    fn test_app_data_deserialization_from_empty_json() {
        let json = "{}";
        let parsed: AppData = serde_json::from_str(json).expect("Empty JSON should parse with defaults");
        assert!(parsed.candidate_profile.is_none());
        assert!(parsed.prospects.is_empty());
        assert!(parsed.hit_list.is_empty());
    }

    #[test]
    fn test_app_data_backward_compatibility_with_partial_fields() {
        let json = r#"{
            "candidateProfile": {
                "fullName": "Barnaby Featherstitch",
                "email": "barnaby@canopy.forest"
            },
            "resumeSource": "/path/to/nest_tailoring_resume.pdf",
            "prospects": [
                {
                    "id": "p-1",
                    "company": "Bramble & Burlap Guild",
                    "jobTitle": "Master Twig Weaver",
                    "url": "https://bramble-burlap.forest/perch/1",
                    "status": "Applied",
                    "dateAdded": "2026-10-01"
                }
            ]
        }"#;

        let parsed: AppData = serde_json::from_str(json).expect("Partial legacy JSON should deserialize");
        let profile = parsed.candidate_profile.expect("Profile should be present");
        assert_eq!(profile.full_name, "Barnaby Featherstitch");
        assert_eq!(profile.email, "barnaby@canopy.forest");
        assert_eq!(parsed.prospects.len(), 1);
        assert_eq!(parsed.prospects[0].company, "Bramble & Burlap Guild");
        assert!(parsed.hit_list.is_empty());
        assert!(parsed.special_fields.is_empty());
    }

    #[test]
    fn test_hit_list_aliases_deserialization() {
        // Test that 'outreachRoster' alias correctly maps to hit_list
        let json = r#"{
            "outreachRoster": [
                {
                    "id": "h-1",
                    "companyName": "Canopy & Cobwebs Haberdashery",
                    "websiteUrl": "https://canopy-cobwebs.forest",
                    "industry": "Fine Leaf Weaving",
                    "status": "Targeted",
                    "contact": {
                        "name": "Penelope Plumage",
                        "title": "Head of Twig Architecture"
                    }
                }
            ]
        }"#;

        let parsed: AppData = serde_json::from_str(json).expect("Alias outreachRoster should deserialize");
        assert_eq!(parsed.hit_list.len(), 1);
        assert_eq!(parsed.hit_list[0].company_name, "Canopy & Cobwebs Haberdashery");
        assert_eq!(parsed.hit_list[0].contact.name, "Penelope Plumage");
    }

    #[test]
    fn test_special_field_roundtrip() {
        let field = SpecialField {
            id: "sp-1".to_string(),
            label: "Cover Note".to_string(),
            content: "Excited about Tailorbird!".to_string(),
        };

        let json = serde_json::to_string(&field).expect("Serialization should succeed");
        let decoded: SpecialField = serde_json::from_str(&json).expect("Deserialization should succeed");
        assert_eq!(decoded.id, "sp-1");
        assert_eq!(decoded.label, "Cover Note");
        assert_eq!(decoded.content, "Excited about Tailorbird!");
    }

    #[test]
    fn test_save_and_load_filesystem_roundtrip() {
        let temp_dir = std::env::temp_dir();
        let unique_name = format!(
            "tailorbird_test_{}.json",
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        );
        let test_file = temp_dir.join(unique_name);

        let test_data = AppData {
            candidate_profile: Some(CandidateProfile {
                full_name: "Robin Needlewing".to_string(),
                email: "robin@nest.forest".to_string(),
                phone: "(555) CHIRP-02".to_string(),
                ..Default::default()
            }),
            search_criteria: Some(SearchCriteria {
                title: "Chief Twig Weaver".to_string(),
                location: "High Canopy".to_string(),
                ..Default::default()
            }),
            prospects: vec![Prospect {
                id: "p-test-1".to_string(),
                company: "Great Oak Guild".to_string(),
                job_title: "Senior Leaf Stitcher".to_string(),
                url: "https://great-oak.forest/perch/1".to_string(),
                status: "Applied".to_string(),
                date_added: "2026-10-02".to_string(),
                notes: "Notes on spider-silk tensile strength and leaf alignment".to_string(),
            }],
            ..Default::default()
        };

        // Test save
        let save_res = save_stored_data_to_path(&test_data, &test_file);
        assert!(save_res.is_ok(), "Saving data to temp file should succeed");

        // Test load
        let loaded = load_stored_data_from_path(&test_file);
        assert_eq!(loaded.prospects.len(), 1);
        assert_eq!(loaded.prospects[0].company, "Great Oak Guild");
        assert_eq!(loaded.prospects[0].notes, "Notes on spider-silk tensile strength and leaf alignment");
        assert!(loaded.candidate_profile.is_some());
        assert_eq!(loaded.candidate_profile.as_ref().unwrap().full_name, "Robin Needlewing");

        // Clean up
        let _ = fs::remove_file(&test_file);
    }
}
