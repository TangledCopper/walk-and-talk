use crate::TouchedEntities;
use crate::inputs::{Action, ActionEvent, ActiveInputDevice, InputContext, PlayerAction};
use crate::menu::{ChangeMenu, id as menu_ids};
use crate::prefab_plugin;
use crate::scenes::SpawnLevelEntity;
use avian3d::{math::*, prelude::*};
use bevy::{ecs::query::Has, input::mouse::AccumulatedMouseMotion, prelude::*};
use std::f32::consts::FRAC_PI_2;

#[derive(Resource, Default)]
pub struct ActiveInteractible(pub Option<Entity>);

#[derive(Resource, Default)]
pub struct PendingInteraction(pub Option<Entity>);

// Layer collision constants used across prefabs
pub const LAYER_DEFAULT: u32 = 1 << 0;
pub const LAYER_PLAYER: u32 = 1 << 1;
pub const LAYER_NPC: u32 = 1 << 2;
pub const LAYER_SENSOR: u32 = 1 << 3;

#[derive(Component)]
pub struct Player;

#[derive(Bundle)]
pub struct PlayerBundle {
    pub marker: Player,
    pub collision_events: CollisionEventsEnabled,
    pub collision_layers: CollisionLayers,
    pub touched_entities: TouchedEntities,
    pub mesh: Mesh3d,
    pub material: MeshMaterial3d<StandardMaterial>,
    pub transform: Transform,
    pub controller: CharacterControllerBundle,
    pub friction: Friction,
    pub restitution: Restitution,
    pub gravity: GravityScale,
}

impl PlayerBundle {
    pub fn new(meshes: &mut Assets<Mesh>, materials: &mut Assets<StandardMaterial>) -> Self {
        Self::new_at(meshes, materials, Vec3::new(0.0, 1.5, 0.0))
    }

    pub fn new_at(
        meshes: &mut Assets<Mesh>,
        materials: &mut Assets<StandardMaterial>,
        position: Vec3,
    ) -> Self {
        Self {
            marker: Player,
            collision_events: CollisionEventsEnabled,
            collision_layers: CollisionLayers::new(
                [LAYER_PLAYER],
                [LAYER_DEFAULT, LAYER_NPC, LAYER_SENSOR],
            ),
            touched_entities: TouchedEntities::default(),
            mesh: Mesh3d(meshes.add(Capsule3d::new(0.4, 1.0))),
            material: MeshMaterial3d(materials.add(Color::srgb(0.8, 0.7, 0.6))),
            transform: Transform::from_xyz(position.x, position.y, position.z),
            controller: CharacterControllerBundle::new(Collider::capsule(0.4, 1.0)).with_movement(
                30.0,
                0.92,
                (30.0_f32).to_radians(),
            ),
            friction: Friction::ZERO.with_combine_rule(CoefficientCombine::Min),
            restitution: Restitution::ZERO.with_combine_rule(CoefficientCombine::Min),
            gravity: GravityScale(2.0),
        }
    }
}

/// Character controller components

#[derive(Component)]
pub struct CharacterController;

#[derive(Component)]
#[component(storage = "SparseSet")]
pub struct Grounded;

#[derive(Component)]
pub struct MovementAcceleration(Scalar);

#[derive(Component)]
pub struct MovementDampingFactor(Scalar);

#[derive(Component)]
pub struct MaxSlopeAngle(Scalar);

#[derive(Bundle)]
pub struct CharacterControllerBundle {
    character_controller: CharacterController,
    body: RigidBody,
    collider: Collider,
    ground_caster: ShapeCaster,
    locked_axes: LockedAxes,
    movement: MovementBundle,
}

#[derive(Bundle)]
pub struct MovementBundle {
    acceleration: MovementAcceleration,
    damping: MovementDampingFactor,
    max_slope_angle: MaxSlopeAngle,
}

impl MovementBundle {
    pub const fn new(acceleration: Scalar, damping: Scalar, max_slope_angle: Scalar) -> Self {
        Self {
            acceleration: MovementAcceleration(acceleration),
            damping: MovementDampingFactor(damping),
            max_slope_angle: MaxSlopeAngle(max_slope_angle),
        }
    }
}

impl Default for MovementBundle {
    fn default() -> Self {
        Self::new(30.0, 0.9, PI * 0.45)
    }
}

impl CharacterControllerBundle {
    pub fn new(collider: Collider) -> Self {
        let mut caster_shape = collider.clone();
        caster_shape.set_scale(Vector::ONE * 0.99, 10);

        Self {
            character_controller: CharacterController,
            body: RigidBody::Dynamic,
            collider,
            ground_caster: ShapeCaster::new(
                caster_shape,
                Vector::ZERO,
                Quaternion::default(),
                Dir3::NEG_Y,
            )
            .with_max_distance(0.2)
            .with_query_filter(SpatialQueryFilter::from_mask(LAYER_DEFAULT)),
            locked_axes: LockedAxes::ROTATION_LOCKED,
            movement: MovementBundle::default(),
        }
    }

    pub fn with_movement(
        mut self,
        acceleration: Scalar,
        damping: Scalar,
        max_slope_angle: Scalar,
    ) -> Self {
        self.movement = MovementBundle::new(acceleration, damping, max_slope_angle);
        self
    }
}

// Player prefab params and camera rig -- Largely inspired/copied from the avian3d examples

pub struct PlayerParams {
    pub position: Vec3,
}

impl Default for PlayerParams {
    fn default() -> Self {
        Self {
            position: Vec3::new(0.0, 1.5, 0.0),
        }
    }
}

pub struct CameraParams {
    pub fov: f32,
    pub distance: f32,
}

impl Default for CameraParams {
    fn default() -> Self {
        Self {
            fov: 70.0,
            distance: 10.0,
        }
    }
}

#[derive(Component)]
pub struct CameraTarget(pub Entity);

#[derive(Bundle)]
pub struct CameraRigBundle {
    pub camera: Camera3d,
    pub projection: Projection,
    pub transform: Transform,
    pub target: CameraTarget,
}

impl CameraRigBundle {
    pub fn new(target: Entity, params: CameraParams) -> Self {
        Self {
            camera: Camera3d::default(),
            projection: Projection::Perspective(PerspectiveProjection {
                fov: params.fov.to_radians(),
                ..default()
            }),
            transform: Transform::from_xyz(0.0, 1.5, 0.0),
            target: CameraTarget(target),
        }
    }
}

impl Player {
    pub fn spawn(
        commands: &mut Commands,
        meshes: &mut Assets<Mesh>,
        materials: &mut Assets<StandardMaterial>,
        player_params: PlayerParams,
        camera_params: CameraParams,
    ) -> Entity {
        let player_entity = commands
            .spawn_level(PlayerBundle::new_at(
                meshes,
                materials,
                player_params.position,
            ))
            .id();

        commands.spawn_level(CameraRigBundle::new(player_entity, camera_params));

        player_entity
    }
}

const JUMP_IMPULSE_BASE: f32 = 4.5;
const JUMP_HOLD_FORCE: f32 = 18.0;

fn update_grounded(
    mut commands: Commands,
    mut query: Query<
        (Entity, &ShapeHits, &Rotation, Option<&MaxSlopeAngle>),
        With<CharacterController>,
    >,
) {
    for (entity, hits, rotation, max_slope_angle) in &mut query {
        let is_grounded = hits.iter().any(|hit| {
            if let Some(angle) = max_slope_angle {
                (rotation * -hit.normal2).angle_between(Vector::Y).abs() <= angle.0
            } else {
                true
            }
        });

        if is_grounded {
            commands.entity(entity).insert(Grounded);
        } else {
            commands.entity(entity).remove::<Grounded>();
        }
    }
}

fn movement(
    time: Res<Time>,
    mut action_reader: MessageReader<ActionEvent>,
    mut controllers: Query<(&MovementAcceleration, &mut LinearVelocity, Has<Grounded>)>,
    camera: Single<&Transform, With<Camera>>,
) {
    let delta_time = time.delta_secs_f64().adjust_precision();
    let (yaw, _, _) = camera.rotation.to_euler(EulerRot::YXZ);
    let camera_yaw = Quat::from_rotation_y(yaw);

    let actions: Vec<&ActionEvent> = action_reader.read().collect();

    let mut direction = Vector2::ZERO;
    let mut should_jump = false;
    let mut jump_hold_value: f32 = 0.0;

    for action in &actions {
        match action.action {
            Action::Player(a) => match a {
                PlayerAction::MoveForward => direction.y += action.value as Scalar,
                PlayerAction::MoveBack => direction.y -= action.value as Scalar,
                PlayerAction::MoveRight => direction.x += action.value as Scalar,
                PlayerAction::MoveLeft => direction.x -= action.value as Scalar,
                PlayerAction::Jump => should_jump = true,
                PlayerAction::JumpHeld => jump_hold_value += action.value,
                _ => {}
            },
            _ => {}
        }
    }

    let direction = direction.clamp_length_max(1.0);

    for (movement_acceleration, mut linear_velocity, is_grounded) in &mut controllers {
        if should_jump && is_grounded {
            linear_velocity.y = JUMP_IMPULSE_BASE;
        }

        if jump_hold_value > 0.0 && linear_velocity.y > 0.0 {
            linear_velocity.y += (jump_hold_value * JUMP_HOLD_FORCE * time.delta_secs()) as Scalar;
        }

        if direction != Vector2::ZERO {
            let wish_dir = camera_yaw * Vec3::new(direction.x as f32, 0.0, -direction.y as f32);
            linear_velocity.x +=
                wish_dir.x.adjust_precision() * movement_acceleration.0 * delta_time;
            linear_velocity.z +=
                wish_dir.z.adjust_precision() * movement_acceleration.0 * delta_time;
        }
    }
}

fn apply_movement_damping(mut query: Query<(&MovementDampingFactor, &mut LinearVelocity)>) {
    for (damping_factor, mut linear_velocity) in &mut query {
        linear_velocity.x *= damping_factor.0;
        linear_velocity.z *= damping_factor.0;
    }
}

fn handle_player_actions(
    mut action_reader: MessageReader<ActionEvent>,
    mut change_menu: MessageWriter<ChangeMenu>,
    active: Res<ActiveInteractible>,
    mut pending: ResMut<PendingInteraction>,
) {
    for event in action_reader.read() {
        match event.action {
            Action::Player(PlayerAction::Pause) => {
                change_menu.write(ChangeMenu(menu_ids::PAUSE));
            }
            Action::Player(PlayerAction::Interact) => {
                if let Some(entity) = active.0 {
                    pending.0 = Some(entity);
                }
            }
            _ => {}
        }
    }
}

fn update_camera_transform(
    accumulated_mouse_motion: Res<AccumulatedMouseMotion>,
    mut action_reader: MessageReader<ActionEvent>,
    active_device: Res<State<ActiveInputDevice>>,
    input_context: Res<State<InputContext>>,
    mut camera_query: Query<(Entity, &mut Transform, &CameraTarget), With<Camera>>,
    player_query: Query<&Transform, Without<Camera>>,
    spatial: SpatialQuery,
    time: Res<Time>,
) {
    if *input_context.get() != InputContext::Game {
        return;
    }
    let actions: Vec<&ActionEvent> = action_reader.read().collect();

    for (_, mut cam_transform, target) in &mut camera_query {
        let Ok(player_transform) = player_query.get(target.0) else {
            continue;
        };

        let (delta_yaw, delta_pitch) = match active_device.get() {
            ActiveInputDevice::Keyboard => {
                let delta = accumulated_mouse_motion.delta;
                (-delta.x * 0.005, -delta.y * 0.005)
            }
            ActiveInputDevice::Gamepad => {
                let (mut x, mut y) = (0.0, 0.0);
                for action in &actions {
                    match action.action {
                        Action::Player(a) => match a {
                            PlayerAction::LookUp => y += action.value as Scalar,
                            PlayerAction::LookDown => y -= action.value as Scalar,
                            PlayerAction::LookRight => x += action.value as Scalar,
                            PlayerAction::LookLeft => x -= action.value as Scalar,
                            _ => {}
                        },
                        _ => {}
                    }
                }
                let sensitivity = 2.0 * time.delta_secs();
                (-x * sensitivity, y * sensitivity)
            }
        };

        let (yaw, pitch, roll) = cam_transform.rotation.to_euler(EulerRot::YXZ);
        let yaw = yaw + delta_yaw;
        const PITCH_LIMIT: f32 = FRAC_PI_2 - 0.01;
        let pitch: f32 = (pitch + delta_pitch).clamp(-PITCH_LIMIT, PITCH_LIMIT);
        cam_transform.rotation = Quat::from_euler(EulerRot::YXZ, yaw, pitch, roll);

        const MAX_DISTANCE: f32 = 10.0;
        cam_transform.translation =
            player_transform.translation + cam_transform.back() * MAX_DISTANCE;

        if let Some(hit) = spatial.cast_ray(
            player_transform.translation.adjust_precision(),
            cam_transform.back(),
            MAX_DISTANCE.adjust_precision(),
            true,
            &SpatialQueryFilter::from_mask([LAYER_DEFAULT, LAYER_PLAYER, LAYER_NPC])
                .with_excluded_entities([target.0]),
        ) {
            cam_transform.translation = player_transform.translation
                + cam_transform.back() * (hit.distance.val_num_f32() - 1.0).max(0.0);
        }
    }
}

prefab_plugin!(
    PlayerPlugin,
    Player,
    [
        update_grounded,
        movement,
        apply_movement_damping,
        handle_player_actions,
        update_camera_transform,
    ]
);
