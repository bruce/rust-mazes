use std::default::Default;
use std::collections::HashMap;

use cell::*;

#[derive(Debug)]
pub struct Grid {
    pub width: usize,
    pub height: usize,
    pub cells: HashMap<CellIndex, Cell>
}
impl Default for Grid {
    fn default() -> Grid {
        Grid { width: 0, height: 0, cells: HashMap::new() }
    }
}

impl Grid {

    pub fn new(cols: usize, rows: usize) -> Grid {
        let mut g = Grid { width: cols, height: rows, .. Default::default() };
        for col in (1..cols) {
            for row in (1..rows) {
                g.cells.insert((col, row),
                               Cell {index: (col, row), .. Default::default() })
            }
        }
        g
    }

}
