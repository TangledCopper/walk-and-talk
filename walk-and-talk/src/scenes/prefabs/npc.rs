use crate::prefab_plugin;
use crate::scenes::SpawnLevelEntity;
use crate::scenes::prefabs::player::{
    ActiveInteractible, LAYER_NPC, LAYER_PLAYER, LAYER_SENSOR, PendingInteraction, Player,
};
use avian3d::prelude::*;
use bevy::prelude::*;

pub type NpcInteractFn = fn(&mut World);

#[derive(Component)]
pub struct NPC {
    pub id: u32,
    pub interact_fn: NpcInteractFn,
}

pub struct NPCParams {
    pub id: u32,
    pub position: Vec3,
    pub interact_fn: NpcInteractFn,
}

impl NPC {
    pub fn spawn(
        commands: &mut Commands,
        meshes: &mut Assets<Mesh>,
        materials: &mut Assets<StandardMaterial>,
        params: NPCParams,
    ) -> Entity {
        commands
            .spawn_level((
                RigidBody::Static,
                Collider::capsule(0.4, 1.0),
                CollisionLayers::new([LAYER_NPC], [LAYER_PLAYER]),
                Transform::from_translation(params.position + Vec3::new(0.0, 1.0, 0.0)),
                Mesh3d(meshes.add(Capsule3d::new(0.4, 1.0))),
                MeshMaterial3d(materials.add(StandardMaterial {
                    base_color: NPC_SLEEP,
                    ..default()
                })),
                CollidingEntities::default(),
            ))
            .with_children(|parent| {
                parent.spawn((
                    NPC {
                        id: params.id,
                        interact_fn: params.interact_fn,
                    },
                    Sensor,
                    CollisionLayers::new([LAYER_SENSOR], [LAYER_PLAYER]),
                    CollisionEventsEnabled,
                    Collider::cuboid(4.0, 1.0, 4.0),
                    CollidingEntities::default(),
                ));
            })
            .id()
    }
}

prefab_plugin!(
    NPCPlugin,
    NPC,
    [on_npc_collision, execute_pending_interaction,]
);

fn on_npc_collision(
    sensors: Query<(Entity, &NPC, &CollidingEntities, &ChildOf)>,
    parents: Query<&MeshMaterial3d<StandardMaterial>>,
    players: Query<Entity, With<Player>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut active: ResMut<ActiveInteractible>,
) {
    let Ok(player) = players.single() else { return };

    let prev = active.0;

    let near_entity = sensors
        .iter()
        .find(|(_, _, colliding, _)| colliding.contains(&player))
        .map(|(e, _, _, _)| e);

    if near_entity != prev {
        if let Some(prev) = prev {
            if let Ok((_, _, _, child_of)) = sensors.get(prev) {
                if let Ok(handle) = parents.get(child_of.0) {
                    if let Some(mat) = materials.get_mut(handle) {
                        mat.base_color = NPC_SLEEP;
                    }
                }
            }
        }
        if let Some(near) = near_entity {
            if let Ok((_, _, _, child_of)) = sensors.get(near) {
                if let Ok(handle) = parents.get(child_of.0) {
                    if let Some(mat) = materials.get_mut(handle) {
                        mat.base_color = NPC_AWAKE;
                    }
                }
            }
        }
    }

    active.0 = near_entity;
}

fn execute_pending_interaction(world: &mut World) {
    let entity = world.resource_mut::<PendingInteraction>().0.take();
    let Some(entity) = entity else { return };

    let interact_fn = world.entity(entity).get::<NPC>().map(|npc| npc.interact_fn);
    if let Some(f) = interact_fn {
        f(world);
    }
}

const NPC_SLEEP: Color = Color::srgb(0.3, 0.3, 0.8);
const NPC_AWAKE: Color = Color::srgb(0.8, 0.2, 0.2);
