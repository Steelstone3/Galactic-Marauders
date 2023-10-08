use bevy::ecs::system::Resource;

#[derive(Resource, Default)]
pub struct Points(pub u32);
