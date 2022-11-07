use rand::Rng;

use crate::{grid::Grid, cell::Neighbor};

pub fn generate(grid: &mut Grid) {
    let mut rng = rand::thread_rng();
    for row in grid.grid.iter_mut() {
        for cell in row.iter_mut() {
            let mut neighbors = Vec::with_capacity(2);
            if cell.col < grid.cols - 1 {
                neighbors.push(Neighbor::East);
            }

            if cell.row < grid.rows - 1 {
                neighbors.push(Neighbor::South);
            }
            if neighbors.len() != 0 {
                cell.links.push(neighbors.swap_remove(rng.gen::<usize>() % neighbors.len()));
            }
        }
    }
}