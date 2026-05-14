use avian3d::prelude::*;
use bevy::prelude::*;

// Example of prefab pattern given this framework:

#[derive(Bundle)]
pub struct FloorBundle {
    rigid_body: RigidBody,
    collider: Collider,
    mesh: Mesh3d,
    material: MeshMaterial3d<StandardMaterial>,
}

impl FloorBundle {
    pub fn flat(meshes: &mut Assets<Mesh>, materials: &mut Assets<StandardMaterial>) -> Self {
        Self {
            rigid_body: RigidBody::Static,
            collider: Collider::cylinder(40.0, 0.1),
            mesh: Mesh3d(meshes.add(Cylinder::new(40.0, 0.1))),
            material: MeshMaterial3d(materials.add(Color::WHITE)),
        }
    }
}
