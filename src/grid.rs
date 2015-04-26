use std::default::Default;

use cell::*;

#[derive(Debug)]
pub struct Grid {
    pub width: usize,
    pub height: usize,
    // Each internal Vec is a column of cells
    pub cells: Vec<Vec<Cell>>
}
impl Default for Grid {
    fn default() -> Grid {
        Grid { width: 0, height: 0, cells: vec![] }
    }
}

impl Grid {

    pub fn new(cols: usize, rows: usize) -> Grid {
        let mut g = Grid { width: cols, height: rows, .. Default::default() };
        for col in (1..cols) {
            let mut col_vec = vec![];
            for row in (1..rows) {
                let cell = Cell {column: col, row: row, .. Default::default() };
                col_vec.push(cell);
            }
            g.cells.push(col_vec);
        }
        g
    }

    pub fn cell(&self, column: usize, row: usize) -> &Cell {
        &self.cells[column - 1][row - 1]
    }

}
