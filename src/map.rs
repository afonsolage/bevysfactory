use bevy::{pbr::CascadeShadowConfigBuilder, prelude::*};
use bevy_xpbd_3d::prelude::*;
use rand::prelude::*;

use crate::resources::ResourceSpot;

pub const MAP_SIZE: u32 = 50;
pub const RESOURCE_SPOT_RADIUS_MIN: f32 = 1.0;
pub const RESOURCE_SPOT_RADIUS_MAX: f32 = 4.0;
pub const RESOURCE_SPOT_MIN_COUNT: u32 = 1;
pub const RESOURCE_SPOT_MAX_COUNT: u32 = 4;

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (setup_flat_map, spawn_node_spots));
    }
}

fn setup_flat_map(
    mut commands: Commands,
    mut std_materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    // ground plane
    let ground_mesh = shape::Plane::from_size(MAP_SIZE as f32).into();
    commands.spawn((
        RigidBody::Static,
        Collider::trimesh_from_bevy_mesh(&ground_mesh).expect("Valid plane mesh"),
        PbrBundle {
            mesh: meshes.add(ground_mesh),
            material: std_materials.add(StandardMaterial {
                base_color: Color::rgb(0.1, 0.7, 0.1).into(),
                metallic: 0.0,
                perceptual_roughness: 0.8,
                ..default()
            }),
            ..default()
        },
    ));

    // center marker
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cylinder {
            height: 0.2,
            radius: 0.1,
            resolution: 3,
            segments: 1,
        })),
        material: std_materials.add(Color::rgb(0.5, 0.1, 0.1).into()),
        transform: Transform::from_xyz(0.0, 0.1, 0.0),
        ..default()
    });

    // directional 'sun' light
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            shadows_enabled: true,
            ..default()
        },
        transform: Transform {
            translation: Vec3::new(0.0, 2.0, 0.0),
            rotation: Quat::from_rotation_x(-std::f32::consts::PI / 4.),
            ..default()
        },
        // The default cascade config is designed to handle large scenes.
        // As this example has a much smaller world, we can tighten the shadow
        // bounds for better visual quality.
        cascade_shadow_config: CascadeShadowConfigBuilder {
            first_cascade_far_bound: 4.0,
            maximum_distance: 100.0,
            ..default()
        }
        .into(),
        ..default()
    });

    // ambient light
    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 0.05,
    });
}

fn spawn_node_spots(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut std_materials: ResMut<Assets<StandardMaterial>>,
) {
    let mut rnd = rand::thread_rng();
    let map_range = -(MAP_SIZE as f32 / 2.0)..(MAP_SIZE / 2) as f32;
    for _ in RESOURCE_SPOT_MIN_COUNT..=RESOURCE_SPOT_MAX_COUNT {
        let radius = rnd.gen_range(RESOURCE_SPOT_RADIUS_MIN..=RESOURCE_SPOT_RADIUS_MAX);
        let mesh = shape::Circle::new(radius).into();
        let x = rnd.gen_range(map_range.clone());
        let z = rnd.gen_range(map_range.clone());
        commands.spawn((
            RigidBody::Static,
            Collider::trimesh_from_bevy_mesh(&mesh).expect("Valid plane mesh"),
            PbrBundle {
                mesh: meshes.add(mesh),
                material: std_materials.add(StandardMaterial {
                    base_color: Color::rgb(0.3, 0.4, 0.3).into(),
                    metallic: 1.0,
                    perceptual_roughness: 0.0,
                    ..default()
                }),
                transform: Transform::from_xyz(x, 0.01, z)
                    .with_rotation(Quat::from_axis_angle(Vec3::X, -std::f32::consts::FRAC_PI_2)),
                ..default()
            },
            ResourceSpot::STONE,
        ));
    }
}
