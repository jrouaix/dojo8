use bevy::{
  color::palettes::css::{BLACK, WHITE},
  prelude::*,
  window::PrimaryWindow,
};

pub struct GameOfLife;

impl Plugin for GameOfLife {
  fn build(&self, app: &mut App) {
    app
      .insert_resource(GameTimer(Timer::from_seconds(2.0, TimerMode::Repeating)))
      .add_systems(Startup, setup_camera)
      .add_systems(PostStartup, setup_entities)
      .add_systems(Update, game_loop);
  }
}

fn setup_camera(mut commands: Commands) {
  commands.spawn(Camera2d);
}

const SQUARE_SIZE: f32 = 40f32;
const HALF_SQUARE_SIZE: f32 = SQUARE_SIZE / 2f32;

pub type Grid = Vec<Vec<(bool, Handle<ColorMaterial>)>>;

#[derive(Resource, Default, Debug)]
struct GameGrid {
  grid: Grid,
}

#[derive(Component)]
struct CellCoords {
  x: usize,
  y: usize,
}

#[derive(Resource)]
struct GameTimer(Timer);

fn setup_entities(
  mut commands: Commands,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<ColorMaterial>>,
  q_camera: Query<(&Camera, &GlobalTransform)>,
  q_window: Query<&Window, With<PrimaryWindow>>,
) {
  let rect = Rectangle::new(SQUARE_SIZE, SQUARE_SIZE);

  // get the camera info and transform
  // assuming there is exactly one main camera entity, so Query::single() is OK
  let (camera, camera_transform) = q_camera.single().unwrap();

  let window = q_window.single().unwrap();

  let vertical_capacity = (window.height() / SQUARE_SIZE) as usize + 2;
  let horizontal_capacity = (window.width() / SQUARE_SIZE) as usize + 2;

  let mut grid = Vec::<Vec<(bool, Handle<ColorMaterial>)>>::new();

  for i in 0..horizontal_capacity {
    grid.push(Vec::new());

    for j in 0..vertical_capacity {
      let spawn_x = (SQUARE_SIZE * i as f32) - HALF_SQUARE_SIZE;
      let spawn_y = (SQUARE_SIZE * j as f32) - HALF_SQUARE_SIZE;

      match camera.viewport_to_world_2d(camera_transform, Vec2 { x: spawn_x, y: spawn_y }) {
        Ok(world_position) => {
          if [(11, 11), (12, 12), (13, 13), (11, 13)].contains(&(i, j)) {
            let color_handle = materials.add(Color::from(WHITE));

            commands.spawn((
              CellCoords { x: i, y: j },
              Mesh2d(meshes.add(rect)),
              MeshMaterial2d(color_handle.clone()),
              Transform::from_xyz(world_position.x, world_position.y, 0f32),
            ));

            grid[i].push((true, color_handle));
          } else {
            let color_handle = materials.add(Color::from(BLACK));

            commands.spawn((
              CellCoords { x: i, y: j },
              Mesh2d(meshes.add(rect)),
              MeshMaterial2d(color_handle.clone()),
              Transform::from_xyz(world_position.x, world_position.y, 0f32),
            ));

            grid[i].push((false, color_handle));
          }
        }
        Err(err) => println!("Err: {err}"),
      }
    }
  }

  commands.insert_resource(GameGrid { grid: grid });
}

fn game_loop(
  time: Res<Time>,
  mut commands: Commands,
  mut res_timer: ResMut<GameTimer>,
  mut res_grid: ResMut<GameGrid>,
  mut res_materials: ResMut<Assets<ColorMaterial>>,
) {
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
