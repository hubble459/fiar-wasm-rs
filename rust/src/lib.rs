extern crate cfg_if;
extern crate wasm_bindgen;

mod utils;

use cfg_if::cfg_if;
use gloo_utils::format::JsValueSerdeExt;
use serde::Serialize;
use serde_repr::Serialize_repr;
use wasm_bindgen::prelude::*;

cfg_if! {
    if #[cfg(feature = "wee_alloc")] {
        extern crate wee_alloc;
        #[global_allocator]
        static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
    }
}

macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize_repr)]
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
#[derive(Debug, Clone, Serialize)]
pub struct Winner {
    player: Cell,
    cells: Vec<usize>,
}

enum MachineState {
    Winner(Winner),
    Continue,
}

#[wasm_bindgen]
impl FourInARow {
    /// Width and height have to be bigger or equal to 4
    pub fn new() -> Self {
        utils::set_panic_hook();
        Self {
            width: 7,
            height: 6,
            cells: vec![Cell::Empty; 7 * 6],
        }
    }

    fn get_position(&self, column: usize, row: usize) -> usize {
        (row * self.width + column) as usize
    }

    fn get_column_row(&self, index: usize) -> (usize, usize) {
        let x = index as f32 / self.width as f32;
        let row = x.floor() as usize;
        let x: usize = (x * 10f32).floor() as usize;

        (x % 10, row)
    }

    /// Index that the dial will fall to
    pub fn get_index(&self, mut column: usize) -> Option<usize> {
        // 0 0 0 0
        // 1 0 0 0
        // 1 1 0 1
        column %= self.width;

        // column 0 would mean index 0
        // column 2 would mean index 10
        for (index, cell) in self.cells[column..]
            .iter()
            .enumerate()
            .step_by(self.width)
            .rev()
        {
            match *cell {
                Cell::Empty => return Some(index + column),
                _ => {}
            }
        }
        return None;
    }

    /// return Option<Winner>
    pub fn is_game_over(&self) -> JsValue {
        let mut winner: Option<Winner> = None;
        let hor_last_col = self.width - 2;
        let vert_last_col = self.height - 2;
        // Horizontal
        for (row, row_cells) in self.cells.windows(self.width).enumerate() {
            winner = None;
            for (column, cell) in row_cells.iter().enumerate() {
                if column >= hor_last_col && winner.as_ref().map_or(false, |w| w.cells.len() < 2) {
                    break;
                };
                console_log!("row: {row}; col: {column}");
                match self.cell_winner_machine(*cell, &mut winner, self.get_position(column, row)) {
                    MachineState::Continue => continue,
                    MachineState::Winner(winner) => {
                        return JsValue::from_serde(&Some(winner)).unwrap()
                    }
                }
            }
        }
        if let Some(Winner { cells, .. }) = &winner {
            if cells.len() == 4 {
                return JsValue::from_serde(&winner).unwrap();
            }
        }
        winner = None;
        // Vertical
        for col_index in 0..self.width {
            let mut index = col_index;
            for row_index in 0..self.height {
                let cell = self.cells[index];

                if row_index >= vert_last_col
                    && winner.as_ref().map_or(false, |w| w.cells.len() < 2)
                {
                    break;
                };

                match self.cell_winner_machine(cell, &mut winner, index) {
                    MachineState::Continue => index += self.width,
                    MachineState::Winner(winner) => {
                        return JsValue::from_serde(&Some(winner)).unwrap()
                    }
                };
            }
        }
        if let Some(Winner { cells, .. }) = &winner {
            if cells.len() == 4 {
                return JsValue::from_serde(&winner).unwrap();
            }
        }
        winner = None;
        // Left -v Right
        for row in 0..self.height - 3 {
            for col_index in 0..self.width - 3 {
                let mut index = col_index + (row * self.width);
                if index >= self.cells.len() {
                    break;
                }
                for row_index in 0..self.height {
                    if index >= self.cells.len() {
                        break;
                    }

                    let cell = self.cells[index];
                    let (col, _) = self.get_column_row(index);

                    if row_index >= vert_last_col
                        && winner.as_ref().map_or(false, |w| w.cells.len() < 2)
                    {
                        break;
                    };

                    match self.cell_winner_machine(cell, &mut winner, index) {
                        MachineState::Continue => index += self.width + 1,
                        MachineState::Winner(winner) => {
                            return JsValue::from_serde(&Some(winner)).unwrap()
                        }
                    };

                    if col == self.width + 1 {
                        break;
                    }
                }
                if row != 0 {
                    break;
                }
            }
        }
        if let Some(Winner { cells, .. }) = &winner {
            if cells.len() == 4 {
                return JsValue::from_serde(&winner).unwrap();
            }
        }
        winner = None;
        for row in 0..self.height - 3 {
            for col_index in 3..self.width {
                let mut index = col_index + (row * self.width);
                if index >= self.cells.len() {
                    break;
                }
                for row_index in 0..self.height {
                    if index >= self.cells.len() {
                        break;
                    }

                    let cell = self.cells[index];
                    let (col, _) = self.get_column_row(index);

                    if row_index >= vert_last_col
                        && winner.as_ref().map_or(false, |w| w.cells.len() < 2)
                    {
                        break;
                    };

                    match self.cell_winner_machine(cell, &mut winner, index) {
                        MachineState::Continue => index += self.width - 1,
                        MachineState::Winner(winner) => {
                            return JsValue::from_serde(&Some(winner)).unwrap()
                        }
                    };

                    if col == 0 {
                        break;
                    }
                }
                if row != 0 {
                    break;
                }
            }
        }

        if let Some(Winner { cells, .. }) = &winner {
            if cells.len() == 4 {
                return JsValue::from_serde(&winner).unwrap();
            }
        }
        return JsValue::NULL;
    }

    fn cell_winner_machine(
        &self,
        cell: Cell,
        winner: &mut Option<Winner>,
        index: usize,
    ) -> MachineState {
        if !matches!(cell, Cell::Empty) {
            if let Some(Winner { player, cells }) = winner {
                let c = *player;
                if cell == c {
                    cells.push(index);
                    if cells.len() == 4 {
                        return MachineState::Winner(winner.clone().unwrap());
                    }
                } else {
                    *winner = Some(Winner {
                        player: c,
                        cells: vec![index],
                    });
                }
            } else {
                *winner = Some(Winner {
                    player: cell,
                    cells: vec![index],
                });
            }
        } else {
            *winner = None;
        }
        return MachineState::Continue;
    }

    pub fn cells(&self) -> *const Cell {
        self.cells.as_ptr()
    }

    /// Returns true if the move was allowed
    pub fn set_index(&mut self, index: usize, cell: Cell) -> bool {
        // If allowed
        if let Some(index) = self.get_index(index) {
            self.cells[index] = cell;
            true
        } else {
            false
        }
    }

    /// Returns true if the move was allowed
    pub fn set(&mut self, column: usize, row: usize, cell: Cell) -> bool {
        self.set_index(self.get_position(column, row), cell)
    }
}
