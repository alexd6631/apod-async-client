use apod_async_client::{APODClient, APODClientError, APODMetadata, Date, RateLimitInfo};

#[tokio::test]
async fn test_ok_response() {
    let server_url = format!("{}/apod", mockito::server_url());

    let _m = mockito::mock("GET", "/apod?api_key=MYKEY&hd=true")
        .with_body(include_str!("data/ok.json"))
        .with_header("x-ratelimit-remaining", "42")
        .with_header("x-ratelimit-limit", "100")
        .create();

    let client = APODClient::config(server_url, "MYKEY");
    let result = client.get_picture(&Date::Today, true).await.unwrap();

    let expected_metadata = APODMetadata {
        title: "The Star Streams of NGC 5907".to_owned(),
        explanation: "explanation ...".to_owned(),
        copyright: Some("R Jay Gabany".to_owned()),
        url: "https://apod.nasa.gov/apod/image/1911/ngc5907_gabany_rcl1024.jpg".to_owned(),
        hd_url: Some("https://apod.nasa.gov/apod/image/1911/ngc5907_gabany_rcl.jpg".to_owned()),
        media_type: "image".to_owned(),
    };
    let expected_limit = RateLimitInfo {
        remaining: 42,
        limit: 100,
    };

    assert_eq!(result, (expected_metadata, expected_limit));
}

#[tokio::test]
async fn test_forbidden_response() {
    let server_url = format!("{}/apod", mockito::server_url());

    let _m = mockito::mock("GET", "/apod?api_key=MYKEY&hd=true")
        .with_status(403)
        .with_header("x-ratelimit-remaining", "42")
        .with_header("x-ratelimit-limit", "100")
        .create();

    let client = APODClient::config(server_url, "MYKEY");
    let err = client.get_picture(&Date::Today, true).await.err().unwrap();

    match err {
        APODClientError::RequestStatusError {
            status: 403,
            source: _,
        } => (),
        e => panic!("Unexepected error {}", e),
    }
}
