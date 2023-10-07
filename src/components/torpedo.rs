use bevy::{ecs::component::Component, time::Timer};

#[derive(Component)]
pub struct Torpedo {
    pub speed: f32,
    pub lifetime: Timer,
}
