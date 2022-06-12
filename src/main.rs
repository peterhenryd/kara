use bevy::prelude::*;

mod infra;
mod world;
mod camera;

fn main() {
    App::new()
        .add_plugin(infra::InfraPlugin)
        .add_plugin(camera::CameraPlugin)
        .add_plugin(world::WorldPlugin)
        .run()
}

