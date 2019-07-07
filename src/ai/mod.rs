use rand::seq::SliceRandom;
use super::Cell;
use rand::Rng;

pub struct Ai;


impl Ai {
    pub fn make_move(cells: &Vec<Cell>) -> Result<usize, &'static str> {
        // not-so-smart AI
        let mut empty_cells_indexes = vec![];
        for (ind, cell) in cells.iter().enumerate() {
            if cell == &Cell::Empty {
                empty_cells_indexes.push(ind);
            }
        }
        let mut rng = rand::thread_rng();
        match empty_cells_indexes.len() {
            0 => Err("no empty cells"),
            _ => Ok(empty_cells_indexes[rng.gen_range(0, empty_cells_indexes.len())]),
        }
        // empty_cells_indexes[rng.gen_range(0, empty_cells_indexes.len())]
    }
}

#[cfg(test)]
mod tests;