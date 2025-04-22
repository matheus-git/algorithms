use rand::Rng;

mod heap;
mod quicksort;
use quicksort::quicksort;                         
fn main() {
    let mut rng = rand::rng();
    let mut a: Vec<usize> = (0..100).map(|_| rng.random_range(0..1000)).collect();
    let len = a.len();
    quicksort(&mut a, 0, (len-1) as isize);
    println!("{:?}", a);
}

