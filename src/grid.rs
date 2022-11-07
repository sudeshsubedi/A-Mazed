use std::fmt::Display;

use colored::Colorize;

use crate::cell::{Cell, Neighbor};

pub struct Grid {
    pub rows: u8,
    pub cols: u8,
    pub grid: Vec<Vec<Cell>>,
    pub pretty: bool,
}

impl Grid {
    pub fn new(rows: u8, cols: u8) -> Self {
        let mut grid: Vec<Vec<Cell>> = Vec::with_capacity(rows as usize *cols as usize);
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
            pretty: false,
        }
    }
}

impl Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.pretty {
            let mut output = "+".on_red().to_string();
            output.push_str("---+".repeat(self.cols as usize).on_red().to_string().as_str());
            output.push_str("\n");

            for row in self.grid.iter() {
                let mut top = "|".on_red().to_string();
                let mut bottom = "+".on_red().to_string();

                for cell in row.iter() {
                    let body = "   ".on_green();
                    let east_boundary  = match cell.links.contains(&Neighbor::East) {
                        true => " ".on_green(),
                        false => "|".on_red(),
                    };
                    top.push_str(body.to_string().as_str());
                    top.push_str(east_boundary.to_string().as_str());

                    let south_boundary = match cell.links.contains(&Neighbor::South) {
                        true =>  "   ".on_green(),
                        false => "---".on_red(),
                    };
                    bottom.push_str(south_boundary.to_string().as_str());
                    bottom.push_str("+".on_red().to_string().as_str()); // corner
                }
                output.push_str(top.as_str());
                output.push_str("\n");
                output.push_str(bottom.as_str());
                output.push_str("\n");
            }
            writeln!(f, "{}", output)
        } else {
            let mut output = "+".to_string();
            output.push_str("---+".repeat(self.cols as usize).as_str());
            output.push_str("\n");

            for row in self.grid.iter() {
                let mut top = "|".to_string();
                let mut bottom = "+".to_string();

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
}