use crate::menu::GameState;
use crate::scenes::prelude::*;
use crate::scenes::id::LEVEL1;
use avian3d::prelude::*;
use bevy::prelude::*;
use crate::menu::{ChangeMenu, id as menu_ids};

// If you are curious about spawn_level vs just commands.spawn, check my scenes.rs trait
// Essentailly, what is happeneing here is I tag all the objects in the scene with "spawn_level"
// so that they are all tagged and can be filtered when despawning. It's not 100% but I'm still
// learning. I hope it's a half decent system
pub fn spawn(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
) {
    // floor - dead simple example of how to create an MVP prefab
    commands.spawn_level(FloorBundle::flat(meshes, materials));

    // this is a cube with an inital velocity from one of the avian examples
    commands.spawn_level((
        RigidBody::Dynamic,
        Collider::cuboid(1.0, 1.0, 1.0),
        AngularVelocity(Vec3::new(2.5, 3.5, 1.5)),
        Mesh3d(meshes.add(Cuboid::from_length(1.0))),
        MeshMaterial3d(materials.add(Color::srgb_u8(124, 144, 255))),
        Transform::from_xyz(0.0, 4.0, 0.0),
    ));

    // Atmospheric light - so you can see :)
    commands.spawn_level((
        PointLight { shadows_enabled: true, ..default() },
        Transform::from_xyz(4.0, 8.0, 4.0),
    ));

    // point light tied to the button
    ButtonLight::spawn(commands, ButtonLightParams {
        listens_for: 0,
        position: Vec3::new(2.0, 2.0, 0.0),
    });

    // Floor button prefab - slightly more intracate but not quite as advanced
    FloorButton::spawn(commands, meshes, materials, FloorButtonParams {
        id: 0,
        position: Vec3::new(0.0, 0.05, 0.0),
    });

    // This is the dialogue NPC. They become red when nearby and if you press interact,
    // you can spawn the dialogue box
    NPC::spawn(commands, meshes, materials, NPCParams {
        id: 0,
        position: Vec3::new(2.0, 0.05, 2.0),
        interact_fn: |w: &mut World| {
            w.resource_mut::<Messages<crate::menu::ChangeMenu>>()
                .write(crate::menu::ChangeMenu(menu_ids::DIALOGUE));
        },
    });

    // This is the "make a choice" NPC. They become red when nearby and if you press interact,
    // you can save the game
    // "making a choice" more or less is just here to express that you can have the
    // NPCs/interactible objects do more than one thing
    NPC::spawn(commands, meshes, materials, NPCParams {
        id: 1,
        position: Vec3::new(-2.0, 0.01, -2.0),
        interact_fn: |w: &mut World| {
            w.resource_mut::<Messages<crate::menu::ChangeMenu>>()
                .write(crate::menu::ChangeMenu(menu_ids::CHOICE_MENU));
        },
    });

    // Finally, the player bundle. Has the camera and everything else.
    Player::spawn(commands, meshes, materials, PlayerParams::default(), CameraParams::default());
}

pub struct Level1;
impl Level for Level1 {
    fn id() -> LevelId { LEVEL1 }
    fn data() -> LevelData {
        LevelData {
            spawn,
            startup: vec![
                LevelStartup::SetGameState(GameState::Playing),
                LevelStartup::ShowMenu(crate::menu::id::INGAME_UI),
            ],
        }
    }
}

