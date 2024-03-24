use ollama_rs::{
    generation::{
        completion::{request::GenerationRequest, GenerationContext, GenerationResponseStream},
        options::GenerationOptions,
    },
    Ollama,
};
use tokio::io::{stdout, AsyncWriteExt};
use tokio_stream::StreamExt;

mod app;
mod database;

use state::Config;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::parse_env()?;
    let ollama = Ollama::default();

    let mut stdout = stdout();

    let mut context: Option<GenerationContext> = None;

    loop {
        stdout.write_all(b"\n> ").await?;
        stdout.flush().await?;

        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;

        let input = input.trim_end();
        if input.eq_ignore_ascii_case("exit") {
            break;
        }

        let options = GenerationOptions::default();
        let options = options.stop(vec!["<|im_end|>".into()]);

        let mut request =
            GenerationRequest::new("nous-hermes-2-pro".into(), input.to_string()).options(options);
        if let Some(context) = context.clone() {
            request = request.context(context);
        }
        let mut stream: GenerationResponseStream = ollama.generate_stream(request).await?;

        while let Some(Ok(res)) = stream.next().await {
            for ele in res.clone() {
                stdout.write_all(ele.response.as_bytes()).await?;
                stdout.flush().await?;

                if let Some(final_data) = ele.final_data {
                    context = Some(final_data.context);
                }
            }
        }
    }

    Ok(())
}
