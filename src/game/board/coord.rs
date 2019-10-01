#[derive(Copy, Clone, Debug)]
pub struct Coord {
    pub x: usize,
    pub y: usize,
    max_dimension: usize,
}

impl std::cmp::PartialEq for Coord {
    fn eq(&self, other: &Coord) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Coord {
    pub fn new(x: usize, y: usize, max_dimension: usize) -> Self {
        Coord {
            x,
            y,
            max_dimension,
        }
    }

    pub fn add(&self, vector: Vector) -> Result<Self, ()> {
        // TODO: enable use of '+' operator (not trivial with Rhs being of different type)
        let new_x = self.x as isize + vector.dx;
        let new_y = self.y as isize + vector.dy;
        if new_x < 0
            || new_y < 0
            || new_x >= self.max_dimension as isize
            || new_y >= self.max_dimension as isize
        {
            Err(())
        } else {
            Ok(Self {
                x: new_x as usize,
                y: new_y as usize,
                max_dimension: self.max_dimension,
            })
        }
    }
}

#[derive(Copy, Clone)]
pub struct Vector {
    pub dx: isize,
    pub dy: isize,
}
