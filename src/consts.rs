const GLIDER_WIDTH: usize = 3;
const GLIDER_HEIGHT: usize = 3;

#[rustfmt::skip]
pub const GLIDER: [[bool; GLIDER_WIDTH]; GLIDER_HEIGHT] = [
  [false, true , false], 
  [false, false, true ], 
  [true , true , true ]
];

pub fn glider_mask(anchor_x: usize, anchor_y: usize, current_x: usize, current_y: usize) -> bool {
  if current_x < anchor_x || current_y < anchor_y {
    return false;
  }

  let x_offset = current_x - anchor_x;
  let y_offset = current_y - anchor_y;

  if x_offset < GLIDER_WIDTH && y_offset < GLIDER_HEIGHT {
    GLIDER[y_offset][x_offset]
  } else {
    false
  }
}

// ⬛ represents a dead cell
// ⬜ represents a live cell

const TEXT_404_WIDTH: usize = 17;
const TEXT_404_HEIGHT: usize = 7;
pub const TEXT_404_TOP_LEFT_X_OFFSET: isize = -(TEXT_404_WIDTH as isize / 2);
pub const TEXT_404_TOP_LEFT_Y_OFFSET: isize = -(TEXT_404_HEIGHT as isize / 2);

#[rustfmt::skip]
pub const TEXT_404: [[bool; TEXT_404_WIDTH]; TEXT_404_HEIGHT] = [
  [false, false, false, true, false, false, false, true, true, true, false, false, false, false, false, true, false],
  [false, false, true, true, false, false, true, false, false, false, true, false, false, false, true, true, false],
  [false, true, false, true, false, false, true, false, false, true, true, false, false, true, false, true, false],
  [true, false, false, true, false, false, true, false, true, false, true, false, true, false, false, true, false],
  [true, true, true, true, true, false, true, true, false, false, true, false, true, true, true, true, true],
  [false, false, false, true, false, false, true, false, false, false, true, false, false, false, false, true, false],
  [false, false, false, true, false, false, false, true, true, true, false, false, false, false, false, true, false]
];

pub fn text_404_mask(anchor_x: usize, anchor_y: usize, current_x: usize, current_y: usize) -> bool {
  if current_x < anchor_x || current_y < anchor_y {
    return false;
  }

  let x_offset = current_x - anchor_x;
  let y_offset = current_y - anchor_y;

  if x_offset < TEXT_404_WIDTH && y_offset < TEXT_404_HEIGHT {
    TEXT_404[y_offset][x_offset]
  } else {
    false
  }
}
