use serde::{
    Deserialize,
    Serialize,
};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(tag = "type", content = "payload")]
pub enum ClipboardContent {
    Image(Vec<u8>),
    Raw { data: Vec<u8>, format: String },
    Text(String),
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ClipboardEventData {
    pub device_name: Option<String>,
    pub content: ClipboardContent,
    pub timestamp: u64,
}
