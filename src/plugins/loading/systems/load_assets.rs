use crate::plugins::loading::resources::{GameAssets, PhysicsObject};
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
            base_color: Color::rgb(1.0, 1.0, 1.0),
            perceptual_roughness: 1.,
            metallic: 0.,
            reflectance: 0.,
            ..default()
        }),
        craft_zone_material: materials.add(StandardMaterial {
            base_color: Color::rgba(1.0, 1.0, 1.0, 0.1),
            perceptual_roughness: 1.,
            metallic: 0.,
            alpha_mode: AlphaMode::Blend,
            reflectance: 0.,
            ..default()
        }),
        pointer_mesh: meshes.add(Mesh::from(shape::Cube { size: 0.2 })),
        craft_zone_mesh: meshes.add(Mesh::from(shape::Icosphere {
            radius: 0.05,
            subdivisions: 9,
        })),

        tree_object: load_scene_with_physics("models/tree.glb#Scene0", &asset_server),
        branch_object: load_scene_with_physics("models/branch.glb#Scene0", &asset_server),
        rock_object: load_scene_with_physics("models/rock.glb#Scene0", &asset_server),
        fire_object: load_scene_with_physics("models/fire.glb#Scene0", &asset_server),
        log_object: load_scene_with_physics("models/log.glb#Scene0", &asset_server),
        stump_object: load_scene_with_physics("models/stump.glb#Scene0", &asset_server),
        cactus_object: load_scene_with_physics("models/cactus.glb#Scene0", &asset_server),
        wooden_shovel_object: load_scene_with_physics(
            "models/wooden_shovel.glb#Scene0",
            &asset_server,
        ),
        spruce_object: load_scene_with_physics("models/spruce.glb#Scene0", &asset_server),
        spruce_snow_object: load_scene_with_physics("models/spruce-snow.glb#Scene0", &asset_server),
        flax_object: load_scene_with_physics("models/flax.glb#Scene0", &asset_server),
        flax_item_object: load_scene_with_physics("models/flax-item.glb#Scene0", &asset_server),
        stone_axe_object: load_scene_with_physics("models/stone-axe.glb#Scene0", &asset_server),
        coarse_string_object: load_scene_with_physics(
            "models/coarse-string.glb#Scene0",
            &asset_server,
        ),

        crosshair_image: asset_server.load("textures/crosshair.png"),
    };

    commands.insert_resource(game_assets);
}
