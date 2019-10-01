use super::coord::Coord;
use super::Square;

#[derive(Copy, Clone, Debug)]
pub struct Move {
    from: Coord,
    to: Coord,
    start_value: u16,
    end_value: u16,
    target_square: Square,
}

impl Move {
    pub fn new(
        from: Coord,
        to: Coord,
        start_value: u16,
        end_value: u16,
        target_square: Square,
    ) -> Self {
        Self {
            from,
            to,
            start_value,
            end_value,
            target_square,
        }
    }
}
