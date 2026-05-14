use super::floor_button::ButtonTriggered;
use crate::prefab_plugin;
use crate::scenes::SpawnLevelEntity;
use bevy::prelude::*;

#[derive(Component)]
pub struct ButtonLight {
    pub listens_for: u32,
}

pub struct ButtonLightParams {
    pub listens_for: u32,
    pub position: Vec3,
}

impl ButtonLight {
    pub fn spawn(commands: &mut Commands, params: ButtonLightParams) -> Entity {
        commands
            .spawn_level((
                ButtonLight {
                    listens_for: params.listens_for,
                },
                PointLight {
                    intensity: 0.0,
                    color: LIGHT_INACTIVE_COLOR,
                    shadows_enabled: true,
                    ..default()
                },
                Transform::from_translation(params.position),
            ))
            .id()
    }
}

prefab_plugin!(ButtonLightPlugin, ButtonLight, [toggle_light,]);

fn toggle_light(
    mut events: MessageReader<ButtonTriggered>,
    mut lights: Query<(&ButtonLight, &mut PointLight)>,
) {
    for ButtonTriggered { id, activated } in events.read() {
        for (light, mut point_light) in &mut lights {
            if light.listens_for == *id {
                point_light.intensity = if *activated { 100_000.0 } else { 0.0 };
                point_light.color = if *activated {
                    LIGHT_ACTIVE_COLOR
                } else {
                    LIGHT_INACTIVE_COLOR
                };
            }
        }
    }
}

const LIGHT_INACTIVE_COLOR: Color = Color::srgb(0.3, 0.3, 1.0);
const LIGHT_ACTIVE_COLOR: Color = Color::srgb(1.0, 0.3, 0.3);
