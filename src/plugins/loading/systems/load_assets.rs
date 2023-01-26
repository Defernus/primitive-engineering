use crate::plugins::{
    craft::resources::CRAFT_ZONE_RADIUS,
    loading::resources::{GameAssets, PhysicsObject},
};
use bevy::{asset::AssetPath, prelude::*};

fn load_scene_with_physics<'a>(
    path: impl Into<AssetPath<'a>>,
    asset_server: &AssetServer,
) -> PhysicsObject {
    let scene_h: Handle<Scene> = asset_server.load(path);

    PhysicsObject {
        scene: scene_h,
        ..default()
    }
}

pub fn load_assets(
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut commands: Commands,
) {
    let game_assets = GameAssets {
        main_font: asset_server.load("fonts/roboto.ttf"),
        default_material: materials.add(StandardMaterial {
            base_color: Color::rgb(1.0, 1.0, 1.0).into(),
            perceptual_roughness: 1.,
            metallic: 0.,
            reflectance: 0.,
            ..default()
        }),
        craft_zone_material: materials.add(StandardMaterial {
            base_color: Color::rgba(1.0, 1.0, 1.0, 0.1).into(),
            perceptual_roughness: 1.,
            metallic: 0.,
            alpha_mode: AlphaMode::Blend,
            reflectance: 0.,
            ..default()
        }),
        pointer_mesh: meshes.add(Mesh::from(shape::Cube { size: 0.2 })),
        craft_zone_mesh: meshes.add(Mesh::from(shape::Icosphere {
            radius: CRAFT_ZONE_RADIUS,
            subdivisions: 9,
        })),

        tree_object: load_scene_with_physics("models/tree.glb#Scene0", &asset_server),
        branch_object: load_scene_with_physics("models/branch.glb#Scene0", &asset_server),
        rock_object: load_scene_with_physics("models/rock.glb#Scene0", &asset_server),
        fire_object: load_scene_with_physics("models/fire.glb#Scene0", &asset_server),

        crosshair_image: asset_server.load("textures/crosshair.png"),
    };

    commands.insert_resource(game_assets);
}
