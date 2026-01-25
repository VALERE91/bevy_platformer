use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use leafwing_input_manager::prelude::*;
use crate::enemy::BPEnemyMarker;
use crate::physic::{PAWN_GROUP, PLAYER_GROUP, WORLD_GROUP};

pub struct BPPlayerPlugin;

impl Plugin for BPPlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(InputManagerPlugin::<Action>::default())
            .add_systems(FixedUpdate, (move_player, jump_player))
            .add_systems(Update, handle_player_collision);
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
    pub collision_groups: CollisionGroups,
    pub physic_events: ActiveEvents,
    pub gravity_scale: GravityScale,

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
            jump_strength: BPPlayerJumpStrength(2500000.),
            run_strength: BPPlayerRunStrength(9500000.),
            mesh: Mesh2d(meshes.add(Circle::new(25.))),
            material: MeshMaterial2d(materials.add(Color::srgb(5.25, 8.4, 8.1))), // RGB values exceed 1 to achieve a bright color for the bloom effect
            transform: Transform::from_xyz(0., 0., 2.),
            rigid_body: RigidBody::Dynamic,
            locked_axes: LockedAxes::ROTATION_LOCKED,
            collider: Collider::ball(25.),
            restitution: Restitution::coefficient(0.1),
            physic_events: ActiveEvents::COLLISION_EVENTS,
            external_force: ExternalForce::default(),
            gravity_scale: GravityScale(3.0),
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

fn handle_player_collision(mut commands: Commands,
                           mut collision_events: MessageReader<CollisionEvent>,
                           enemy_query: Query<&Transform, With<BPEnemyMarker>>,
                           mut player_query: Query<(&Transform, &mut ExternalImpulse), With<BPPlayerMarker>>,) {
    for event in collision_events.read() {
        if let CollisionEvent::Started(e1, e2, _flags) = event {

            let mut player : Option<&Entity> = None;
            let mut enemy : Option<&Entity> = None;;
            if player_query.contains(*e1) && enemy_query.contains(*e2) {
                player = Some(e1);
                enemy = Some(e2);
            } else if player_query.contains(*e2) && enemy_query.contains(*e1) {
                player = Some(e2);
                enemy = Some(e1);
            }

            if player.is_none() || enemy.is_none() {
                return;
            }
            let Some(player) = player else { return; };
            let Some(enemy) = enemy else { return; };

            if let Ok(mut player_query) = player_query.get_mut(*player) {
                if let Ok(enemy_transform) = enemy_query.get(*enemy) {
                    if player_query.0.translation.y > enemy_transform.translation.y + 20. {
                        //Enemy dead
                        commands.entity(*enemy).despawn();
                        player_query.1.impulse = Vec2::new(0., 1000000.);
                    }
                    else {
                        commands.entity(*player).despawn();
                    }
                }
            }
        }
    }
}