pub fn encrypt(text: &str, key: i32) -> String {
    let mut encrypted = String::new();
    for char in text.chars() {
        let character = char as u32;
        if character == 32 || character == 46 {
            encrypted.push(char);
        } else {
            let mut temp = character as i32 + key;
            if temp > 90 {
                temp -= 26;
            } else if temp < 65 {
                temp += 26;
            }
            let shifted_char = temp as u8 as char;
            encrypted.push(shifted_char);
        }
    }
    encrypted
}

fn main() {
    let encrypted = encrypt("I LOVE YOU.", 3);
    let decrypted = encrypt(&encrypted, -3);

    println!("{} => {}", encrypted, decrypted);
}
