// Result for running this binary using `cargo run -p example`
// Possible plaintext: EUAGXKIGVGHRKULGSGFOTMZNOTMY
// Vigenere Key: NOW, Shift Key: 1
// Possible plaintext: DTZFWJHFUFGQJTKFRFENSLYMNSLX
// Vigenere Key: NOW, Shift Key: 2
// Possible plaintext: CSYEVIGETEFPISJEQEDMRKXLMRKW
// Vigenere Key: NOW, Shift Key: 3
// Possible plaintext: BRXDUHFDSDEOHRIDPDCLQJWKLQJV
// Vigenere Key: NOW, Shift Key: 4
// Possible plaintext: AQWCTGECRCDNGQHCOCBKPIVJKPIU
// Vigenere Key: NOW, Shift Key: 5
// Possible plaintext: ZPVBSFDBQBCMFPGBNBAJOHUIJOHT
// Vigenere Key: NOW, Shift Key: 6
// Possible plaintext: YOUARECAPABLEOFAMAZINGTHINGS
// Vigenere Key: NOW, Shift Key: 7
// Possible plaintext: XNTZQDBZOZAKDNEZLZYHMFSGHMFR
// Vigenere Key: NOW, Shift Key: 8
// Possible plaintext: WMSYPCAYNYZJCMDYKYXGLERFGLEQ
// Vigenere Key: NOW, Shift Key: 9
// Possible plaintext: VLRXOBZXMXYIBLCXJXWFKDQEFKDP
// Vigenere Key: NOW, Shift Key: 10
// Possible plaintext: UKQWNAYWLWXHAKBWIWVEJCPDEJCO
// Vigenere Key: NOW, Shift Key: 11
// Possible plaintext: TJPVMZXVKVWGZJAVHVUDIBOCDIBN
// Vigenere Key: NOW, Shift Key: 12
// Possible plaintext: SIOULYWUJUVFYIZUGUTCHANBCHAM
// Vigenere Key: NOW, Shift Key: 13
// Possible plaintext: RHNTKXVTITUEXHYTFTSBGZMABGZL
// Vigenere Key: NOW, Shift Key: 14
// Possible plaintext: QGMSJWUSHSTDWGXSESRAFYLZAFYK
// Vigenere Key: NOW, Shift Key: 15
// Possible plaintext: PFLRIVTRGRSCVFWRDRQZEXKYZEXJ
// Vigenere Key: NOW, Shift Key: 16
// Possible plaintext: OEKQHUSQFQRBUEVQCQPYDWJXYDWI
// Vigenere Key: NOW, Shift Key: 17
// Possible plaintext: NDJPGTRPEPQATDUPBPOXCVIWXCVH
// Vigenere Key: NOW, Shift Key: 18
// Possible plaintext: MCIOFSQODOPZSCTOAONWBUHVWBUG
// Vigenere Key: NOW, Shift Key: 19
// Possible plaintext: LBHNERPNCNOYRBSNZNMVATGUVATF
// Vigenere Key: NOW, Shift Key: 20
// Possible plaintext: KAGMDQOMBMNXQARMYMLUZSFTUZSE
// Vigenere Key: NOW, Shift Key: 21
// Possible plaintext: JZFLCPNLALMWPZQLXLKTYRESTYRD
// Vigenere Key: NOW, Shift Key: 22
// Possible plaintext: IYEKBOMKZKLVOYPKWKJSXQDRSXQC
// Vigenere Key: NOW, Shift Key: 23
// Possible plaintext: HXDJANLJYJKUNXOJVJIRWPCQRWPB
// Vigenere Key: NOW, Shift Key: 24
// Possible plaintext: GWCIZMKIXIJTMWNIUIHQVOBPQVOA
// Vigenere Key: NOW, Shift Key: 25

fn vigenere_decrypt(ciphertext: &str, key: &str) -> String {
    let mut decrypted_text = String::new();

    for (i, c) in ciphertext.chars().enumerate() {
        let c_val = c as u8 - 'A' as u8;
        let k_val = key.chars().nth(i % key.len()).unwrap() as u8 - 'A' as u8;
        decrypted_text
            .push((((c_val as isize - k_val as isize).rem_euclid(26)) as u8 + 'A' as u8) as char);
    }

    decrypted_text
}

fn shift_decrypt(text: &str, shift: u8) -> String {
    let mut decrypted_text = String::new();

    for c in text.chars() {
        decrypted_text.push((((c as u8 - 'A' as u8 + 26 - shift) % 26) + 'A' as u8) as char);
    }

    decrypted_text
}

fn main() {
    let ciphertext = "SJXUMHWVSUWOYJIUHDTDQAOKCIJM";

    for second_char in 'A'..='Z' {
        for third_char in 'A'..='Z' {
            let key = format!("N{}{}", second_char, third_char);
            let vigenere_decrypted = vigenere_decrypt(ciphertext, &key);

            for shift in 1..=25 {
                let final_decrypted = shift_decrypt(&vigenere_decrypted, shift);
                if final_decrypted.chars().nth(3).unwrap()
                    == final_decrypted.chars().nth(7).unwrap()
                    && final_decrypted.chars().nth(3).unwrap()
                        == final_decrypted.chars().nth(9).unwrap()
                    && final_decrypted.chars().nth(3).unwrap()
                        == final_decrypted.chars().nth(15).unwrap()
                    && final_decrypted.chars().nth(3).unwrap()
                        == final_decrypted.chars().nth(17).unwrap()
                {
                    println!("Possible plaintext: {}", final_decrypted);
                    println!("Vigenere Key: {}, Shift Key: {}", key, shift);
                }
            }
        }
    }
}
