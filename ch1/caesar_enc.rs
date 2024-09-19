fn encrypt_caesar(text: &str, shift: i16) -> String{
    let code_a = 'A' as i16;
    let code_z = 'Z' as i16;

    let mut result = String::new();
    for c in text.chars(){
        let mut code = c as i16; // convert char to i16
        if code_a <= code && code <= code_z{
            code += shift + 26;
            code = (code - code_a) % 26 + code_a;
        }
        result.push((code as u8) as char);
    }

    result
}


fn main(){
    println!("This function is used to encrypt a message using the Caesar cipher.");
    let enc = encrypt_caesar("HELLO", 3);
    println!("Encrypted message: {}", enc);
    let dec = encrypt_caesar(&enc, -3);
    println!("Decrypted message: {}", dec);
}