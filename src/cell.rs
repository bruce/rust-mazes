use std::rc::Rc;
use std::default::Default;

#[derive(Debug)]
pub struct Cell {
    pub column: usize,
    pub row: usize,
    pub placement: Placement,
    pub north: Option<Rc<Cell>>,
    pub south: Option<Rc<Cell>>,
    pub east: Option<Rc<Cell>>,
    pub west: Option<Rc<Cell>>
}

impl Default for Cell {
    fn default() -> Cell {
        Cell { column: 0, row: 0,
               placement: Placement::Normal,
               north: None, south: None,
               east: None, west: None }
    }
}

#[derive(Debug)]
pub enum Placement {
    Normal,
    Under
}
