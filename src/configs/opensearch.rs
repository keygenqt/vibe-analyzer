use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenSearchConfig {
    pub host: String,
    pub username: String,
    pub password: String,
}

impl Default for OpenSearchConfig {
    fn default() -> Self {
        Self {
            host: "http://localhost:9200".to_string(),
            username: "admin".to_string(),
            password: "password".to_string(),
        }
    }
}
