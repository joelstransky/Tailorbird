use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkHistoryEntry {
    pub id: String,
    pub company: String,
    pub role: String,
    pub start_date: String,
    pub end_date: String,
    pub summary: String,
}

/// Reads resume text from either a local file (.pdf, .txt, .md) or a public Google Doc URL.
pub fn load_resume_text(source: &str) -> Result<String, String> {
    let trimmed = source.trim();
    if trimmed.is_empty() {
        return Err("No resume file path or URL provided.".to_string());
    }

    if trimmed.starts_with("http://") || trimmed.starts_with("https://") {
        let export_url = if trimmed.contains("docs.google.com/document/d/") {
            // Convert to Google Docs TXT export endpoint
            // https://docs.google.com/document/d/{DOC_ID}/... -> https://docs.google.com/document/d/{DOC_ID}/export?format=txt
            let parts: Vec<&str> = trimmed.split("/d/").collect();
            if parts.len() > 1 {
                let doc_id = parts[1].split('/').next().unwrap_or("");
                if !doc_id.is_empty() {
                    format!("https://docs.google.com/document/d/{}/export?format=txt", doc_id)
                } else {
                    trimmed.to_string()
                }
            } else {
                trimmed.to_string()
            }
        } else {
            trimmed.to_string()
        };

        match ureq::get(&export_url).call() {
            Ok(resp) => {
                resp.into_string().map_err(|e| format!("Failed to read response text: {}", e))
            }
            Err(e) => Err(format!("Failed to fetch public document from URL: {}", e)),
        }
    } else {
        let path = Path::new(trimmed);
        if !path.exists() {
            return Err(format!("File not found at path: {}", trimmed));
        }

        let ext = path
            .extension()
            .and_then(|s| s.to_str())
            .unwrap_or("")
            .to_lowercase();

        if ext == "pdf" {
            pdf_extract::extract_text(path).map_err(|e| format!("Failed to extract text from PDF: {}", e))
        } else {
            std::fs::read_to_string(path).map_err(|e| format!("Failed to read text file: {}", e))
        }
    }
}

/// Parses unstructured resume text into a structured list of work experience entries.
pub fn parse_work_history(text: &str) -> Vec<WorkHistoryEntry> {
    let mut entries = Vec::new();
    let lines: Vec<&str> = text
        .lines()
        .map(|l| l.trim())
        .filter(|l| !l.is_empty())
        .collect();

    // Check if it's JSON formatted
    if let Ok(json_entries) = serde_json::from_str::<Vec<WorkHistoryEntry>>(text) {
        return json_entries;
    }

    // Heuristic parser: Looks for sections like "EXPERIENCE", "WORK HISTORY", "EMPLOYMENT", etc.
    let mut in_experience_section = false;
    let mut current_company = String::new();
    let mut current_role = String::new();
    let mut current_dates = String::new();
    let mut current_bullets: Vec<String> = Vec::new();
    let mut entry_counter = 1;

    let flush_entry = |entries: &mut Vec<WorkHistoryEntry>, company: &str, role: &str, dates: &str, bullets: &[String], counter: &mut usize| {
        if !company.is_empty() || !role.is_empty() {
            let (start, end) = parse_date_range(dates);
            entries.push(WorkHistoryEntry {
                id: format!("exp-{}", counter),
                company: company.to_string(),
                role: role.to_string(),
                start_date: start,
                end_date: end,
                summary: bullets.join("\n"),
            });
            *counter += 1;
        }
    };

    let exp_keywords = ["EXPERIENCE", "WORK HISTORY", "EMPLOYMENT", "WORK EXPERIENCE", "PROFESSIONAL EXPERIENCE"];
    let end_keywords = ["EDUCATION", "SKILLS", "CERTIFICATIONS", "PROJECTS", "PUBLICATIONS", "AWARDS", "LANGUAGES"];

    for line in &lines {
        let upper = line.to_uppercase();

        if exp_keywords.iter().any(|&k| upper == k || upper.starts_with(k) && upper.len() < 35) {
            in_experience_section = true;
            continue;
        }

        if in_experience_section && end_keywords.iter().any(|&k| upper == k || upper.starts_with(k) && upper.len() < 35) {
            break;
        }

        let contains_date = has_date_pattern(line);

        if in_experience_section || entries.is_empty() {
            if contains_date {
                // Flush previous entry if we have one
                flush_entry(&mut entries, &current_company, &current_role, &current_dates, &current_bullets, &mut entry_counter);
                current_bullets.clear();

                // If line contains date and text (e.g. "Google - Senior Engineer | 2021 - Present")
                let parts: Vec<&str> = line.split(&['|', '•', '—', '-'][..]).collect();
                if parts.len() >= 2 {
                    current_dates = parts.last().unwrap_or(&"").trim().to_string();
                    let prefix = parts[..parts.len() - 1].join(" - ");
                    let title_parts: Vec<&str> = prefix.split(&['@', ',', '-'][..]).collect();
                    if title_parts.len() >= 2 {
                        current_role = title_parts[0].trim().to_string();
                        current_company = title_parts[1].trim().to_string();
                    } else {
                        current_role = prefix.trim().to_string();
                        current_company = "Company".to_string();
                    }
                } else {
                    current_dates = line.to_string();
                }
            } else if line.starts_with('-') || line.starts_with('•') || line.starts_with('*') {
                current_bullets.push(line.trim_start_matches(&['-', '•', '*', ' '][..]).to_string());
            } else if current_role.is_empty() {
                current_role = line.to_string();
            } else if current_company.is_empty() {
                current_company = line.to_string();
            } else {
                current_bullets.push(line.to_string());
            }
        }
    }

    // Flush last entry
    flush_entry(&mut entries, &current_company, &current_role, &current_dates, &current_bullets, &mut entry_counter);

    // If heuristic parser found nothing (e.g. non-standard format), create sensible default entry from first non-empty lines
    if entries.is_empty() && !lines.is_empty() {
        let preview = lines.iter().take(4).cloned().collect::<Vec<&str>>().join(" ");
        entries.push(WorkHistoryEntry {
            id: "exp-1".to_string(),
            company: "Primary Experience".to_string(),
            role: lines.first().cloned().unwrap_or("Software Engineer").to_string(),
            start_date: "2020".to_string(),
            end_date: "Present".to_string(),
            summary: preview,
        });
    }

    entries
}

fn has_date_pattern(line: &str) -> bool {
    let lower = line.to_lowercase();
    lower.contains("present")
        || lower.contains("current")
        || (lower.contains("20") && (lower.contains('-') || lower.contains("to") || lower.contains('–')))
        || (lower.contains("19") && (lower.contains('-') || lower.contains("to")))
}

fn parse_date_range(dates: &str) -> (String, String) {
    let parts: Vec<&str> = dates.split(&['-', '–', '—'][..]).collect();
    if parts.len() >= 2 {
        (parts[0].trim().to_string(), parts[1].trim().to_string())
    } else {
        (dates.trim().to_string(), "Present".to_string())
    }
}
