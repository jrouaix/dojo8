use bevy::prelude::*;
use game_of_life::GameOfLife;

mod consts;
pub mod game_of_life;

fn main() {
  App::new()
    .add_plugins((
      DefaultPlugins.build().set(WindowPlugin {
        primary_window: Some(Window {
          fit_canvas_to_parent: true,
          present_mode: bevy::window::PresentMode::AutoNoVsync,
          canvas: Some("#game_of_life".to_string()),
          title: "Game of Life".to_string(),
          ..Default::default()
        }),
        ..Default::default()
      }),
      GameOfLife,
    ))
    .run();
}
