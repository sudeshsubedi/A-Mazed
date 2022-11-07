#![allow(dead_code)]

mod cell;
mod grid;
mod algorithms;

use crate::grid::Grid;
use crate::algorithms::binarysearch;

fn main() {
    let mut maze = Grid::new(10, 10);
    binarysearch::generate(&mut maze);
    println!("{}", maze);
}
