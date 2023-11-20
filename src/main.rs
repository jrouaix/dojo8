fn main() {
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
  conway_next_gen(grid);
}

const SIZE_GRILLE: usize = 8;
type Grid = [[bool; SIZE_GRILLE]; SIZE_GRILLE];

fn conway_next_gen(grid: Grid) -> Grid {
  let mut res = grid.clone();

  for (i, line) in grid.iter().enumerate() {
    for (j, &cell) in line.iter().enumerate() {
      if cell {
        let neighbours = counter_neighbours(&grid, i, j);
        if neighbours < 2 || neighbours > 3 {
          res[i][j] = false;
        }
      } else {
        let neighbours = counter_neighbours(&grid, i, j);
        if neighbours == 3 {
          res[i][j] = true;
        }
      }
    }
  }
  res
}

fn counter_neighbours(grid: &Grid, x: usize, y: usize) -> u8 {
  let indexes: [(isize, isize); 8] = [(-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, -1), (1, 0), (1, 1)];
  fn is_alive(grid: &Grid, x: &usize, y: &usize) -> bool {
    grid[*x][*y]
  }
  fn is_valid_index((x, y): &(isize, isize)) -> bool {
    *x >= 0_isize && *x < SIZE_GRILLE as isize && *y >= 0_isize && *y < SIZE_GRILLE as isize
  }
  indexes
    .into_iter()
    .map(|(tx, ty)| (x as isize + tx, y as isize + ty))
    .filter(is_valid_index)
    .map(|(x, y)| (x as usize, y as usize))
    .filter(|(x, y)| is_alive(grid, x, y))
    .count() as u8
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
  use pretty_assertions::assert_eq;

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

  #[test]
  fn test_dans_un_cas_de_surpopulation_la_cellule_doit_mourrir() {
    #[rustfmt::skip]
    let grid = parser(&vec![
      "........",
      "........",
      "..XXX...",
      "..XX....",
      "........",
      "........",
      "........",
      "........"
    ]);

    #[rustfmt::skip]
    let res = parser(&vec![
      "........",
      "...X....",
      "..X.X...",
      "..X.X...",
      "........",
      "........",
      "........",
      "........"
    ]);

    let next = conway_next_gen(grid);

    assert_eq!(next, res);
  }

  #[test]
  fn test_bordure_ne_doit_pas_planter() {
    #[rustfmt::skip]
    let grid = parser(&vec![
      "........",
      "........",
      "..XX....",
      "..XX....",
      "........",
      "........",
      "X.......",
      "........"
    ]);

    #[rustfmt::skip]
    let res = parser(&vec![
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
    assert_eq!(next, res);
  }

  #[test]
  fn test_de_naissance() {
    #[rustfmt::skip]
    let grid = parser(&vec![
      "........",
      "........",
      "..XXX...",
      "........",
      "........",
      "........",
      "........",
      "........"
    ]);

    #[rustfmt::skip]
    let res = parser(&vec![
      "........",
      "...X....",
      "...X....",
      "...X....",
      "........",
      "........",
      "........",
      "........"
    ]);

    let next = conway_next_gen(grid);
    assert_eq!(next, res);
  }
}
