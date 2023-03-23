extern crate cfg_if;
extern crate wasm_bindgen;

mod utils;

use cfg_if::cfg_if;
use wasm_bindgen::prelude::*;

cfg_if! {
    if #[cfg(feature = "wee_alloc")] {
        extern crate wee_alloc;
        #[global_allocator]
        static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
    }
}

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cell {
    PlayerOne,
    PlayerTwo,
    Empty,
}

#[wasm_bindgen]
#[derive(Debug)]
pub struct FourInARow {
    pub width: usize,
    pub height: usize,
    cells: Vec<Cell>,
}

#[wasm_bindgen]
impl FourInARow {
    fn get_position(&self, column: usize, row: usize) -> usize {
        (row * self.width + column) as usize
    }

    /// Index that the dial will fall to
    pub fn get_index(&self, column: usize) -> Option<usize> {
        // 0 0 0 0
        // 1 0 0 0
        // 1 1 0 1

        // column 0 would mean index 0
        // column 2 would mean index 10
        let size = self.cells.len();
        for (index, cell) in self.cells.iter().step_by(self.width).enumerate() {
            match *cell {
                Cell::Empty => {},
                _ if index > size => {
                    // Not empty, so previous one is index
                    return Some(index - self.width)
                },
                _ => return None,
            }
        }

        // All are empty so last position is the index
        Some(size - (self.width - column))
    }

    pub fn is_game_over(&self) -> bool {
        let mut winner: Vec<(usize, Cell)> = vec![];
        // Horizontal
        for (row, cells) in self.cells.windows(self.width).enumerate() {
            winner.clear();
            for (column, cell) in cells.iter().enumerate() {
                let pos = self.get_position(column, row);
                if let Some((_, c)) = winner.first() {
                    if cell == c {
                        winner.push((pos, *cell));
                    }
                } else {
                    winner.clear();
                    winner.push((pos, *cell));
                }
            }
        }

        winner.len() == 4
    }

    pub fn cells(&self) -> *const Cell {
        self.cells.as_ptr()
    }

    pub fn new() -> Self {
        utils::set_panic_hook();
        Self { width: 7, height: 6, cells: vec![Cell::Empty;7 * 6] }
    }
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello, {name}!"));
}
