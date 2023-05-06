use hmac::{Hmac, Mac, NewMac};
use sha2::Sha256;

type HmacSha256 = Hmac<Sha256>;

fn main() {
    let key = b"secret_key";
    let message = b"Hello, World!";

    let mut mac = HmacSha256::new_from_slice(key).expect("Invalid key length");
    mac.update(message);

    let result = mac.finalize();
    let code_bytes = result.into_bytes();

    println!("HMAC: {:?}", code_bytes);
}
