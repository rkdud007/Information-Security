use crate::Cipher;

pub struct AutoKey {}
impl Cipher for AutoKey {
    fn encrypt(text: &str, keyword: Option<&str>, _key: Option<i32>) -> String {
        // stretch keyword to plain text size
        let mut encrypted = String::new();

        let keywords = format!("{}{}", keyword.unwrap(), text).to_uppercase();

        for (i, char) in text.to_uppercase().chars().enumerate() {
            // encrypt char
            let space = char as u8 - 'A' as u8;
            let mut encrypted_index = keywords.chars().nth(i).unwrap() as u8 + space;
            if encrypted_index > 'Z' as u8 {
                encrypted_index -= 26;
            }
            encrypted.push(encrypted_index as char);
        }
        encrypted
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn extended_keyword(text: &str, keyword: &str) -> String {
        let keywords = keyword
            .to_uppercase()
            .chars()
            .cycle()
            .take(text.len())
            .collect::<String>();
        keywords
    }

    #[test]
    fn test_autokey_encrypt() {
        let encrypted = AutoKey::encrypt("wearediscoveredsaveyourself", Some("deceptive"), None);
        assert_eq!(encrypted, "ZICVTWQNGKZEIIGASXSTSLVVWLA");
    }

    // #[test]
    // fn test_caesar_decrypt() {
    //     //*Hint: 4th, 8th, 10th, 16th, and 18th letters of plaintext are the same letters!
    //     // The first letter of the vigenere cipher key is "N" and length is 3
    //     let cipher_text = "SJXUMHWVSUWOYJIUHDTDQAOKCIJM";
    //     let keywords = extended_keyword(cipher_text, "NAP");
    //     assert_eq!(keywords, "I LOVE YOU.");

    //     // let decrypted = Vigenere::encrypt("L ORYH BRX.", None, Some(-3));
    //     // assert_eq!(decrypted, "I LOVE YOU.");
    // }
}
