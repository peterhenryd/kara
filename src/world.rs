use bevy::prelude::*;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_world);
    }
}

pub struct World<'a, 'w, 's> {
    commands: Commands<'w, 's>,
    meshes: ResMut<'a, Assets<Mesh>>,
    materials: ResMut<'a, Assets<StandardMaterial>>
}

impl<'a, 'w, 's> World<'a, 'w, 's> {
    pub fn set_block(&mut self, color: Color, translation: Vec3) {
        self.commands.spawn_bundle(PbrBundle {
            mesh: self.meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
            material: self.materials.add(color.into()),
            transform: Transform::from_translation(translation),
            ..default()
        });
    }
}

fn spawn_world(
    mut commands: Commands,
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn_bundle(PointLightBundle {
        transform: Transform::from_xyz(4.0, 10.0, 4.0),
        point_light: PointLight {
            intensity: 9000.0,
            shadows_enabled: true,
            range: 30.0,
            ..default()
        },
        ..default()
    });

    let mut world = World { commands, meshes, materials };

    world.set_block(Color::LIME_GREEN, Vec3::new(0.0, 0.0, 0.0));
}