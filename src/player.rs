use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use leafwing_input_manager::prelude::*;
use crate::physic::{PAWN_GROUP, PLAYER_GROUP, WORLD_GROUP};

pub struct BPPlayerPlugin;

impl Plugin for BPPlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(InputManagerPlugin::<Action>::default())
            .add_systems(FixedUpdate, (move_player, jump_player));
    }
}

#[derive(Actionlike, PartialEq, Eq, Hash, Clone, Copy, Debug, Reflect)]
pub enum Action {
    #[actionlike(Axis)]
    Run,
    Jump,
}

#[derive(Component)]
pub struct BPPlayerMarker;

#[derive(Component)]
pub struct BPPlayerRunStrength(pub f32);

#[derive(Component)]
pub struct BPPlayerJumpStrength(pub f32);

#[derive(Bundle)]
pub struct BPPlayerBundle {
    // Markers & Logic
    pub marker: BPPlayerMarker,
    pub run_strength: BPPlayerRunStrength,
    pub jump_strength: BPPlayerJumpStrength,

    // Physics
    pub rigid_body: RigidBody,
    pub collider: Collider,
    pub locked_axes: LockedAxes,
    pub external_force: ExternalForce,
    pub external_impulse: ExternalImpulse,
    pub damping: Damping,
    pub restitution: Restitution,
    //pub mass: AdditionalMassProperties,
    pub collision_groups: CollisionGroups,

    // Input (The Bundle from Leafwing)
    pub input_map: InputMap<Action>,

    // Visuals
    pub mesh: Mesh2d,
    pub material: MeshMaterial2d<ColorMaterial>,
    pub transform: Transform,
}

impl BPPlayerBundle {
    pub fn new(meshes: &mut ResMut<Assets<Mesh>>,
               materials: &mut ResMut<Assets<ColorMaterial>>) -> Self {

        let input_map = InputMap::default()
            .with_axis(Action::Run, VirtualAxis::new(KeyCode::KeyA, KeyCode::KeyD))
            .with(Action::Jump, KeyCode::Space);

        let mut player_damping = Damping::default();
        player_damping.linear_damping = 5.;

        Self {
            marker: BPPlayerMarker {},
            jump_strength: BPPlayerJumpStrength(1000000.),
            run_strength: BPPlayerRunStrength(9000000.),
            mesh: Mesh2d(meshes.add(Circle::new(25.))),
            material: MeshMaterial2d(materials.add(Color::srgb(5.25, 8.4, 8.1))), // RGB values exceed 1 to achieve a bright color for the bloom effect
            transform: Transform::from_xyz(0., 0., 2.),
            rigid_body: RigidBody::Dynamic,
            locked_axes: LockedAxes::ROTATION_LOCKED,
            collider: Collider::ball(25.),
            restitution: Restitution::coefficient(0.1),
            //mass: AdditionalMassProperties::Mass(0.1),
            external_force: ExternalForce::default(),
            external_impulse: ExternalImpulse::default(),
            damping: player_damping,
            input_map,
            collision_groups: CollisionGroups::new(
                PLAYER_GROUP,
                PLAYER_GROUP | WORLD_GROUP | PAWN_GROUP,
            )
        }
    }
}

fn move_player(mut query: Query<(&ActionState<Action>,
                                 &BPPlayerRunStrength,
                                 &mut ExternalForce), With<BPPlayerMarker>>) {
    for (action_state, run_strength, mut external_force) in &mut query {
        if let Some(axis_data) = action_state.axis_data(&Action::Run) {
            external_force.force = Vec2::new(axis_data.value * run_strength.0,0.);
        }
    }
}

fn jump_player(mut query: Query<(&ActionState<Action>,
                                 &BPPlayerJumpStrength,
                                 &mut ExternalImpulse), With<BPPlayerMarker>>) {
    for(action_state, jump_strength, mut external_impulse) in &mut query {
        if !action_state.just_pressed(&Action::Jump) {
            return;
        }

        external_impulse.impulse = Vec2::new(0., jump_strength.0);
    }
}