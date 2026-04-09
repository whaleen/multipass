use camino::Utf8PathBuf;
use multipass_mcp::handle_request;
use serde_json::json;
use tempfile::TempDir;

#[test]
fn initialize_reports_current_package_version() -> anyhow::Result<()> {
    let temp = TempDir::new()?;
    let ship_root = Utf8PathBuf::from_path_buf(temp.path().to_path_buf()).unwrap();

    let response = handle_request(
        &ship_root,
        json!({"jsonrpc":"2.0","id":1,"method":"initialize","params":{}}),
    )?;

    assert_eq!(response["result"]["serverInfo"]["name"], "multipass-rs");
    assert_eq!(
        response["result"]["serverInfo"]["version"],
        env!("CARGO_PKG_VERSION")
    );
    Ok(())
}

#[test]
fn add_search_and_delete_record_over_protocol() -> anyhow::Result<()> {
    let temp = TempDir::new()?;
    let ship_root = Utf8PathBuf::from_path_buf(temp.path().to_path_buf()).unwrap();

    let add = handle_request(
        &ship_root,
        json!({
            "jsonrpc":"2.0",
            "id":2,
            "method":"tools/call",
            "params":{
                "name":"multipass_add_record",
                "arguments":{
                    "wing":"manual",
                    "room":"notes",
                    "content":"operator note from protocol test"
                }
            }
        }),
    )?;
    let add_payload = content_json(&add)?;
    let record_id = add_payload["record"]["id"].as_str().unwrap().to_string();

    let search = handle_request(
        &ship_root,
        json!({
            "jsonrpc":"2.0",
            "id":3,
            "method":"tools/call",
            "params":{
                "name":"multipass_search",
                "arguments":{
                    "query":"operator",
                    "wing":"manual",
                    "limit":5
                }
            }
        }),
    )?;
    let search_payload = content_json(&search)?;
    assert_eq!(search_payload.as_array().unwrap().len(), 1);

    let delete = handle_request(
        &ship_root,
        json!({
            "jsonrpc":"2.0",
            "id":4,
            "method":"tools/call",
            "params":{
                "name":"multipass_delete_record",
                "arguments":{"id":record_id}
            }
        }),
    )?;
    let delete_payload = content_json(&delete)?;
    assert_eq!(delete_payload["ok"], true);
    Ok(())
}

fn content_json(response: &serde_json::Value) -> anyhow::Result<serde_json::Value> {
    let text = response["result"]["content"][0]["text"]
        .as_str()
        .unwrap_or_default();
    Ok(serde_json::from_str(text)?)
}
