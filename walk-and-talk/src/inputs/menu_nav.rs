use crate::inputs::{Action, ActionEvent, InputContext, MenuAction};
use crate::menu::layout::{MenuLayout, PendingButtonPress};
use crate::menu::prefabs::button::{HOVERED_BUTTON_COLOR, NORMAL_BUTTON_COLOR};
use bevy::prelude::*;

#[derive(Resource, Default, Debug, Clone, Copy, PartialEq, Eq)]
pub struct MenuFocus {
    pub row: usize,
    pub col: usize,
}

pub struct MenuNavPlugin;

impl Plugin for MenuNavPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<MenuFocus>()
            .init_resource::<PendingButtonPress>()
            .add_systems(
                Update,
                (
                    handle_menu_navigation,
                    handle_menu_hover,
                    sync_button_highlight,
                    handle_menu_clicks,
                )
                    .run_if(in_state(InputContext::Menu)),
            )
            .add_systems(
                Update,
                execute_pending_button.run_if(in_state(InputContext::Menu)),
            );
    }
}

fn unique_sorted_dedup(vals: impl Iterator<Item = usize>) -> Vec<usize> {
    let mut v: Vec<usize> = vals.collect();
    v.sort();
    v.dedup();
    v
}

fn move_vertical(focus: MenuFocus, delta: i32, coords: &[(usize, usize)]) -> MenuFocus {
    let rows = unique_sorted_dedup(coords.iter().map(|(r, _)| *r));
    if rows.is_empty() {
        return focus;
    }

    let n = rows.len() as i32;
    let cur = rows.iter().position(|&r| r == focus.row).unwrap_or(0) as i32;

    for step in 1..=n {
        let next_row = rows[((cur + delta * step).rem_euclid(n)) as usize];
        let next_col = if coords.contains(&(next_row, focus.col)) {
            focus.col
        } else if let Some(&(_, c)) = coords
            .iter()
            .filter(|(r, _)| *r == next_row)
            .min_by_key(|(_, c)| c)
        {
            c
        } else {
            continue;
        };
        return MenuFocus {
            row: next_row,
            col: next_col,
        };
    }
    focus
}

fn move_horizontal(focus: MenuFocus, delta: i32, coords: &[(usize, usize)]) -> MenuFocus {
    let row_cols = unique_sorted_dedup(
        coords
            .iter()
            .filter(|(r, _)| *r == focus.row)
            .map(|(_, c)| *c),
    );
    let cols = if row_cols.len() > 1 {
        row_cols
    } else {
        unique_sorted_dedup(coords.iter().map(|(_, c)| *c))
    };
    if cols.len() <= 1 {
        return focus;
    }

    let n = cols.len() as i32;
    let cur = cols.iter().position(|&c| c == focus.col).unwrap_or(0) as i32;

    for step in 1..=n {
        let next_col = cols[((cur + delta * step).rem_euclid(n)) as usize];
        if coords.contains(&(focus.row, next_col)) {
            return MenuFocus {
                row: focus.row,
                col: next_col,
            };
        }
        if let Some(&(r, _)) = coords
            .iter()
            .filter(|(_, c)| *c == next_col)
            .min_by_key(|(r, _)| r)
        {
            return MenuFocus {
                row: r,
                col: next_col,
            };
        }
    }
    focus
}

fn handle_menu_navigation(
    mut events: MessageReader<ActionEvent>,
    mut focus: ResMut<MenuFocus>,
    layout: Res<MenuLayout>,
    mut pending: ResMut<PendingButtonPress>,
) {
    let coords = layout.coords();
    if coords.is_empty() {
        return;
    }

    for event in events.read() {
        match event.action {
            Action::Menu(MenuAction::Up) => *focus = move_vertical(*focus, -1, &coords),
            Action::Menu(MenuAction::Down) => *focus = move_vertical(*focus, 1, &coords),
            Action::Menu(MenuAction::Left) => *focus = move_horizontal(*focus, -1, &coords),
            Action::Menu(MenuAction::Right) => *focus = move_horizontal(*focus, 1, &coords),
            Action::Menu(MenuAction::Select) => pending.0 = Some((focus.row, focus.col)),
            _ => {}
        }
    }
}

fn execute_pending_button(world: &mut World) {
    let pending = world.resource_mut::<PendingButtonPress>().0.take();

    if let Some((row, col)) = pending {
        let on_press = world
            .resource::<MenuLayout>()
            .get(row, col)
            .map(|e| e.on_press);

        if let Some(f) = on_press {
            f(world);
        }
    }
}

fn handle_menu_clicks(
    interactions: Query<(Entity, &Interaction), (Changed<Interaction>, With<Button>)>,
    layout: Res<MenuLayout>,
    mut focus: ResMut<MenuFocus>,
    mut pending: ResMut<PendingButtonPress>,
) {
    for (entity, interaction) in &interactions {
        if *interaction == Interaction::Pressed {
            for (r, row) in layout.rows.iter().enumerate() {
                for (c, entry) in row.iter().enumerate() {
                    if entry.entity == entity {
                        focus.row = r;
                        focus.col = c;
                        pending.0 = Some((r, c));
                    }
                }
            }
        }
    }
}

fn handle_menu_hover(
    interactions: Query<(Entity, &Interaction), (Changed<Interaction>, With<Button>)>,
    layout: Res<MenuLayout>,
    mut focus: ResMut<MenuFocus>,
) {
    for (entity, interaction) in &interactions {
        if *interaction == Interaction::Hovered {
            for (r, row) in layout.rows.iter().enumerate() {
                for (c, entry) in row.iter().enumerate() {
                    if entry.entity == entity {
                        focus.row = r;
                        focus.col = c;
                    }
                }
            }
        }
    }
}

fn sync_button_highlight(
    focus: Res<MenuFocus>,
    layout: Res<MenuLayout>,
    mut buttons: Query<(Entity, &mut BackgroundColor), With<Button>>,
) {
    let focused_entity = layout.get(focus.row, focus.col).map(|e| e.entity);
    for (entity, mut bg) in &mut buttons {
        *bg = if Some(entity) == focused_entity {
            HOVERED_BUTTON_COLOR.into()
        } else {
            NORMAL_BUTTON_COLOR.into()
        };
    }
}
