use std::borrow::Cow;

use reqwest::header::HeaderMap;
use thiserror::Error;
use url::Url;

use crate::APODMetadata;
use crate::Date;

/// Client errors
#[derive(Error, Debug)]
pub enum APODClientError {
    /// Service URL cannot be created
    #[error("Service URL cannot be created")]
    InvalidURL { source: url::ParseError },
    /// Rate limit exceeded for this API key
    #[error("Rate limit exceeded for this API key")]
    RateLimitError,
    /// IO error encountered while performing request
    #[error("IO error encountered while performing request")]
    IOError {
        source: Box<dyn std::error::Error + Send + Sync>,
    },
    /// Request failed with invalid HTTP status code
    #[error("Request failed with invalid HTTP status code: {}", status)]
    RequestStatusError {
        status: u16,
        source: Box<dyn std::error::Error + Send + Sync>,
    },
    /// Error while decoding response content
    #[error("Error while decoding response content")]
    DecodeError {
        source: Box<dyn std::error::Error + Send + Sync>,
    },
}

pub type Result<T> = std::result::Result<T, APODClientError>;

/// An asynchronous client for NASA "Astronomy Picture Of the Day" service
pub struct APODClient {
    base_url: Cow<'static, str>,
    api_key: Cow<'static, str>,
}

/// Holds info about API rate limit
#[derive(Debug, PartialEq, Eq)]
pub struct RateLimitInfo {
    /// Remaining requests for this API key and IP address
    pub remaining: i32,
    /// Limit for this API key and IP address
    pub limit: i32,
}

impl APODClient {
    /// Build a client using the provided `api_key`.
    ///
    /// `api_key` can be passed as a `&'static str` or as a owned `String`
    ///
    /// # Example
    /// ```
    /// use apod_async_client::APODClient;
    /// APODClient::new("DEMO_KEY");
    /// APODClient::new(String::from("DEMO_KEY"));
    /// ```
    pub fn new<S>(api_key: S) -> Self
    where
        S: Into<Cow<'static, str>>,
    {
        Self::config("https://api.nasa.gov/planetary/apod", api_key)
    }

    /// Build a client by overriding server `base_url` and providing `api_key`
    pub fn config<SA, SB>(base_url: SA, api_key: SB) -> Self
    where
        SA: Into<Cow<'static, str>>,
        SB: Into<Cow<'static, str>>,
    {
        APODClient {
            base_url: base_url.into(),
            api_key: api_key.into(),
        }
    }

    /// Retrieve Metadata for `AstronomyPicture` for the given `date`
    ///
    /// `RateLimitInfo` are returned as well
    pub async fn get_picture(&self, date: &Date, hd: bool) -> Result<(APODMetadata, RateLimitInfo)> {
        let url = self.build_url(date, hd)?;
        let response = reqwest::get(url)
            .await
            .map_err(|e| APODClientError::IOError {
                source: Box::new(e),
            })?;

        let rate_limit_info = get_rate_limit_info(response.headers());
        if rate_limit_info.remaining == 0 {
            return Err(APODClientError::RateLimitError);
        }

        let response = response.error_for_status().map_err(|e| {
            let status = e
                .status()
                .map(|s| s.as_u16())
                .expect("status code should be defined");
            APODClientError::RequestStatusError {
                status,
                source: Box::new(e),
            }
        })?;

        let pic = response
            .json()
            .await
            .map_err(|e| APODClientError::DecodeError {
                source: Box::new(e),
            })?;

        Ok((pic, rate_limit_info))
    }

    fn build_url(&self, date: &Date, hd: bool) -> Result<Url> {
        let hd_param = hd.to_string();
        let mut params = vec![
            ("api_key", self.api_key.as_ref()),
            ("hd", &hd_param)
        ];
        let maybe_date_param = date.as_param();
        if let Some(date_param) = maybe_date_param.as_ref() {
            params.push(("date", date_param))
        }
        Url::parse_with_params(&self.base_url, &params)
            .map_err(|source| APODClientError::InvalidURL { source })
    }
}

fn get_rate_limit_info(headers: &HeaderMap) -> RateLimitInfo {
    let remaining: i32 = headers
        .get("x-ratelimit-remaining")
        .and_then(|h| h.to_str().ok())
        .and_then(|s| s.parse().ok())
        .unwrap_or(-1);

    let limit: i32 = headers
        .get("x-ratelimit-limit")
        .and_then(|h| h.to_str().ok())
        .and_then(|s| s.parse().ok())
        .unwrap_or(-1);

    RateLimitInfo { remaining, limit }
}

#[cfg(test)]
mod tests {
    use crate::client::APODClient;
    use crate::Date;

    #[test]
    fn test_build_url() {
        let client = APODClient::new("my_key");
        let url = client.build_url(&Date::Today, false).unwrap();
        assert_eq!(
            "https://api.nasa.gov/planetary/apod?api_key=my_key&hd=false",
            url.as_str()
        )
    }

    #[test]
    fn test_build_url_with_date() {
        let d = Date::Date {
            day: 1,
            month: 6,
            year: 1986,
        };
        let client = APODClient::new("my_key");
        let url = client.build_url(&d, true).unwrap();
        assert_eq!(
            "https://api.nasa.gov/planetary/apod?api_key=my_key&hd=true&date=1986-06-01",
            url.as_str()
        )
    }
}
