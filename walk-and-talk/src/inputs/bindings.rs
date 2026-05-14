use crate::inputs::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::hash::Hash;

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum InputMode {
    Held,
    JustPressed,
    HeldTimed {
        max_duration: f32,
    },
    RepeatOnHold {
        initial_delay: f32,
        repeat_interval: f32,
    },
}

impl Default for InputBindings {
    fn default() -> Self {
        Self {
            keyboard: vec![],
            gamepad_buttons: vec![],
            gamepad_axes: vec![],
        }
    }
}

pub fn translate_keyboard(
    context: Res<State<InputContext>>,
    bindings: Res<InputBindings>,
    keys: Res<ButtonInput<KeyCode>>,
    mut action_events: MessageWriter<ActionEvent>,
    mut hold_state: ResMut<InputHoldState>,
    time: Res<Time>,
) {
    let filtered_bindings: Vec<_> = bindings
        .keyboard
        .iter()
        .filter(|(_, action, _)| match context.get() {
            InputContext::Game => !matches!(action, Action::Menu(_)),
            InputContext::Menu => !matches!(action, Action::Player(_)),
        })
        .cloned()
        .collect();

    process_digital_bindings(
        &filtered_bindings,
        |k| keys.just_pressed(k),
        |k| keys.pressed(k),
        |k| keys.just_released(k),
        &mut hold_state.hold_start,
        time.elapsed_secs(),
        &mut action_events,
    );
}

pub fn translate_gamepad(
    context: Res<State<InputContext>>,
    bindings: Res<InputBindings>,
    active_gamepad: Res<ActiveGamepad>,
    gamepads: Query<&Gamepad>,
    mut action_events: MessageWriter<ActionEvent>,
    mut hold_state: ResMut<InputHoldState>,
    time: Res<Time>,
) {
    let Some(entity) = active_gamepad.0 else {
        return;
    };
    let Ok(gamepad) = gamepads.get(entity) else {
        return;
    };

    let filtered_bindings: Vec<_> = bindings
        .gamepad_buttons
        .iter()
        .filter(|(_, action, _)| match context.get() {
            InputContext::Game => !matches!(action, Action::Menu(_)),
            InputContext::Menu => !matches!(action, Action::Player(_)),
        })
        .cloned()
        .collect();

    process_digital_bindings(
        &filtered_bindings,
        |b| gamepad.just_pressed(b),
        |b| gamepad.pressed(b),
        |b| gamepad.just_released(b),
        &mut hold_state.gamepad_hold_start,
        time.elapsed_secs(),
        &mut action_events,
    );

    // Emit axis events - behavior depends on context
    const AXIS_DEADZONE: f32 = 0.05;
    const AXIS_THRESHOLD: f32 = 0.2;
    let is_menu = *context.get() == InputContext::Menu;

    let filtered_axes: Vec<_> = bindings
        .gamepad_axes
        .iter()
        .filter(|(_, _, action)| match context.get() {
            InputContext::Game => !matches!(action, Action::Menu(_)),
            InputContext::Menu => !matches!(action, Action::Player(_)),
        })
        .collect();

    for (axis, dir, action) in &filtered_axes {
        if let Some(value) = gamepad.get(*axis) {
            let is_above = match dir {
                AxisDirection::Positive => value > AXIS_THRESHOLD,
                AxisDirection::Negative => value < -AXIS_THRESHOLD,
            };

            if is_menu {
                let key = (*axis, *dir);
                let was_triggered = hold_state
                    .axis_triggered
                    .get(&key)
                    .copied()
                    .unwrap_or(false);

                if is_above && !was_triggered {
                    action_events.write(ActionEvent {
                        action: *action,
                        value: value.abs(),
                    });
                    hold_state.axis_triggered.insert(key, true);
                }

                if value.abs() < AXIS_DEADZONE {
                    hold_state.axis_triggered.remove(&key);
                }
            } else {
                if is_above {
                    action_events.write(ActionEvent {
                        action: *action,
                        value: value.abs(),
                    });
                }
            }
        }
    }
}

fn process_held_timed<K: Eq + Hash + Copy>(
    key: K,
    max_duration: f32,
    just_pressed: bool,
    pressed: bool,
    just_released: bool,
    hold_map: &mut HashMap<K, f32>,
    action: Action,
    elapsed: f32,
    action_events: &mut MessageWriter<ActionEvent>,
) {
    if just_pressed {
        hold_map.insert(key, elapsed);
        action_events.write(ActionEvent { action, value: 1.0 });
    }
    if pressed {
        if let Some(start) = hold_map.get(&key) {
            let held_for = elapsed - start;
            if held_for < max_duration {
                action_events.write(ActionEvent {
                    action: Action::Player(PlayerAction::JumpHeld),
                    value: 1.0 - (held_for / max_duration),
                });
            }
        }
    }
    if just_released {
        hold_map.remove(&key);
    }
}

fn process_repeat_on_hold<K: Eq + Hash + Copy>(
    key: K,
    initial_delay: f32,
    repeat_interval: f32,
    just_pressed: bool,
    pressed: bool,
    just_released: bool,
    hold_map: &mut HashMap<K, f32>,
    action: Action,
    elapsed: f32,
    action_events: &mut MessageWriter<ActionEvent>,
) {
    if just_pressed {
        hold_map.insert(key, elapsed);
        action_events.write(ActionEvent { action, value: 1.0 });
    }
    if pressed {
        if let Some(start_time) = hold_map.get(&key) {
            let held_for = elapsed - start_time;
            if held_for > initial_delay {
                let repeats = ((held_for - initial_delay) / repeat_interval).floor() as i32;
                let last_repeat_time =
                    start_time + initial_delay + (repeats as f32 * repeat_interval);
                if elapsed - last_repeat_time >= repeat_interval - 0.001 {
                    action_events.write(ActionEvent { action, value: 1.0 });
                }
            }
        }
    }
    if just_released {
        hold_map.remove(&key);
    }
}

fn process_digital_bindings<K: Eq + Hash + Copy>(
    bindings: &[(K, Action, InputMode)],
    just_pressed: impl Fn(K) -> bool,
    pressed: impl Fn(K) -> bool,
    just_released: impl Fn(K) -> bool,
    hold_map: &mut HashMap<K, f32>,
    elapsed: f32,
    action_events: &mut MessageWriter<ActionEvent>,
) {
    for (key, action, mode) in bindings {
        match mode {
            InputMode::Held => {
                if pressed(*key) {
                    action_events.write(ActionEvent {
                        action: *action,
                        value: 1.0,
                    });
                }
            }
            InputMode::JustPressed => {
                if just_pressed(*key) {
                    action_events.write(ActionEvent {
                        action: *action,
                        value: 1.0,
                    });
                }
            }
            InputMode::HeldTimed { max_duration } => {
                process_held_timed(
                    *key,
                    *max_duration,
                    just_pressed(*key),
                    pressed(*key),
                    just_released(*key),
                    hold_map,
                    *action,
                    elapsed,
                    action_events,
                );
            }
            InputMode::RepeatOnHold {
                initial_delay,
                repeat_interval,
            } => {
                process_repeat_on_hold(
                    *key,
                    *initial_delay,
                    *repeat_interval,
                    just_pressed(*key),
                    pressed(*key),
                    just_released(*key),
                    hold_map,
                    *action,
                    elapsed,
                    action_events,
                );
            }
        }
    }
}
