use crate::prefab_plugin;
use crate::scenes::SpawnLevelEntity;
use crate::scenes::prefabs::player::{LAYER_PLAYER, LAYER_SENSOR, Player};
use avian3d::prelude::*;
use bevy::prelude::*;
use std::collections::HashMap;

// This is a bit more fleshed out, but solid example of the prefab plugin with a basic component
#[derive(Component)]
pub struct FloorButton {
    pub id: u32,
}

#[derive(Message, Clone)]
pub struct ButtonTriggered {
    pub id: u32,
    pub activated: bool,
}

pub struct FloorButtonParams {
    pub id: u32,
    pub position: Vec3,
}

impl FloorButton {
    pub fn spawn(
        commands: &mut Commands,
        meshes: &mut Assets<Mesh>,
        materials: &mut Assets<StandardMaterial>,
        params: FloorButtonParams,
    ) -> Entity {
        commands
            .spawn_level((
                FloorButton { id: params.id },
                Sensor,
                CollisionLayers::new([LAYER_SENSOR], [LAYER_PLAYER]),
                CollisionEventsEnabled,
                RigidBody::Static,
                Collider::cuboid(1.0, 0.1, 1.0),
                Transform::from_translation(params.position),
                Mesh3d(meshes.add(Cuboid::new(1.0, 0.1, 1.0))),
                MeshMaterial3d(materials.add(StandardMaterial {
                    base_color: BUTTON_INACTIVE_COLOR,
                    ..default()
                })),
                CollidingEntities::default(),
            ))
            .id()
    }
}

prefab_plugin!(
    FloorButtonPlugin,
    FloorButton,
    [on_button_collision, update_button_color,]
);

fn on_button_collision(
    buttons: Query<(&FloorButton, &CollidingEntities)>,
    players: Query<Entity, With<Player>>,
    mut writer: MessageWriter<ButtonTriggered>,
    mut state: Local<HashMap<u32, bool>>,
) {
    let Ok(player) = players.single() else { return };

    for (button, colliding) in &buttons {
        let is_activated = colliding.contains(&player);
        let was_activated = state.get(&button.id).copied().unwrap_or(false);

        if is_activated != was_activated {
            state.insert(button.id, is_activated);
            writer.write(ButtonTriggered {
                id: button.id,
                activated: is_activated,
            });
        }
    }
}

fn update_button_color(
    mut events: MessageReader<ButtonTriggered>,
    buttons: Query<(&FloorButton, &MeshMaterial3d<StandardMaterial>)>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for ButtonTriggered { id, activated } in events.read() {
        for (button, mat_handle) in &buttons {
            if button.id == *id {
                if let Some(mat) = materials.get_mut(mat_handle) {
                    mat.base_color = if *activated {
                        BUTTON_ACTIVE_COLOR
                    } else {
                        BUTTON_INACTIVE_COLOR
                    };
                }
            }
        }
    }
}

const BUTTON_INACTIVE_COLOR: Color = Color::srgb(0.3, 0.3, 0.8);
const BUTTON_ACTIVE_COLOR: Color = Color::srgb(0.8, 0.2, 0.2);
