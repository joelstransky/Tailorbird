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

fn repair_ligatures(s: &str) -> String {
    let mut result = s
        .replace('\u{FB01}', "fi")
        .replace('\u{FB02}', "fl")
        .replace('\u{FB00}', "ff")
        .replace('\u{FB03}', "ffi")
        .replace('\u{FB04}', "ffl");

    // Repair broken ligature extraction where PDF inserted a spurious space
    for word_prefix in &["fi nanc", "fi rst", "fi nd", "fi eld", "fi le", "fi nal", "fl ex", "fl ag", "fl ow"] {
        let fixed = word_prefix.replace(' ', "");
        result = result.replace(word_prefix, &fixed);
    }
    result
}

fn is_bullet_line(line: &str) -> bool {
    let trimmed = line.trim_start();
    if trimmed.is_empty() {
        return false;
    }
    let bullet_chars = ['•', '○', '●', '◦', '▪', '▫', '■', '*', '‣', '⁃', '►', '✓', '✔', '·', '–', '—'];
    if bullet_chars.iter().any(|&c| trimmed.starts_with(c)) {
        return true;
    }
    if trimmed.starts_with('-') {
        let after = trimmed.trim_start_matches('-');
        if after.starts_with(' ') || after.starts_with('\t') {
            return true;
        }
    }
    if let Some(first_ch) = trimmed.chars().next() {
        if first_ch.is_ascii_digit() {
            let rest = trimmed.trim_start_matches(|c: char| c.is_ascii_digit());
            if rest.starts_with(". ") || rest.starts_with(") ") {
                return true;
            }
        }
    }
    false
}

fn clean_bullet(line: &str) -> String {
    let mut s = line.trim();
    while let Some(first_char) = s.chars().next() {
        if ['•', '○', '●', '◦', '▪', '▫', '■', '*', '‣', '⁃', '►', '✓', '✔', '·', ' ', '\t'].contains(&first_char) {
            s = &s[first_char.len_utf8()..];
        } else if s.starts_with("- ") {
            s = &s[2..];
        } else {
            break;
        }
    }
    repair_ligatures(s.trim())
}

fn is_role_like(s: &str) -> bool {
    let lower = s.to_lowercase();
    let keywords = [
        "developer", "engineer", "lead", "architect", "designer", "manager",
        "director", "specialist", "consultant", "analyst", "administrator",
        "coordinator", "officer", "intern", "producer", "artist", "programmer",
        "technician", "scientist", "head of", "vp", "vice president", "founder",
        "creator", "executive", "supervisor", "fellow", "instructor", "author",
        "contributor", "contractor"
    ];
    keywords.iter().any(|&k| lower.contains(k))
}

fn is_company_like(s: &str) -> bool {
    let lower = s.to_lowercase();
    let keywords = [
        "inc", "corp", "llc", "ltd", "company", "corporation", "technologies",
        "technology", "systems", "labs", "group", "studio", "studios",
        "university", "college", "bank", "solutions", "interactive", "gaming",
        "enterprises", "media", "agency", "foundation", "network", "services",
        "partners", "health", "hospital", "institute"
    ];
    keywords.iter().any(|&k| lower.contains(k))
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
    // Very long sentences are usually bullet content or paragraphs, not headers
    if line.len() > 140 && !line.contains('|') {
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

fn parse_line_with_date(line: &str) -> (Vec<String>, String) {
    if line.contains('|') {
        let parts: Vec<&str> = line.split('|').map(|s| s.trim()).filter(|s| !s.is_empty()).collect();
        if parts.len() >= 3 {
            if has_date_pattern(parts[2]) {
                return (vec![parts[0].to_string(), parts[1].to_string()], parts[2].to_string());
            } else if has_date_pattern(parts[0]) {
                return (vec![parts[1].to_string(), parts[2].to_string()], parts[0].to_string());
            }
        } else if parts.len() == 2 {
            if has_date_pattern(parts[1]) {
                let (r, c) = split_role_company(parts[0]);
                if !r.is_empty() && !c.is_empty() {
                    return (vec![r, c], parts[1].to_string());
                }
                return (vec![parts[0].to_string()], parts[1].to_string());
            } else if has_date_pattern(parts[0]) {
                let (r, c) = split_role_company(parts[1]);
                if !r.is_empty() && !c.is_empty() {
                    return (vec![r, c], parts[0].to_string());
                }
                return (vec![parts[1].to_string()], parts[0].to_string());
            }
        }
    }

    for delim in &[" – ", " — ", " - "] {
        if line.contains(delim) {
            let parts: Vec<&str> = line.split(delim).map(|s| s.trim()).collect();
            if parts.len() >= 3 && has_date_pattern(parts.last().unwrap_or(&"")) {
                let dates = parts.last().unwrap_or(&"").to_string();
                return (parts[..parts.len() - 1].iter().map(|s| s.to_string()).collect(), dates);
            } else if parts.len() == 2 && has_date_pattern(parts[1]) {
                let (r, c) = split_role_company(parts[0]);
                if !r.is_empty() && !c.is_empty() {
                    return (vec![r, c], parts[1].to_string());
                }
                return (vec![parts[0].to_string()], parts[1].to_string());
            }
        }
    }

    if let (Some(open), Some(close)) = (line.find('('), line.rfind(')')) {
        if open < close {
            let inside = &line[open + 1..close];
            if has_date_pattern(inside) {
                let outside = line[..open].trim();
                let (r, c) = split_role_company(outside);
                if !r.is_empty() && !c.is_empty() {
                    return (vec![r, c], inside.to_string());
                }
                return (vec![outside.to_string()], inside.to_string());
            }
        }
    }

    (vec![line.to_string()], "Present".to_string())
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
    let end_keywords = ["EDUCATION", "SKILLS", "CERTIFICATIONS", "PROJECTS", "PUBLICATIONS", "AWARDS", "LANGUAGES", "TECHNICAL SKILLS"];

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
    let mut pending_header_candidate: Option<String> = None;

    let flush_entry = |entries: &mut Vec<WorkHistoryEntry>, company: &mut String, role: &mut String, dates: &mut String, bullets: &mut Vec<String>, counter: &mut usize| {
        if !company.is_empty() || !role.is_empty() {
            let (start, end) = parse_date_range(dates);
            entries.push(WorkHistoryEntry {
                id: format!("exp-{}", counter),
                company: if company.is_empty() { "Organization".to_string() } else { company.clone() },
                role: if role.is_empty() { "Role".to_string() } else { role.clone() },
                start_date: start,
                end_date: end,
                summary: bullets.join("\n"),
            });
            *counter += 1;
            company.clear();
            role.clear();
            dates.clear();
            bullets.clear();
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
            // If we already had an active job with dates, flush it before starting the next
            if !current_dates.is_empty() {
                flush_entry(&mut entries, &mut current_company, &mut current_role, &mut current_dates, &mut current_bullets, &mut entry_counter);
            }

            let (entities, dates) = parse_line_with_date(line);
            current_dates = dates;

            if entities.len() >= 2 {
                let e0 = entities[0].trim();
                let e1 = entities[1].trim();
                if is_role_like(e1) && is_company_like(e0) {
                    current_role = e1.to_string();
                    current_company = e0.to_string();
                } else {
                    current_role = e0.to_string();
                    current_company = e1.to_string();
                }
                pending_header_candidate = None;
            } else if entities.len() == 1 {
                let entity = entities[0].trim();
                if let Some(prev) = pending_header_candidate.take() {
                    if is_role_like(&prev) || is_company_like(entity) {
                        current_role = prev;
                        current_company = entity.to_string();
                    } else if is_company_like(&prev) || is_role_like(entity) {
                        current_role = entity.to_string();
                        current_company = prev;
                    } else {
                        current_role = prev;
                        current_company = entity.to_string();
                    }
                } else if is_company_like(entity) {
                    current_company = entity.to_string();
                } else {
                    current_role = entity.to_string();
                }
            }
        } else if is_bullet_line(line) {
            if let Some(prev) = pending_header_candidate.take() {
                let cleaned = clean_bullet(&prev);
                if !cleaned.is_empty() {
                    current_bullets.push(cleaned);
                }
            }
            let bullet = clean_bullet(line);
            if !bullet.is_empty() {
                current_bullets.push(bullet);
            }
        } else if current_dates.is_empty() {
            if line.len() < 90 {
                pending_header_candidate = Some(line.to_string());
            }
        } else {
            // current_dates is NOT empty
            // Check if this line is an uppercase/role-like header candidate for the NEXT job
            let is_candidate = (is_role_like(line) || (line.len() < 55 && line.to_uppercase() == *line && !line.ends_with('.')))
                && !line.ends_with('.');

            if is_candidate {
                if let Some(prev) = pending_header_candidate.take() {
                    let cleaned = clean_bullet(&prev);
                    if !cleaned.is_empty() {
                        current_bullets.push(cleaned);
                    }
                }
                pending_header_candidate = Some(line.to_string());
            } else {
                if let Some(prev) = pending_header_candidate.take() {
                    let cleaned = clean_bullet(&prev);
                    if !cleaned.is_empty() {
                        current_bullets.push(cleaned);
                    }
                }
                let cleaned = clean_bullet(line);
                if !cleaned.is_empty() {
                    if let Some(last_bullet) = current_bullets.last_mut() {
                        if !last_bullet.ends_with('.') && !last_bullet.ends_with(':') && !last_bullet.ends_with(';') {
                            last_bullet.push(' ');
                            last_bullet.push_str(&cleaned);
                        } else {
                            current_bullets.push(cleaned);
                        }
                    } else {
                        current_bullets.push(cleaned);
                    }
                }
            }
        }
    }

    if let Some(prev) = pending_header_candidate.take() {
        let cleaned = clean_bullet(&prev);
        if !cleaned.is_empty() {
            current_bullets.push(cleaned);
        }
    }

    // Flush last entry
    flush_entry(&mut entries, &mut current_company, &mut current_role, &mut current_dates, &mut current_bullets, &mut entry_counter);

    // Fallback if empty
    if entries.is_empty() && !lines.is_empty() {
        let preview = lines.iter().take(4).cloned().collect::<Vec<&str>>().join(" ");
        entries.push(WorkHistoryEntry {
            id: "exp-1".to_string(),
            company: "Primary Experience".to_string(),
            role: lines.first().cloned().unwrap_or("Master Leaf Stitcher").to_string(),
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
BARNABY FEATHERSTITCH
Master Leaf Stitcher & Nest Architect
barnaby@canopy.forest

EXPERIENCE

Senior Leaf Stitcher | Canopy & Cobwebs Haberdashery | September 2019 – March 2020
○  Increased nest durability through data-driven wind-tunnel testing of leafy canopies.
○  Established formal puncture and stitching patterns improving cradle stability across fig and mango leaves.

Lead Twig Weaver III | Bramble & Burlap Guild | June 2018 – September 2019
○  Revolutionized 20-year-old hollow branch nests by engineering multi-layered lichen insulation with moss lining.
○  Eliminated substantial twig loss by researching and deploying interlocking pine-needle lattices.
○  Served as the resident flock artisan, standardizing nest inspection pipelines and fledging safety best practices.

Berry Quality Inspector | Great Oak Foraging Co | October 2017 – February 2018
○  Inspected mission-critical ripe elderberries and sweet rowan berries for the migratory flock.

EDUCATION

State Aviary Academy - B.S. Avian Nest Architecture & Fiber Weaving
"#;

        let entries = parse_work_history(resume_text);
        assert_eq!(entries.len(), 3);

        // Verify middle entry
        let middle_job = &entries[1];
        assert_eq!(middle_job.role, "Lead Twig Weaver III");
        assert_eq!(middle_job.company, "Bramble & Burlap Guild");
        assert_eq!(middle_job.start_date, "June 2018");
        assert_eq!(middle_job.end_date, "September 2019");

        // Verify that the bullet point with "20-year-old" was NOT split into a new entry
        assert!(middle_job.summary.contains("Revolutionized 20-year-old hollow branch nests by engineering multi-layered lichen insulation with moss lining."));
        assert!(middle_job.summary.contains("Eliminated substantial twig loss"));
        assert!(middle_job.summary.contains("Served as the resident flock artisan"));
    }

    #[test]
    fn test_parse_google_doc_two_line_header_and_ligatures() {
        let text = r#"
WORK EXPERIENCE

SENIOR CANOPY ARCHITECT & TWIG LEAD
Canopy Loan Guild | August 2020 – Present
Architected resilient canopy shelters and led automated twig weaving workflows across the western forest preserve.
○ Improved nest safety scores from a C to an A across aviary properties.
○ Reduced weaving time by 40% by consolidating 6 branches into a central grove.
○ Cleared thousands of legacy branch violations.
○ Led the design and development of multiple fi nancial calculators for seed reserves.
○ Built custom low-code data-viz extensions for flock science products.

MIGRATION ADVISOR
Wild Willow Logistics | January 2018 – July 2020
● Coordinated annual seasonal migration routes across three mountain ranges.
● Maintained 99.9% flock arrival punctuality.
"#;
        let entries = parse_work_history(text);
        assert_eq!(entries.len(), 2);

        let first = &entries[0];
        assert_eq!(first.role, "SENIOR CANOPY ARCHITECT & TWIG LEAD");
        assert_eq!(first.company, "Canopy Loan Guild");
        assert_eq!(first.start_date, "August 2020");
        assert_eq!(first.end_date, "Present");
        assert!(first.summary.contains("Architected resilient canopy shelters"));
        assert!(first.summary.contains("Improved nest safety scores"));
        assert!(first.summary.contains("financial calculators"));
        assert!(!first.summary.contains("fi nancial"));

        let second = &entries[1];
        assert_eq!(second.role, "MIGRATION ADVISOR");
        assert_eq!(second.company, "Wild Willow Logistics");
        assert_eq!(second.start_date, "January 2018");
        assert_eq!(second.end_date, "July 2020");
        assert!(second.summary.contains("Coordinated annual seasonal migration routes"));
    }

    #[test]
    fn test_parse_single_line_pipe_header_with_bullets() {
        let text = r#"
Senior Software Developer | Canopy Solutions | August 2020 – Present
○ Improved Lighthouse scores from a C to an A across company properties.
○ Reduced development time by 40% by consolidating 6 projects into a monorepo.
○ Cleared thousands of legacy WCAG violations.
○ Led the design and development of multiple fi nancial calculators.
○ Built custom low-code data-viz extensions for our data science products.
"#;
        let entries = parse_work_history(text);
        assert_eq!(entries.len(), 1);
        let entry = &entries[0];
        assert_eq!(entry.role, "Senior Software Developer");
        assert_eq!(entry.company, "Canopy Solutions");
        assert_eq!(entry.start_date, "August 2020");
        assert_eq!(entry.end_date, "Present");
        assert!(entry.summary.contains("Improved Lighthouse scores"));
        assert!(entry.summary.contains("financial calculators"));
    }
}



