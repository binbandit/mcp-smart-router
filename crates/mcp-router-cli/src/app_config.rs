use hashbrown::HashMap;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct AppConfig {
    pub server: ServerSettings,
    pub downstreams: HashMap<String, DownstreamConfig>,
}

#[derive(Debug, Deserialize)]
pub struct ServerSettings {
    pub transport: String,
    pub port: Option<u16>,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum DownstreamConfig {
    Stdio {
        command: String,
        args: Vec<String>,
        #[serde(default)]
        env: HashMap<String, String>,
    },
    Sse {
        url: String,
    },
}
