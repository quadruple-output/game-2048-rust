use super::*;

#[test]
fn shift_empty_board() { shift_and_expect([0, 0, 0, 0], [0, 0, 0, 0]); }

#[test]
fn shift_impossible() { shift_and_expect([2, 4, 8, 16], [2, 4, 8, 16]); }

#[test]
fn shift_2_0_0_0() { shift_and_expect([2, 0, 0, 0], [2, 0, 0, 0]); }

#[test]
fn shift_0_2_0_0() { shift_and_expect([0, 2, 0, 0], [2, 0, 0, 0]); }

#[test]
fn shift_0_0_2_0() { shift_and_expect([0, 0, 2, 0], [2, 0, 0, 0]); }

#[test]
fn shift_0_2_0_2() { shift_and_expect([0, 2, 0, 2], [4, 0, 0, 0]); }

#[test]
fn shift_0_0_2_2() { shift_and_expect([0, 0, 2, 2], [4, 0, 0, 0]); }

#[test]
fn shift_0_2_2_2() { shift_and_expect([0, 2, 2, 2], [4, 2, 0, 0]); }

#[test]
fn shift_2_2_0_0() { shift_and_expect([2, 2, 0, 0], [4, 0, 0, 0]); }

#[test]
fn shift_2_0_0_2() { shift_and_expect([2, 0, 0, 2], [4, 0, 0, 0]); }

#[test]
fn shift_2_0_0_4() { shift_and_expect([2, 0, 0, 4], [2, 4, 0, 0]); }

#[test]
fn shift_4_0_2_2() { shift_and_expect([4, 0, 2, 2], [4, 4, 0, 0]); }

#[test]
fn shift_4_0_2_0() { shift_and_expect([4, 0, 2, 0], [4, 2, 0, 0]); }

#[test]
fn shift_2_0_2_2() { shift_and_expect([2, 0, 2, 2], [4, 2, 0, 0]); }

#[test]
fn shift_4_2_2_0() { shift_and_expect([4, 2, 2, 0], [4, 4, 0, 0]); }

#[test]
fn shift_4_2_0_2() { shift_and_expect([4, 2, 0, 2], [4, 4, 0, 0]); }

#[test]
fn shift_8_4_0_2() { shift_and_expect([8, 4, 0, 2], [8, 4, 2, 0]); }

#[test]
fn shift_2_2_2_2() { shift_and_expect([2, 2, 2, 2], [4, 4, 0, 0]); }

#[test]
fn shift_4_2_2_2() { shift_and_expect([4, 2, 2, 2], [4, 4, 2, 0]); }

#[test]
fn shift_4_2_2_4() { shift_and_expect([4, 2, 2, 4], [4, 4, 4, 0]); }

fn shift_and_expect(probe: [u16; 4], exp: [u16; 4]) {
  let act = shift(probe);
  if act != exp {
    panic!("\n  probe:  {:?}\n  result: {:?} expected: {:?}", probe, act, exp);
  }
}

fn shift(column: [u16; 4]) -> [u16; 4] {
  let mut board = board_with_column(column);
  board
        .shift_up()
        .or_else(|| Some(Vec::new())) // unfortunately, we need a type annotation here
        .unwrap();
  first_column_from_board(board)
}

fn first_column_from_board(board: Board) -> [u16; 4] {
  let mut result = [0, 0, 0, 0];
  for (y, square_value) in result.iter_mut().enumerate() {
    *square_value = match board.at_xy(0, y) {
      Square::Empty => 0,
      Square::Value(val) => val
    };
  }
  // The code below yields this clippy warning:
  //
  // the loop variable `y` is used to index `result`
  // note: `#[warn(clippy::needless_range_loop)]` on by default
  // help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#needless_range_loop
  //
  //  for y in 0..board.size_y() {
  //    result[y] = match board.at_xy(0, y) {
  //      Square::Empty => 0,
  //      Square::Value(val) => val
  //    };
  //  }
  result
}

fn board_with_column(column: [u16; 4]) -> Board {
  let mut board = Board::new(1, 4);
  for (y, value) in column.iter().enumerate() {
    board.put(board.coord(0, y), match *value {
           0 => Square::Empty,
           _ => Square::Value(*value)
         });
  }
  board
}
