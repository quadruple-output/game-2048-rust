use super::coord::Coord;

#[derive(Copy, Clone, Debug)]
pub enum Move {
	Appear { at: Coord, value: u16 },
	Shift { from: Coord, to: Coord, value: u16 },
	Merge { from: Coord, to: Coord, start_value: u16, end_value: u16 },
	Stay { at: Coord, value: u16 }
}
