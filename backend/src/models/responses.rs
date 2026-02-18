use serde::{Deserialize, Serialize};

#[derive(Deserialize,Serialize)]
pub struct EchoResponse {
    pub message: String,
}
