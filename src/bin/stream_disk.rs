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
    let file = tokio::fs::File::open(&args.source).await?;
    let byte_stream =
        tokio_util::codec::FramedRead::new(file, tokio_util::codec::BytesCodec::new());

    let body_stream = reqwest::Body::wrap_stream(byte_stream);
    let filepart = reqwest::multipart::Part::stream(body_stream)
        .file_name(args.source.clone())
        .mime_str(
            mime_guess::from_path(&args.source)
                .first_or_text_plain()
                .as_ref(),
        )?;
    let multipart = reqwest::multipart::Form::new()
        .part("File", filepart)
        .text("WorkspaceId", args.workspace_id.to_string())
        .text("FolderId", args.folder_id.to_string());

    let response = send_request(&mediabank_url, &api_token, multipart).await?;
    let prettified = serde_json::to_string_pretty(&response).unwrap();

    println!("{}", prettified);

    Ok(())
}
