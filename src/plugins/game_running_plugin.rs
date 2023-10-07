use crate::systems::{
    laser_lifetime::laser_lifetime, player_movement::player_movement, spawn_laser::spawn_laser,
    spawn_torpedo::spawn_torpedo, torpedo_lifetime::torpedo_lifetime,
};
use bevy::prelude::{App, Plugin, Update};

pub struct GameRunningPlugin;

impl Plugin for GameRunningPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, player_movement)
            .add_systems(Update, spawn_torpedo)
            .add_systems(Update, torpedo_lifetime)
            .add_systems(Update, spawn_laser)
            .add_systems(Update, laser_lifetime);
    }
}
