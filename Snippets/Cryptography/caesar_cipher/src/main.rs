fn caesar_cipher(text: &str, shift: i32) -> String {
    text.chars()
        .map(|c| {
            if c.is_alphabetic() {
                let base = if c.is_lowercase() { b'a' } else { b'A' };
                let c_u8 = c as u8;
                let shifted = (c_u8 - base + shift as u8) % 26 + base;
                shifted as char
            } else {
                c
            }
        })
        .collect()
}

fn main() {
    let text = "Hello, World!";
    let shift = 3;
    let encrypted = caesar_cipher(text, shift);
    let decrypted = caesar_cipher(&encrypted, -shift);
    println!("Encrypted: {}", encrypted);
    println!("Decrypted: {}", decrypted);
}
