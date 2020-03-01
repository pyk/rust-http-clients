extern crate hyper;

use hyper::body::HttpBody as _;
use hyper::Client;
use tokio::io::{stdout, AsyncWriteExt as _};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // This is where we will setup our HTTP client requests.
    println!("Hello?");

    // Still inside `async fn main`...
    let client = Client::new();

    // Parse an `http::Uri`...
    let uri = "http://httpbin.org/ip".parse()?;

    // Await the response...
    let mut resp = client.get(uri).await?;

    println!("Response: {}", resp.status());

    // And now...
    while let Some(chunk) = resp.body_mut().data().await {
        stdout().write_all(&chunk?).await?;
    }

    Ok(())
}
