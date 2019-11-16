# apod-async-client #

`apod-async-client` is a crate providing a simple client for NASA ["Astronomy
Picture of the Day" API](https://api.nasa.gov/#browseAPI).

The client is based on [reqwest](https://github.com/seanmonstar/reqwest) and
[tokio](https://github.com/tokio-rs/tokio), and requires the tokio reactor to be
setup in your application.

```rust
use apod_async_client::{APODClient, APODClientError, Date};
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), APODClientError> {
    let client = APODClient::new("DEMO_KEY");
    let (metadata, rate_limit) = client.get_picture(&Date::Today, true).await?;
    Ok(())
}
```

This crate is quite simple, but allowed me to exercise the following points :
  * Learn about Rust asynchronous IO, async/await and futures management
  * Practice error design in a Rust library, using
    [thiserror](https://github.com/dtolnay/thiserror)
  * Practice HTTP mocking using [mockito](https://github.com/lipanski/mockito),
    tests and documentation in Rust.

