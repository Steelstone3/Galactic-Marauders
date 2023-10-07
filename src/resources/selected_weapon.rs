use bevy::ecs::system::Resource;

#[derive(Resource)]
pub struct SelectedWeapon(pub u32);
