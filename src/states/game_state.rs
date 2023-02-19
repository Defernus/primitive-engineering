use bevy::reflect::Reflect;

#[derive(Debug, Clone, Eq, PartialEq, Hash, Reflect)]
#[derive(Default)]
pub enum GameState {
    #[default]
    AssetsLoading,

    MenuMain,
    MenuNewGame,
    MenuLoadGame,
    MenuSettings,

    WorldLoading,
    WorldCreating,

    InGame,
}


