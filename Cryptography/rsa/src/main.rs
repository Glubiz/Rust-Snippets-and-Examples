use rsa::{PublicKey, RsaPrivateKey, PaddingScheme};
use rand::rngs::OsRng;

fn main() {
    // Generate a pair of RSA keys
    let mut rng = OsRng;
    let bits = 2048;
    let private_key = RsaPrivateKey::new(&mut rng, bits).expect("Failed to generate a key");
    let public_key = private_key.to_public_key();

    // Encrypt a message using the public key
    let message = b"Hello, World!";
    let padding = PaddingScheme::new_pkcs1v15_encrypt();
    let encrypted = public_key.encrypt(&mut rng, padding, &message[..]).expect("Failed to encrypt");

    // Decrypt the message using the private key
    let padding = PaddingScheme::new_pkcs1v15_decrypt();
    let decrypted = private_key.decrypt(padding, &encrypted).expect("Failed to decrypt");

    println!("Original message: {:?}", message);
    println!("Encrypted message: {:?}", encrypted);
    println!("Decrypted message: {:?}", decrypted);
}
