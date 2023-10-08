use bevy::ecs::system::Resource;

#[derive(Resource)]
pub struct EnemyCount(pub u32);
