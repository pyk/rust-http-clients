# A Guide to Use HTTP Clients Library In Rust
This repository contains an example of how to use HTTP client library in Rust.

We will start with low level library called `Hyper` and move up with high-level
library called `reqwest`.

Libraries:
- Hyper
- reqwest

You can add more library in the issue.

# GET Request

Below is an example code on how to perform HTTP `GET` request using the
`hyper` library

```rust
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
```

Maybe you are beginner like me and have the following questions:

1. What is `#[tokio::main]` ? Why is there?
2. Why returns `Result<(), Box<dyn std::error::Error + Send + Sync>>` ?

See you next time
