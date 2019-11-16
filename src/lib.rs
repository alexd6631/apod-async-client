//!
//! An asynchronous client for NASA ["Astronomy Picture of the Day" API](https://api.nasa.gov/#browseAPI).
//!
//! The client is based on [reqwest](https://github.com/seanmonstar/reqwest) and
//! [tokio](https://github.com/tokio-rs/tokio), and requires the tokio reactor to be setup
//! (which is usually done by annotating your main function with `[tokio::main]`)
//!
//! # Example
//! ```
//!
//! use apod_async_client::{APODClient, APODClientError, Date};
//! use std::error::Error;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), APODClientError> {
//!     let client = APODClient::new("DEMO_KEY");
//!     let (metadata, rate_limit) = client.get_picture(&Date::Today, true).await?;
//!     Ok(())
//! }
//!
//!```

mod client;
mod date;
mod model;

pub use client::{APODClient, APODClientError, RateLimitInfo};
pub use date::Date;
pub use model::APODMetadata;
