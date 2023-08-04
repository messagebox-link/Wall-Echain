use base58::{ToBase58, FromBase58};
use digest::Digest;
use hex::{decode, encode};
use sha2::Sha256;
use sha3::Keccak256;

// Address: ETH to TRX
pub fn eth2trx(address: &str) -> String {
    let addr = address.replace("0x", "41");    
    let h = decode(addr).unwrap();    
    convert_b58encode(h)
}
fn convert_b58encode<T: AsRef<[u8]>>(raw: T) -> String {
    let mut hasher = Sha256::new();
    hasher.update(raw.as_ref());
    let digest1 = hasher.finalize();
    let mut hasher = Sha256::new();
    hasher.update(&digest1);
    let digest = hasher.finalize();
    let mut raw = raw.as_ref().to_owned();    
    raw.extend(&digest[..4]);    
    raw.to_base58()
}

// Address: TRX to ETH 
pub fn trx2eth(addr: &str) -> String {
    let mut addr_vec_u8 = b58decode_check(addr);
    addr_vec_u8 = addr_vec_u8[1..].to_vec();
    let mut addr = format!("0x{}", encode(addr_vec_u8));
    eip55_checksum(unsafe { &mut addr.as_bytes_mut()[2..] });
    addr
}
// Base58check decode.
pub fn b58decode_check(s: &str) -> Vec<u8> {
    let mut result = s.from_base58().unwrap();

    let check = result.split_off(result.len() - 4);

    let mut hasher = Sha256::new();
    hasher.update(&result);
    let digest1 = hasher.finalize();

    let mut hasher = Sha256::new();
    hasher.update(&digest1);
    let digest = hasher.finalize();

    if check != &digest[..4] {
        panic!("b58decode_check error")
    } else {
        result
    }
}
fn eip55_checksum(hex_address: &mut [u8]) {
    let mut hasher = Keccak256::new();
    hasher.update(&hex_address);
    let hashed_address = encode(hasher.finalize());

    hex_address
        .iter_mut()
        .zip(hashed_address.as_bytes().iter())
        .for_each(|(c, &h)| match *c {
            b'a'..=b'f' if h > b'7' => {
                *c = c.to_ascii_uppercase();
            }
            _ => (),
        });
}

// Address: TRX to ETH batch
pub fn trx2eth_batch(trx_addrs: Vec<String>) -> Vec<String> {
    let eth_addrs = trx_addrs.into_iter().map(|trx_add|{
        trx2eth(&trx_add)
    }).collect::<Vec<_>>();
    eth_addrs
}

#[test]
fn test() {
    let eth_addr = String::from("0xa614f803B6FD780986A42c78Ec9c7f77e6DeD13C");
    let trx_addr = eth2trx(&eth_addr);
    println!("trx_addr: {}", trx_addr); // TR7NHqjeKQxGTCi8q8ZY4pL8otSzgjLj6t

    let trx_addr = String::from("TR7NHqjeKQxGTCi8q8ZY4pL8otSzgjLj6t");
    let eth_addr = trx2eth(&trx_addr);
    println!("eth_addr: {}", eth_addr); // 0xa614f803B6FD780986A42c78Ec9c7f77e6DeD13C

    let trx_addrs = vec![
        "TR7NHqjeKQxGTCi8q8ZY4pL8otSzgjLj6t".to_string(),
        "TR7NHqjeKQxGTCi8q8ZY4pL8otSzgjLj6t".to_string(),
    ];
    let eth_addrs = trx2eth_batch(trx_addrs);
    println!("eth_addrs: {:#?}", eth_addrs);
}
