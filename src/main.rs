#![allow(dead_code)]

mod cell;
mod grid;
mod algorithms;
mod app;

use crate::grid::Grid;
use crate::algorithms::binarysearch;
use app::Cli;
use clap::Parser;

fn main() {
    let args = Cli::parse();
    let mut maze = Grid::new(args.rows, args.cols);
    binarysearch::generate(&mut maze);
    println!("{}", maze);
}
