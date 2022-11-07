use std::fmt::Display;

use crate::cell::{Cell, Neighbor};

pub struct Grid {
    pub rows: u8,
    pub cols: u8,
    pub grid: Vec<Vec<Cell>>,
}

impl Grid {
    pub fn new(rows: u8, cols: u8) -> Self {
        let mut grid: Vec<Vec<Cell>> = Vec::with_capacity((rows*cols) as usize);
        for i in 0..rows {
            let mut row: Vec<Cell> = Vec::with_capacity(cols as usize);
            for j in 0..cols {
                row.push(Cell::new(i as u8, j as u8));
            }
            grid.push(row);
        }
        Grid {
            rows,
            cols,
            grid,
        }
    }
}

impl Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut output = String::from("+");
        output.push_str("---+".repeat(self.cols as usize).as_str());
        output.push_str("\n");

        for row in self.grid.iter() {
            let mut top = String::from("|");
            let mut bottom = String::from("+");

            for cell in row.iter() {
                let body = "   ";
                let east_boundary  = match cell.links.contains(&Neighbor::East) {
                    true => " ",
                    false => "|",
                };
                top.push_str(body);
                top.push_str(east_boundary);

                let south_boundary = match cell.links.contains(&Neighbor::South) {
                    true =>  "   ",
                    false => "---",
                };
                bottom.push_str(south_boundary);
                bottom.push_str("+"); // corner
            }
            output.push_str(top.as_str());
            output.push_str("\n");
            output.push_str(bottom.as_str());
            output.push_str("\n");
        }

        writeln!(f, "{}", output)
    }
}