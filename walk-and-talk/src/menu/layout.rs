use super::prefabs::button::ButtonFn;
use bevy::prelude::*;

#[derive(Clone)]
pub struct ButtonEntry {
    pub entity: Entity,
    pub on_press: ButtonFn,
}

#[derive(Resource, Default, Clone)]
pub struct MenuLayout {
    pub rows: Vec<Vec<ButtonEntry>>,
}

#[derive(Resource, Default)]
pub struct PendingButtonPress(pub Option<(usize, usize)>);

impl MenuLayout {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_column(mut self, items: Vec<(Entity, ButtonFn)>) -> Self {
        for (entity, on_press) in items {
            self.rows.push(vec![ButtonEntry { entity, on_press }]);
        }
        self
    }

    pub fn add_row(mut self, items: Vec<(Entity, ButtonFn)>) -> Self {
        self.rows.push(
            items
                .into_iter()
                .map(|(entity, on_press)| ButtonEntry { entity, on_press })
                .collect(),
        );
        self
    }

    pub fn get(&self, row: usize, col: usize) -> Option<&ButtonEntry> {
        self.rows.get(row)?.get(col)
    }

    pub fn coords(&self) -> Vec<(usize, usize)> {
        self.rows
            .iter()
            .enumerate()
            .flat_map(|(r, row)| row.iter().enumerate().map(move |(c, _)| (r, c)))
            .collect()
    }

    pub fn clear(&mut self) {
        self.rows.clear();
    }
}
