use polywrap_core::uri::{ Uri };
use polywrap_client::{
    client::{ PolywrapClient },
    msgpack::{ msgpack }
};
use polywrap_client::builder::types::{
  ClientBuilder,
  BuilderConfig,
  ClientConfigHandler
};

use std::env::{ current_dir };
use serde::Deserialize;

#[tokio::main]
async fn main() {
    let uri = Uri::try_from(format!(
        "fs/{}",
        current_dir().unwrap().as_path().join(
            "../../build"
        ).display(),
    )).unwrap();

    let mut builder = BuilderConfig::new(None);
    let config = builder.build();

    let client = PolywrapClient::new(config);

    let resp = client.invoke::<Vec<u8>>(
        &uri,
        "encodeMessage",
        Some(&msgpack!({
            "message": {
              "actor": "foo-bar".to_string(),
              "args": "{ \"arg1\": [1, 2, 3] }".to_string()
            }
        })),
        None,
        None
    ).unwrap();

    println!("{:?}", resp);
}