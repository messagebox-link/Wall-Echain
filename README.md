# Wall-Echain 🤖

Wall-Echain is a product for obtaining block information, transaction details, and the latest block height of the Ethereum system through JSON-RPC.

[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)
[![Crates.io](https://img.shields.io/crates/v/wall-echain.svg)](https://crates.io/crates/wall-echain)
[![Documentation](https://docs.rs/wall-echain/badge.svg)](https://docs.rs/wall-echain)

## Features 🌟

- Customized RPC server list, retry count, chain ID
- Get detailed information on the Ethereum blockchain
- Query transaction details
- Real-time tracking of the latest block height

## Usage 💡

- First, install Wall-Echain
  1. Cargo.toml
     ```toml
     # Cargo.toml
     [dependencies]
     wall-echain = "0.1.0"
     ```
  2. Cargo add
     ```shell
     Cargo add wall-echain
     ```
- Next, you can use the following code to query the latest block height on the ethereum blockchain.

  ```rust
  let w = wall::new(Vec["https://cloudflare-eth.com/".to_string()],None, None);
  if let Some(t) = w.get_latest_number().await {
    println!("{:?}", t);
  }

  ```

- You can also use the following code to query transaction details by transaction hash.

  ```rust

  let w = wall::new(Vec["https://cloudflare-eth.com/".to_string()],None, None);
  if let Some(t) = w.get_transaction_receipt_for_hash("0x80fdaa7f5f54cbe28b84f41afb9543cf0c9eb0d9f4b8a620c2fb5faf0b1c2810").await {
    println!("{:?}", t);
  }

  ```

- You can also use the following code to query block information by specifying the block height.

  ```rust
  let w = wall::new(Vec["https://cloudflare-eth.com/".to_string()],None, None);
  if let Some(t) = w.get_transactions_for_block("0xe5b544", false).await {
    println!("{:?}", t);
  }

  ```

## TODO 🚧

- [ ] Support all JSON-RPC interfaces

## Contribution 💪

Contributions to the Wall-Echain project are welcome!

## License 📜

Wall-Echain is licensed under the MIT license, see the LICENSE file for details.
