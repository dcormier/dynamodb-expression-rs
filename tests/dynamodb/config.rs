use aws_sdk_dynamodb::{config::Builder, Client};
use tokio::sync::OnceCell;

#[derive(Debug, Clone)]
pub struct Config {
    pub endpoint: Option<String>,
    pub table_name: String,

    client: OnceCell<Client>,
}

impl Config {
    async fn dynamodb_config(&self) -> aws_sdk_dynamodb::Config {
        let mut builder = Builder::from(&aws_config::from_env().load().await);

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
