pub mod rod_cutting;
use rod_cutting::rod_cutting;

fn main() {
    let prices = vec![1, 5, 8, 9, 10, 17, 17, 20];
        let length = 7;

    let result = rod_cutting(&prices, length);
    println!("{}", result);

}
