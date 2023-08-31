use sha2::{Sha256, Digest};
use bip39::Language;
use num_bigint::BigUint;
use num_traits::ToBytes;
use num_traits::Num;

pub fn generate_last_word(seed: Vec<String>) -> Vec<String> {
    let word_list: Vec<&str> = Language::English.word_list().to_vec();

    let seed_phrase_index: Vec<usize> = seed
    .iter()
    .map(|word| {
        word_list.iter().position(|&x| x == *word).unwrap()
    })
    .collect();

    let seed_phrase_binary: Vec<String> = seed_phrase_index.iter().map(|&number| {
        format!("{:011b}", number)
    }).collect();

    let num_missing_bits = 11 - ((seed.len() as f32) / 3.0).ceil() as usize;

    let mut missing_bits_possible: Vec<String> = Vec::new();
    for x in 0..(2u32.pow(num_missing_bits as u32)) {
        missing_bits_possible.push(format!("{:0width$b}", x, width = num_missing_bits));
    }

    let mut entropy_possible: Vec<String> = Vec::new();
    for bits in &missing_bits_possible {
        let combined_entropy = format!("{}{}", seed_phrase_binary[..seed_phrase_binary.len()].join(""), bits);
        entropy_possible.push(combined_entropy);
    }

    let mut checksum: Vec<String> = Vec::new();
    for entropy in &entropy_possible {
        let entropy_int = BigUint::from_str_radix(entropy, 2).unwrap();
        let entropy_bytes = entropy_int.to_be_bytes();
        let hash_result = sha256_hash(&entropy_bytes);
        let hash_byte = hash_result[0];
        let hash_bits = format!("{:08b}", hash_byte);
        let trimmed_checksum = hash_bits[..11 - num_missing_bits].to_string();
        checksum.push(trimmed_checksum);
    }

    let last_word_bits: Vec<String> = missing_bits_possible
    .iter()
    .zip(checksum.iter())
    .map(|(i, j)| format!("{}{}", i, j))
    .collect();

    let last_word: Vec<String> = last_word_bits.iter()
        .map(|bits| word_list[u32::from_str_radix(bits, 2).unwrap() as usize])
        .map(|s| s.to_string()).collect();
    return last_word
}

fn sha256_hash(data: &[u8]) -> Vec<u8> {
    let mut hasher = Sha256::new();
    hasher.update(data);
    hasher.finalize().to_vec()
}