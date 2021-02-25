mod components;

mod prelude {
    pub use crate::components::*;
    pub use bevy::prelude::*;
}

use prelude::*;

fn player_input(
    keyboard_input: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut players: Query<&mut Velocity, With<Player>>,
) {
    let delta = time.delta_seconds();
    for mut velocity in players.iter_mut() {
        keyboard_input.get_pressed().for_each(|key| match key {
            KeyCode::W => velocity.0.x += 0.2 * delta,
            KeyCode::S => velocity.0.x -= 0.2 * delta,
            KeyCode::A => velocity.0.z -= 0.2 * delta,
            KeyCode::D => velocity.0.z += 0.2 * delta,
            _ => {}
        });
    }
}

fn movement(mut physics_objects: Query<(&mut Transform, &Velocity)>) {
    physics_objects
        .iter_mut()
        .for_each(|(mut transform, velocity)| {
            transform.translation += velocity.0;
        })
}

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
        .with(Player)
        .with(Velocity(Vec3::new(0.0, 0.0, 0.0)))
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
        .add_system(player_input.system())
        .add_system(movement.system())
        .run();
}
