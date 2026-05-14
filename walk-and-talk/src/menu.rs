use crate::inputs::{ActiveInputDevice, InputContext};
use crate::menu::layout::MenuLayout;
use avian3d::prelude::*;
use bevy::{
    prelude::*,
    window::{CursorGrabMode, CursorOptions},
};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum MenuStartup {
    SetGameState(GameState),
    SetInputContext(InputContext),
}
pub mod id;
pub mod layout;
pub mod prefabs;
pub mod prelude;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct MenuId(pub u32);

pub trait Menu {
    fn id() -> MenuId;
    fn data() -> MenuData;
}

#[derive(Component)]
pub struct MenuEntity;

#[derive(Resource)]
pub struct CurrentMenu(pub MenuId);

#[derive(Message)]
pub struct ChangeMenu(pub MenuId);

#[derive(States, Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GameState {
    #[default]
    Playing, // Plays physics, correct input context, the works
    Paused,   // no physics, menu input context
    Dialogue, // physics, menu input context
}
pub type SpawnFn =
    fn(&mut Commands, &mut Assets<Mesh>, &mut Assets<StandardMaterial>) -> Option<MenuLayout>;

pub struct MenuData {
    pub spawn: SpawnFn,
    pub startup: Vec<MenuStartup>,
}

#[derive(Resource)]
pub struct MenuRegistry {
    menus: HashMap<MenuId, MenuData>,
}

impl MenuRegistry {
    pub fn new() -> Self {
        Self {
            menus: HashMap::new(),
        }
    }

    pub fn register(mut self, id: MenuId, data: MenuData) -> Self {
        self.menus.insert(id, data);
        self
    }

    pub fn register_menu<M: Menu>(self) -> Self {
        self.register(M::id(), M::data())
    }

    pub fn get(&self, id: MenuId) -> Option<&MenuData> {
        self.menus.get(&id)
    }
}

pub trait SpawnMenuEntity {
    fn spawn_menu(&mut self, bundle: impl Bundle) -> EntityCommands<'_>;
}

impl SpawnMenuEntity for Commands<'_, '_> {
    fn spawn_menu(&mut self, bundle: impl Bundle) -> EntityCommands<'_> {
        self.spawn((MenuEntity, bundle))
    }
}

pub struct MenuLoaderPlugin;

impl Plugin for MenuLoaderPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameState>()
            .insert_resource(CurrentMenu(id::MAIN_MENU))
            .init_resource::<layout::MenuLayout>()
            .insert_resource(id::registry())
            .add_message::<ChangeMenu>()
            .add_systems(OnEnter(GameState::Paused), (pause_physics, release_cursor))
            .add_systems(
                OnEnter(GameState::Playing),
                (resume_physics, capture_cursor),
            )
            .add_systems(OnEnter(GameState::Dialogue), (release_cursor))
            .add_systems(Startup, spawn_initial_menu)
            .add_systems(Update, handle_menu_change);
    }
}

fn load_menu(
    id: MenuId,
    commands: &mut Commands,
    registry: &MenuRegistry,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
    next_game_state: &mut NextState<GameState>,
    next_input_context: &mut NextState<InputContext>,
    to_despawn: impl Iterator<Item = Entity>,
) {
    for entity in to_despawn {
        commands.entity(entity).despawn();
    }

    let Some(data) = registry.get(id) else {
        warn!("No menu registered with id {:?}", id);
        return;
    };

    let layout = (data.spawn)(commands, meshes, materials);
    commands.insert_resource(layout.unwrap_or_default());

    for action in &data.startup {
        match action {
            MenuStartup::SetGameState(state) => {
                next_game_state.set(*state);
            }
            MenuStartup::SetInputContext(context) => {
                next_input_context.set(*context);
            }
        }
    }
}

fn spawn_initial_menu(
    mut commands: Commands,
    registry: Res<MenuRegistry>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut next_game_state: ResMut<NextState<GameState>>,
    mut next_input_context: ResMut<NextState<InputContext>>,
) {
    load_menu(
        id::MAIN_MENU,
        &mut commands,
        &registry,
        &mut meshes,
        &mut materials,
        &mut next_game_state,
        &mut next_input_context,
        std::iter::empty(),
    );
}

fn handle_menu_change(
    mut commands: Commands,
    mut messages: MessageReader<ChangeMenu>,
    mut current: ResMut<CurrentMenu>,
    registry: Res<MenuRegistry>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut next_game_state: ResMut<NextState<GameState>>,
    mut next_input_context: ResMut<NextState<InputContext>>,
    entities: Query<Entity, With<MenuEntity>>,
) {
    if let Some(ChangeMenu(next_id)) = messages.read().last() {
        let next_id = *next_id;
        current.0 = next_id;
        load_menu(
            next_id,
            &mut commands,
            &registry,
            &mut meshes,
            &mut materials,
            &mut next_game_state,
            &mut next_input_context,
            entities.iter(),
        );
    }
}

fn capture_cursor(mut cursor: Single<&mut CursorOptions>) {
    cursor.visible = false;
    cursor.grab_mode = CursorGrabMode::Locked;
}

fn release_cursor(
    mut cursor: Single<&mut CursorOptions>,
    input_device: Res<State<ActiveInputDevice>>,
) {
    if *input_device.get() == ActiveInputDevice::Keyboard {
        cursor.visible = true;
        cursor.grab_mode = CursorGrabMode::None;
    }
}

fn pause_physics(mut physics_time: ResMut<Time<Physics>>) {
    physics_time.pause();
}

fn resume_physics(mut physics_time: ResMut<Time<Physics>>) {
    physics_time.unpause();
}
