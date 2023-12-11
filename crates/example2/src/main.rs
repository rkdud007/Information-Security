use std::vec;

fn autokey_encrypt(text: &str, keyword: Option<&str>, _key: Option<i32>) -> String {
    // stretch keyword to plain text size
    let mut encrypted = String::new();

    let keywords = format!("{}{}", keyword.unwrap(), text).to_uppercase();
    println!("keywords: {}", keywords);
    println!("plain: {}", text);

    for (i, char) in text.to_uppercase().chars().enumerate() {
        if char as u8 == b'?' {
            encrypted.push('?');
        } else {
            let mut encrypted_index = keywords.chars().nth(i).unwrap() as u8 + char as u8;
            encrypted_index = encrypted_index % 26 + 'A' as u8;
            encrypted.push(encrypted_index as char);
        }
    }
    encrypted
}

fn give_me_nth(p_text: &str, c_text: &str) {
    for (i, char) in p_text.chars().enumerate() {
        if char as u8 != b'?' {
            println!(
                "plain: {}, cipher: {}",
                char,
                c_text.chars().nth(i).unwrap()
            );
        }
    }
}

fn main() {
    let primier_key = "????????????????????";

    //let mut plain_text = "DECRYPTINGACIPHERGENREALLYREQUIRESTHESECRETKEYHOWEVERTHISPROBLEMISSOLVABLEWITHOUTRRRRRRRTHESECRETKEY".to_string();
    let mut plain_text = "DECRYPTINGACIPHERGENREALLYREQUIRESTHESECRETKEYHOWEVERTHISPROBLEZISSOLVABLEWITHOUTREQUIRETHESECRETKEY".to_string();
    let cipher_text_13rd = "????????????????????RRAYLLEEDUIREFGHRSEPRRTXRYUOWEVRRTUVSPROOLRZVSFOLINBLEJVGUOHTXNOJVAGTHEFECERGXRY";

    give_me_nth(&plain_text, cipher_text_13rd);

    // let mut key1 = "????????????????????DECRYPTINGACIPHERGEN??????????????????????????????????????
    // ??????ISSOLVABLEWITHOU";
    // let mut plain_text = "DECRYPTINGACIPHERGEN??????????????????????????????????????
    // ??????ISSOLVABLEWITHOU????????????????????";
    // // let result1 = vec![key1.chars().nth(0).unwrap() as u8+plain_text.chars().nth(0).unwrap() as u8 %26, ]

    let mut i = 1;
    loop {
        if i == 14 {
            break;
        }
        let result = autokey_encrypt(&plain_text, Some(primier_key), None);
        println!("result {}: {}", i, result);
        plain_text = result;
        i += 1;
    }
    give_me_nth(&plain_text, cipher_text_13rd);
}
