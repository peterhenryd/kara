use bevy::prelude::*;
use crate::App;

pub struct InfraPlugin;

impl Plugin for InfraPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(
            WindowDescriptor {
                title: "Kara".to_string(),
                width: 640.0,
                height: 480.0,
                ..Default::default()
            })
            .add_plugins(DefaultPlugins)
            .add_system(cursor_lock);
    }
}

fn cursor_lock(
    mut windows: ResMut<Windows>,
    btn: Res<Input<MouseButton>>,
    key: Res<Input<KeyCode>>,
) {
    let window = windows.get_primary_mut().unwrap();

    if btn.just_pressed(MouseButton::Left) {
        window.set_cursor_lock_mode(true);
        window.set_cursor_visibility(false);
    }

    if key.just_pressed(KeyCode::Escape) {
        window.set_cursor_lock_mode(false);
        window.set_cursor_visibility(true);
    }
}