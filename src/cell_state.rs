pub struct CellState {
    pub row: i32,
    pub col: i32,
    pub mine: bool,
    pub revealed: bool,
    pub flagged: bool,
    pub neighbor_mines: i32,
}
