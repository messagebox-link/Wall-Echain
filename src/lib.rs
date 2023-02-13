use log::debug;
use serde_json::Value;
use types::JsonRpc;

use crate::{http::http_send, types::JsonRpcRequest};

mod http;
pub mod types;
mod utils;

#[derive(Debug, Clone)]
pub struct Wall {
    /// RPC Server URLs
    pub urls: Vec<String>,
    /// Re-try quest
    pub retry: u32,
    /// ChainId
    pub id: String,
}

impl Wall {
    pub fn new(urls: Vec<String>, retry: Option<u32>, id: Option<String>) -> Self {
        Wall {
            urls,
            retry: retry.unwrap_or(3),
            id: id.unwrap_or("1".to_string()),
        }
    }

    pub async fn get_latest_number(&self) -> Option<JsonRpc> {
        self.send_rpc_request("eth_blockNumber", vec![]).await
    }

    pub async fn get_transactions_for_block(&self, number: &str, is_show: bool) -> Option<JsonRpc> {
        self.send_rpc_request(
            "eth_getBlockByNumber",
            vec![Value::String(number.to_string()), Value::Bool(is_show)],
        )
        .await
    }

    pub async fn get_transaction_receipt_for_hash(&self, hash: &str) -> Option<JsonRpc> {
        self.send_rpc_request(
            "eth_getTransactionReceipt",
            vec![Value::String(hash.to_string())],
        )
        .await
    }

    async fn send_rpc_request(&self, method: &str, params: Vec<Value>) -> Option<JsonRpc> {
        let query = JsonRpcRequest {
            method: method.to_string(),
            params: Value::Array(params),
            id: self.id.parse().unwrap(),
            jsonrpc: "2.0".to_string(),
        };

        debug!("🔧 Query: {:?}", query);

        http_send(self.urls.as_ref(), self.retry, query).await
    }
}

#[cfg(test)]
mod tests {
    use super::Wall;
    fn init() -> Wall {
        Wall::new(
            vec![
                "https://cloudflare-eth.com/".to_string(),
                // "https://eth.rpc.blxrbdn.com".to_string(),
                // "https://rpc.builder0x69.io".to_string(),
            ],
            None,
            None,
        )
    }

    #[async_std::test]
    async fn test_get_latest_number() {
        let bs = init();
        assert_eq!(bs.get_latest_number().await.unwrap().result.is_some(), true);
    }

    #[async_std::test]
    async fn test_get_transactions_for_block() {
        let bs = init();

        assert_eq!(
            bs.get_transactions_for_block("0xe5b544", false)
                .await
                .unwrap()
                .result
                .is_some(),
            true
        )
    }

    #[async_std::test]
    async fn test_get_transactions_for_block_error() {
        let bs = init();

        assert_eq!(
            bs.get_transactions_for_block("1233333", false)
                .await
                .is_some(),
            false
        )
    }

    #[async_std::test]
    async fn test_get_transaction_receipt_for_hash_error() {
        let bs = init();

        println!(
            "{:?}",
            bs.get_transaction_receipt_for_hash(
                "0x890137f33ba2ddfa9778d55336e83bcac477232d1de7abb3f3dca0ae019798"
            )
            .await
        );
        assert_eq!(
            false,
            bs.get_transaction_receipt_for_hash(
                "0x890137f33ba2ddfa9778d55336e83bcac477232d1de7abb3f3dca0ae019798"
            )
            .await
            .unwrap()
            .result
            .is_some()
        );
    }

    #[async_std::test]
    async fn test_get_transaction_receipt_for_hash() {
        let bs = init();
        assert_eq!(
            true,
            bs.get_transaction_receipt_for_hash(
                "0x80fdaa7f5f54cbe28b84f41afb9543cf0c9eb0d9f4b8a620c2fb5faf0b1c2810"
            )
            .await
            .unwrap()
            .result
            .is_some()
        );
    }
}
