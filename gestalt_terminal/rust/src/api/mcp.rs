use crate::frb_generated::StreamSink;
use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::sync::{Mutex, LazyLock};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum McpComponent {
    Card { title: String, content: String },
    Button { label: String, action_id: String },
    Markdown { content: String },
    Row { children: Vec<McpComponent> },
    Column { children: Vec<McpComponent> },
}

static MCP_SINK: LazyLock<Mutex<Option<StreamSink<McpComponent>>>> = LazyLock::new(|| Mutex::new(None));

pub fn stream_mcp_ui(sink: StreamSink<McpComponent>) -> Result<()> {
    {
        let mut guard = MCP_SINK.lock().unwrap();
        *guard = Some(sink.clone());
    }

    // Send initial state
    sink.add(McpComponent::Card {
        title: "Gestalt Agent".into(),
        content: "System Initialized. Waiting for events...".into(),
    }).ok();

    Ok(())
}

#[flutter_rust_bridge::frb(sync)]
pub fn simulate_agent_event(event_type: String) -> Result<()> {
    let guard = MCP_SINK.lock().unwrap();
    if let Some(sink) = guard.as_ref() {
        let component = match event_type.as_str() {
             "analysis" => McpComponent::Markdown {
                content: "# Analysis\n\n- System Status: **Online**\n- FPS: **120**".into()
            },
            "action" => McpComponent::Row {
                children: vec![
                    McpComponent::Button { label: "Approve".into(), action_id: "approve".into() },
                    McpComponent::Button { label: "Reject".into(), action_id: "reject".into() },
                ]
            },
             _ => McpComponent::Card { title: "Event".into(), content: event_type },
        };
        sink.add(component).ok();
    }
    Ok(())
}
