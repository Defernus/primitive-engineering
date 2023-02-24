use crate::{
    plugins::{
        game_world::resources::{meta::GameWorldMeta, GameWorld},
        loading::resources::GameAssets,
        objects::resources::objects_registry::ObjectsRegistry,
        player::{
            components::{PlayerComponent, PlayerHand, PlayerHeadComponent},
            resources::PlayerStats,
        },
    },
    states::game_state::GameState,
};
use bevy::prelude::*;

pub fn world_loading_system(
    objects_registry: Res<ObjectsRegistry>,
    assets: Res<GameAssets>,
    mut commands: Commands,
    mut game_state: ResMut<State<GameState>>,

    meta: Res<GameWorldMeta>,
    mut player_q: Query<(&mut Transform, &mut PlayerComponent)>,
    player_hand_q: Query<Entity, With<PlayerHand>>,
    mut head_q: Query<&mut Transform, (With<PlayerHeadComponent>, Without<PlayerComponent>)>,
    mut player_stats: ResMut<PlayerStats>,
) {
    let world = GameWorld::new();
    commands.insert_resource(world);

    if let Some(player_save) = meta.load_player() {
        let player = player_q.single_mut();
        let mut head = head_q.single_mut();
        let hand = player_hand_q.single();

        player_save.apply_to_player(
            &objects_registry,
            &assets,
            &mut commands,
            hand,
            player,
            &mut head,
            &mut player_stats,
        );
    } else {
        warn!("No player save found, creating new player");
    }

    game_state.set(GameState::InGame).unwrap();
}
