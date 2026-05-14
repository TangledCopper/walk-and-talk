use avian3d::prelude::*;
use bevy::{ecs::entity::EntityHashSet, prelude::*};
mod inputs;
mod menu;
mod scenes;

#[derive(Component, Default, Deref, DerefMut)]
struct TouchedEntities(EntityHashSet);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(inputs::InputsPlugin)
        .add_plugins(PhysicsPlugins::default())
        .add_plugins(menu::MenuLoaderPlugin)
        .add_plugins(scenes::prefabs::PrefabsPlugin)
        .add_plugins(scenes::LevelLoaderPlugin)
        .run();
}
