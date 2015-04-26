extern crate mazes;

#[allow(unused_imports)]
use mazes::grid::Grid;

#[cfg(not(test))]
fn main() {
    let (cols, rows) = (100, 100);
    let grid = Grid::new(cols, rows);
    println!("Grid is {} by {}", grid.width, grid.height);
}
