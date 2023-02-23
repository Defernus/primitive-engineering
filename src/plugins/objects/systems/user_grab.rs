use crate::plugins::{
    game_world::resources::GameWorld,
    loading::resources::GameAssets,
    objects::components::{
        items::{drop_item, grab_item, ItemComponent, ItemGrabbed},
        GameWorldObject,
    },
    player::{components::PlayerHand, events::UseGrabPlaceEvent, resources::look_at::PlayerLookAt},
};
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

#[allow(clippy::too_many_arguments)]
pub fn use_grab_system(
    mut use_place_grab_e: EventReader<UseGrabPlaceEvent>,
    player_hand_q: Query<(Entity, &GlobalTransform), With<PlayerHand>>,
    mut commands: Commands,
    mut item_grabbed_q: Query<(Entity, &mut GameWorldObject), With<ItemGrabbed>>,
    item_q: Query<Entity, (With<ItemComponent>, Without<ItemGrabbed>)>,
    mut object_q: Query<(&mut GameWorldObject, &GlobalTransform), Without<ItemGrabbed>>,
    colliders_q: Query<&Parent, With<Collider>>,
    mut world: ResMut<GameWorld>,
    look_at: Res<PlayerLookAt>,
    assets: Res<GameAssets>,
) {
    for _ in use_place_grab_e.iter() {
        let look_at = look_at
            .target
            .map(|entity| match colliders_q.get(entity) {
                Ok(parent) => Some(parent.get()),
                Err(_) => None,
            })
            .flatten();

        let mut hand_item = item_grabbed_q.iter_mut().next();

        // try to use object
        if let Some((obj, obj_entity)) = look_at
            .map(|entity| Some((object_q.get_mut(entity).ok()?, entity)))
            .flatten()
        {
            let (mut obj, transform) = obj;

            if obj.0.on_use(
                &mut commands,
                &assets,
                obj_entity,
                transform.compute_transform(),
                &mut hand_item,
            ) {
                continue;
            }
        }

        // try to drop item from hand
        if let Some((item_entity, _)) = hand_item {
            let (_, transform) = player_hand_q.single();

            drop_item(
                commands.entity(item_entity),
                transform.compute_transform(),
                &mut world,
            );
        }

        // try to grab item
        if let Some(entity) = look_at {
            if let Ok(item) = item_q.get(entity) {
                let (hand, _) = player_hand_q.single();
                grab_item(commands.entity(item), hand);
            }
        }
    }
}
