use bevy::{prelude::*, window::PresentMode};
use game_of_life::GameOfLife;

mod consts;
pub mod game_of_life;

fn main() {
  App::new()
    .add_plugins((
      DefaultPlugins.build().set(WindowPlugin {
        primary_window: Some(Window {
          present_mode: PresentMode::AutoVsync,
          // Tells Wasm to resize the window according to the available canvas
          fit_canvas_to_parent: true,
          // Tells Wasm not to override default event handling, like F5, Ctrl+R etc.
          prevent_default_event_handling: false,
          ..Default::default()
        }),
        ..Default::default()
      }),
      GameOfLife,
    ))
    .run();
}
