use std::ops::Deref;

use futures::StreamExt;
use ollama_rs::{
    generation::{
        completion::{request::GenerationRequest, GenerationContext, GenerationResponseStream},
        options::GenerationOptions,
    },
    Ollama,
};
use url::Url;

#[derive(Debug, Clone)]
pub struct Agent(Ollama);

impl Agent {
    pub fn new(url: &Url) -> Self {
        let scheme = url.scheme();
        let host = url.host_str().unwrap_or("http://localhost");
        let port = url.port().unwrap_or(11434);
        let host = format!("{}://{}", scheme, host);
        Self(Ollama::new(host.to_string(), port))
    }

    pub async fn complete(
        &self,
        input: &str,
        context: Option<GenerationContext>,
    ) -> Result<(String, Option<GenerationContext>), AgentError> {
        let input = input.trim();
        let options = GenerationOptions::default();
        let options = options.stop(vec!["<|im_end|>".to_string()]);

        let request =
            GenerationRequest::new("nous-hermes-2-pro".into(), input.to_string()).options(options);
        if let Some(context) = context.clone() {
            request.clone().context(context);
        }
        let mut stream: GenerationResponseStream =
            self.generate_stream(request.clone()).await.unwrap();
        let mut response_buffer = Vec::new();
        let mut next_context = None;
        while let Some(Ok(response)) = stream.next().await {
            for ele in response.clone() {
                response_buffer.extend(ele.response.as_bytes());

                if let Some(final_data) = ele.final_data {
                    next_context = Some(final_data.context);
                }
            }
        }
        let response = String::from_utf8(response_buffer).unwrap();
        Ok((response, next_context))
    }

    pub async fn complete_stream(
        &self,
        input: &str,
        context: Option<GenerationContext>,
    ) -> Result<GenerationResponseStream, AgentError> {
        let input = input.trim();
        let options = GenerationOptions::default();
        let options = options.stop(vec!["<|im_end|>".to_string()]);
        let request =
            GenerationRequest::new("nous-hermes-2-pro".into(), input.to_string()).options(options);
        if let Some(context) = context.clone() {
            request.clone().context(context);
        }
        let stream: GenerationResponseStream = self.generate_stream(request.clone()).await.unwrap();
        Ok(stream)
    }
}

impl Deref for Agent {
    type Target = Ollama;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug, thiserror::Error)]
pub enum AgentError {
    #[error("default error: {0}")]
    DefaultError(anyhow::Error),
}
