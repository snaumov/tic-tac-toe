mod utils;
mod ai;
use std::fmt;
extern crate web_sys;
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, tic-tac-toe!");
}

macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

#[wasm_bindgen]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cell {
    Empty = 0,
    Cross = 1,
    Nought = 2,
}

#[wasm_bindgen]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GameStatus {
    IsActive = 0,
    NoughtsWin = 1,
    CrossesWin = 2,
    Draw = 3
}

#[wasm_bindgen]
pub struct Game {
    crosses_to_move: bool,
    cells: Vec<Cell>,
    width: u32,
    height: u32,
    lines: Vec<Vec<u32>>,
    game_status: GameStatus,
}

impl Game {
    fn get_index(&self, row: u32, column: u32) -> usize {
        (row * self.width + column) as usize
    }

    fn has_empty_cells(&self) -> bool {
        match self.cells.iter().find(|&&cell| cell == Cell::Empty) {
            Some(_) => true,
            None => false
        }
    }

    fn is_game_active(&self) -> bool {
        match &self.game_status {
            GameStatus::IsActive => true,
            _ => false,
        }
    }

    fn check_winner(&mut self) {
        log!("{:?}", self.cells);
        for line in &self.lines {
            log!("{:?}", line);
            // log!("{}", line[0] as usize);
            log!("{}", self.cells[line[0] as usize]);
            log!("{}", self.cells[line[1] as usize]);
            // log!("{}", self.cells[line[1] as usize] == self.cells[line[2] as usize]);
            if (self.cells[line[0] as usize] == self.cells[line[1] as usize]) 
            && (self.cells[line[1] as usize] == self.cells[line[2] as usize]) {
                log!("{}", self.cells[line[0] as usize]);
                if self.cells[line[0] as usize] == Cell::Cross {
                    // crosses won
                    log!("crosses won");
                    self.game_status = GameStatus::CrossesWin;
                }

                if self.cells[line[0] as usize] == Cell::Nought {
                    // noughts won
                    self.game_status = GameStatus::NoughtsWin;
                }
            }
        }     
    }

    fn ai_make_move(&mut self) {
        match ai::Ai::make_move(&self.cells) {
            Ok(val) => {
                self.cells[val] = if self.crosses_to_move { Cell::Cross } else { Cell::Nought };
                self.crosses_to_move = !self.crosses_to_move;
            },
            Err(err) => log!("{}", err),
        };
    }

}

fn get_index(row: u32, column: u32, width: u32) -> u32 {
    row * width + column
}

#[wasm_bindgen]
impl Game {
    pub fn new() -> Game {
        utils::set_panic_hook();
        let width = 3;
        let height = 3;
        let total_cells = width * height;

        let mut cells: Vec<Cell> = (0..total_cells).map(|_i| Cell::Empty).collect();
        let mut lines = vec![]; 

        // adding horizontal lines 
        for row in 0..height {
            let mut line = vec![];
            for col in 0..width {
                line.push(get_index(row, col, width));
            }
            lines.push(line);
        }

        // adding vertical lines
        for col in 0..width {
            let mut line = vec![];
            for row in 0..height {
                line.push(get_index(row, col, width));
            }
            lines.push(line);
        }

        // adding diagonals
        lines.push(vec![0, 4, 8]);
        lines.push(vec![2, 4, 6]);

        log!("{:?}", lines);
        log!("{:?}", cells);

        // cells[0] = Cell::Cross;
        // cells[1] = Cell::Nought;

        Game {
            crosses_to_move: true,
            cells,
            width,
            height,
            lines,
            game_status: GameStatus::IsActive,
        }
    }

    pub fn make_move(&mut self, row: u32, column: u32) {
        if(self.has_empty_cells() && self.is_game_active()) {
            let idx = self.get_index(row, column);

            self.cells[idx].fill(match self.crosses_to_move {
                true => Cell::Cross,
                _ => Cell::Nought,
            });

            self.crosses_to_move = !self.crosses_to_move;
            self.check_winner();
            if (self.has_empty_cells() && self.is_game_active()) {
                self.ai_make_move();
                self.check_winner();
            }
        }
       
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn cells(&self) -> *const Cell {
        self.cells.as_ptr()
    }

    pub fn status(&self) -> GameStatus {
        self.game_status
    }
}

impl Cell {
    fn fill(&mut self, value: Cell) {
        *self = match value {
            Cell::Cross => Cell::Cross,
            Cell::Nought => Cell::Nought,
            _ => Cell::Cross,
        }
    }
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let value = match self {
            Cell::Cross => "Cross",
            Cell::Nought => "Nought",
            Cell::Empty => "Empty",
        };
        write!(f, "{}", value)?;

        Ok(())
    }
}
