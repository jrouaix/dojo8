use std::str::FromStr;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

const ALIVE_CHAR: char = 'o';
const DEAD_CHAR: char = '.';

use CellState::*;

#[derive(Debug, Copy, Clone, PartialEq, Default)]
enum CellState {
  #[default]
  Dead,
  Alive,
}

impl CellState {
  pub fn is_alive(&self) -> bool {
    self == &CellState::Alive
  }

  fn next(self, neighbours: u8) -> Self {
    match (self, neighbours) {
      (Alive, 2) | (Alive, 3) => Alive,
      (Dead, 3) => Alive,
      _ => CellState::Dead,
    }
  }

  pub fn random() -> Self {
    match rand::random::<bool>() {
      false => Dead,
      true => Alive,
    }
  }
}

#[cfg(test)]
mod cell_state_tests {
  use super::*;
  use test_case::test_case;

  /// Any live cell with fewer than two live neighbours dies, as if caused by underpopulation.
  #[test_case(Alive, 0, Dead)]
  #[test_case(Alive, 1, Dead)]
  fn underpopulation(start_state: CellState, neighbours: u8, expected_state: CellState) {
    assert_eq!(start_state.next(neighbours), expected_state);
  }

  /// Any live cell with two or three live neighbours lives on to the next generation.
  #[test_case(Alive, 2, Alive)]
  #[test_case(Alive, 3, Alive)]
  fn lives_on(start_state: CellState, neighbours: u8, expected_state: CellState) {
    assert_eq!(start_state.next(neighbours), expected_state);
  }

  /// Any live cell with more than three live neighbours dies, as if by overpopulation.
  #[test_case(Alive, 4, Dead)]
  #[test_case(Alive, 5, Dead)]
  #[test_case(Alive, 6, Dead)]
  #[test_case(Alive, 7, Dead)]
  #[test_case(Alive, 8, Dead)]
  fn overpopulation(start_state: CellState, neighbours: u8, expected_state: CellState) {
    assert_eq!(start_state.next(neighbours), expected_state);
  }

  /// Any dead cell with exactly three live neighbours becomes a live cell, as if by reproduction.
  #[test_case(Dead, 3, Alive)]
  fn reproduction(start_state: CellState, neighbours: u8, expected_state: CellState) {
    assert_eq!(start_state.next(neighbours), expected_state);
  }
}

const GRID_SIZE: usize = 16;
type InnerGrid = [[CellState; GRID_SIZE]; GRID_SIZE];

#[derive(Debug, PartialEq, Default, Clone)]
pub struct Grid {
  inner: InnerGrid,
}

impl Grid {
  pub fn random() -> Self {
    let mut grid = Self::default();
    for line in &mut grid.inner {
      for cell in line {
        *cell = CellState::random();
      }
    }
    grid
  }

  pub fn next(&self) -> Self {
    let mut next_inner = self.inner.clone();
    for (x, line) in self.inner.iter().enumerate() {
      for (y, &cell) in line.iter().enumerate() {
        let neighbours = self.count_alive_neighbours(x, y);
        next_inner[x][y] = cell.next(neighbours);
      }
    }
    Self { inner: next_inner }
  }

  fn count_alive_neighbours(&self, x: usize, y: usize) -> u8 {
    Direction::iter()
      .filter_map(|d: Direction| {
        let translation = d.get_coordinates();
        let neighbour_x = x.checked_add_signed(translation.0);
        let neighbour_y = y.checked_add_signed(translation.1);
        match (neighbour_x, neighbour_y) {
          (Some(x), Some(y)) => self.inner.get(x).and_then(|l| l.get(y)),
          _ => None,
        }
      })
      .filter(|cell| cell.is_alive())
      .count() as u8
  }
}

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

impl std::fmt::Display for Grid {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let separator = "\n";
    let mut s: String = String::new();

    for line in &self.inner {
      for element in line {
        s.push(match element {
          CellState::Dead => DEAD_CHAR,
          CellState::Alive => ALIVE_CHAR,
        });
      }
      s.push_str(&separator);
    }
    write!(f, "{}", s)
  }
}

impl FromStr for Grid {
  type Err = ();

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let mut grid = InnerGrid::default();
    for (i, line) in s.lines().take(grid.len()).enumerate() {
      for (j, char) in line.chars().take(grid[i].len()).enumerate() {
        if char != DEAD_CHAR {
          grid[i][j] = CellState::Alive;
        }
      }
    }
    Ok(Self { inner: grid })
  }
}

#[cfg(test)]
mod grid_parse_and_display_tests {
  use super::*;

  #[test]
  fn test_empty() {
    let grid = Grid::from_str("").unwrap();
    assert_eq!(grid, Grid::default());
  }

  #[test]
  fn random_identity() {
    fn identity(grid: Grid) -> Grid {
      grid.to_string().parse().expect("Failed to parse grid")
    }

    for _ in 0..100 {
      let grid = Grid::random();
      assert_eq!(identity(grid.clone()), grid);
    }
  }
}
