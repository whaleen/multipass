use std::io::{self, BufRead, BufReader, Write};

use anyhow::{Context, bail};
use camino::Utf8Path;
use chrono::Utc;
use multipass_core::{Record, RecordMetadata, SearchQuery};
use multipass_store::ShipDb;
use serde_json::{Value, json};
use uuid::Uuid;

pub fn tool_names() -> &'static [&'static str] {
    &[
        "multipass_status",
        "multipass_list_wings",
        "multipass_list_rooms",
        "multipass_search",
        "multipass_add_record",
        "multipass_delete_record",
    ]
}

pub fn serve_stdio(ship_root: &Utf8Path) -> anyhow::Result<()> {
    let stdin = io::stdin();
    let stdout = io::stdout();
    let mut reader = BufReader::new(stdin.lock());
    let mut writer = stdout.lock();

    while let Some(request) = read_message(&mut reader)? {
        let response = match handle_request(ship_root, request) {
            Ok(response) => response,
            Err(err) => json!({
                "jsonrpc": "2.0",
                "id": Value::Null,
                "error": {
                    "code": -32000,
                    "message": err.to_string(),
                }
            }),
        };
        write_message(&mut writer, &response)?;
    }

    Ok(())
}

pub fn handle_request(ship_root: &Utf8Path, request: Value) -> anyhow::Result<Value> {
    let id = request.get("id").cloned().unwrap_or(Value::Null);
    let method = request
        .get("method")
        .and_then(Value::as_str)
        .context("missing method")?;
    let params = request.get("params").cloned().unwrap_or_else(|| json!({}));

    let result = match method {
        "initialize" => json!({
            "protocolVersion": "2024-11-05",
            "serverInfo": {
                "name": "multipass-rs",
                "version": env!("CARGO_PKG_VERSION"),
            },
            "capabilities": {
                "tools": {}
            }
        }),
        "tools/list" => json!({ "tools": tool_definitions() }),
        "tools/call" => handle_tool_call(ship_root, &params)?,
        "ping" => json!({}),
        other => bail!("unsupported method: {other}"),
    };

    Ok(json!({
        "jsonrpc": "2.0",
        "id": id,
        "result": result,
    }))
}

fn handle_tool_call(ship_root: &Utf8Path, params: &Value) -> anyhow::Result<Value> {
    let name = params
        .get("name")
        .and_then(Value::as_str)
        .context("missing tool name")?;
    let arguments = params
        .get("arguments")
        .cloned()
        .unwrap_or_else(|| json!({}));
    let db = ShipDb::open(ship_root)?;

    let payload = match name {
        "multipass_status" => {
            let stats = db.stats()?;
            json!({
                "ship": ship_root.as_str(),
                "records": stats.total_records,
                "wings": stats.wings,
                "rooms": stats.rooms,
            })
        }
        "multipass_list_wings" => {
            let stats = db.stats()?;
            json!(stats.wings)
        }
        "multipass_list_rooms" => {
            let stats = db.stats()?;
            json!(stats.rooms)
        }
        "multipass_search" => {
            let query = SearchQuery {
                query: arguments
                    .get("query")
                    .and_then(Value::as_str)
                    .unwrap_or_default()
                    .to_string(),
                wing: arguments
                    .get("wing")
                    .and_then(Value::as_str)
                    .map(ToString::to_string),
                room: arguments
                    .get("room")
                    .and_then(Value::as_str)
                    .map(ToString::to_string),
                limit: arguments.get("limit").and_then(Value::as_u64).unwrap_or(5) as usize,
            };
            json!(db.search(&query)?)
        }
        "multipass_add_record" => {
            let record = record_from_arguments(&arguments)?;
            db.insert_record(&record)?;
            json!({
                "ok": true,
                "record": {
                    "id": record.id,
                    "wing": record.wing,
                    "room": record.room,
                    "corridor": record.corridor,
                    "source_path": record.source_path.map(|p| p.to_string()),
                    "added_by": record.added_by,
                    "created_at": record.created_at.to_rfc3339(),
                    "chunk_index": record.metadata.chunk_index,
                }
            })
        }
        "multipass_delete_record" => {
            let record_id = arguments
                .get("id")
                .and_then(Value::as_str)
                .context("missing record id")?;
            json!({
                "ok": db.delete_record(record_id)?,
                "id": record_id,
            })
        }
        other => bail!("unsupported tool: {other}"),
    };

    Ok(json!({
        "content": [
            {
                "type": "text",
                "text": serde_json::to_string_pretty(&payload)?,
            }
        ]
    }))
}

fn tool_definitions() -> Vec<Value> {
    vec![
        json!({
            "name": "multipass_status",
            "description": "Return high-level ship status and counts.",
            "inputSchema": {
                "type": "object",
                "properties": {},
                "additionalProperties": false
            }
        }),
        json!({
            "name": "multipass_list_wings",
            "description": "List wings known to the current ship.",
            "inputSchema": {
                "type": "object",
                "properties": {},
                "additionalProperties": false
            }
        }),
        json!({
            "name": "multipass_list_rooms",
            "description": "List rooms known to the current ship.",
            "inputSchema": {
                "type": "object",
                "properties": {},
                "additionalProperties": false
            }
        }),
        json!({
            "name": "multipass_search",
            "description": "Search records in the current ship.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "query": { "type": "string" },
                    "wing": { "type": "string" },
                    "room": { "type": "string" },
                    "limit": { "type": "integer", "minimum": 1 }
                },
                "required": ["query"],
                "additionalProperties": false
            }
        }),
        json!({
            "name": "multipass_add_record",
            "description": "Add a manual record to the current ship.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "wing": { "type": "string" },
                    "room": { "type": "string" },
                    "content": { "type": "string" },
                    "corridor": { "type": "string" },
                    "source_path": { "type": "string" },
                    "added_by": { "type": "string" }
                },
                "required": ["wing", "room", "content"],
                "additionalProperties": false
            }
        }),
        json!({
            "name": "multipass_delete_record",
            "description": "Delete a record from the current ship by id.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "id": { "type": "string" }
                },
                "required": ["id"],
                "additionalProperties": false
            }
        }),
    ]
}

fn record_from_arguments(arguments: &Value) -> anyhow::Result<Record> {
    Ok(Record {
        id: Uuid::new_v4().to_string(),
        wing: arguments
            .get("wing")
            .and_then(Value::as_str)
            .context("missing wing")?
            .to_string(),
        room: arguments
            .get("room")
            .and_then(Value::as_str)
            .context("missing room")?
            .to_string(),
        corridor: arguments
            .get("corridor")
            .and_then(Value::as_str)
            .map(ToString::to_string),
        source_path: arguments
            .get("source_path")
            .and_then(Value::as_str)
            .map(|path| path.into()),
        source_mtime: None,
        added_by: arguments
            .get("added_by")
            .and_then(Value::as_str)
            .unwrap_or("mcp")
            .to_string(),
        created_at: Utc::now(),
        content: arguments
            .get("content")
            .and_then(Value::as_str)
            .context("missing content")?
            .to_string(),
        metadata: RecordMetadata { chunk_index: 0 },
    })
}

fn read_message(reader: &mut impl BufRead) -> anyhow::Result<Option<Value>> {
    let mut content_length = None;
    loop {
        let mut line = String::new();
        let bytes = reader.read_line(&mut line)?;
        if bytes == 0 {
            return Ok(None);
        }
        let trimmed = line.trim_end();
        if trimmed.is_empty() {
            break;
        }
        if let Some((name, value)) = trimmed.split_once(':')
            && name.eq_ignore_ascii_case("content-length")
        {
            content_length = Some(value.trim().parse::<usize>()?);
        }
    }

    let length = content_length.context("missing Content-Length header")?;
    let mut body = vec![0_u8; length];
    reader.read_exact(&mut body)?;
    let value = serde_json::from_slice::<Value>(&body)?;
    Ok(Some(value))
}

fn write_message(writer: &mut impl Write, value: &Value) -> anyhow::Result<()> {
    let body = serde_json::to_vec(value)?;
    write!(writer, "Content-Length: {}\r\n\r\n", body.len())?;
    writer.write_all(&body)?;
    writer.flush()?;
    Ok(())
}
