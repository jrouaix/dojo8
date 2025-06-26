use bevy::prelude::*;
use game_of_life::GameOfLife;

mod consts;
pub mod game_of_life;

fn main() {
  App::new()
    .add_plugins((
      DefaultPlugins.build().set(WindowPlugin {
        primary_window: Some(Window {
          resizable: false,
          fit_canvas_to_parent: true,
          mode: bevy::window::WindowMode::BorderlessFullscreen(MonitorSelection::Primary),
          ..Default::default()
        }),
        ..Default::default()
      }),
      GameOfLife,
    ))
    .run();
}
