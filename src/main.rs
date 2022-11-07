#![allow(dead_code)]

mod cell;
mod grid;
mod algorithms;
mod app;

use crate::grid::Grid;
use crate::algorithms::{ binarysearch, sidewinder };
use app::Cli;
use clap::Parser;

fn main() {
    let args = Cli::parse();
    let mut maze = Grid::new(args.rows, args.cols);
    sidewinder::generate(&mut maze);
    maze.pretty = args.pretty;
    println!("{}", maze);
}
