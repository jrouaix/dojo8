use indoc::indoc;
use std::{fmt::Display, ptr::eq};
use std::{thread, time};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(Debug, Copy, Clone, PartialEq, Default)]
enum CellState {
  #[default]
  Dead,
  Alive,
}

impl CellState {
  fn is_alive(&self) -> bool {
    self == &CellState::Alive
  }
}

const GRID_SIZE: usize = 8;
type Grid = [[CellState; GRID_SIZE]; GRID_SIZE];
struct Wrap(Grid);

#[derive(Clone, Copy, EnumIter)]
enum Direction {
  North,
  NorthEast,
  East,
  SouthEast,
  South,
  SouthWest,
  West,
  NorthWest,
}

impl Direction {
  pub fn get_coordinates(&self) -> (isize, isize) {
    match self {
      Direction::North => (-1, 0),
      Direction::NorthEast => (-1, 1),
      Direction::East => (0, 1),
      Direction::SouthEast => (1, 1),
      Direction::South => (1, 0),
      Direction::SouthWest => (1, -1),
      Direction::West => (0, -1),
      Direction::NorthWest => (-1, -1),
    }
  }
}

fn main() {
  let grid = parser(indoc!(
    ".X......
     ..X.....
     XXX.....
     ........
     ........
     ........
     ........
     ........
     "
  ));

  println!("\ninitial grid :");
  display(grid);

  let two_seconds = time::Duration::from_secs(2);
  let mut prev = grid;

  loop {
    let next = conway_next_generation(prev);
    println!("\n\ngrid after a new generation :");
    display(next);
    thread::sleep(two_seconds);
    println!();

    if prev == next {
      break;
    }

    prev = next;
  }
}

fn conway_next_generation(grid: Grid) -> Grid {
  let mut res = grid.clone();

  for (x, line) in grid.iter().enumerate() {
    for (y, &cell) in line.iter().enumerate() {
      let neighbours = count_neighbours(&grid, x, y);
      apply_conways_rules(cell, neighbours, x, y, &mut res);
    }
  }
  res
}

fn apply_conways_rules(cell: CellState, neighbours: u8, x: usize, y: usize, res: &mut [[CellState; 8]; 8]) {
  match cell {
    CellState::Alive => {
      if is_underpopulation(neighbours) || is_overpopulation(neighbours) {
        die(res, x, y);
      }
    }

    CellState::Dead => {
      if is_livable_condition(neighbours) {
        resurrect(res, x, y);
      }
    }
  }
}

fn is_livable_condition(neighbours: u8) -> bool {
  neighbours == 3
}

fn is_overpopulation(neighbours: u8) -> bool {
  neighbours > 3
}

fn is_underpopulation(neighbours: u8) -> bool {
  neighbours < 2
}

fn resurrect(res: &mut [[CellState; 8]; 8], x: usize, y: usize) {
  res[x][y] = CellState::Alive;
}

fn die(res: &mut [[CellState; 8]; 8], x: usize, y: usize) {
  res[x][y] = CellState::Dead
}

fn count_neighbours(grid: &Grid, x: usize, y: usize) -> u8 {
  Direction::iter().map(|d: Direction| count_neighbour(grid, x, y, d)).sum()
}

fn count_neighbour(grid: &[[CellState; 8]; 8], x: usize, y: usize, direction: Direction) -> u8 {
  let increment = direction.get_coordinates();

  if let Some(x) = x.checked_add_signed(increment.0) {
    if let Some(y) = y.checked_add_signed(increment.1) {
      if let Some(cell) = grid.get(x).and_then(|l| l.get(y)) {
        if cell.is_alive() {
          return 1;
        }
      }
    }
  }
  0
}

impl Display for Wrap {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let separator = "\n";
    let mut s = String::new();

    for line in &self.0 {
      for element in line {
        s.push_str(match element {
          CellState::Dead => ".",
          CellState::Alive => "X",
        });

        if eq(element, line.last().unwrap()) {
          s.push_str(&separator);
        }
      }
    }
    write!(f, "{}", s)
  }
}

fn display(grid: Grid) {
  print!("{}", Wrap(grid));
}

fn parser(grid: &str) -> Grid {
  parser_slice(grid.lines().collect::<Vec<&str>>().as_slice())
}

fn parser_slice(grid: &[&str]) -> Grid {
  let mut string_grid = Grid::default();

  for (i, line) in grid.iter().take(string_grid.len()).enumerate() {
    for (j, char) in line.chars().take(string_grid[i].len()).enumerate() {
      if char == 'X' || char == 'x' {
        string_grid[i][j] = CellState::Alive;
      }
    }
  }
  string_grid
}

// tests module
#[cfg(test)]
mod tests {
  use super::*;
  use crate::CellState::Alive;
  use crate::CellState::Dead;

  #[test]
  fn test_empty() {
    let grid = parser(indoc!(
      "........
       ........
       ........
       ........
       ........
       ........
       ........
       ........
       "
    ));

    let next = conway_next_generation(grid);

    assert_eq!(next, grid)
  }

  #[test]
  fn test_parse_one_cell() {
    let grid: Grid = [
      [Dead, Dead, Dead, Dead, Dead, Dead, Dead, Dead],
      [Dead, Alive, Dead, Dead, Dead, Dead, Dead, Dead],
      [Dead, Dead, Dead, Dead, Dead, Dead, Dead, Dead],
      [Dead, Dead, Dead, Dead, Dead, Dead, Dead, Dead],
      [Dead, Dead, Dead, Dead, Dead, Dead, Dead, Dead],
      [Dead, Dead, Dead, Dead, Dead, Dead, Dead, Dead],
      [Dead, Dead, Dead, Dead, Dead, Dead, Dead, Dead],
      [Dead, Dead, Dead, Dead, Dead, Dead, Dead, Dead],
    ];

    let res = parser(indoc!(
      "........
       .X......
       ........
       ........
       ........
       ........
       ........
       ........
       "
    ));
    assert_eq!(res, grid);
  }

  #[test]
  fn test_format_one_cell_grid() {
    let grid = parser(indoc!(
      "........
       .X......
       ........
       ........
       ........
       ........
       ........
       ........
       "
    ));

    let expected_grid = indoc!(
      "........
       .X......
       ........
       ........
       ........
       ........
       ........
       ........
       "
    );

    let formated_grid = format!("{}", Wrap(grid));
    assert_eq!(formated_grid, expected_grid);
  }

  #[test]
  fn test_parse_antipanic() {
    parser(indoc!(
      ".X......
       .XX.....X
       .X......
       ........
       ........
       ........
       ........
       ........
       "
    ));
  }

  #[test]
  fn test_parse_empty() {
    let res = parser(indoc!(
      "........
       ........
       ........
       ........
       ........
       ........
       ........
       ........
       "
    ));
    assert_eq!(res, Grid::default());
  }

  #[test]
  fn test_1_cellule_en_vie_doit_mourrir() {
    let grid = parser(indoc!(
      "........
       ........
       .X......
       ........
       ........
       ........
       ........
       ........
       "
    ));

    let next = conway_next_generation(grid);

    assert_eq!(next, Grid::default());
  }

  #[test]
  fn test_un_carre_ne_doit_pas_bouger() {
    let grid = parser(indoc!(
      "........
       .XX.....
       .XX.....
       ........
       ........
       ........
       ........
       ........
       "
    ));

    let next = conway_next_generation(grid);

    assert_eq!(next, grid);
  }
}
