pub use super::id::*;
pub use super::{
    ChangeMenu, GameState, Menu, MenuData, MenuEntity, MenuId, MenuStartup, SpawnMenuEntity,
};
use bevy::prelude::*;

pub struct MenuPrefabsPlugin;

impl Plugin for MenuPrefabsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<super::layout::MenuLayout>();
    }
}
