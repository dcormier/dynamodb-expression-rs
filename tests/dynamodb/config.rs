use aws_config::{BehaviorVersion, SdkConfig};
use aws_sdk_dynamodb::{config::Region, Client};
use tokio::sync::OnceCell;

#[derive(Debug, Clone)]
pub struct Config {
    pub endpoint: Option<String>,
    pub table_name: String,
    client: OnceCell<Client>,
}

impl Config {
    /// For testing against dynamodb-local on its default port (8000).
    ///
    /// You can run this from docker as:
    /// ```
    /// docker run -p 127.0.0.1:8000:8000 amazon/dynamodb-local
    /// ```
    pub fn new_local() -> Self {
        Self {
            endpoint: Some("http://127.0.0.1:8000".into()),
            table_name: "dynamodb-expression-test".into(),
            client: OnceCell::default(),
        }
    }

    async fn sdk_config(&self) -> SdkConfig {
        let mut loader = aws_config::defaults(BehaviorVersion::latest());

        if let Some(endpoint) = &self.endpoint {
            println!("Using DynamoDB endpoint: {endpoint}");

            loader = loader
                .endpoint_url(endpoint)
                .test_credentials()
                .region(Region::new("us-local-1"));
        }

        loader.load().await
    }

    /// The client is initialized only the first time this is called.
    pub async fn client(&self) -> &Client {
        self.client
            .get_or_init(|| async { Client::new(&self.sdk_config().await) })
            .await
    }
}
