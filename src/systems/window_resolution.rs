use bevy::{
    prelude::{Query, ResMut, With},
    window::{PrimaryWindow, Window},
};

use crate::resources::window_size::WindowSize;

pub fn current_window_resolution(
    mut window_size: ResMut<WindowSize>,
    window: Query<&Window, With<PrimaryWindow>>,
) {
    let primary = window.get_single().unwrap();

    window_size.width = primary.width();

    window_size.height = primary.height();
}
