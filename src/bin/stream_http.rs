//! This does not work as expected. The response indicates that the upload is successful but the
//! file is not viewable on the site.

use clap::Parser;
use request::{send_request, Args};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv()?;
    let mediabank_url =
        std::env::var("MEDIABANK_URL").expect("MEDIABANK_URL should be set in .env");
    let api_token = std::env::var("API_TOKEN").expect("API_TOKEN should be set in .env");
    let args = Args::parse();

    eprintln!("Uploading a file to mediabank with a stream from the response of a http request");

    let client = reqwest::Client::new();

    let response = client.get(args.source).send().await?;
    let content_length = response.content_length();
    let file_name = response
        .url()
        .path_segments()
        .map(|segments| segments.last().unwrap())
        .unwrap()
        .to_string();
    let bytes_stream = response.bytes_stream();

    let body_stream = reqwest::Body::wrap_stream(bytes_stream);
    let file_part = if let Some(content_length) = content_length {
        reqwest::multipart::Part::stream_with_length(body_stream, content_length)
    } else {
        reqwest::multipart::Part::stream(body_stream)
    };

    let file_part = file_part.file_name(file_name.clone()).mime_str(
        mime_guess::from_path(&file_name)
            .first_or_text_plain()
            .as_ref(),
    )?;
    let multipart = reqwest::multipart::Form::new()
        .part("File", file_part)
        .text("WorkspaceId", args.workspace_id.to_string())
        .text("FolderId", args.folder_id.to_string());

    let response = send_request(&mediabank_url, &api_token, multipart).await?;
    let prettified = serde_json::to_string_pretty(&response).unwrap();

    println!("{}", prettified);

    Ok(())
}
