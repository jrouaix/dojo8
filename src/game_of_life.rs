use bevy::{
  color::palettes::css::{BLACK, WHITE},
  prelude::*,
  window::PrimaryWindow,
};

use crate::consts::{is_near_text_404, text_404_mask, TEXT_404_TOP_LEFT_X_OFFSET, TEXT_404_TOP_LEFT_Y_OFFSET};

pub struct GameOfLife;

impl Plugin for GameOfLife {
  fn build(&self, app: &mut App) {
    app
      .insert_resource(GameTimer(Timer::from_seconds(0.1f32, TimerMode::Repeating)))
      .insert_resource(WaitForUserTimer(Timer::from_seconds(5f32, TimerMode::Once)))
      .add_systems(Startup, setup_camera)
      .add_systems(PostStartup, setup_entities)
      .add_systems(Update, game_loop);
  }
}

fn setup_camera(mut commands: Commands, q_window: Query<&Window, With<PrimaryWindow>>) {
  let window = q_window.single().unwrap();
  info!("Setting up camera with window size: {}x{}", window.width(), window.height());

  // Set the camera to cover the entire window
  commands.spawn((Camera2d::default(), Transform::from_xyz(window.width() / 2f32, window.height() / 2f32, 100f32)));
  info!("Camera2d has been spawned");
}

const SQUARE_SIZE: f32 = 10f32;
const HALF_SQUARE_SIZE: f32 = SQUARE_SIZE / 2f32;

pub type Grid = Vec<Vec<(bool, Handle<ColorMaterial>)>>;

#[derive(Resource, Default, Debug)]
struct GameGrid {
  grid: Grid,
}
#[derive(Resource)]
struct GameTimer(Timer);

#[derive(Resource)]
struct WaitForUserTimer(Timer);

fn setup_entities(
  mut commands: Commands,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<ColorMaterial>>,
  q_window: Query<&Window, With<PrimaryWindow>>,
) {
  let rect = Rectangle::new(SQUARE_SIZE, SQUARE_SIZE);

  let window = q_window.single().unwrap();

  let vertical_capacity = (window.height() / SQUARE_SIZE) as usize;
  let horizontal_capacity = (window.width() / SQUARE_SIZE) as usize;

  let grid_center_x = horizontal_capacity.div_euclid(2);
  let grid_center_y = vertical_capacity.div_euclid(2);

  let anchor_404_x = grid_center_x as isize + TEXT_404_TOP_LEFT_X_OFFSET;
  let anchor_404_y = grid_center_y as isize + TEXT_404_TOP_LEFT_Y_OFFSET;

  let mut grid = Vec::<Vec<(bool, Handle<ColorMaterial>)>>::new();

  for i in 0..horizontal_capacity {
    grid.push(Vec::new());

    for j in 0..vertical_capacity {
      let spawn_x = (SQUARE_SIZE * i as f32) - HALF_SQUARE_SIZE;
      let spawn_y = window.height() - (SQUARE_SIZE * j as f32) - HALF_SQUARE_SIZE;

      let is_alive = rand::random_bool(0.2); // Randomly decide if the cell is alive or dead

      if text_404_mask(anchor_404_x as usize, anchor_404_y as usize, i, j)
        || (is_alive && !is_near_text_404(anchor_404_x as usize, anchor_404_y as usize, i, j))
      {
        let color_handle = materials.add(Color::from(WHITE));
        commands.spawn((Mesh2d(meshes.add(rect)), MeshMaterial2d(color_handle.clone()), Transform::from_xyz(spawn_x, spawn_y, 0f32)));
        grid[i].push((true, color_handle));
      } else {
        let color_handle = materials.add(Color::from(BLACK));
        commands.spawn((Mesh2d(meshes.add(rect)), MeshMaterial2d(color_handle.clone()), Transform::from_xyz(spawn_x, spawn_y, 0f32)));
        grid[i].push((false, color_handle));
      }
    }
  }

  commands.insert_resource(GameGrid { grid: grid });
}

fn game_loop(
  time: Res<Time>,
  mut res_timer: ResMut<GameTimer>,
  mut res_user_timer: ResMut<WaitForUserTimer>,
  mut res_grid: ResMut<GameGrid>,
  mut res_materials: ResMut<Assets<ColorMaterial>>,
) {
  if !res_user_timer.0.finished() {
    res_user_timer.0.tick(time.delta());
    return;
  }

  let grid = &mut res_grid.grid;

  if res_timer.0.tick(time.delta()).just_finished() {
    let mut res = grid.clone();

    for (i, line) in grid.iter().enumerate() {
      for (j, cell) in line.iter().enumerate() {
        if cell.0 {
          let neighbours = counter_neighbours(&grid, i, j);
          if neighbours < 2 || neighbours > 3 {
            res[i][j].0 = false;
          }
        } else {
          let neighbours = counter_neighbours(&grid, i, j);
          if neighbours == 3 {
            res[i][j].0 = true;
          }
        }
      }
    }

    for (i, line) in res.iter().enumerate() {
      for (j, cell) in line.iter().enumerate() {
        grid[i][j].0 = cell.0;
        let color_mat = res_materials.get_mut(&grid[i][j].1).unwrap();
        // Update the color based on the cell state
        if cell.0 {
          color_mat.color = Color::from(WHITE);
        } else {
          color_mat.color = Color::from(BLACK);
        }
      }
    }
  }
}

fn counter_neighbours(grid: &Grid, x: usize, y: usize) -> u8 {
  let indexes: [(isize, isize); 8] = [(-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, -1), (1, 0), (1, 1)];

  fn is_alive(grid: &Grid, x: &usize, y: &usize) -> bool {
    grid[*x][*y].0
  }
  fn is_valid_index((x, y): &(isize, isize), grid: &Grid) -> bool {
    *x >= 0_isize && *x < grid.len() as isize && *y >= 0_isize && *y < grid[0].len() as isize
  }
  indexes
    .into_iter()
    .map(|(tx, ty)| (x as isize + tx, y as isize + ty))
    .filter(|it| is_valid_index(it, grid))
    .map(|(x, y)| (x as usize, y as usize))
    .filter(|(x, y)| is_alive(grid, x, y))
    .count() as u8
}
