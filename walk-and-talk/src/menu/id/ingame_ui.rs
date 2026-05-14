use crate::menu::prelude::*;
use crate::menu::layout::MenuLayout;
use crate::menu::InputContext;
use bevy::prelude::*;

pub fn spawn(
    commands: &mut Commands,
    _meshes: &mut Assets<Mesh>,
    _materials: &mut Assets<StandardMaterial>,
) -> Option<MenuLayout> {
    commands.spawn_menu((
        Text::new("This is\ntext with\nline breaks\nin the top left."),
        TextFont { font_size: 25.0, ..default() },
    ));
    None
}

pub struct IngameUI;
impl Menu for IngameUI {
    fn id() -> MenuId { INGAME_UI }
    fn data() -> MenuData {
        MenuData {
            spawn,
            startup: vec![
                MenuStartup::SetInputContext(InputContext::Game),
                MenuStartup::SetGameState(GameState::Playing),
            ],
        }
    }
}
