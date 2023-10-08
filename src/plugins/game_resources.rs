use crate::resources::{
    laser_ammunition::LaserAmmunition, points::Points, selected_weapon::SelectedWeapon,
    torpedo_ammunition::TorpedoAmmunition, window_size::WindowSize,
};
use bevy::prelude::{App, Plugin};

pub struct GameResourcesPlugin;

impl Plugin for GameResourcesPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(WindowSize::default())
            .insert_resource(Points::default())
            .insert_resource(LaserAmmunition(50.0))
            .insert_resource(TorpedoAmmunition(10.0))
            .insert_resource(SelectedWeapon(1));
    }
}
