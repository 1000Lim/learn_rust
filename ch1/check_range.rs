fn main(){
    println!("int range:");
    println!("i8 = {}~{}", i8::min_value(), i8::max_value());
    println!("i16 = {}~{}", i16::min_value(), i16::max_value());
    println!("i32 = {}~{}", i32::min_value(), i32::max_value());
    println!("i64 = {}~{}", i64::min_value(), i64::max_value());
    println!("i128 = {}~{}", i128::min_value(), i128::max_value());
    println!("uint range:");
    println!("u8 = {}~{}", u8::min_value(), u8::max_value());
    println!("u16 = {}~{}", u16::min_value(), u16::max_value());
    println!("u32 = {}~{}", u32::min_value(), u32::max_value());
    println!("u64 = {}~{}", u64::min_value(), u64::max_value());
    println!("f32 = {}~{}", f32::MIN, f32::MAX);
    println!("OS dependent range:");
    println!("isize = {}~{}", isize::min_value(), isize::max_value());
    println!("usize = {}~{}", usize::min_value(), usize::max_value()); 
    println!("usize = {}", usize::BITS);
    
}