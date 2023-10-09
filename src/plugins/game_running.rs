use crate::systems::{
    alien_laser_lifetime::alien_laser_lifetime, alien_laser_movement::alien_laser_movement,
    alien_movement::alien_movement, player_laser_lifetime::player_laser_lifetime,
    player_laser_movement::player_laser_movement, player_movement::player_movement,
    player_torpedo_lifetime::player_torpedo_lifetime,
    player_torpedo_movement::player_torpedo_movement, player_weapon_select::player_weapon_select,
    spawn_alien::spawn_alien, spawn_alien_laser::spawn_alien_laser,
    spawn_player_laser::spawn_player_laser, spawn_player_torpedo::spawn_player_torpedo,
};
use bevy::prelude::{App, Plugin, Update};

pub struct GameRunningPlugin;

impl Plugin for GameRunningPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, spawn_alien)
            .add_systems(Update, spawn_alien_laser)
            .add_systems(Update, alien_laser_movement)
            .add_systems(Update, alien_laser_lifetime)
            .add_systems(Update, alien_movement)
            .add_systems(Update, spawn_player_laser)
            .add_systems(Update, player_laser_movement)
            .add_systems(Update, player_laser_lifetime)
            .add_systems(Update, spawn_player_torpedo)
            .add_systems(Update, player_torpedo_movement)
            .add_systems(Update, player_torpedo_lifetime)
            .add_systems(Update, player_movement)
            .add_systems(Update, player_weapon_select);
    }
}
