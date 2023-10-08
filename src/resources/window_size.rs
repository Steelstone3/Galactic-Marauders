use bevy::ecs::system::Resource;

#[derive(Resource, Default)]
pub struct WindowSize {
    pub width: f32,
    pub height: f32,
}
