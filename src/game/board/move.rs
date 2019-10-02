use super::coord::Coord;

#[derive(Copy, Clone, Debug)]
pub struct Move {
	from: Coord,
	to: Coord,
	start_value: u16,
	end_value: u16,
}

impl Move {
	pub fn new(from: Coord, to: Coord, start_value: u16, end_value: u16) -> Self {
		Self { from, to, start_value, end_value }
	}
}
