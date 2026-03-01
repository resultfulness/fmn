use serde::Deserialize;

#[derive(Deserialize)]
pub struct EchoRequest {
    pub message: String,
}
impl EchoRequest {
    pub fn new(message: &str) -> Self {
        Self {
            message: message.into(),
        }
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct ItemCreateRequest {
    pub name: String,
    pub icon: String,
    pub unit: String,
}
#[derive(Deserialize, Debug, Clone)]
pub struct ItemUpdateRequest {
    pub name: Option<String>,
    pub icon: Option<String>,
    pub unit: Option<String>,
}
