use flutter_rust_bridge::StreamSink;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum AgentEvent {
    Thought(String),
    Action(String),
    Result(String),
    Done(String),
}

#[flutter_rust_bridge::frb(sync)] // Synchronous mode for simplicity of the demo
pub fn greet(name: String) -> String {
    format!("Hello, {name}!")
}

#[flutter_rust_bridge::frb(init)]
pub fn init_app() {
    // Default utilities - feel free to customize
    flutter_rust_bridge::setup_default_user_utils();
}

pub fn stream_agent_events(sink: StreamSink<AgentEvent>) -> anyhow::Result<()> {
    tokio::spawn(async move {
        sink.add(AgentEvent::Thought("Cyber-Terminal Neural Link Active...".to_string())).ok();
        tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
        sink.add(AgentEvent::Action("Synchronizing with Gestalt-Core...".to_string())).ok();
        tokio::time::sleep(tokio::time::Duration::from_millis(800)).await;
        sink.add(AgentEvent::Result("Connection established. 120fps streaming ready.".to_string())).ok();
        sink.add(AgentEvent::Done("Welcome, User.".to_string())).ok();
    });
    Ok(())
}
