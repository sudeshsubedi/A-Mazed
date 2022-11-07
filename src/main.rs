#![allow(dead_code)]

mod cell;
mod grid;
mod algorithms;
mod app;

use crate::app::AlgorithmChoice;
use crate::grid::Grid;
use crate::algorithms::{ binarytree, sidewinder };
use app::Cli;
use clap::Parser;

fn main() {
    let args = Cli::parse();
    let mut maze = Grid::new(args.rows, args.cols);
    match args.algorithm_choice {
        AlgorithmChoice::BinaryTree => binarytree::generate(&mut maze),
        AlgorithmChoice::SideWinder => sidewinder::generate(&mut maze),
    }
    maze.pretty = args.pretty;
    println!("{}", maze);
}
