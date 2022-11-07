#[derive(PartialEq, Eq)]
pub enum Neighbor {
    East, West, North, South,
}

pub struct Cell {
    pub row: u8,
    pub col: u8,
    pub links: Vec<Neighbor>,
}

impl Cell {
    pub fn new(row: u8, col: u8) -> Self {
        Cell {
            row, col,
            links: Vec::with_capacity(4),
        }
    }
}