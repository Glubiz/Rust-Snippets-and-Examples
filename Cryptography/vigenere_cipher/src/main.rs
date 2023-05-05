fn vigenere_cipher(text: &str, keyword: &str, encrypt: bool) -> String {
    let keyword = keyword.repeat((text.len() + keyword.len() - 1) / keyword.len());
    text.chars()
        .zip(keyword.chars())
        .map(|(c, k)| {
            if c.is_alphabetic() {
                let base = if c.is_lowercase() { b'a' } else { b'A' };
                let shift = if encrypt { k as i32 } else { -k as i32 };
                let c_u8 = c as u8;
                let shifted = (c_u8 - base).wrapping_add(shift as u8) % 26 + base;
                shifted as char
            } else {
                c
            }
        })
        .collect()
}

fn main() {
    let text = "Hello, World!";
    let keyword = "key";
    let encrypted = vigenere_cipher(text, keyword, true);
    let decrypted = vigenere_cipher(&encrypted, keyword, false);
    println!("Encrypted: {}", encrypted);
    println!("Decrypted: {}", decrypted);
}
