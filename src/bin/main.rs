use std::io::Write;
use std::{thread, time};

use gameoflife::Grid;

fn main() -> anyhow::Result<()> {
  let mut console = std::io::stdout().lock();
  console.flush()?;

  let mut grid = Grid::random();
  loop {
    clearscreen::clear()?;
    write!(console, "{grid}")?;
    console.flush()?;
    grid = grid.next();
    thread::sleep(time::Duration::from_millis(300));
  }
}

// // tests module
// #[cfg(test)]
// mod tests {
//   use super::*;
//   use crate::CellState::Alive;
//   use crate::CellState::Dead;

//   #[test]
//   fn test_empty() {
//     let grid = parser(indoc!(
//       "........
//        ........
//        ........
//        ........
//        ........
//        ........
//        ........
//        ........
//        "
//     ));

//     let next = conway_next_generation(grid);

//     assert_eq!(next, grid)
//   }

//   #[test]
//   fn test_parse_one_cell() {
//     let grid: Grid = [
//       [Dead, Dead, Dead, Dead, Dead, Dead, Dead, Dead],
//       [Dead, Alive, Dead, Dead, Dead, Dead, Dead, Dead],
//       [Dead, Dead, Dead, Dead, Dead, Dead, Dead, Dead],
//       [Dead, Dead, Dead, Dead, Dead, Dead, Dead, Dead],
//       [Dead, Dead, Dead, Dead, Dead, Dead, Dead, Dead],
//       [Dead, Dead, Dead, Dead, Dead, Dead, Dead, Dead],
//       [Dead, Dead, Dead, Dead, Dead, Dead, Dead, Dead],
//       [Dead, Dead, Dead, Dead, Dead, Dead, Dead, Dead],
//     ];

//     let res = parser(indoc!(
//       "........
//        .X......
//        ........
//        ........
//        ........
//        ........
//        ........
//        ........
//        "
//     ));
//     assert_eq!(res, grid);
//   }

//   #[test]
//   fn test_format_one_cell_grid() {
//     let grid = parser(indoc!(
//       "........
//        .X......
//        ........
//        ........
//        ........
//        ........
//        ........
//        ........
//        "
//     ));

//     let expected_grid = indoc!(
//       "........
//        .X......
//        ........
//        ........
//        ........
//        ........
//        ........
//        ........
//        "
//     );

//     let formated_grid = format!("{}", Wrap(grid));
//     assert_eq!(formated_grid, expected_grid);
//   }

//   #[test]
//   fn test_parse_antipanic() {
//     parser(indoc!(
//       ".X......
//        .XX.....X
//        .X......
//        ........
//        ........
//        ........
//        ........
//        ........
//        "
//     ));
//   }

//   #[test]
//   fn test_parse_empty() {
//     let res = parser(indoc!(
//       "........
//        ........
//        ........
//        ........
//        ........
//        ........
//        ........
//        ........
//        "
//     ));
//     assert_eq!(res, Grid::default());
//   }

//   #[test]
//   fn test_1_cellule_en_vie_doit_mourrir() {
//     let grid = parser(indoc!(
//       "........
//        ........
//        .X......
//        ........
//        ........
//        ........
//        ........
//        ........
//        "
//     ));

//     let next = conway_next_generation(grid);

//     assert_eq!(next, Grid::default());
//   }

//   #[test]
//   fn test_un_carre_ne_doit_pas_bouger() {
//     let grid = parser(indoc!(
//       "........
//        .XX.....
//        .XX.....
//        ........
//        ........
//        ........
//        ........
//        ........
//        "
//     ));

//     let next = conway_next_generation(grid);

//     assert_eq!(next, grid);
//   }
// }