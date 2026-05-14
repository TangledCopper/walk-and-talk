use crate::inputs::{ActiveGamepad, ActiveInputDevice};
use bevy::{input::gamepad::GamepadEvent, input::mouse::AccumulatedMouseMotion, prelude::*};

pub fn detect_active_device(
    // This is  basically an easy way for users to swap between input devices
    mut next_state: ResMut<NextState<ActiveInputDevice>>,
    mut active_gamepad: ResMut<ActiveGamepad>,
    keys: Res<ButtonInput<KeyCode>>,
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    accumulated_mouse_motion: Res<AccumulatedMouseMotion>,
    mut gamepad_events: MessageReader<GamepadEvent>,
) {
    if keys.get_just_pressed().next().is_some()
        || mouse_buttons.get_just_pressed().next().is_some()
        || accumulated_mouse_motion.delta != Vec2::ZERO
    {
        next_state.set(ActiveInputDevice::Keyboard);
        active_gamepad.0 = None;
    }
    for event in gamepad_events.read() {
        match event {
            GamepadEvent::Button(e) => {
                next_state.set(ActiveInputDevice::Gamepad);
                active_gamepad.0 = Some(e.entity);
            }
            GamepadEvent::Axis(e) if e.value.abs() > 0.1 => {
                next_state.set(ActiveInputDevice::Gamepad);
                active_gamepad.0 = Some(e.entity);
            }
            _ => {}
        }
    }
}
