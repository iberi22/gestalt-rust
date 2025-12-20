use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WidgetType {
    Button,
    Markdown,
    CodeBlock,
    Input,
    Card,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UiElement {
    #[serde(rename = "type")]
    pub widget_type: WidgetType,
    pub content: String, // Can be JSON string or simple text depending on widget
    pub id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerResponse {
    pub text_response: String,
    pub ui_component: Option<UiElement>,
}
