mod caesar;
mod vigenere;

trait Cipher {
    fn encrypt(text: &str, keyword: Option<&str>, key: Option<i32>) -> String;
}
