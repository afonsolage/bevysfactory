use bevy::{
    prelude::*,
    render::{
        settings::{Backends, WgpuSettings},
        RenderPlugin,
    },
};
use bevy_editor_pls::EditorPlugin;
use bevy_xpbd_3d::prelude::*;
use fly_by_cam::{FlyByCamera, FlyByCameraPlugin};
use map::MapPlugin;
use resources::{ResourceNode, ResourcePath, ResourcesPlugin};

pub mod fly_by_cam;
pub mod map;
pub mod resources;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(RenderPlugin {
            wgpu_settings: WgpuSettings {
                backends: Some(Backends::VULKAN),
                ..default()
            },
        }))
        .init_resource::<LastNode>()
        .add_plugins(PhysicsPlugins::default())
        .add_plugins(EditorPlugin::default())
        .add_plugins(FlyByCameraPlugin)
        .add_plugins((ResourcesPlugin, MapPlugin))
        .add_systems(Startup, setup)
        .add_systems(Update, (bevy::window::close_on_esc, on_ground_click))
        .run();
}

/// set up a simple 3D scene
fn setup(mut commands: Commands) {
    // camera
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        FlyByCamera,
        Name::new("Main Camera"),
    ));

    // Spawn a simple and fake crosshair
    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent.spawn(NodeBundle {
                style: Style {
                    width: Val::Px(8.0),
                    height: Val::Px(8.0),
                    border: UiRect::all(Val::Px(1.0)),
                    // horizontally center child text
                    justify_content: JustifyContent::Center,
                    // vertically center child text
                    align_items: AlignItems::Center,
                    ..default()
                },
                border_color: BorderColor(Color::BLACK),
                background_color: Color::rgba(1.0, 1.0, 1.0, 0.1).into(),
                ..default()
            });
        });
}

#[derive(Component)]
pub struct CameraRayCaster;

#[derive(Resource, Default)]
pub struct LastNode(Option<Entity>);

fn on_ground_click(
    mouse_btn: Res<Input<MouseButton>>,
    q_camera: Query<&Transform, With<FlyByCamera>>,
    spatial_query: SpatialQuery,
    q_transforms: Query<&Transform, With<ResourceNode>>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut last_node_res: ResMut<LastNode>,
) {
    if mouse_btn.just_pressed(MouseButton::Left) {
        if let Ok(camera) = q_camera.get_single() {
            if let Some(first_hit) = spatial_query.cast_ray(
                camera.translation,
                camera.forward(),
                1000.0,
                true,
                SpatialQueryFilter::default(),
            ) {
                let spawn_location =
                    camera.translation + camera.forward() * first_hit.time_of_impact;

                let new_node_translation = spawn_location + Vec3::new(0.0, 0.5, 0.0);

                let entity = commands
                    .spawn((
                        PbrBundle {
                            mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
                            material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
                            transform: Transform::from_translation(new_node_translation),
                            ..default()
                        },
                        ResourceNode,
                    ))
                    .id();

                if let Some(last_node) = last_node_res.0.replace(entity) {
                    if let Ok(last_node_transform) = q_transforms.get(last_node) {
                        commands.spawn(ResourcePath(
                            last_node_transform.translation,
                            new_node_translation,
                        ));
                    }
                }
            }
        } else {
            println!("No camera found!");
        }
    }
}
