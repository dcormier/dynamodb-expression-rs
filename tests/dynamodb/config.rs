use aws_config::BehaviorVersion;
use aws_sdk_dynamodb::{
    config::{Builder, Credentials, Region},
    Client,
};
use tokio::sync::OnceCell;

#[derive(Debug, Clone)]
pub struct Config {
    pub endpoint: Option<String>,
    pub creds: Option<(String, String)>,
    pub table_name: String,

    #[allow(unused)]
    client: OnceCell<Client>,
}

impl Config {
    /// For testing against dynamodb-local on its default port (8000).
    ///
    /// You can run this from docker as:
    /// ```
    /// docker run -p 127.0.0.1:8000:8000 amazon/dynamodb-local
    /// ```
    #[allow(unused)]
    pub fn new_local() -> Self {
        Self {
            endpoint: Some("http://127.0.0.1:8000".into()),
            creds: Some((
                "AKIAIOSFODNN7EXAMPLE".into(),
                "wJalrXUtnFEMI/K7MDENG/bPxRfiCYEXAMPLEKEY".into(),
            )),
            table_name: "dynamodb-expression-test".into(),
            client: OnceCell::default(),
        }
    }

    async fn dynamodb_config(&self) -> aws_sdk_dynamodb::Config {
        let mut loader = aws_config::defaults(BehaviorVersion::latest());

        if let Some((key, secret)) = &self.creds {
            let creds = Credentials::new(key, secret, None, None, "StaticDummy");

            loader = loader
                .credentials_provider(creds)
                .region(Region::new("us-local-1"));
        }

        let mut builder = Builder::from(&loader.load().await);

        if let Some(endpoint) = &self.endpoint {
            println!("Using DynamoDB endpoint: {endpoint}");

            builder = builder.endpoint_url(endpoint)
        }

        builder.build()
    }

    async fn to_client(&self) -> Client {
        Client::from_conf(self.dynamodb_config().await)
    }

    /// The client is initialized only the first time this is called.
    pub async fn client(&self) -> &Client {
        self.client.get_or_init(|| self.to_client()).await
    }
}
