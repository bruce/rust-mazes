use std::rc::Rc;
use std::default::Default;

pub type CellIndex = (usize, usize); // col, row

#[derive(Debug)]
pub struct Cell {
    pub index: CellIndex,
    pub placement: Placement
}

impl Default for Cell {
    fn default() -> Cell {
        Cell { index: (0, 0),
               placement: Placement::Normal }
    }
}

#[derive(Debug)]
pub enum Placement {
    Normal,
    Under
}
