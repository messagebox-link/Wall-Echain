use hex::decode;
use hex::encode;
use rlp::RlpStream;
use sha3::Keccak256;
use sha3::Digest;
use super::convert_hex::hex2num;

pub fn from_nonce2contract(from: String, nonce: String) -> String {
    let sender_bytes = decode(&from[2..]).expect("Invalid from address");
    let nonce = hex2num(&nonce).expect("Invalid from address") as u64;

    // create RLP stream, add "from"、"nonce"
    let mut rlp_stream = RlpStream::new_list(2);
    rlp_stream.append(&sender_bytes).append(&nonce);
    let rlp_encoded = rlp_stream.out().to_vec();

    let hash = Keccak256::digest(&rlp_encoded);

    let contract_address = &hash[hash.len() - 20..];

    let contract_address_hex = encode(contract_address);

    format!("0x{}", contract_address_hex)
}

#[test]
fn test() {
    let from = "0x36928500bc1dcd7af6a2b4008875cc336b927d57".to_string();
    let nonce = "0x6".to_string();
    let contract = from_nonce2contract(from.clone(), nonce.clone());
    println!("{}", contract);
}