use grid::Grid;

#[test]
fn grid_new() {
    let g = Grid::new(2, 3);
    assert_eq!(g.height, 3);
    assert_eq!(g.width, 2);
}

#[test]
fn grid_cell() {
    let g = Grid::new(10, 20);
    let c = g.cell(8, 6);
    assert_eq!(c.column, 8);
    assert_eq!(c.row, 6);
}
