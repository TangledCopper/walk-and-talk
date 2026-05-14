use crate::inputs::bindings::InputMode;
use bevy::prelude::*;
use std::collections::HashMap;

pub mod bindings;
mod detect;
mod loader;
pub mod menu_nav;

pub struct InputsPlugin;

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum ActiveInputDevice {
    #[default]
    Keyboard,
    Gamepad,
}

#[derive(Resource, Default)]
pub struct ActiveGamepad(pub Option<Entity>);

#[derive(Resource)]
pub struct InputHoldState {
    pub hold_start: HashMap<KeyCode, f32>,
    pub gamepad_hold_start: HashMap<GamepadButton, f32>,
    pub axis_triggered: HashMap<(GamepadAxis, AxisDirection), bool>,
}

impl Default for InputHoldState {
    fn default() -> Self {
        Self {
            hold_start: HashMap::new(),
            gamepad_hold_start: HashMap::new(),
            axis_triggered: HashMap::new(),
        }
    }
}

#[derive(Resource, serde::Serialize, serde::Deserialize)]
pub struct InputBindings {
    pub keyboard: Vec<(KeyCode, Action, InputMode)>,
    pub gamepad_buttons: Vec<(GamepadButton, Action, InputMode)>,
    pub gamepad_axes: Vec<(GamepadAxis, AxisDirection, Action)>,
}

#[derive(Message, Debug)]
pub struct ActionEvent {
    pub action: Action,
    pub value: f32,
}

#[derive(
    States, Default, Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize,
)]
pub enum InputContext {
    #[default]
    Game,
    Menu,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum Action {
    Player(PlayerAction),
    Menu(MenuAction),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum PlayerAction {
    MoveForward,
    MoveBack,
    MoveLeft,
    MoveRight,
    LookUp,
    LookLeft,
    LookRight,
    LookDown,
    Jump,
    JumpHeld,
    Interact,
    Pause,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum MenuAction {
    Up,
    Down,
    Left,
    Right,
    Select,
    Back,
    Unpause,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum AxisDirection {
    Positive,
    Negative,
}

impl Plugin for InputsPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<ActiveInputDevice>()
            .init_state::<InputContext>()
            .insert_resource(ActiveGamepad::default())
            .insert_resource(InputHoldState::default())
            .add_message::<ActionEvent>()
            .add_plugins(menu_nav::MenuNavPlugin)
            .add_systems(Startup, loader::load_bindings)
            .add_systems(Update, detect::detect_active_device)
            .add_systems(
                Update,
                bindings::translate_keyboard.run_if(in_state(ActiveInputDevice::Keyboard)),
            )
            .add_systems(
                Update,
                bindings::translate_gamepad.run_if(in_state(ActiveInputDevice::Gamepad)),
            );
    }
}
