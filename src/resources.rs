use bevy::prelude::*;
use rand::prelude::*;

pub struct ResourcesPlugin;

impl Plugin for ResourcesPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (spawn_resources, move_resources, despawn_resources));
    }
}

#[derive(Component)]
pub struct ResourcePath(pub Vec3, pub Vec3);

impl ResourcePath {
    pub fn dir(&self) -> Vec3 {
        (self.1 - self.0).normalize()
    }

    pub fn target(&self) -> Vec3 {
        self.1
    }
}

#[derive(Component)]
pub struct ResourceNode;

#[derive(Component)]
pub struct ResourceItem(Entity);

fn spawn_resources(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    q_new_paths: Query<(Entity, &ResourcePath), Added<ResourcePath>>,
) {
    let mut rng = thread_rng();
    let r = rng.gen_range(0.0..1.0);
    let g = rng.gen_range(0.0..1.0);
    let b = rng.gen_range(0.0..1.0);

    for (path_entity, path) in &q_new_paths {
        commands.spawn((
            PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Cube { size: 0.1 })),
                material: materials.add(Color::rgb(r, g, b).into()),
                transform: Transform::from_translation(path.0),
                ..default()
            },
            ResourceItem(path_entity.clone()),
        ));
    }
}

fn move_resources(
    mut q: Query<(&mut Transform, &ResourceItem), With<ResourceItem>>,
    q_paths: Query<&ResourcePath>,
    time: Res<Time>,
) {
    for (mut transform, resource_item) in q.iter_mut() {
        if let Ok(path) = q_paths.get(resource_item.0) {
            transform.translation += path.dir() * time.delta_seconds();
        }
    }
}

fn despawn_resources(
    mut commands: Commands,
    q_paths: Query<&ResourcePath>,
    q_resources: Query<(Entity, &Transform, &ResourceItem), With<ResourceItem>>,
) {
    for (entity, transform, item) in &q_resources {
        if let Ok(path) = q_paths.get(item.0) {
            if transform.translation.distance(path.target()).abs() > 0.1 {
                // Doesn't despawm
                continue;
            }
        }

        commands.entity(entity).despawn();
    }
}
