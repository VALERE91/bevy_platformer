mod player;
mod camera;
mod debug;
mod enemy;
mod physic;
mod state;
mod ui;
mod utils;
mod particle;

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use crate::camera::{BPCameraBundle, BPCameraPlugin};
use crate::debug::BPDebugPlugin;
use crate::enemy::{BPEnemyBundle, BPEnemyPlugin};
use crate::player::{BPPlayerBundle, BPPlayerPlugin};
use crate::state::BPGameState;
use crate::ui::BPUIPlugin;
use crate::utils::BPGameCleanupMarker;

fn main() {
    let mut app = App::new();

    app
        .add_plugins(DefaultPlugins)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugins(BPPlayerPlugin)
        .add_plugins(particle::BPParticlePlugin)
        .add_plugins(BPCameraPlugin)
        .add_plugins(BPEnemyPlugin)
        .add_plugins(BPUIPlugin)
        .add_systems(Startup, setup)
        .add_systems(OnEnter(BPGameState::InGame), setup_game)
        .add_systems(OnExit(BPGameState::InGame), cleanup_game);

    app.init_state::<BPGameState>();

    if cfg!(debug_assertions) {
        app.add_plugins(BPDebugPlugin);
    }

    app.run();
}

#[derive(Component)]
#[require(BPGameCleanupMarker)]
struct BPLevelElement;

fn setup(mut commands: Commands) {
    // Spawn the camera
    commands.spawn(BPCameraBundle::new(5.0));
}

fn setup_game(mut commands: Commands,
              mut meshes: ResMut<Assets<Mesh>>,
              mut materials: ResMut<Assets<ColorMaterial>>,) {
    //Spawn the player
    commands.spawn(BPPlayerBundle::new(&mut meshes, &mut materials));
    //Spawn the enemy
    commands.spawn(BPEnemyBundle::new(&mut meshes, &mut materials));

    // Spawn the ground
    commands.spawn((
        BPLevelElement,
        Mesh2d(meshes.add(Rectangle::new(1500.0, 50.0))),
        MeshMaterial2d(materials.add(Color::srgb(34.0/255.0, 34.0/255.0, 34.0/255.0))),
        Transform::from_xyz(0., -200., 0.),
        RigidBody::Fixed,
        Collider::cuboid(750., 25.),
        CollisionGroups::new(physic::WORLD_GROUP, physic::WORLD_GROUP | physic::PLAYER_GROUP | physic::PAWN_GROUP),
    ));

    //Spawn the invisible wall
    commands.spawn((
        BPLevelElement,
        Transform::from_xyz(700., -150., 0.),
        Sensor::default(),
        Collider::cuboid(10., 25.),
        CollisionGroups::new(physic::INVISIBLE_WALL_GROUP, physic::WORLD_GROUP | physic::PLAYER_GROUP | physic::PAWN_GROUP),
    ));

    //Spawn the invisible wall
    commands.spawn((
        BPLevelElement,
        Transform::from_xyz(-700., -150., 0.),
        Sensor::default(),
        Collider::cuboid(10., 25.),
        CollisionGroups::new(physic::INVISIBLE_WALL_GROUP, physic::WORLD_GROUP | physic::PLAYER_GROUP | physic::PAWN_GROUP),
    ));
}

fn cleanup_game(mut commands: Commands, cleanup_query: Query<Entity, With<BPGameCleanupMarker>>) {
    for entity in &cleanup_query {
        commands.entity(entity).despawn();
    }
}