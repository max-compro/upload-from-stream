use clap::Parser;

#[derive(Debug, Parser)]
pub struct Args {
    /// To which workspace the fie is uploaded to.
    #[arg(short, long)]
    pub workspace_id: u64,
    /// To which folder the file is uploaded to.
    #[arg(short, long)]
    pub folder_id: u64,
    /// From where the file is uploaded.
    ///
    /// This can be a path or an url depending on which binary is executed:
    ///
    /// multipart => path,
    ///
    /// stream_disk => path,
    ///
    /// stream_http => url,
    pub source: String,
}

/// Sends the http request and deserializes the response into dynamic json
pub async fn send_request(
    url: &str,
    api_token: &str,
    multipart: reqwest::multipart::Form,
) -> reqwest::Result<serde_json::Value> {
    let client = reqwest::Client::new();

    client
        .post(url)
        .multipart(multipart)
        .header("X-Api-Key", api_token)
        .send()
        .await?
        .json()
        .await
}
