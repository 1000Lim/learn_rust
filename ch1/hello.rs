fn main() { 
    // Print to console
    println!("Hello, world!");

    // Variable binding
    let banana_price = 300;
    println!("banana's price is {}", banana_price);

    // Variable binding with type annotation
    let distance_to_moon: f64 = 384_400.0;
    let car_speed: f64 = 80.0;
    let train_speed:f64 = 300.0;
    let time_to_moon_by_car:f64 = distance_to_moon / car_speed;
    let time_to_moon_by_train:f64 = distance_to_moon / train_speed;
    println!("It takes {} hours to reach moon by car", time_to_moon_by_car);
    println!("It takes {} hours to reach moon by train", time_to_moon_by_train);
}