#[derive(Copy, Clone, Debug)]
pub struct Coord {
	pub x: usize,
	pub y: usize,
	max_x: usize,
	max_y: usize
}

impl std::cmp::PartialEq for Coord {
	fn eq(&self, other: &Coord) -> bool { self.x == other.x && self.y == other.y }
}

impl Coord {
	pub fn new(x: usize, y: usize, max_x: usize, max_y: usize) -> Self { Coord { x, y, max_x, max_y } }

	pub fn add(&self, vector: Vector) -> Result<Self, ()> {
		// TODO: enable use of '+' operator (not trivial with Rhs being of different
		// type)
		let new_x = self.x as isize + vector.dx;
		let new_y = self.y as isize + vector.dy;
		if new_x < 0 || new_y < 0 || new_x > self.max_x as isize || new_y > self.max_y as isize {
			Err(())
		} else {
			Ok(Self { x: new_x as usize, y: new_y as usize, max_x: self.max_x, max_y: self.max_y })
		}
	}
}

#[derive(Copy, Clone)]
pub struct Vector {
	pub dx: isize,
	pub dy: isize
}

impl Vector {
	pub fn new(dx: isize, dy: isize) -> Self { Self { dx, dy } }
}
