use crate::cell_state::CellState;
use crate::click_side::ClickSide;

pub enum GameSignal {
    TilePress {
		row: i32,
		col: i32,
		click_side: ClickSide,
	},
    FacePress,
    StateChange(CellState),
    ClockTick,
}

