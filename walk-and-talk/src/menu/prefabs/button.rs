use crate::menu::MenuEntity;
use bevy::prelude::*;

pub type ButtonFn = fn(&mut World);

pub const NORMAL_BUTTON_COLOR: Color = Color::srgb(0.15, 0.15, 0.25);
pub const HOVERED_BUTTON_COLOR: Color = Color::srgb(0.4, 0.4, 0.7);

pub fn spawn_button(commands: &mut Commands, label: &'static str) -> Entity {
    commands
        .spawn((
            MenuEntity,
            Button,
            Node {
                width: Val::Px(200.0),
                height: Val::Px(60.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(NORMAL_BUTTON_COLOR),
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new(label),
                TextFont {
                    font_size: 24.0,
                    ..default()
                },
                TextColor(Color::WHITE),
            ));
        })
        .id()
}

// This isn't necesary, but just a quick and dirty way to add menus quickly that are spaced evenly
fn spawn_button_container(
    commands: &mut Commands,
    buttons: &[(&'static str,)],
    flex_direction: FlexDirection,
    gap: Val,
) -> Vec<Entity> {
    let mut button_entities = Vec::new();

    let mut node = Node {
        width: Val::Percent(100.0),
        height: Val::Percent(100.0),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        flex_direction,
        ..default()
    };
    match flex_direction {
        FlexDirection::Column => node.row_gap = gap,
        FlexDirection::Row => node.column_gap = gap,
        _ => {}
    }

    commands.spawn((MenuEntity, node)).with_children(|parent| {
        for (label,) in buttons {
            let id = parent
                .spawn((
                    Button,
                    Node {
                        width: Val::Px(200.0),
                        height: Val::Px(60.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    BackgroundColor(NORMAL_BUTTON_COLOR),
                ))
                .with_children(|btn| {
                    btn.spawn((
                        Text::new(*label),
                        TextFont {
                            font_size: 24.0,
                            ..default()
                        },
                        TextColor(Color::WHITE),
                    ));
                })
                .id();
            button_entities.push(id);
        }
    });

    button_entities
}

pub fn spawn_button_column(commands: &mut Commands, buttons: &[(&'static str,)]) -> Vec<Entity> {
    spawn_button_container(commands, buttons, FlexDirection::Column, Val::Px(10.0))
}

pub fn spawn_button_row(commands: &mut Commands, buttons: &[(&'static str,)]) -> Vec<Entity> {
    spawn_button_container(commands, buttons, FlexDirection::Row, Val::Px(10.0))
}
