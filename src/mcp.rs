use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::io::{BufRead, BufReader, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc, Mutex,
};
use std::time::{SystemTime, UNIX_EPOCH};

use crate::prospect::{
    load_stored_data, save_stored_data, AppData, HitListContact, HitListTarget, Prospect,
    SpecialField,
};
use crate::resume::WorkHistoryEntry;

pub const DEFAULT_MCP_PORT: u16 = 42427;
const MAX_UNDO_STACK: usize = 25;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct McpUndoSnapshot {
    pub action_label: String,
    pub timestamp: u64,
    pub data: AppData,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct McpUiStatus {
    pub enabled: bool,
    pub port: u16,
    pub endpoint: String,
    pub undo_count: usize,
    pub last_undo_label: Option<String>,
    pub last_action: Option<String>,
}

pub struct McpManager {
    enabled: Arc<AtomicBool>,
    port: u16,
    undo_stack: Arc<Mutex<Vec<McpUndoSnapshot>>>,
    last_action: Arc<Mutex<Option<String>>>,
}

impl McpManager {
    pub fn new(port: u16) -> Self {
        Self {
            enabled: Arc::new(AtomicBool::new(false)),
            port,
            undo_stack: Arc::new(Mutex::new(Vec::new())),
            last_action: Arc::new(Mutex::new(None)),
        }
    }

    pub fn is_enabled(&self) -> bool {
        self.enabled.load(Ordering::SeqCst)
    }

    pub fn status(&self) -> McpUiStatus {
        let enabled = self.is_enabled();
        let (undo_count, last_undo_label) = if let Ok(guard) = self.undo_stack.lock() {
            (
                guard.len(),
                guard.last().map(|s| s.action_label.clone()),
            )
        } else {
            (0, None)
        };
        let last_action = self
            .last_action
            .lock()
            .ok()
            .and_then(|g| g.clone());

        McpUiStatus {
            enabled,
            port: self.port,
            endpoint: format!("http://127.0.0.1:{}/mcp", self.port),
            undo_count,
            last_undo_label,
            last_action,
        }
    }

    #[cfg(test)]
    pub fn push_snapshot(&self, label: impl Into<String>, data: &AppData) {
        let label_str = label.into();
        let now_secs = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0);

        if let Ok(mut stack) = self.undo_stack.lock() {
            if stack.len() >= MAX_UNDO_STACK {
                stack.remove(0);
            }
            stack.push(McpUndoSnapshot {
                action_label: label_str.clone(),
                timestamp: now_secs,
                data: data.clone(),
            });
        }
        if let Ok(mut last) = self.last_action.lock() {
            *last = Some(label_str);
        }
    }

    pub fn pop_undo(&self) -> Option<McpUndoSnapshot> {
        let popped = if let Ok(mut stack) = self.undo_stack.lock() {
            stack.pop()
        } else {
            None
        };
        if let Some(ref snap) = popped {
            if let Ok(mut last) = self.last_action.lock() {
                *last = Some(format!("Reverted: {}", snap.action_label));
            }
        }
        popped
    }

    pub fn set_enabled<F>(&self, enable: bool, on_change: F)
    where
        F: Fn(String) + Send + Sync + 'static,
    {
        let was_enabled = self.enabled.swap(enable, Ordering::SeqCst);
        if enable && !was_enabled {
            let enabled_flag = self.enabled.clone();
            let port = self.port;
            let undo_stack = self.undo_stack.clone();
            let last_action = self.last_action.clone();
            let on_change = Arc::new(on_change);

            if let Ok(mut last) = self.last_action.lock() {
                *last = Some(format!("Listening on 127.0.0.1:{}", port));
            }

            std::thread::spawn(move || {
                let addr = format!("127.0.0.1:{}", port);
                let listener = match TcpListener::bind(&addr) {
                    Ok(l) => {
                        println!("[Tailorbird MCP] Server listening on http://{}/mcp", addr);
                        l
                    }
                    Err(e) => {
                        eprintln!("[Tailorbird MCP] Failed to bind {}: {}", addr, e);
                        enabled_flag.store(false, Ordering::SeqCst);
                        if let Ok(mut last) = last_action.lock() {
                            *last = Some(format!("Port {} busy", port));
                        }
                        on_change(format!("Port {} busy", port));
                        return;
                    }
                };

                for stream_res in listener.incoming() {
                    if !enabled_flag.load(Ordering::SeqCst) {
                        break;
                    }
                    if let Ok(stream) = stream_res {
                        if !enabled_flag.load(Ordering::SeqCst) {
                            break;
                        }
                        let undo_clone = undo_stack.clone();
                        let last_clone = last_action.clone();
                        let cb_clone = on_change.clone();
                        let enabled_clone = enabled_flag.clone();
                        std::thread::spawn(move || {
                            handle_http_connection(
                                stream,
                                port,
                                &enabled_clone,
                                &undo_clone,
                                &last_clone,
                                &*cb_clone,
                            );
                        });
                    }
                }
                println!("[Tailorbird MCP] Server stopped listening on 127.0.0.1:{}", port);
            });
        } else if !enable && was_enabled {
            if let Ok(mut last) = self.last_action.lock() {
                *last = None;
            }
            // Wake up blocking listener.incoming() immediately so the port is released
            let addr = format!("127.0.0.1:{}", self.port);
            let _ = TcpStream::connect(&addr);
        }
    }
}

fn handle_http_connection<F>(
    mut stream: TcpStream,
    port: u16,
    enabled: &Arc<AtomicBool>,
    undo_stack: &Arc<Mutex<Vec<McpUndoSnapshot>>>,
    last_action: &Arc<Mutex<Option<String>>>,
    on_change: &F,
) where
    F: Fn(String),
{
    let _ = stream.set_read_timeout(Some(std::time::Duration::from_secs(10)));
    let _ = stream.set_write_timeout(Some(std::time::Duration::from_secs(10)));

    let mut reader = match stream.try_clone() {
        Ok(s) => BufReader::new(s),
        Err(_) => return,
    };

    let mut request_line = String::new();
    if reader.read_line(&mut request_line).is_err() || request_line.is_empty() {
        return;
    }

    let parts: Vec<&str> = request_line.trim().split_whitespace().collect();
    if parts.len() < 2 {
        return;
    }
    let method = parts[0].to_uppercase();
    let path = parts[1];

    let mut content_length: usize = 0;
    let mut accept_header = String::new();
    loop {
        let mut line = String::new();
        if reader.read_line(&mut line).is_err() {
            break;
        }
        let trimmed = line.trim();
        if trimmed.is_empty() {
            break;
        }
        if let Some((k, v)) = trimmed.split_once(':') {
            let key = k.trim().to_ascii_lowercase();
            let val = v.trim();
            if key == "content-length" {
                content_length = val.parse::<usize>().unwrap_or(0);
            } else if key == "accept" {
                accept_header = val.to_ascii_lowercase();
            }
        }
    }

    if !enabled.load(Ordering::SeqCst) {
        let _ = write_http_response(&mut stream, 503, "application/json", r#"{"error":"MCP server is disabled"}"#);
        return;
    }

    if method == "OPTIONS" {
        let _ = write_http_response(&mut stream, 204, "text/plain", "");
        return;
    }

    if method == "GET" {
        if path.starts_with("/sse") || accept_header.contains("text/event-stream") {
            // Legacy SSE endpoint handshake
            let sse_headers = format!(
                "HTTP/1.1 200 OK\r\n\
                 Content-Type: text/event-stream\r\n\
                 Cache-Control: no-cache\r\n\
                 Connection: keep-alive\r\n\
                 Access-Control-Allow-Origin: *\r\n\r\n\
                 event: endpoint\r\n\
                 data: http://127.0.0.1:{}/mcp\r\n\r\n",
                port
            );
            let _ = stream.write_all(sse_headers.as_bytes());
            let _ = stream.flush();
            return;
        }

        let info = json!({
            "name": "tailorbird-mcp",
            "version": "0.1.0",
            "status": "ready",
            "endpoint": format!("http://127.0.0.1:{}/mcp", port),
            "description": "Tailorbird Local MCP Server for managing candidate profile, work history, prospects, and outreach cards."
        });
        let _ = write_http_response(&mut stream, 200, "application/json", &info.to_string());
        return;
    }

    if method != "POST" {
        let _ = write_http_response(&mut stream, 405, "application/json", r#"{"error":"Method not allowed"}"#);
        return;
    }

    let mut body_bytes = vec![0u8; content_length.min(10 * 1024 * 1024)];
    if content_length > 0 && reader.read_exact(&mut body_bytes).is_err() {
        let _ = write_http_response(&mut stream, 400, "application/json", r#"{"error":"Failed to read request body"}"#);
        return;
    }

    let body_str = String::from_utf8_lossy(&body_bytes);
    let parsed: Value = match serde_json::from_str(&body_str) {
        Ok(v) => v,
        Err(e) => {
            let err_resp = json!({
                "jsonrpc": "2.0",
                "id": Value::Null,
                "error": { "code": -32700, "message": format!("Parse error: {}", e) }
            });
            let _ = write_http_response(&mut stream, 200, "application/json", &err_resp.to_string());
            return;
        }
    };

    if let Some(arr) = parsed.as_array() {
        let mut responses = Vec::new();
        for item in arr {
            if let Some(resp) = process_jsonrpc_message(item, undo_stack, last_action, on_change) {
                responses.push(resp);
            }
        }
        if responses.is_empty() {
            let _ = write_http_response(&mut stream, 202, "application/json", "");
        } else {
            let out = Value::Array(responses).to_string();
            let _ = write_http_response(&mut stream, 200, "application/json", &out);
        }
    } else if let Some(resp) = process_jsonrpc_message(&parsed, undo_stack, last_action, on_change) {
        let _ = write_http_response(&mut stream, 200, "application/json", &resp.to_string());
    } else {
        // JSON-RPC notification (no id)
        let _ = write_http_response(&mut stream, 202, "application/json", "");
    }
}

fn write_http_response(
    stream: &mut TcpStream,
    status_code: u16,
    content_type: &str,
    body: &str,
) -> std::io::Result<()> {
    let status_text = match status_code {
        200 => "OK",
        202 => "Accepted",
        204 => "No Content",
        400 => "Bad Request",
        405 => "Method Not Allowed",
        503 => "Service Unavailable",
        _ => "OK",
    };
    let bytes = body.as_bytes();
    let header = format!(
        "HTTP/1.1 {} {}\r\n\
         Content-Type: {}\r\n\
         Content-Length: {}\r\n\
         Access-Control-Allow-Origin: *\r\n\
         Access-Control-Allow-Methods: GET, POST, OPTIONS\r\n\
         Access-Control-Allow-Headers: Content-Type, Accept, Mcp-Session-Id\r\n\
         Connection: close\r\n\r\n",
        status_code,
        status_text,
        content_type,
        bytes.len()
    );
    stream.write_all(header.as_bytes())?;
    if !bytes.is_empty() {
        stream.write_all(bytes)?;
    }
    stream.flush()
}

fn process_jsonrpc_message<F>(
    req: &Value,
    undo_stack: &Arc<Mutex<Vec<McpUndoSnapshot>>>,
    last_action: &Arc<Mutex<Option<String>>>,
    on_change: &F,
) -> Option<Value>
where
    F: Fn(String),
{
    let id = req.get("id").cloned();
    let method = req.get("method").and_then(|m| m.as_str()).unwrap_or("");
    let params = req.get("params").cloned().unwrap_or_else(|| json!({}));

    // Notifications without an `id` do not expect a JSON-RPC response object
    id.as_ref()?;
    let req_id = id.unwrap_or(Value::Null);

    match method {
        "initialize" => {
            let client_version = params
                .get("protocolVersion")
                .and_then(|v| v.as_str())
                .unwrap_or("2024-11-05");
            Some(json!({
                "jsonrpc": "2.0",
                "id": req_id,
                "result": {
                    "protocolVersion": client_version,
                    "capabilities": {
                        "tools": {
                            "listChanged": false
                        }
                    },
                    "serverInfo": {
                        "name": "tailorbird",
                        "version": "0.1.0"
                    }
                }
            }))
        }
        "ping" => Some(json!({
            "jsonrpc": "2.0",
            "id": req_id,
            "result": {}
        })),
        "tools/list" => Some(json!({
            "jsonrpc": "2.0",
            "id": req_id,
            "result": {
                "tools": mcp_tool_definitions()
            }
        })),
        "tools/call" => {
            let tool_name = params.get("name").and_then(|v| v.as_str()).unwrap_or("");
            let args = params.get("arguments").cloned().unwrap_or_else(|| json!({}));

            let mut current_data = load_stored_data();
            let pre_write_clone = current_data.clone();

            match execute_mcp_tool(tool_name, &args, &mut current_data) {
                Ok(exec_out) => {
                    if let Some(snapshot_label) = exec_out.write_label {
                        // Push pre-write snapshot onto undo stack
                        let now_secs = SystemTime::now()
                            .duration_since(UNIX_EPOCH)
                            .map(|d| d.as_secs())
                            .unwrap_or(0);
                        if let Ok(mut stack) = undo_stack.lock() {
                            if stack.len() >= MAX_UNDO_STACK {
                                stack.remove(0);
                            }
                            stack.push(McpUndoSnapshot {
                                action_label: snapshot_label.clone(),
                                timestamp: now_secs,
                                data: pre_write_clone,
                            });
                        }
                        if let Ok(mut last) = last_action.lock() {
                            *last = Some(snapshot_label.clone());
                        }
                        if let Err(e) = save_stored_data(&current_data) {
                            return Some(json!({
                                "jsonrpc": "2.0",
                                "id": req_id,
                                "result": {
                                    "isError": true,
                                    "content": [{ "type": "text", "text": format!("Failed to save data: {}", e) }]
                                }
                            }));
                        }
                        on_change(snapshot_label);
                    }

                    Some(json!({
                        "jsonrpc": "2.0",
                        "id": req_id,
                        "result": {
                            "isError": false,
                            "content": [
                                {
                                    "type": "text",
                                    "text": exec_out.response_text
                                }
                            ]
                        }
                    }))
                }
                Err(err_msg) => Some(json!({
                    "jsonrpc": "2.0",
                    "id": req_id,
                    "result": {
                        "isError": true,
                        "content": [
                            {
                                "type": "text",
                                "text": err_msg
                            }
                        ]
                    }
                })),
            }
        }
        _ => Some(json!({
            "jsonrpc": "2.0",
            "id": req_id,
            "error": {
                "code": -32601,
                "message": format!("Method not found: {}", method)
            }
        })),
    }
}

pub struct ToolExecutionOutput {
    pub response_text: String,
    /// When `Some(label)`, indicates the tool modified `AppData` and should record an Undo snapshot.
    pub write_label: Option<String>,
}

pub fn normalize_prospect_status(raw: &str) -> String {
    let lower = raw.trim().to_ascii_lowercase();
    match lower.as_str() {
        "interested" | "saved" | "bookmarked" | "new" | "todo" | "target" => "Interested".to_string(),
        "applied" | "submitted" | "sent" => "Applied".to_string(),
        "screening" | "screen" | "phone screen" | "recruiter screen" | "responded" => "Screening".to_string(),
        "interviewing" | "interview" | "onsite" | "technical" | "final round" => "Interviewing".to_string(),
        "offer" | "offered" | "accepted" => "Offer".to_string(),
        "rejected" | "archived" | "closed" | "declined" | "withdrawn" => "Rejected".to_string(),
        _ => "Interested".to_string(),
    }
}

pub fn normalize_outreach_status(raw: &str) -> String {
    let lower = raw.trim().to_ascii_lowercase();
    match lower.as_str() {
        "contacted" | "reached out" | "sent" | "messaged" | "replied" | "meeting" | "emailed" => {
            "Contacted".to_string()
        }
        _ => "Targeted".to_string(),
    }
}

fn current_date_iso() -> String {
    // Simple UTC date string fallback using UNIX_EPOCH days calculation
    let secs = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);
    let days = (secs / 86400) as i64;
    // Civil date from days since 1970-01-01 (Howard Hinnant algorithm)
    let z = days + 719468;
    let era = if z >= 0 { z } else { z - 146096 } / 146097;
    let doe = (z - era * 146097) as u64;
    let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146096) / 365;
    let y = yoe as i64 + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let m = if mp < 10 { mp + 3 } else { mp - 9 };
    let year = if m <= 2 { y + 1 } else { y };
    format!("{:04}-{:02}-{:02}", year, m, d)
}

fn generate_id(prefix: &str) -> String {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_nanos())
        .unwrap_or(0);
    format!("{}-{}", prefix, nanos % 1_000_000_000)
}

pub fn execute_mcp_tool(
    tool_name: &str,
    args: &Value,
    data: &mut AppData,
) -> Result<ToolExecutionOutput, String> {
    match tool_name {
        "get_tailorbird_data" => {
            let section = args
                .get("section")
                .and_then(|v| v.as_str())
                .unwrap_or("all")
                .to_ascii_lowercase();
            let payload = match section.as_str() {
                "profile" => json!({ "candidateProfile": data.candidate_profile }),
                "work_history" | "workhistory" => json!({ "workHistory": data.work_history }),
                "special_fields" | "specialfields" | "snippets" => {
                    json!({ "specialFields": data.special_fields })
                }
                "prospects" => json!({ "prospects": data.prospects }),
                "outreach" | "outreach_roster" | "hit_list" | "roster" => {
                    json!({ "outreachRoster": data.hit_list })
                }
                _ => json!({
                    "candidateProfile": data.candidate_profile,
                    "workHistory": data.work_history,
                    "specialFields": data.special_fields,
                    "prospects": data.prospects,
                    "outreachRoster": data.hit_list,
                    "searchCriteria": data.search_criteria
                }),
            };
            let pretty = serde_json::to_string_pretty(&payload).unwrap_or_else(|_| "{}".to_string());
            Ok(ToolExecutionOutput {
                response_text: pretty,
                write_label: None,
            })
        }

        "update_candidate_profile" => {
            let mut profile = data.candidate_profile.clone().unwrap_or_default();

            if let Some(v) = args.get("fullName").and_then(|v| v.as_str()) {
                profile.full_name = v.trim().to_string();
            }
            if let Some(v) = args.get("role").and_then(|v| v.as_str()) {
                profile.role = v.trim().to_string();
            }
            if let Some(v) = args.get("pronouns").and_then(|v| v.as_str()) {
                profile.pronouns = v.trim().to_string();
            }
            if let Some(v) = args.get("email").and_then(|v| v.as_str()) {
                profile.email = v.trim().to_string();
            }
            if let Some(v) = args.get("phone").and_then(|v| v.as_str()) {
                profile.phone = v.trim().to_string();
            }
            if let Some(v) = args.get("location").and_then(|v| v.as_str()) {
                profile.location = v.trim().to_string();
            }
            if let Some(v) = args.get("currentCompany").and_then(|v| v.as_str()) {
                profile.current_company = v.trim().to_string();
            }
            if let Some(v) = args.get("linkedin").and_then(|v| v.as_str()) {
                profile.linkedin = v.trim().to_string();
            }
            if let Some(v) = args.get("github").and_then(|v| v.as_str()) {
                profile.github = v.trim().to_string();
            }
            if let Some(v) = args.get("portfolioUrl").and_then(|v| v.as_str()) {
                profile.portfolio_url = v.trim().to_string();
            }
            if let Some(v) = args.get("experienceYears").and_then(|v| v.as_str()) {
                profile.experience_years = v.trim().to_string();
            }
            if let Some(v) = args.get("gender").and_then(|v| v.as_str()) {
                profile.gender = v.trim().to_string();
            }
            if let Some(v) = args.get("race").and_then(|v| v.as_str()) {
                profile.race = v.trim().to_string();
            }
            if let Some(v) = args.get("veteranStatus").and_then(|v| v.as_str()) {
                profile.veteran_status = v.trim().to_string();
            }
            if let Some(v) = args.get("disabilityStatus").and_then(|v| v.as_str()) {
                profile.disability_status = v.trim().to_string();
            }
            if let Some(v) = args.get("resumePath").and_then(|v| v.as_str()) {
                profile.resume_path = v.trim().to_string();
                data.resume_source = v.trim().to_string();
            }
            if let Some(v) = args.get("coverLetterPath").and_then(|v| v.as_str()) {
                profile.cover_letter_path = v.trim().to_string();
                data.cover_letter_source = v.trim().to_string();
            }

            let summary_name = if profile.full_name.is_empty() {
                "Candidate Profile".to_string()
            } else {
                profile.full_name.clone()
            };
            data.candidate_profile = Some(profile.clone());

            Ok(ToolExecutionOutput {
                response_text: format!(
                    "Updated candidate profile for {}.",
                    summary_name
                ),
                write_label: Some(format!("Profile: {}", summary_name)),
            })
        }

        "set_work_history" => {
            let mode = args
                .get("mode")
                .and_then(|v| v.as_str())
                .unwrap_or("append")
                .to_ascii_lowercase();
            let entries_val = args
                .get("entries")
                .and_then(|v| v.as_array())
                .ok_or_else(|| "Missing 'entries' array parameter.".to_string())?;

            let mut parsed_entries = Vec::new();
            for (idx, item) in entries_val.iter().enumerate() {
                let id = item
                    .get("id")
                    .and_then(|v| v.as_str())
                    .filter(|s| !s.trim().is_empty())
                    .map(|s| s.to_string())
                    .unwrap_or_else(|| format!("{}-{}", generate_id("wh"), idx));
                let company = item
                    .get("company")
                    .and_then(|v| v.as_str())
                    .unwrap_or("")
                    .trim()
                    .to_string();
                let role = item
                    .get("role")
                    .and_then(|v| v.as_str())
                    .unwrap_or("")
                    .trim()
                    .to_string();
                let start_date = item
                    .get("startDate")
                    .and_then(|v| v.as_str())
                    .unwrap_or("")
                    .trim()
                    .to_string();
                let end_date = item
                    .get("endDate")
                    .and_then(|v| v.as_str())
                    .unwrap_or("")
                    .trim()
                    .to_string();
                let summary = item
                    .get("summary")
                    .and_then(|v| v.as_str())
                    .unwrap_or("")
                    .trim()
                    .to_string();

                parsed_entries.push(WorkHistoryEntry {
                    id,
                    company,
                    role,
                    start_date,
                    end_date,
                    summary,
                });
            }

            let count = parsed_entries.len();
            if mode == "replace" {
                data.work_history = parsed_entries;
            } else {
                for new_entry in parsed_entries {
                    if let Some(existing) = data.work_history.iter_mut().find(|e| {
                        e.id == new_entry.id
                            || (!new_entry.company.is_empty()
                                && e.company.eq_ignore_ascii_case(&new_entry.company)
                                && e.role.eq_ignore_ascii_case(&new_entry.role))
                    }) {
                        *existing = new_entry;
                    } else {
                        data.work_history.push(new_entry);
                    }
                }
            }

            if let Some(ref mut prof) = data.candidate_profile {
                prof.work_history = data.work_history.clone();
            }

            Ok(ToolExecutionOutput {
                response_text: format!(
                    "Saved {} work history position(s) (total: {}).",
                    count,
                    data.work_history.len()
                ),
                write_label: Some(format!("Work History ({} role{})", count, if count == 1 { "" } else { "s" })),
            })
        }

        "upsert_special_field" => {
            let label = args
                .get("label")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .trim()
                .to_string();
            let content = args
                .get("content")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .trim()
                .to_string();
            if label.is_empty() {
                return Err("Parameter 'label' is required for upsert_special_field.".to_string());
            }

            let id_arg = args.get("id").and_then(|v| v.as_str()).unwrap_or("").trim();
            if let Some(existing) = data.special_fields.iter_mut().find(|f| {
                (!id_arg.is_empty() && f.id == id_arg) || f.label.eq_ignore_ascii_case(&label)
            }) {
                existing.label = label.clone();
                existing.content = content;
            } else {
                data.special_fields.push(SpecialField {
                    id: if id_arg.is_empty() {
                        generate_id("sf")
                    } else {
                        id_arg.to_string()
                    },
                    label: label.clone(),
                    content,
                });
            }

            if let Some(ref mut prof) = data.candidate_profile {
                prof.special_fields = data.special_fields.clone();
            }

            Ok(ToolExecutionOutput {
                response_text: format!("Saved custom snippet '{}'.", label),
                write_label: Some(format!("Snippet: {}", label)),
            })
        }

        "add_prospect" => {
            let company = args
                .get("company")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .trim()
                .to_string();
            let job_title = args
                .get("jobTitle")
                .or_else(|| args.get("role"))
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .trim()
                .to_string();
            if company.is_empty() && job_title.is_empty() {
                return Err("At least 'company' or 'jobTitle' must be provided.".to_string());
            }
            let url = args
                .get("url")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .trim()
                .to_string();
            let raw_status = args
                .get("status")
                .and_then(|v| v.as_str())
                .unwrap_or("Interested");
            let status = normalize_prospect_status(raw_status);
            let notes = args
                .get("notes")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .trim()
                .to_string();
            let date_added = args
                .get("dateAdded")
                .and_then(|v| v.as_str())
                .filter(|s| !s.trim().is_empty())
                .map(|s| s.trim().to_string())
                .unwrap_or_else(current_date_iso);

            let prospect = Prospect {
                id: generate_id("prospect"),
                company: if company.is_empty() {
                    "Unknown Company".to_string()
                } else {
                    company.clone()
                },
                job_title: if job_title.is_empty() {
                    "Open Role".to_string()
                } else {
                    job_title.clone()
                },
                url,
                status: status.clone(),
                date_added,
                notes,
            };

            let display_company = prospect.company.clone();
            let display_title = prospect.job_title.clone();
            data.prospects.insert(0, prospect);

            Ok(ToolExecutionOutput {
                response_text: format!(
                    "Added prospect '{} — {}' with status '{}'.",
                    display_company, display_title, status
                ),
                write_label: Some(format!("Add Prospect: {}", display_company)),
            })
        }

        "update_prospect_status" => {
            let id_filter = args
                .get("id")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .trim();
            let company_filter = args
                .get("company")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .trim()
                .to_ascii_lowercase();
            let title_filter = args
                .get("jobTitle")
                .or_else(|| args.get("role"))
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .trim()
                .to_ascii_lowercase();

            let target = data.prospects.iter_mut().find(|p| {
                if !id_filter.is_empty() && p.id == id_filter {
                    return true;
                }
                let comp_match = !company_filter.is_empty()
                    && p.company.to_ascii_lowercase().contains(&company_filter);
                let title_match = !title_filter.is_empty()
                    && p.job_title.to_ascii_lowercase().contains(&title_filter);
                if !company_filter.is_empty() && !title_filter.is_empty() {
                    comp_match && title_match
                } else {
                    comp_match || title_match
                }
            });

            let prospect = target.ok_or_else(|| {
                "Could not find a matching prospect by id, company, or jobTitle.".to_string()
            })?;

            if let Some(raw_status) = args.get("status").and_then(|v| v.as_str()) {
                prospect.status = normalize_prospect_status(raw_status);
            }
            if let Some(notes) = args.get("notes").and_then(|v| v.as_str()) {
                prospect.notes = notes.trim().to_string();
            }
            if let Some(url) = args.get("url").and_then(|v| v.as_str()) {
                if !url.trim().is_empty() {
                    prospect.url = url.trim().to_string();
                }
            }
            if let Some(new_title) = args.get("newJobTitle").and_then(|v| v.as_str()) {
                if !new_title.trim().is_empty() {
                    prospect.job_title = new_title.trim().to_string();
                }
            }

            let company_name = prospect.company.clone();
            let updated_status = prospect.status.clone();

            Ok(ToolExecutionOutput {
                response_text: format!(
                    "Updated prospect '{}' to status '{}'.",
                    company_name, updated_status
                ),
                write_label: Some(format!("{} → {}", company_name, updated_status)),
            })
        }

        "add_outreach_card" => {
            let company_name = args
                .get("companyName")
                .or_else(|| args.get("company"))
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .trim()
                .to_string();
            if company_name.is_empty() {
                return Err("Parameter 'companyName' is required for add_outreach_card.".to_string());
            }

            let website_url = args
                .get("websiteUrl")
                .or_else(|| args.get("url"))
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .trim()
                .to_string();
            let industry = args
                .get("industry")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .trim()
                .to_string();
            let status = normalize_outreach_status(
                args.get("status")
                    .and_then(|v| v.as_str())
                    .unwrap_or("Targeted"),
            );

            let contact_obj = args.get("contact").cloned().unwrap_or_else(|| json!({}));
            let contact = HitListContact {
                name: contact_obj
                    .get("name")
                    .or_else(|| args.get("contactName"))
                    .and_then(|v| v.as_str())
                    .unwrap_or("")
                    .trim()
                    .to_string(),
                title: contact_obj
                    .get("title")
                    .or_else(|| args.get("contactTitle"))
                    .and_then(|v| v.as_str())
                    .unwrap_or("")
                    .trim()
                    .to_string(),
                linkedin_url: contact_obj
                    .get("linkedinUrl")
                    .or_else(|| args.get("contactLinkedin"))
                    .and_then(|v| v.as_str())
                    .unwrap_or("")
                    .trim()
                    .to_string(),
                email: contact_obj
                    .get("email")
                    .or_else(|| args.get("contactEmail"))
                    .and_then(|v| v.as_str())
                    .unwrap_or("")
                    .trim()
                    .to_string(),
                twitter_url: contact_obj
                    .get("twitterUrl")
                    .or_else(|| args.get("contactTwitter"))
                    .and_then(|v| v.as_str())
                    .unwrap_or("")
                    .trim()
                    .to_string(),
                github_url: contact_obj
                    .get("githubUrl")
                    .or_else(|| args.get("contactGithub"))
                    .and_then(|v| v.as_str())
                    .unwrap_or("")
                    .trim()
                    .to_string(),
            };

            let draft_message = args
                .get("draftMessage")
                .or_else(|| args.get("message"))
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .trim()
                .to_string();
            let value_angle = args
                .get("valueAngle")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .trim()
                .to_string();
            let notes = args
                .get("notes")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .trim()
                .to_string();
            let today = current_date_iso();

            let card = HitListTarget {
                id: generate_id("target"),
                company_name: company_name.clone(),
                website_url,
                industry,
                status: status.clone(),
                contact,
                value_angle,
                draft_message,
                notes,
                date_added: today.clone(),
                last_contacted: if status == "Contacted" {
                    today
                } else {
                    String::new()
                },
            };

            data.hit_list.insert(0, card);

            Ok(ToolExecutionOutput {
                response_text: format!(
                    "Added outreach card for '{}' ({}).",
                    company_name, status
                ),
                write_label: Some(format!("Outreach: {}", company_name)),
            })
        }

        "update_outreach_card" => {
            let id_filter = args
                .get("id")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .trim();
            let company_filter = args
                .get("companyName")
                .or_else(|| args.get("company"))
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .trim()
                .to_ascii_lowercase();

            let card = data
                .hit_list
                .iter_mut()
                .find(|c| {
                    if !id_filter.is_empty() && c.id == id_filter {
                        return true;
                    }
                    !company_filter.is_empty()
                        && c.company_name.to_ascii_lowercase().contains(&company_filter)
                })
                .ok_or_else(|| {
                    "Could not find a matching outreach card by 'id' or 'companyName'.".to_string()
                })?;

            if let Some(status_raw) = args.get("status").and_then(|v| v.as_str()) {
                card.status = normalize_outreach_status(status_raw);
                if card.status == "Contacted" && card.last_contacted.is_empty() {
                    card.last_contacted = current_date_iso();
                }
            }
            if let Some(v) = args.get("websiteUrl").and_then(|v| v.as_str()) {
                card.website_url = v.trim().to_string();
            }
            if let Some(v) = args.get("industry").and_then(|v| v.as_str()) {
                card.industry = v.trim().to_string();
            }
            if let Some(v) = args.get("draftMessage").or_else(|| args.get("message")).and_then(|v| v.as_str()) {
                card.draft_message = v.trim().to_string();
            }
            if let Some(v) = args.get("valueAngle").and_then(|v| v.as_str()) {
                card.value_angle = v.trim().to_string();
            }
            if let Some(v) = args.get("notes").and_then(|v| v.as_str()) {
                card.notes = v.trim().to_string();
            }
            if let Some(contact_obj) = args.get("contact").and_then(|v| v.as_object()) {
                if let Some(v) = contact_obj.get("name").and_then(|v| v.as_str()) {
                    card.contact.name = v.trim().to_string();
                }
                if let Some(v) = contact_obj.get("title").and_then(|v| v.as_str()) {
                    card.contact.title = v.trim().to_string();
                }
                if let Some(v) = contact_obj.get("linkedinUrl").and_then(|v| v.as_str()) {
                    card.contact.linkedin_url = v.trim().to_string();
                }
                if let Some(v) = contact_obj.get("email").and_then(|v| v.as_str()) {
                    card.contact.email = v.trim().to_string();
                }
                if let Some(v) = contact_obj.get("twitterUrl").and_then(|v| v.as_str()) {
                    card.contact.twitter_url = v.trim().to_string();
                }
                if let Some(v) = contact_obj.get("githubUrl").and_then(|v| v.as_str()) {
                    card.contact.github_url = v.trim().to_string();
                }
            }

            let company_name = card.company_name.clone();
            let status = card.status.clone();

            Ok(ToolExecutionOutput {
                response_text: format!(
                    "Updated outreach card for '{}' (status: {}).",
                    company_name, status
                ),
                write_label: Some(format!("Outreach: {}", company_name)),
            })
        }

        _ => Err(format!("Unknown tool: {}", tool_name)),
    }
}

pub fn mcp_tool_definitions() -> Value {
    json!([
        {
            "name": "get_tailorbird_data",
            "description": "Read the user's Tailorbird job search workspace data (candidate profile, work history, custom snippets, job prospects, and outreach cards).",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "section": {
                        "type": "string",
                        "enum": ["all", "profile", "work_history", "special_fields", "prospects", "outreach_roster"],
                        "description": "Which section of Tailorbird data to retrieve (defaults to 'all')."
                    }
                }
            }
        },
        {
            "name": "update_candidate_profile",
            "description": "Populate or update the user's candidate profile fields in Tailorbird. Only provided fields are updated.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "fullName": { "type": "string", "description": "Candidate full name, e.g. 'Robin Featherstone'" },
                    "role": { "type": "string", "description": "Target role/title, e.g. 'Senior Nest Architect'" },
                    "pronouns": { "type": "string", "description": "Preferred pronouns" },
                    "email": { "type": "string", "description": "Email address, e.g. 'robin@maplegrove.dev'" },
                    "phone": { "type": "string", "description": "Phone number, e.g. '(555) 247-3637'" },
                    "location": { "type": "string", "description": "Location, e.g. 'Maple Grove Canopy'" },
                    "linkedin": { "type": "string", "description": "LinkedIn or professional profile URL" },
                    "github": { "type": "string", "description": "GitHub profile URL" },
                    "portfolioUrl": { "type": "string", "description": "Personal website or portfolio URL" },
                    "experienceYears": { "type": "string", "description": "Years of professional experience" },
                    "gender": { "type": "string" },
                    "race": { "type": "string" },
                    "veteranStatus": { "type": "string" },
                    "disabilityStatus": { "type": "string" },
                    "resumePath": { "type": "string", "description": "Local filesystem path to resume file" },
                    "coverLetterPath": { "type": "string", "description": "Local filesystem path to cover letter file" }
                }
            }
        },
        {
            "name": "set_work_history",
            "description": "Add, update, or replace positions in the user's Tailorbird Work History.",
            "inputSchema": {
                "type": "object",
                "required": ["entries"],
                "properties": {
                    "mode": {
                        "type": "string",
                        "enum": ["append", "replace"],
                        "description": "'append' (default) merges/adds positions; 'replace' overwrites the entire work history list."
                    },
                    "entries": {
                        "type": "array",
                        "items": {
                            "type": "object",
                            "required": ["company", "role"],
                            "properties": {
                                "id": { "type": "string" },
                                "company": { "type": "string", "description": "Company name, e.g. 'Wren Industries'" },
                                "role": { "type": "string", "description": "Job title, e.g. 'Senior Nest Architect'" },
                                "startDate": { "type": "string", "description": "Start date, e.g. '2022-03'" },
                                "endDate": { "type": "string", "description": "End date, e.g. 'Present'" },
                                "summary": { "type": "string", "description": "Accomplishments and responsibilities" }
                            }
                        }
                    }
                }
            }
        },
        {
            "name": "upsert_special_field",
            "description": "Create or update a reusable custom snippet / application Q&A response in Tailorbird.",
            "inputSchema": {
                "type": "object",
                "required": ["label", "content"],
                "properties": {
                    "id": { "type": "string" },
                    "label": { "type": "string", "description": "Snippet title, e.g. 'Why Canopy Architecture?'" },
                    "content": { "type": "string", "description": "Reusable text content" }
                }
            }
        },
        {
            "name": "add_prospect",
            "description": "Add a new job listing to the Tailorbird Prospects tracker.",
            "inputSchema": {
                "type": "object",
                "required": ["company", "jobTitle"],
                "properties": {
                    "company": { "type": "string", "description": "Company name, e.g. 'Falcon Dynamics'" },
                    "jobTitle": { "type": "string", "description": "Role title, e.g. 'High Velocity Flight Engineer'" },
                    "url": { "type": "string", "description": "Job posting URL" },
                    "status": {
                        "type": "string",
                        "enum": ["Interested", "Applied", "Screening", "Interviewing", "Offer", "Rejected"],
                        "description": "Initial application stage (defaults to 'Interested')"
                    },
                    "notes": { "type": "string", "description": "Optional notes on the role" }
                }
            }
        },
        {
            "name": "update_prospect_status",
            "description": "Update the application status or notes of an existing job prospect in Tailorbird (matches by id, company name, or jobTitle).",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "id": { "type": "string", "description": "Prospect ID if known" },
                    "company": { "type": "string", "description": "Company name to match, e.g. 'Falcon Dynamics'" },
                    "jobTitle": { "type": "string", "description": "Job title to match" },
                    "status": {
                        "type": "string",
                        "enum": ["Interested", "Applied", "Screening", "Interviewing", "Offer", "Rejected"],
                        "description": "New status stage"
                    },
                    "notes": { "type": "string", "description": "Updated notes" },
                    "url": { "type": "string", "description": "Updated job posting URL" }
                }
            }
        },
        {
            "name": "add_outreach_card",
            "description": "Add a target company & decision-maker contact card to the Tailorbird Outreach Roster.",
            "inputSchema": {
                "type": "object",
                "required": ["companyName"],
                "properties": {
                    "companyName": { "type": "string", "description": "Target company name, e.g. 'Owl Analytics'" },
                    "websiteUrl": { "type": "string", "description": "Company website URL" },
                    "industry": { "type": "string", "description": "Industry or focus area" },
                    "status": {
                        "type": "string",
                        "enum": ["Targeted", "Contacted"],
                        "description": "Outreach stage ('Targeted' or 'Contacted')"
                    },
                    "contact": {
                        "type": "object",
                        "properties": {
                            "name": { "type": "string", "description": "Contact person name, e.g. 'Finley Sparrow'" },
                            "title": { "type": "string", "description": "Contact title, e.g. 'Seed Procurement Lead'" },
                            "linkedinUrl": { "type": "string" },
                            "email": { "type": "string", "description": "e.g. 'nest@aviary.example'" },
                            "twitterUrl": { "type": "string" },
                            "githubUrl": { "type": "string" }
                        }
                    },
                    "draftMessage": { "type": "string", "description": "Customized outreach message and value alignment hook" },
                    "notes": { "type": "string", "description": "Research and follow-up notes" }
                }
            }
        },
        {
            "name": "update_outreach_card",
            "description": "Update an existing company card in the Tailorbird Outreach Roster (matches by id or companyName).",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "id": { "type": "string" },
                    "companyName": { "type": "string", "description": "Company name to match, e.g. 'Owl Analytics'" },
                    "status": {
                        "type": "string",
                        "enum": ["Targeted", "Contacted"]
                    },
                    "websiteUrl": { "type": "string" },
                    "industry": { "type": "string" },
                    "contact": {
                        "type": "object",
                        "properties": {
                            "name": { "type": "string" },
                            "title": { "type": "string" },
                            "linkedinUrl": { "type": "string" },
                            "email": { "type": "string" },
                            "twitterUrl": { "type": "string" },
                            "githubUrl": { "type": "string" }
                        }
                    },
                    "draftMessage": { "type": "string" },
                    "notes": { "type": "string" }
                }
            }
        }
    ])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mcp_update_profile_and_undo_stack() {
        let manager = McpManager::new(DEFAULT_MCP_PORT);
        let mut data = AppData::default();

        // Snapshot before write
        manager.push_snapshot("Profile: Robin Featherstone", &data);
        let args = json!({
            "fullName": "Robin Featherstone",
            "role": "Senior Nest Architect",
            "email": "robin@maplegrove.dev",
            "phone": "(555) 247-3637",
            "location": "Maple Grove Canopy"
        });
        let res = execute_mcp_tool("update_candidate_profile", &args, &mut data)
            .expect("update_candidate_profile should succeed");
        assert!(res.write_label.is_some());
        assert_eq!(
            data.candidate_profile.as_ref().unwrap().full_name,
            "Robin Featherstone"
        );
        assert_eq!(manager.status().undo_count, 1);

        // Pop undo and verify original empty state is restored
        let snap = manager.pop_undo().expect("Snapshot should exist");
        assert_eq!(snap.action_label, "Profile: Robin Featherstone");
        assert!(snap.data.candidate_profile.is_none());
        assert_eq!(manager.status().undo_count, 0);
    }

    #[test]
    fn test_mcp_prospect_add_and_status_update() {
        let mut data = AppData::default();
        let add_args = json!({
            "company": "Falcon Dynamics",
            "jobTitle": "High Velocity Flight Engineer",
            "url": "https://aviary.network/jobs/falcon-1",
            "status": "Saved"
        });
        execute_mcp_tool("add_prospect", &add_args, &mut data).expect("add_prospect should work");
        assert_eq!(data.prospects.len(), 1);
        assert_eq!(data.prospects[0].company, "Falcon Dynamics");
        assert_eq!(data.prospects[0].status, "Interested");

        let update_args = json!({
            "company": "falcon",
            "status": "Interview",
            "notes": "Panel flight check on Friday"
        });
        execute_mcp_tool("update_prospect_status", &update_args, &mut data)
            .expect("update_prospect_status should work");
        assert_eq!(data.prospects[0].status, "Interviewing");
        assert_eq!(data.prospects[0].notes, "Panel flight check on Friday");
    }

    #[test]
    fn test_mcp_outreach_card_add_and_update() {
        let mut data = AppData::default();
        let add_args = json!({
            "companyName": "Owl Analytics",
            "industry": "Nocturnal Telemetry",
            "websiteUrl": "https://maplegrove.dev/owl",
            "status": "Targeted",
            "contact": {
                "name": "Finley Sparrow",
                "title": "Night Shift Data Scientist",
                "email": "nest@aviary.example"
            },
            "draftMessage": "Hello Finley, excited about Owl Analytics' canopy sensors."
        });
        execute_mcp_tool("add_outreach_card", &add_args, &mut data)
            .expect("add_outreach_card should succeed");
        assert_eq!(data.hit_list.len(), 1);
        assert_eq!(data.hit_list[0].company_name, "Owl Analytics");
        assert_eq!(data.hit_list[0].contact.name, "Finley Sparrow");

        let upd_args = json!({
            "companyName": "Owl Analytics",
            "status": "Reached Out",
            "notes": "Sent intro note via carrier pigeon"
        });
        execute_mcp_tool("update_outreach_card", &upd_args, &mut data)
            .expect("update_outreach_card should succeed");
        assert_eq!(data.hit_list[0].status, "Contacted");
        assert_eq!(data.hit_list[0].notes, "Sent intro note via carrier pigeon");
    }
}
