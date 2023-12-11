use std::{
    collections::HashSet,
    fs::{self, File},
    io::Write,
};

#[derive(Eq, Hash, PartialEq)]
struct CoefficientSet {
    coefficients: Vec<u8>,
}

fn main() {
    //? Read the ciphertext from a file.
    let ciphertext = fs::read_to_string("ciphertext.txt").expect("Failed to read file");

    //? Convert the ciphertext into a vector of bits.
    let mut ciphertext_bits = vec![];
    for c in ciphertext.chars() {
        if c.is_whitespace() {
            continue; // Skip whitespace characters.
        }
        ciphertext_bits.push(c.to_digit(2).unwrap() as u8);
    }

    //? Generate the result file.
    let mut file_result = File::create("result.txt").expect("Unable to create file");
    //? Keep track of valid coefficient sets for later analysis.
    let mut valid_coefficient_sets = HashSet::new();

    //? Iterate over possible values of n (5 to 7).
    for n in 5..=7 {
        //? Generate all combinations of initial keys and coefficients for the given n. ( brute force )
        let combinations = generate_initial_key_and_coefficients(n);

        for (initial_key, coefficients) in combinations {
            //? Validate each coefficient set.
            if is_valid_coefficients(&coefficients, n) {
                //? Store valid coefficient sets for later analysis.
                valid_coefficient_sets.insert(CoefficientSet {
                    coefficients: coefficients.clone(),
                });

                //? Generate the key string for the current combination.
                let key_string = generate_key_string(&initial_key, &coefficients, 344);

                //? Decrypt the ciphertext using the generated key string.
                let plaintext_bits = decrypt(&ciphertext_bits, &key_string);

                //? Convert the decrypted bits back into a string.
                let plaintext = bits_to_string(&plaintext_bits);

                //? Check if the plaintext contains only uppercase English letters.
                if plaintext.chars().all(|c| c.is_ascii_uppercase()) {
                    //? Write results to the file if plaintext is valid.
                    writeln!(file_result, "n: {}", n).expect("Unable to write to file");
                    writeln!(file_result, "Initial key: {:?}", initial_key)
                        .expect("Unable to write to file");
                    writeln!(file_result, "Coefficients: {:?}", coefficients)
                        .expect("Unable to write to file");
                    writeln!(file_result, "Decrypted plaintext: {}", plaintext)
                        .expect("Unable to write to file");
                }
            }
        }
    }

    //? Analyze and count the symmetric coefficient sets.
    let symmetric_count = valid_coefficient_sets
        .iter()
        .filter(|set| is_symmetric(set, &valid_coefficient_sets))
        .count();

    //? Write the count of symmetric coefficient sets to the file.
    writeln!(
        file_result,
        "Number of symmetric coefficient sets: {}",
        symmetric_count
    )
    .expect("Unable to write to file");
}

//? Generates all combinations of initial keys and coefficients for a given n.
fn generate_initial_key_and_coefficients(n: usize) -> Vec<(Vec<u8>, Vec<u8>)> {
    let mut combinations = Vec::new();

    //? Calculate the number of possible combinations, which is 2^n.
    let max_combinations = 1 << n;

    //? Iterate through all possible combinations for initial keys and coefficients.
    for key_combination in 0..max_combinations {
        let initial_key = (0..n)
            .map(|bit_index| (key_combination >> bit_index) & 1)
            .collect::<Vec<u8>>();

        for coeff_combination in 0..max_combinations {
            let coefficients = (0..n)
                .map(|bit_index| (coeff_combination >> bit_index) & 1)
                .collect::<Vec<u8>>();

            combinations.push((initial_key.clone(), coefficients));
        }
    }

    combinations
}

//? Checks if a given coefficient set is symmetric.
fn is_symmetric(set: &CoefficientSet, all_sets: &HashSet<CoefficientSet>) -> bool {
    //? Create a reversed version of the coefficients, excluding the first element.
    let mut reversed = set.coefficients[1..].to_vec();
    reversed.reverse();
    reversed.insert(0, set.coefficients[0]); //? Add the first element back to the start.

    //? Check if the reversed set exists in the collection of all valid sets.
    all_sets.contains(&CoefficientSet {
        coefficients: reversed,
    })
}

fn is_valid_coefficients(coefficients: &[u8], n: usize) -> bool {
    let mut seen_states = std::collections::HashSet::new();
    let mut current_state = 1; //? Start with a non-zero state

    for _ in 0..(1 << n) {
        //? Iterate up to 2^n to find the period
        if seen_states.contains(&current_state) {
            break; //? Cycle detected
        }
        seen_states.insert(current_state);

        let mut next_state = 0;
        for (i, &coeff) in coefficients.iter().enumerate() {
            if coeff == 1 {
                next_state ^= (current_state >> i) & 1;
            }
        }

        current_state = (current_state >> 1) | (next_state << (n - 1));
    }

    seen_states.len() == (1 << n) - 1
}

fn generate_key_string(initial_key: &[u8], coefficients: &[u8], length: usize) -> Vec<u8> {
    //? Use the given formula to generate a key string of the specified length
    let mut key_string = Vec::with_capacity(length);
    //? Start with the initial key values
    for &bit in initial_key {
        key_string.push(bit);
    }
    //? Generate the rest of the key string
    for i in 0..length - initial_key.len() {
        let next_bit = coefficients
            .iter()
            .enumerate()
            .map(|(j, &a)| a * key_string[i + j])
            .fold(0, |acc, x| acc ^ x);
        key_string.push(next_bit);
    }
    key_string
}

fn decrypt(ciphertext: &[u8], key_string: &[u8]) -> Vec<u8> {
    //? XOR operation between ciphertext and key string
    ciphertext
        .iter()
        .zip(key_string.iter())
        .map(|(&c, &k)| c ^ k)
        .collect()
}

fn bits_to_string(bits: &[u8]) -> String {
    //? Convert a sequence of bits to a string
    bits.chunks(8)
        .map(|chunk| {
            let byte = chunk.iter().fold(0, |acc, &b| (acc << 1) | b);
            byte as char
        })
        .collect()
}
