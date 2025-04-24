mod heap;
mod quicksort;
mod counting_sort;

use counting_sort::counting_sort;                         

fn main() {
    let mut arr = vec![8,4,5,2,9,3,1];
    counting_sort(&mut arr);
    println!("{:?}", arr);
}
