use sgx_types::sgx_sha256_hash_t;
use sgx_crypto_helper::rsa3072::Rsa3072PubKey;

use utils;

#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
	pub account: String,
	pub amount: u32,
	pub sha256: sgx_sha256_hash_t
}

pub fn from_slice(bytes: &[u8]) -> [u8; 32] {
	let mut array = [0; 32];
	let bytes = &bytes[..array.len()]; // panics if not enough data
	array.copy_from_slice(bytes);
	array
}

pub fn get_encrypted_msg(rsa_pubkey: Rsa3072PubKey) -> Vec<u8> {
	let hash: Vec<String> = utils::get_wasm_hash();
	println!("Wasm Hash: {:?}", hash[0]);
	println!("Wasm Binary : {:?}", hash[1]);

	let sha = hex::decode(hash[0].clone()).unwrap();
	let sha256: sgx_sha256_hash_t = from_slice(&sha);

	let account: String = "Alice".to_string();
	let amount = 42;

	let message = Message { account, amount, sha256 };
	let plaintext = serde_json::to_vec(&message).unwrap();
	let mut payload_encrypted: Vec<u8> = Vec::new();

	rsa_pubkey.encrypt_buffer(&plaintext, &mut payload_encrypted).unwrap();
	payload_encrypted
}
