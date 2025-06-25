use bevy::{prelude::*, window::PrimaryWindow};
pub struct HelloPlugin;

impl Plugin for HelloPlugin {
  fn build(&self, app: &mut App) {
    app.add_systems(Startup, (setup_camera, setup_entities).chain());
  }
}

fn setup_camera(mut commands: Commands) {
  commands.spawn(Ortho::new_2d());
}

fn setup_entities(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<ColorMaterial>>) {
  let rect = Rectangle::new(25f32, 25f32);
  let red = Color::linear_rgb(1f32, 0f32, 0f32);

  commands.spawn((Mesh2d(meshes.add(rect)), MeshMaterial2d(materials.add(red)), Transform::from_translation(coords)));
}
