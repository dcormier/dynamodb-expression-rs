use std::collections::HashMap;

use aws_sdk_dynamodb::{
    primitives::Blob,
    types::AttributeValue::{self, Bool, Bs, Ns, Null, Ss, B, L, M, N, S},
};
use base64::engine::{general_purpose, Engine, GeneralPurpose};
use itermap::IterMap;
use itertools::Itertools;

use super::DebugItem;

pub const ATTR_ID: &str = "id";
pub const ATTR_MAP: &str = "map";
pub const ATTR_LIST: &str = "list";
pub const ATTR_STRING: &str = "string";
pub const ATTR_STRINGS: &str = "strings";
pub const ATTR_NUM: &str = "num";
pub const ATTR_NUMS: &str = "nums";
pub const ATTR_BLOB: &str = "blob";
pub const ATTR_BLOBS: &str = "blobs";
pub const ATTR_BOOL: &str = "bool";
pub const ATTR_NULL: &str = "null";

pub fn new_item<T>(id: T) -> HashMap<String, AttributeValue>
where
    T: Into<String>,
{
    let values = vec![
        (ATTR_STRING, S("foo".into())),
        (ATTR_STRINGS, Ss(into_strings(["a", "b", "c"]))),
        (ATTR_NUM, N("42".into())),
        (
            ATTR_NUMS,
            Ns(into_strings([
                "1",
                "2.0000000000000000000000000000000000001",
                // "2.00000000000000000000000000000000000001", // Too long.
                "3",
            ])),
        ),
        (ATTR_NULL, Null(true)),
        (ATTR_BOOL, Bool(true)),
        (ATTR_BLOB, B(Blob::new(base64_encode("foo")))),
        (ATTR_BLOBS, Bs(into_blobs(["foo", "bar", "baz"]))),
    ];

    let mut list = values.iter().map(|(_k, v)| v).cloned().collect_vec();
    // Nested list.
    list.push(L(list.clone()));

    let mut item = into_map(values);
    let map = M(item.clone());
    list.push(map.clone());

    item.insert(ATTR_LIST.into(), L(list));

    // Nested map.
    item.insert(ATTR_MAP.into(), M(item.clone()));
    item.insert(ATTR_ID.into(), S(id.into()));

    item
}

fn into_strings<I, T>(strings: I) -> Vec<String>
where
    I: IntoIterator<Item = T>,
    T: Into<String>,
{
    strings.into_iter().map(Into::into).collect()
}

fn into_blobs<I, T>(binaries: I) -> Vec<Blob>
where
    I: IntoIterator<Item = T>,
    T: AsRef<[u8]>,
{
    binaries
        .into_iter()
        .map(base64_encode)
        .map(Blob::new)
        .collect()
}

fn into_map<I, K>(strings: I) -> HashMap<String, AttributeValue>
where
    I: IntoIterator<Item = (K, AttributeValue)>,
    K: Into<String>,
{
    strings.into_iter().map_keys(Into::into).collect()
}

/// The base64 encoding used by DynamoDB.
/// Import `base64::engine::Engine` to use it to encode or decode base64.
pub const DYNAMODB_BASE64: GeneralPurpose = general_purpose::STANDARD;

/// Produces base64 the way DynamoDB wants it.
pub fn base64_encode<T>(b: T) -> String
where
    T: AsRef<[u8]>,
{
    DYNAMODB_BASE64.encode(b)
}

/// Decodes base64 from DynamoDB.
#[allow(unused)]
pub fn base64_decode<T>(b: T) -> Result<Vec<u8>, base64::DecodeError>
where
    T: AsRef<[u8]>,
{
    DYNAMODB_BASE64.decode(b)
}

#[test]
fn item() {
    let item = new_item("ITEM ID");
    println!("{:#?}", DebugItem(item));
}
