use std::error::Error;

use aws_sdk_dynamodb::{
    error::SdkError,
    operation::{create_table::CreateTableError, delete_table::DeleteTableError},
    types::{
        AttributeDefinition, KeySchemaElement, KeyType, ProvisionedThroughput, ScalarAttributeType,
    },
    Client,
};
use easy_error::ErrorExt;
use itertools::Itertools;

use super::item::ATTR_ID;

/// Creates the table if it doesn't already exist. Logs success or failure.
#[allow(unused)]
pub async fn create_table(
    client: &Client,
    table_name: &str,
) -> Result<(), SdkError<CreateTableError>> {
    client
        .create_table()
        .table_name(table_name)
        .key_schema(
            KeySchemaElement::builder()
                .key_type(KeyType::Hash)
                .attribute_name(ATTR_ID)
                .build(),
        )
        .attribute_definitions(
            AttributeDefinition::builder()
                .attribute_name(ATTR_ID)
                .attribute_type(ScalarAttributeType::S)
                .build(),
        )
        .provisioned_throughput(
            ProvisionedThroughput::builder()
                .read_capacity_units(1)
                .write_capacity_units(1)
                .build(),
        )
        .send()
        .await
        .map(|output| {
            println!(
                "Created table {:?}",
                output
                    .table_description
                    .and_then(|desc| desc.table_name)
                    .unwrap()
            );
        })
        .or_else(|err| match &err {
            SdkError::ServiceError(service_err)
                if service_err.err().is_resource_in_use_exception() =>
            {
                // The table already exists. Task failed successfully.

                println!("Using existing table {table_name:?}");

                Ok(())
            }
            _ => {
                eprintln!(
                    "Error creating table {:?}:\n\t{}",
                    table_name,
                    err.iter_chain()
                        .map(ToString::to_string)
                        .collect_vec()
                        .join(";\n\t"),
                );
                Err(err)
            }
        })
}

#[allow(unused)]
pub async fn delete_table(
    client: &Client,
    table_name: &str,
) -> Result<(), SdkError<DeleteTableError>> {
    client
        .delete_table()
        .table_name(table_name)
        .send()
        .await
        .map(|output| {
            println!(
                "Deleted table {:?}",
                output
                    .table_description
                    .and_then(|desc| desc.table_name)
                    .unwrap()
            );
        })
        .or_else(|err| match &err {
            SdkError::ServiceError(service_err)
                if service_err.err().is_resource_not_found_exception() =>
            {
                // The table doesn't exist. Task failed successfully.
                Ok(())
            }
            _ => {
                eprintln!(
                    "Error deleting table {:?}: {}",
                    table_name,
                    err.iter_chain()
                        .map(ToString::to_string)
                        .collect_vec()
                        .join(";\n\t"),
                );
                Err(err)
            }
        })
}

/// If the table already exists, it's deleted and recreated. If it doesn't exist, it's created.
#[allow(unused)]
pub async fn clean_table(
    client: &Client,
    table_name: &str,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    delete_table(client, table_name).await?;
    create_table(client, table_name).await?;
    Ok(())
}
