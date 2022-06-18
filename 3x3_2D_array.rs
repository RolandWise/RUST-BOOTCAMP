extern crate ndarray;

use ndarray::arr2;

fn main() {
    let mut a2 = arr2(&[[1, 2, 3], [4, 5, 6], [7, 8, 9]]);
    a2[[2, 1]] = 10;
    println!("The 2D array is {:?}", a2);
}
