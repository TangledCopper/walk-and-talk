use crate::menu::prelude::*;
use crate::menu::layout::MenuLayout;
use crate::menu::InputContext;
use bevy::prelude::*;

// small template to start from blank
pub fn spawn(
    _commands: &mut Commands,
    _meshes: &mut Assets<Mesh>,
    _materials: &mut Assets<StandardMaterial>,
) -> Option<MenuLayout> {
    None
}

pub struct NoMenu;
impl Menu for NoMenu {
    fn id() -> MenuId { NO_MENU }
    fn data() -> MenuData {
        MenuData {
            spawn,
            startup: vec![
                MenuStartup::SetInputContext(InputContext::Game),
            ],
        }
    }
}
