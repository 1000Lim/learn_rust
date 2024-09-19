fn main(){
    let count_of_500_coutns: i64 = 10;
    let count_of_100_coutns: i64 = 3;
    let count_of_50_coutns : i64= 10;

    let target_price: i64 = 3950;

    for count_of_500 in 0..=count_of_500_coutns {
        for count_of_100 in 0..=count_of_100_coutns {
            for count_of_50 in 0..=count_of_50_coutns {
                let totoal_price = count_of_500 * 500 + count_of_100 * 100 + count_of_50 * 50;
                if target_price == totoal_price {
                    println!("500: {}, 100: {}, 50: {}", count_of_500, count_of_100, count_of_50);
                }
            }
        }
    }
}