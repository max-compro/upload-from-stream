//! This example works as expected, the file is uploaded to Mediabank successfully.

use clap::Parser;
use request::{send_request, Args};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv()?;
    let mediabank_url =
        std::env::var("MEDIABANK_URL").expect("MEDIABANK_URL should be set in .env");
    let api_token = std::env::var("API_TOKEN").expect("API_TOKEN should be set in .env");
    let args = Args::parse();

    eprintln!("Uploading a file to mediabank with content read to memory");

    let file_path = args.source;
    let bytes = tokio::fs::read(&file_path).await?;

    let file_part = reqwest::multipart::Part::bytes(bytes)
        .file_name(file_path.clone())
        .mime_str(
            mime_guess::from_path(&file_path)
                .first_or_text_plain()
                .as_ref(),
        )?;

    let multipart = reqwest::multipart::Form::new()
        .part("File", file_part)
        .text("WorkSpaceId", args.workspace_id.to_string())
        .text("FolderId", args.folder_id.to_string());

    let response = send_request(&mediabank_url, &api_token, multipart).await?;
    let prettified = serde_json::to_string_pretty(&response).unwrap();

    println!("{}", prettified);

    Ok(())
}
