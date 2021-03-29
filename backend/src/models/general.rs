use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Error {
    pub status_code: String,
    pub error: String,
}
