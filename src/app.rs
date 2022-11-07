use clap::{ Parser, ValueEnum };

/// A simple maze generation program exploring different algorithms
#[derive(Parser, Debug)]
#[command(author, version, about, long_about=None)]
pub struct Cli {
    /// No of rows in grid
    #[arg(short, long, default_value_t=15)]
    pub rows: u8,

    /// No of columns in grid
    #[arg(short, long, default_value_t=15)]
    pub cols: u8,

    /// option to enable pretty print
    #[arg(short, long, default_value_t=false)]
    pub pretty: bool,

    #[arg(value_enum, short, long, default_value_t=AlgorithmChoice::BinaryTree)]
    pub algorithm_choice: AlgorithmChoice,
}

#[derive(Debug, Clone, ValueEnum)]
pub enum AlgorithmChoice {
    BinaryTree,
    SideWinder,
}