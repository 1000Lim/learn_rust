fn encrypt_func(text: &str, shift: i16) -> String {
    let a = 'A' as i16; 
    let is_az = |c| 'A' <= c && c <= 'Z';
    let conv = |c| (c as i16 - a + shift + 26) % 26 + a;
    text.chars().map(|c| if is_az(c) {
        (conv(c as i16) as u8) as char
    } else {
        c
    }).collect()

}

fn main(){
    let encrypted = encrypt_func("HELLO", 3);
    let dec = encrypt_func(&encrypted, -3);
    println!("{} => {}", encrypted,  dec);

}