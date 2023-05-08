use std::{io::Error, time::Duration};

use async_std::io;
use log::{debug, error};

use crate::{
    types::{JsonRpc, JsonRpcRequest},
    utils::random_number,
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
