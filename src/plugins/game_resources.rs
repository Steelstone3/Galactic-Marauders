use crate::resources::{
    enemy_count::EnemyCount, enemy_limit::EnemyLimit, laser_ammunition::LaserAmmunition,
    points::Points, selected_weapon::SelectedWeapon, torpedo_ammunition::TorpedoAmmunition,
    window_size::WindowSize,
};
use bevy::prelude::{App, Plugin};

pub struct GameResourcesPlugin;

impl Plugin for GameResourcesPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(WindowSize::default())
            .insert_resource(Points::default())
            .insert_resource(LaserAmmunition(10.0))
            .insert_resource(TorpedoAmmunition(3.0))
            .insert_resource(SelectedWeapon(1))
            .insert_resource(EnemyCount(0))
            .insert_resource(EnemyLimit(1));
    }
}
