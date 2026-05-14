use crate::menu::id as menu_ids;
use crate::menu::layout::MenuLayout;
use crate::menu::prelude::*;
use crate::menu::InputContext;
use bevy::prelude::*;
use bevy::ecs::message::Messages;
use crate::menu::prefabs::button::spawn_button_row;

pub fn spawn(
    commands: &mut Commands,
    _meshes: &mut Assets<Mesh>,
    _materials: &mut Assets<StandardMaterial>,
) -> Option<MenuLayout> {
    commands.spawn_menu((
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            flex_direction: FlexDirection::Column,
            row_gap: Val::Px(10.0),
            ..default()
        },
        BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.3)),
    ));

    commands.spawn_menu((
        Text::new("Would you like to continue?"),
        TextFont { font_size: 48.0, ..default() },
        TextColor(Color::WHITE),
    ));


    let ids = spawn_button_row(commands, &[
        ("Yes",),
        ("No",),
    ]);

    Some(MenuLayout::new().add_row(vec![
        (ids[0], |w: &mut World| {
            w.resource_mut::<Messages<crate::menu::ChangeMenu>>()
                .write(crate::menu::ChangeMenu(menu_ids::INGAME_UI));
        }),
        (ids[1], |_w: &mut World| {
            info!("Options not implemented");
        }),
    ]))
}

pub struct ChoiceUI;
impl Menu for ChoiceUI {
    fn id() -> MenuId { CHOICE_MENU }
    fn data() -> MenuData {
        MenuData {
            spawn,
            startup: vec![
                MenuStartup::SetGameState(GameState::Paused),
                MenuStartup::SetInputContext(InputContext::Menu),
            ],
        }
    }
}
