use crate::scenes::prelude::*;
use bevy::prelude::*;
use crate::menu::id as menu_ids;
use crate::scenes::id; 
use crate::menu::GameState;

// small template to start from blank
pub fn spawn(
    commands: &mut Commands,
    _meshes: &mut Assets<Mesh>,
    _materials: &mut Assets<StandardMaterial>,
) {
    commands.spawn_level(Camera2d);
}

pub struct Blank;
impl Level for Blank {
    fn id() -> LevelId { id::BLANK }
    fn data() -> LevelData { 
        LevelData {
        spawn,
        startup: vec![
            LevelStartup::SetGameState(GameState::Paused),
            LevelStartup::ShowMenu(menu_ids::MAIN_MENU),
        ],
    }

    }
}
