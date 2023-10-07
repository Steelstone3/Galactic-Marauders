use crate::resources::{
    laser_ammunition::LaserAmmunition, points::Points, selected_weapon::SelectedWeapon,
    torpedo_ammunition::TorpedoAmmunition,
};
use bevy::prelude::{App, Plugin};

pub struct GameResourcesPlugin;

impl Plugin for GameResourcesPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Points(0))
            .insert_resource(LaserAmmunition(50.0))
            .insert_resource(TorpedoAmmunition(10.0))
            .insert_resource(SelectedWeapon(1));
    }
}
