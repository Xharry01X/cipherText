
use std::collections::HashMap;


fn generate_key() -> HashMap<char, char> {
    use rand::seq::SliceRandom;
    use rand::thread_rng;
    
    let mut rng = thread_rng();
    let  alphabet: Vec<char> = ('a'..='z').collect();
    let mut shuffled = alphabet.clone();
    shuffled.shuffle(&mut rng);
    
    alphabet.iter().zip(shuffled.iter()).map(|(&k, &v)| (k, v)).collect()
}


fn encrypt(text: &str, key: &HashMap<char, char>) -> String {
    text.chars()
        .map(|c| {
            if c.is_ascii_alphabetic() {
                *key.get(&c.to_ascii_lowercase()).unwrap_or(&c)
            } else {
                c
            }
        })
        .collect()
}

fn decrypt(ciphertext: &str, key: &HashMap<char, char>) -> String {
    let reverse_key: HashMap<char, char> = key.iter().map(|(&k, &v) | (v,k)).collect();

    ciphertext.chars()
    .map(|c| {
        if c.is_alphabetic() {
            *reverse_key.get(&c.to_ascii_lowercase()).unwrap_or(&c)
        } else {
           c
        }
    })
    .collect()
}




fn main() {
    
 let key = generate_key();
 println!("Substitution Key: {:?}",key);

 let plaintext = "hello world";
    println!("Plaintext: {}", plaintext);
    
    let ciphertext = encrypt(plaintext, &key);
    println!("Ciphertext: {}", ciphertext);
    
    let decrypted = decrypt(&ciphertext, &key);
    println!("Decrypted: {}", decrypted);

}
