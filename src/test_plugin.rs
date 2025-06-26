use bevy::{
  color::palettes::css::{BLACK, WHITE},
  prelude::*,
  window::PrimaryWindow,
};
pub struct HelloPlugin;

impl Plugin for HelloPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_systems(Startup, (setup_camera).chain())
      .add_systems(PostStartup, (setup_entities).chain());
  }
}

fn setup_camera(mut commands: Commands) {
  commands.spawn(Camera2d);
}

const SQUARE_SIZE: f32 = 50f32;
const HALF_SQUARE_SIZE: f32 = SQUARE_SIZE / 2f32;

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

  for i in 0..horizontal_capacity {
    for j in 0..vertical_capacity {
      let spawn_x = (SQUARE_SIZE * i as f32) - HALF_SQUARE_SIZE;
      let spawn_y = (SQUARE_SIZE * j as f32) - HALF_SQUARE_SIZE;

      let is_white = (i % 2 == 0 && j % 2 == 0) || (i % 2 == 1 && j % 2 == 1);

      match camera.viewport_to_world_2d(camera_transform, Vec2 { x: spawn_x, y: spawn_y }) {
        Ok(world_position) => {
          commands.spawn((
            Mesh2d(meshes.add(rect)),
            MeshMaterial2d(materials.add(if is_white { Color::from(BLACK) } else { Color::from(WHITE) })),
            Transform::from_xyz(world_position.x, world_position.y, 0f32),
          ));
        }
        Err(err) => println!("Err: {err}"),
      }
    }
  }
}
