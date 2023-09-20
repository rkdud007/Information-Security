use crate::Cipher;

struct Caesar {}
impl Cipher for Caesar {
    fn encrypt(text: &str, _keyword: Option<&str>, key: Option<i32>) -> String {
        let mut encrypted = String::new();
        for char in text.chars() {
            let character = char as u32;
            if character == 32 || character == 46 {
                encrypted.push(char);
            } else {
                let mut temp = character as i32 + key.unwrap() as i32;
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_caesar_encrypt() {
        let encrypted = Caesar::encrypt("I LOVE YOU.", None, Some(3));
        assert_eq!(encrypted, "L ORYH BRX.");
    }

    #[test]
    fn test_caesar_decrypt() {
        let decrypted = Caesar::encrypt("L ORYH BRX.", None, Some(-3));
        assert_eq!(decrypted, "I LOVE YOU.");
    }
}
