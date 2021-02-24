mod components;

mod prelude {
    pub use crate::components::*;
    pub use bevy::prelude::*;
}

use prelude::*;

fn setup(
    commands: &mut Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands
        .spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Plane { size: 5.0 })),
            material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
            ..Default::default()
        })
        .spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Icosphere {
                radius: 1.0,
                subdivisions: 24,
            })),
            material: materials.add(Color::rgb(1.0, 0.2, 0.2).into()),
            ..Default::default()
        })
        .spawn(LightBundle {
            transform: Transform::from_translation(Vec3::new(4.0, 8.0, 4.0)),
            ..Default::default()
        })
        .spawn(Camera3dBundle {
            transform: Transform::from_translation(Vec3::new(-2.0, 2.5, 5.0))
                .looking_at(Vec3::default(), Vec3::unit_y()),
            ..Default::default()
        });
}

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup.system())
        .run();
}
