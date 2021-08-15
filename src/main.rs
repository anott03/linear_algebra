mod vector;
mod fraction;

use crate::vector::vector::*;

fn main() {
    let v = vec![3, 1, 2];
    let basis = vec![vec![1, 1, 1], vec![1, -1, 0]];

    match projection_onto_basis(&v, &basis) {
        Ok(proj) => println!("final result: {:?}", proj),
        Err(err) => println!("Error: {}", err),
    }

    match vector_subtraction(&vec![1, 2, 3], &vec![0, 1, 2]) {
        Ok(vec) => println!("difference: {:?}", vec),
        Err(err) => println!("Error: {}", err),
    }
}
