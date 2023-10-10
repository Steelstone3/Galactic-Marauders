use bevy::prelude::{info, Input, KeyCode, Res, ResMut};

use crate::resources::selected_weapon::SelectedWeapon;

pub fn player_weapon_select(
    input: Res<Input<KeyCode>>,
    mut selected_weapon: ResMut<SelectedWeapon>,
) {
    // Weapon 1
    if input.just_pressed(KeyCode::Key1) {
        selected_weapon.0 = 1;

        info!("Laser selected")
    }
    // Weapon 2
    else if input.just_pressed(KeyCode::Key2) {
        selected_weapon.0 = 2;

        info!("Torpedo selected")
    }
}
