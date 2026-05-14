use crate::menu::{ChangeMenu, GameState};
use bevy::prelude::*;
use std::collections::HashMap;

pub mod id;
pub mod prefabs;
pub mod prelude;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct LevelId(pub u32);

pub trait Level {
    fn id() -> LevelId;
    fn data() -> LevelData;
}

#[derive(Component)]
pub struct LevelEntity;

#[derive(Resource)]
pub struct CurrentLevel(pub LevelId);

#[derive(Message)]
pub struct ChangeLevel(pub LevelId);

type SpawnFn = fn(&mut Commands, &mut Assets<Mesh>, &mut Assets<StandardMaterial>);

#[derive(Debug, Clone)]
pub enum LevelStartup {
    SetGameState(GameState),
    ShowMenu(crate::menu::MenuId),
}

pub struct LevelData {
    pub spawn: SpawnFn,
    pub startup: Vec<LevelStartup>,
}

#[derive(Resource)]
pub struct LevelRegistry {
    levels: HashMap<LevelId, LevelData>,
}

impl LevelRegistry {
    pub fn new() -> Self {
        Self {
            levels: HashMap::new(),
        }
    }

    pub fn register(mut self, id: LevelId, data: LevelData) -> Self {
        self.levels.insert(id, data);
        self
    }

    pub fn register_level<L: Level>(self) -> Self {
        self.register(L::id(), L::data())
    }

    pub fn get(&self, id: LevelId) -> Option<&LevelData> {
        self.levels.get(&id)
    }
}

pub trait SpawnLevelEntity {
    fn spawn_level(&mut self, bundle: impl Bundle) -> EntityCommands<'_>;
}

impl SpawnLevelEntity for Commands<'_, '_> {
    fn spawn_level(&mut self, bundle: impl Bundle) -> EntityCommands<'_> {
        self.spawn((LevelEntity, bundle))
    }
}

pub struct LevelLoaderPlugin;

impl Plugin for LevelLoaderPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(CurrentLevel(id::BLANK))
            .insert_resource(id::registry()) // scenes.rs has zero level knowledge
            .add_message::<ChangeLevel>()
            .add_systems(Startup, spawn_initial_level)
            .add_systems(Update, handle_level_change);
    }
}

fn load_level(
    id: LevelId,
    commands: &mut Commands,
    registry: &LevelRegistry,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
    menu_writer: &mut MessageWriter<ChangeMenu>,
    next_game_state: &mut NextState<GameState>,
    to_despawn: impl Iterator<Item = Entity>,
) {
    for entity in to_despawn {
        commands.entity(entity).despawn();
    }

    let Some(data) = registry.get(id) else {
        warn!("No level registered with id {:?}", id);
        return;
    };

    (data.spawn)(commands, meshes, materials);

    for action in &data.startup {
        match action {
            LevelStartup::SetGameState(state) => {
                next_game_state.set(*state);
            }
            LevelStartup::ShowMenu(menu_id) => {
                menu_writer.write(ChangeMenu(*menu_id));
            }
        }
    }
}

fn spawn_initial_level(
    mut commands: Commands,
    registry: Res<LevelRegistry>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut menu_writer: MessageWriter<ChangeMenu>,
    mut next_game_state: ResMut<NextState<GameState>>,
) {
    load_level(
        id::BLANK,
        &mut commands,
        &registry,
        &mut meshes,
        &mut materials,
        &mut menu_writer,
        &mut next_game_state,
        std::iter::empty(),
    );
}

fn handle_level_change(
    mut commands: Commands,
    mut messages: MessageReader<ChangeLevel>,
    mut current: ResMut<CurrentLevel>,
    registry: Res<LevelRegistry>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut menu_writer: MessageWriter<ChangeMenu>,
    mut next_game_state: ResMut<NextState<GameState>>,
    entities: Query<Entity, With<LevelEntity>>,
) {
    if let Some(ChangeLevel(next_id)) = messages.read().last() {
        let next_id = *next_id;
        current.0 = next_id;
        load_level(
            next_id,
            &mut commands,
            &registry,
            &mut meshes,
            &mut materials,
            &mut menu_writer,
            &mut next_game_state,
            entities.iter(),
        );
    }
}
