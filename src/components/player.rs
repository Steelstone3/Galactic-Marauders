use bevy::ecs::component::Component;

#[derive(Component)]
pub struct Player {
    pub speed: f32,
}
