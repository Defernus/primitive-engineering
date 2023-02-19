use crate::plugins::{
    game_world::resources::GameWorld,
    objects::components::items::{drop_item, grab_item, ItemComponent, ItemGrabbed},
    player::{components::PlayerHand, events::UseGrabPlaceEvent, resources::look_at::PlayerLookAt},
};
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

#[allow(clippy::too_many_arguments)]
pub fn grab(
    mut use_place_grab_e: EventReader<UseGrabPlaceEvent>,
    player_hand_q: Query<(Entity, &GlobalTransform), With<PlayerHand>>,
    mut commands: Commands,
    mut item_grabbed_q: Query<Entity, With<ItemGrabbed>>,
    item_q: Query<Entity, (With<ItemComponent>, Without<ItemGrabbed>)>,
    colliders_q: Query<&Parent, With<Collider>>,
    mut world: ResMut<GameWorld>,
    look_at: Res<PlayerLookAt>,
) {
    for _ in use_place_grab_e.iter() {
        for item in item_grabbed_q.iter_mut() {
            let (_, transform) = player_hand_q.single();
            drop_item(
                commands.entity(item),
                transform.compute_transform(),
                &mut world,
            );
        }

        if let Some(entity) = look_at.target {
            let parent = match colliders_q.get(entity) {
                Ok(parent) => parent,
                Err(_) => continue,
            };

            if let Ok(item) = item_q.get(parent.get()) {
                let (hand, _) = player_hand_q.single();
                grab_item(commands.entity(item), hand);
            }
        }
    }
}
