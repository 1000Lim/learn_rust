fn main() {
    let s1 = String::from("Sentence from the outer block.");
    {
        let s2 = s1;
        println!("s2: {}", s2);
    }
    // This line will cause an error because s1 is moved to s2
    println!("s1: {}", s1);
}
