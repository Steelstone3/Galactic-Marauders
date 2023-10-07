use bevy::{ecs::component::Component, time::Timer};

#[derive(Component)]
pub struct Mine {
    pub speed: f32,
    pub lifetime: Timer,
}
