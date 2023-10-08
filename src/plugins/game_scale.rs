use crate::systems::window_resolution::current_window_resolution;
use bevy::prelude::{App, Plugin, Update};

pub struct GameScalePlugin;

impl Plugin for GameScalePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, current_window_resolution);
    }
}
