fn main() {
  println!("hello")
}

const SIZE_GRILLE: usize = 8;
type Grid = [[bool; SIZE_GRILLE]; SIZE_GRILLE];

fn conway_next_gen(grid: Grid) -> Grid {
  let mut res = grid.clone();

  for (i, line) in grid.iter().enumerate() {
    for (j, &cell) in line.iter().enumerate() {
      if cell {
        let neighbours = counter_neighbours(&grid, i, j);
        if neighbours < 2 {
          res[i][j] = false;
        }
      }
    }
  }
  res
}

fn counter_neighbours(grid: &Grid, x: usize, y: usize) -> u8 {
  let mut nb_arrround: u8 = 0;

  if is_valid_index(x - 1, y - 1) && grid[x - 1][y - 1] {
    nb_arrround += 1;
  }

  if is_valid_index(x - 1, y) && grid[x - 1][y] {
    nb_arrround += 1;
  }

  if is_valid_index(x - 1, y + 1) && grid[x - 1][y + 1] {
    nb_arrround += 1;
  }

  if is_valid_index(x, y - 1) && grid[x][y - 1] {
    nb_arrround += 1;
  }

  if is_valid_index(x, y + 1) && grid[x][y + 1] {
    nb_arrround += 1;
  }

  if is_valid_index(x + 1, y - 1) && grid[x + 1][y - 1] {
    nb_arrround += 1;
  }

  if is_valid_index(x + 1, y) && grid[x + 1][y] {
    nb_arrround += 1;
  }

  if is_valid_index(x + 1, y + 1) && grid[x + 1][y + 1] {
    nb_arrround += 1;
  }

  nb_arrround
}

fn is_valid_index(x: usize, y: usize) -> bool {
  (x >= 0 && x <= SIZE_GRILLE) && (y >= 0 && y <= SIZE_GRILLE)
}

fn parser(grid: &[&str]) -> Grid {
  let mut string_grid = Grid::default();

  for (i, line) in grid.iter().take(string_grid.len()).enumerate() {
    for (j, char) in line.chars().take(string_grid[i].len()).enumerate() {
      if char == 'X' || char == 'x' {
        string_grid[i][j] = true;
      }
    }
  }
  string_grid
}

// tests module
#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_empty() {
    #[rustfmt::skip]
    let grid = parser(&vec![
      "........",
      "........",
      "........",
      "........",
      "........",
      "........",
      "........",
      "........"
    ]);

    let next = conway_next_gen(grid);

    assert_eq!(next, grid)
  }

  #[test]
  fn test_parse_one_cell() {
    let grid: Grid = [
      [false, false, false, false, false, false, false, false],
      [false, true, false, false, false, false, false, false],
      [false, false, false, false, false, false, false, false],
      [false, false, false, false, false, false, false, false],
      [false, false, false, false, false, false, false, false],
      [false, false, false, false, false, false, false, false],
      [false, false, false, false, false, false, false, false],
      [false, false, false, false, false, false, false, false],
    ];

    #[rustfmt::skip]
    let res = parser(&vec![
      "........",
      ".X......",
      "........",
      "........",
      "........",
      "........",
      "........",
      "........"
    ]);
    assert_eq!(res, grid);
  }

  #[test]
  fn test_parse_antipanic() {
    #[rustfmt::skip]
    parser(&vec![
      "........",
      ".X......X",
      "........",
      "........",
      "........",
      "........",
      "........",
      "........",
      "........",
    ]);
  }

  #[test]
  fn test_parse_empty() {
    #[rustfmt::skip]
    let res = parser(&vec![
      "........",
      "........",
      "........",
      "........",
      "........",
      "........",
      "........",
      "........",
    ]);
    assert_eq!(res, Grid::default());
  }

  #[test]
  fn test_1_cellule_en_vie_doit_mourrir() {
    #[rustfmt::skip]
    let grid = parser(&vec![
      "........",
      ".X......",
      "........",
      "........",
      "........",
      "........",
      "........",
      "........"
    ]);

    let next = conway_next_gen(grid);

    assert_eq!(next, Grid::default());
  }

  #[test]
  fn test_un_carre_ne_doit_pas_bouger() {
    #[rustfmt::skip]
    let grid = parser(&vec![
      "........",
      "........",
      "..XX....",
      "..XX....",
      "........",
      "........",
      "........",
      "........"
    ]);

    let next = conway_next_gen(grid);

    assert_eq!(next, grid);
  }
}
