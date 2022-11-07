use rand::Rng;

use crate::{
    grid::Grid,
    cell::{ Cell, Neighbor }
};

pub fn generate(grid: &mut Grid) {
    let mut rng = rand::thread_rng();
    grid.grid.reverse();
    for row in grid.grid.iter_mut() {
        let mut run: Vec<&mut Cell> = Vec::with_capacity(grid.cols as usize);
        for cell in row.iter_mut() {
            let at_east_border = cell.col == (grid.cols - 1);
            let at_south_border = cell.row == (grid.rows-1);
            
            let should_close_out = at_east_border || (!at_south_border && rng.gen_bool(0.5));
            if !should_close_out {
                cell.links.push(Neighbor::East);
                run.push(cell);
            } else if !at_south_border{
                run.push(cell);
                let n = run.len();
                run[rng.gen::<usize>() % n].links.push(Neighbor::South);
                run.clear();
            }
        } 
    }
    grid.grid.reverse();
}