use bevy::reflect::Reflect;

#[derive(Debug, Clone, Eq, PartialEq, Hash, Reflect)]
pub enum GameState {
    AssetsLoading,

    MenuMain,
    MenuNewGame,
    MenuLoadGame,
    MenuSettings,

    WorldLoading,
    WorldCreating,

    InGame,
}

impl Default for GameState {
    fn default() -> Self {
        GameState::AssetsLoading
    }
}
