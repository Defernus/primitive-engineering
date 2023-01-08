use self::systems::{
    main_menu,
    new_game::{init_new_game, new_game},
};
use crate::states::game_state::GameState;
use bevy::prelude::*;

mod systems;

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_update(GameState::MenuMain).with_system(main_menu))
            .add_system_set(SystemSet::on_enter(GameState::MenuNewGame).with_system(init_new_game))
            .add_system_set(SystemSet::on_update(GameState::MenuNewGame).with_system(new_game));
    }
}
