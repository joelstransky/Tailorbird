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
        } else if ext == "docx" || ext == "doc" {
            Err("Word documents (.docx / .doc) are binary formats that cannot be read as plain text. Please export or save your resume as PDF, TXT, or Markdown, or link a public Google Doc URL.".to_string())
        } else {
            std::fs::read_to_string(path).map_err(|e| format!("Failed to read text file: {}", e))
        }
    }
}

fn is_bullet_line(line: &str) -> bool {
    let trimmed = line.trim_start();
    if trimmed.is_empty() {
        return false;
    }
    let bullet_chars = ['•', '○', '●', '◦', '▪', '▫', '■', '*', '‣', '⁃', '►', '✓', '✔'];
    if bullet_chars.iter().any(|&c| trimmed.starts_with(c)) {
        return true;
    }
    if trimmed.starts_with('-') {
        let after = trimmed.trim_start_matches('-');
        if after.starts_with(' ') || after.starts_with('\t') {
            return true;
        }
    }
    false
}

fn clean_bullet(line: &str) -> String {
    let mut s = line.trim();
    while let Some(first_char) = s.chars().next() {
        if ['•', '○', '●', '◦', '▪', '▫', '■', '*', '‣', '⁃', '►', '✓', '✔', ' ', '\t'].contains(&first_char) {
            s = &s[first_char.len_utf8()..];
        } else if s.starts_with("- ") {
            s = &s[2..];
        } else {
            break;
        }
    }
    s.trim().to_string()
}

fn has_4digit_year(s: &str) -> bool {
    let bytes = s.as_bytes();
    if bytes.len() < 4 {
        return false;
    }
    for i in 0..=(bytes.len() - 4) {
        if (bytes[i] == b'1' && bytes[i + 1] == b'9' && bytes[i + 2].is_ascii_digit() && bytes[i + 3].is_ascii_digit())
            || (bytes[i] == b'2' && bytes[i + 1] == b'0' && bytes[i + 2].is_ascii_digit() && bytes[i + 3].is_ascii_digit())
        {
            let prev_digit = if i > 0 { bytes[i - 1].is_ascii_digit() } else { false };
            let next_digit = if i + 4 < bytes.len() { bytes[i + 4].is_ascii_digit() } else { false };
            if !prev_digit && !next_digit {
                return true;
            }
        }
    }
    false
}

fn has_month_name(s: &str) -> bool {
    let lower = s.to_lowercase();
    let months = [
        "jan", "feb", "mar", "apr", "may", "jun",
        "jul", "aug", "sep", "oct", "nov", "dec",
        "january", "february", "march", "april", "june",
        "july", "august", "september", "october", "november", "december"
    ];
    for word in lower.split(|c: char| !c.is_alphabetic()) {
        if months.contains(&word) {
            return true;
        }
    }
    false
}

fn has_date_pattern(line: &str) -> bool {
    if is_bullet_line(line) {
        return false;
    }
    let lower = line.to_lowercase();
    let has_present = lower.contains("present") || lower.contains("current");
    let has_yr = has_4digit_year(line);
    let has_mo = has_month_name(line);

    if has_present && (has_yr || has_mo) {
        return true;
    }
    if has_yr && (has_mo || lower.contains('-') || lower.contains('–') || lower.contains('—') || lower.contains("to")) {
        return true;
    }
    false
}

fn parse_header_line(line: &str) -> (String, String, String) {
    if line.contains('|') {
        let parts: Vec<&str> = line.split('|').map(|s| s.trim()).filter(|s| !s.is_empty()).collect();
        if parts.len() >= 3 {
            if has_date_pattern(parts[2]) {
                return (parts[0].to_string(), parts[1].to_string(), parts[2].to_string());
            } else if has_date_pattern(parts[0]) {
                return (parts[1].to_string(), parts[2].to_string(), parts[0].to_string());
            } else {
                return (parts[0].to_string(), parts[1].to_string(), parts[2].to_string());
            }
        } else if parts.len() == 2 {
            if has_date_pattern(parts[1]) {
                let (r, c) = split_role_company(parts[0]);
                return (r, c, parts[1].to_string());
            } else if has_date_pattern(parts[0]) {
                let (r, c) = split_role_company(parts[1]);
                return (r, c, parts[0].to_string());
            } else {
                return (parts[0].to_string(), parts[1].to_string(), "Present".to_string());
            }
        }
    }

    for delim in &[" – ", " — ", " - "] {
        if line.contains(delim) {
            let parts: Vec<&str> = line.split(delim).map(|s| s.trim()).collect();
            if parts.len() >= 3 && has_date_pattern(parts.last().unwrap_or(&"")) {
                let dates = parts.last().unwrap_or(&"").to_string();
                return (parts[0].to_string(), parts[1].to_string(), dates);
            } else if parts.len() == 2 && has_date_pattern(parts[1]) {
                let (r, c) = split_role_company(parts[0]);
                return (r, c, parts[1].to_string());
            }
        }
    }

    (line.to_string(), String::new(), "Present".to_string())
}

fn split_role_company(s: &str) -> (String, String) {
    if let Some(idx) = s.to_lowercase().find(" at ") {
        let role = s[..idx].trim().to_string();
        let company = s[idx + 4..].trim().to_string();
        return (role, company);
    }
    if s.contains('@') {
        let parts: Vec<&str> = s.split('@').collect();
        return (parts[0].trim().to_string(), parts.get(1).unwrap_or(&"").trim().to_string());
    }
    if s.contains(',') {
        let parts: Vec<&str> = s.split(',').collect();
        return (parts[0].trim().to_string(), parts.get(1).unwrap_or(&"").trim().to_string());
    }
    (s.trim().to_string(), String::new())
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

    let exp_keywords = ["EXPERIENCE", "WORK HISTORY", "EMPLOYMENT", "WORK EXPERIENCE", "PROFESSIONAL EXPERIENCE"];
    let end_keywords = ["EDUCATION", "SKILLS", "CERTIFICATIONS", "PROJECTS", "PUBLICATIONS", "AWARDS", "LANGUAGES"];

    let has_exp_section = lines.iter().any(|l| {
        let upper = l.to_uppercase();
        exp_keywords.iter().any(|&k| upper == k || (upper.starts_with(k) && upper.len() < 35))
    });

    let mut in_experience = !has_exp_section;
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

    for line in &lines {
        let upper = line.to_uppercase();

        if exp_keywords.iter().any(|&k| upper == k || (upper.starts_with(k) && upper.len() < 35)) {
            in_experience = true;
            continue;
        }

        if in_experience && has_exp_section && end_keywords.iter().any(|&k| upper == k || (upper.starts_with(k) && upper.len() < 35)) {
            break;
        }

        if !in_experience {
            continue;
        }

        if has_date_pattern(line) {
            flush_entry(&mut entries, &current_company, &current_role, &current_dates, &current_bullets, &mut entry_counter);
            current_bullets.clear();

            let (r, c, d) = parse_header_line(line);
            current_role = r;
            current_company = c;
            current_dates = d;
        } else if is_bullet_line(line) {
            let bullet = clean_bullet(line);
            if !bullet.is_empty() {
                current_bullets.push(bullet);
            }
        } else if current_role.is_empty() {
            current_role = line.to_string();
        } else if current_company.is_empty() {
            current_company = line.to_string();
        } else if current_dates.is_empty() && (has_4digit_year(line) || has_month_name(line)) {
            current_dates = line.to_string();
        } else if let Some(last_bullet) = current_bullets.last_mut() {
            last_bullet.push(' ');
            last_bullet.push_str(line.trim());
        } else {
            current_bullets.push(line.to_string());
        }
    }

    // Flush last entry
    flush_entry(&mut entries, &current_company, &current_role, &current_dates, &current_bullets, &mut entry_counter);

    // Fallback if empty
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

fn parse_date_range(dates: &str) -> (String, String) {
    let parts: Vec<&str> = dates.split(&['-', '–', '—'][..]).collect();
    if parts.len() >= 2 {
        (parts[0].trim().to_string(), parts[1].trim().to_string())
    } else {
        (dates.trim().to_string(), "Present".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_work_history_sample() {
        let resume_text = r#"
ALEX MERCER
Senior Software Engineer
alex.mercer@example.com

EXPERIENCE

Frontend Web Developer | Apex Media Corp | September 2019 – March 2020
○  Increased conversion rates through data-driven A/B testing of high-traffic pages.
○  Established formal testing and component patterns improving code stability across React and legacy
codebases.

Frontend Developer III | Nexus Gaming Technologies | June 2018 – September 2019
○  Revolutionized 20-year-old Java interfaces by engineering a React-based thick client with embedded
Chromium.
○  Eliminated substantial Oracle licensing costs by researching and deploying an open-source Chromium
integration framework for Java.
○  Served as the resident React lead, standardizing production pipelines and DevOps best practices.

Ed Tech Course Designer | Global Learning Systems | October 2017 – February 2018
○  Designed mission-critical UI for instructional modules.

EDUCATION

State University - B.S. Computer Science
"#;

        let entries = parse_work_history(resume_text);
        assert_eq!(entries.len(), 3);

        // Verify middle entry
        let middle_job = &entries[1];
        assert_eq!(middle_job.role, "Frontend Developer III");
        assert_eq!(middle_job.company, "Nexus Gaming Technologies");
        assert_eq!(middle_job.start_date, "June 2018");
        assert_eq!(middle_job.end_date, "September 2019");

        // Verify that the bullet point with "20-year-old" was NOT split into a new entry
        assert!(middle_job.summary.contains("Revolutionized 20-year-old Java interfaces by engineering a React-based thick client with embedded Chromium."));
        assert!(middle_job.summary.contains("Eliminated substantial Oracle licensing costs"));
        assert!(middle_job.summary.contains("Served as the resident React lead"));
    }
}



