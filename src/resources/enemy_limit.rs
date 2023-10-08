use bevy::ecs::system::Resource;

#[derive(Resource)]
pub struct EnemyLimit(pub u32);
