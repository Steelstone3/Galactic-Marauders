use crate::systems::{
    mine_lifetime::mine_lifetime, player_movement::player_movement, spawn_mine::spawn_mine,
};
use bevy::prelude::{App, Plugin, Update};

pub struct GameRunningPlugin;

impl Plugin for GameRunningPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, player_movement)
            .add_systems(Update, spawn_mine)
            .add_systems(Update, mine_lifetime);
    }
}
