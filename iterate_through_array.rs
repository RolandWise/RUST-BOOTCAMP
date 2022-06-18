extern crate ndarray;

use ndarray::arr3;

fn main() {
    let a3 = arr3(&[[[1, 2], [3, 4]], [[5, 6], [7, 8]], [[9, 0], [1, 2]]]);
    for row in a3.genrows() {
        // Each row is a 1D array
        println!("row is {:?}", row);
    }
    for a2 in a3.outer_iter() {
        println!("2D array is {:?}", a2);
    }
}
