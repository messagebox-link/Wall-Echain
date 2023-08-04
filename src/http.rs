use std::{io::Error, time::Duration};

use async_std::io;
use log::{debug, error};

use crate::{
    types::{JsonRpc, JsonRpcRequest}, 
    utils::random_number::random_number,
};

pub async fn http_send(urls: &Vec<String>, retry: u32, query: JsonRpcRequest) -> Option<JsonRpc> {
    let mut retry = retry;

    while retry > 0 {
        let url = urls.get(random_number(urls.len()))?;
        debug!("🔧 Random url: {:?}", url);

        let req = match surf::post(url).body_json(&query) {
            Ok(req) => req,
            Err(e) => {
                error!("😮‍💨 Failed to build request: {:?}", e);
                retry -= 1;
                continue;
            }
        };

        match io::timeout(Duration::from_secs(60), async {
            let mut res = match req.await {
                Ok(res) => res,
                Err(e) => {
                    error!("😮‍💨 Failed to send request: {:?}", e);
                    return Err(Error::new(io::ErrorKind::Other, "Oh No!"));
                }
            };

            match res.body_json::<JsonRpc>().await {
                Ok(t) => Ok(t),
                Err(e) => {
                    error!("😮‍💨 JSON type conversion failed: {:?} {:?}", e, res);
                    return Err(Error::new(
                        io::ErrorKind::Other,
                        "JSON type conversion failed",
                    ));
                }
            }
        })
        .await
        {
            Ok(t) => return Some(t),
            Err(_) => retry -= 1,
        }
    }

    error!("😮‍💨 The request does not seem to be working properly, please check if the RPC server is working");

    None
}

// batch request
pub async fn http_send_batch(urls: &Vec<String>, retry: u32, query_batch: Vec<JsonRpcRequest>) -> Option<Vec<JsonRpc>> {
    let mut retry = retry;

    while retry > 0 {
        let url = urls.get(random_number(urls.len()))?;
        debug!("🔧 Random url: {:?}", url);

        let req = match surf::post(url).body_json(&query_batch) {
            Ok(req) => req,
            Err(e) => {
                error!("😮‍💨 Failed to build request: {:?}", e);
                retry -= 1;
                continue;
            }
        };

        match io::timeout(Duration::from_secs(60), async {
            let mut res = match req.await {
                Ok(res) => res,
                Err(e) => {
                    error!("😮‍💨 Failed to send request: {:?}", e);
                    return Err(Error::new(io::ErrorKind::Other, "Oh No!"));
                }
            };

            match res.body_json::<Vec<JsonRpc>>().await {
                Ok(t) => Ok(t),
                Err(e) => {
                    error!("😮‍💨 JSON type conversion failed: {:?} {:?}", e, res);
                    return Err(Error::new(
                        io::ErrorKind::Other,
                        "JSON type conversion failed",
                    ));
                }
            }
        })
        .await
        {
            Ok(mut t) => {// sort by id 
                t.sort_by(|a, b|a.id.cmp(&b.id));
                return Some(t)
            },
            Err(_) => retry -= 1,
        }
    }

    error!("😮‍💨 The request does not seem to be working properly, please check if the RPC server is working");

    None
}

#[cfg(test)]
mod ron_test_http {
    use serde_json::Value;
    use crate::types::JsonRpcRequest;
    use super::http_send_batch;

    #[async_std::test]
    async fn test_http_send_batch() {
        let urls = vec![
            "http://eth.node.diagonalley.xyz/".to_string(),
        ];
        let query_batch = vec![
            JsonRpcRequest {
                method: "eth_getCode".to_string(),
                params: Value::Array(vec![
                    serde_json::Value::String("0x1497dd518b392b26bcf799abd9943190aa1edbf3".to_string()),
                    serde_json::Value::String("latest".to_string()),
                ]),
                id: 1,
                jsonrpc: "2.0".to_string(),
            },
            JsonRpcRequest {
                method: "eth_getCode".to_string(),
                params: Value::Array(vec![
                    serde_json::Value::String("0x1497dd518b392b26bcf799abd9943190aa1edbf4".to_string()),
                    serde_json::Value::String("latest".to_string()),
                ]),
                id: 2,
                jsonrpc: "2.0".to_string(),
            },
        ];
        let res = http_send_batch(&urls, 1, query_batch).await;
        assert_eq!(true, res.is_some());
        // println!("res: {:#?}", res);
    }
}