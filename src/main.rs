use bevy::prelude::*;
use plugins::{
    game_resources::GameResourcesPlugin, game_running::GameRunningPlugin,
    game_scale::GameScalePlugin, game_start::GameStartPlugin,
};

mod components;
mod plugins;
mod resources;
mod systems;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Galactic Marauders".to_string(),
                        resolution: (640.0, 480.0).into(),
                        ..Default::default()
                    }),
                    ..Default::default()
                }),
            GameResourcesPlugin,
            GameStartPlugin,
            GameRunningPlugin,
            GameScalePlugin,
        ))
        .run();
}
