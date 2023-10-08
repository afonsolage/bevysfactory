use bevy::prelude::*;

pub struct ResourcesPlugin;

impl Plugin for ResourcesPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_resources)
            .add_systems(Update, move_resources);
    }
}

#[derive(Component)]
pub struct ResourceItem;

fn spawn_resources(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // cube
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 0.1 })),
            material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
            transform: Transform::from_xyz(0.0, 1.0, 0.0),
            ..default()
        },
        ResourceItem,
    ));
}

fn move_resources(mut q: Query<&mut Transform, With<ResourceItem>>, time: Res<Time>) {
    for mut transform in q.iter_mut() {
        transform.translation += Vec3::new(0.1, 0.0, 0.0) * time.delta_seconds();
    }
}
