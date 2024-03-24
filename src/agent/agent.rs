use ollama_rs::Ollama;
use url::Url;

#[derive(Debug, Clone)]
pub struct Agent(Ollama);

impl Agent {
    pub fn new(url: &Url) -> Self {
        let host = url.host_str().unwrap_or("localhost");
        let port = url.port().unwrap_or(11434);
        Self(Ollama::new(host.to_string(), port))
    }
    pub fn get_ollama(&self) -> &Ollama {
        &self.0
    }
}
