mod player;
mod camera;
mod debug;
mod enemy;
mod physic;

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use crate::camera::{BPCameraBundle, BPCameraPlugin};
use crate::debug::BPDebugPlugin;
use crate::player::{BPPlayerBundle, BPPlayerPlugin};

fn main() {
    let mut app = App::new();
    app
        .add_plugins(DefaultPlugins)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugins(BPPlayerPlugin)
        .add_plugins(BPCameraPlugin)
        .add_systems(Startup, setup);

    if cfg!(debug_assertions) {
        app.add_plugins(BPDebugPlugin);
    }

    app.run();
}

fn setup(mut commands: Commands,
         mut meshes: ResMut<Assets<Mesh>>,
         mut materials: ResMut<Assets<ColorMaterial>>,) {
    // Spawn the camera
    commands.spawn(BPCameraBundle::new(2.0));

    // Spawn the ground
    commands.spawn((
        Mesh2d(meshes.add(Rectangle::new(1500.0, 50.0))),
        MeshMaterial2d(materials.add(Color::srgb(34.0/255.0, 34.0/255.0, 34.0/255.0))),
        Transform::from_xyz(0., -200., 0.),
        RigidBody::Fixed,
        Collider::cuboid(750., 25.),
        CollisionGroups::new(physic::WORLD_GROUP, physic::WORLD_GROUP | physic::PLAYER_GROUP | physic::PAWN_GROUP),
    ));

    //Spawn the player
    commands.spawn(BPPlayerBundle::new(meshes, materials));
}

