use bevy::prelude::*;
use game_of_life::GameOfLife;

pub mod game_of_life;

fn main() {
  App::new()
    .add_plugins((
      DefaultPlugins
        .build()
        .set(WindowPlugin { primary_window: Some(Window { fit_canvas_to_parent: true, ..Default::default() }), ..Default::default() }),
      GameOfLife,
    ))
    .run();
}
