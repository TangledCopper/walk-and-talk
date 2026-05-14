use crate::menu::prelude::*;
use crate::menu::layout::MenuLayout;
use crate::menu::InputContext;
use bevy::prelude::*;
use crate::menu::id as menu_ids;
use bevy::ecs::message::Messages;

// Yes, I'm aware there's not a super sophisticated dialogue tree system, but I felt like those
// aspects vary wildly from game to game. This is simply to get soemthing working in your own game
// and get moving on how you would like to design it. 
pub fn spawn(
    commands: &mut Commands,
    _meshes: &mut Assets<Mesh>,
    _materials: &mut Assets<StandardMaterial>,
) -> Option<MenuLayout> {

    let mut btn = Entity::PLACEHOLDER;

    commands.spawn_menu((
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::FlexEnd,
            align_items: AlignItems::Center,
            ..default()
        },
    )).with_children(|overlay| {
        overlay.spawn((
            Node {
                width: Val::Percent(100.0),
                max_width: Val::Vw(80.0),
                margin: UiRect::bottom(Val::Vh(4.0)),
                flex_direction: FlexDirection::Column,
                border: UiRect::all(Val::Px(2.0)),
                padding: UiRect::axes(Val::Vw(3.0), Val::Vh(2.0)),
                row_gap: Val::Vh(1.5),
                ..default()
            },
            BackgroundColor(Color::srgb(0.1, 0.1, 0.15)),
            BorderColor::all(Color::srgb(0.3, 0.3, 0.4)),
        )).with_children(|box_parent| {
            box_parent.spawn((
                Text::new("Welcome to walk-and-talk! This dialogue box here is to test the wrapping of the text in the dialogue menu. How did I do?"),
                TextFont { font_size: 20.0, ..default() },
                TextColor(Color::srgb(0.9, 0.9, 0.9)),
                TextLayout::new_with_linebreak(LineBreak::WordBoundary),
            ));

            btn = box_parent.spawn((
                Button,
                Node {
                    width: Val::Px(200.0),
                    height: Val::Px(60.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    align_self: AlignSelf::FlexEnd,
                    ..default()
                },
                BackgroundColor(Color::srgb(0.15, 0.15, 0.25)),
            )).with_children(|btn_parent| {
                btn_parent.spawn((
                    Text::new("Resume"),
                    TextFont { font_size: 24.0, ..default() },
                    TextColor(Color::WHITE),
                ));
            }).id();
        });
    });

    Some(MenuLayout::new().add_column(vec![
        (btn, |w: &mut World| {
            w.resource_mut::<Messages<crate::menu::ChangeMenu>>()
                .write(crate::menu::ChangeMenu(menu_ids::INGAME_UI));
        }),
    ]))

}

pub struct DialogueUI;
impl Menu for DialogueUI {
    fn id() -> MenuId { DIALOGUE }
    fn data() -> MenuData {
        MenuData {
            spawn,
            startup: vec![
                MenuStartup::SetInputContext(InputContext::Menu),
                MenuStartup::SetGameState(GameState::Dialogue),
            ],
        }
    }
}
