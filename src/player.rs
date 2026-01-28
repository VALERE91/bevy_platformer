use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use leafwing_input_manager::prelude::*;
use rand::Rng;
use crate::BPLevelElement;
use crate::enemy::BPEnemyMarker;
use crate::particle::BPSpawnDustMessage;
use crate::physic::{PAWN_GROUP, PLAYER_GROUP, WORLD_GROUP};
use crate::state::BPGameState;
use crate::utils::BPGameCleanupMarker;

pub struct BPPlayerPlugin;

impl Plugin for BPPlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(InputManagerPlugin::<Action>::default())
            .add_systems(FixedUpdate, (move_player, jump_player, procedural_animation_system)
                .run_if(in_state(BPGameState::InGame)))
            .add_systems(Update, (handle_player_collision)
                .run_if(in_state(BPGameState::InGame)));
    }
}

#[derive(Actionlike, PartialEq, Eq, Hash, Clone, Copy, Debug, Reflect)]
pub enum Action {
    #[actionlike(Axis)]
    Run,
    Jump,
}

#[derive(Component)]
#[require(BPGameCleanupMarker)]
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
    pub velocity: Velocity,

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
            velocity: Velocity::default(),
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
                                 &Velocity,
                                 &Transform,
                                 &mut ExternalForce), With<BPPlayerMarker>>,
                mut particle_writer: MessageWriter<BPSpawnDustMessage>) {
    for (action_state,
        run_strength,
        velocity,
        transform,
        mut external_force) in &mut query {
        if let Some(axis_data) = action_state.axis_data(&Action::Run) {
            external_force.force = Vec2::new(axis_data.value * run_strength.0,0.);

            if velocity.linvel.x.abs() > 50.0 {
                // 10% chance per frame to spawn a dust mote
                if rand::rng().random_bool(0.1) {
                    particle_writer.write(BPSpawnDustMessage {
                        position: Vec2::new(transform.translation.x, transform.translation.y - 25.0),
                        count: 1,
                    });
                }
            }
        }
    }
}

fn jump_player(mut query: Query<(&ActionState<Action>,
                                 &BPPlayerJumpStrength,
                                 &Transform,
                                 &mut ExternalImpulse), With<BPPlayerMarker>>,
               mut particle_writer: MessageWriter<BPSpawnDustMessage>) {
    for(action_state, jump_strength, transform, mut external_impulse) in &mut query {
        if !action_state.just_pressed(&Action::Jump) {
            return;
        }

        external_impulse.impulse = Vec2::new(0., jump_strength.0);

        particle_writer.write(BPSpawnDustMessage {
            position: Vec2::new(transform.translation.x, transform.translation.y - 25.0), // -25 is feet
            count: 10,
        });
    }
}

fn handle_player_collision(mut commands: Commands,
                           mut collision_events: MessageReader<CollisionEvent>,
                           enemy_query: Query<&Transform, With<BPEnemyMarker>>,
                           world_query: Query<&Transform, With<BPLevelElement>>,
                           mut player_query: Query<(&Transform, &mut ExternalImpulse), With<BPPlayerMarker>>,
                           mut next_state: ResMut<NextState<BPGameState>>,
                           mut particle_writer: MessageWriter<BPSpawnDustMessage>) {
    for event in collision_events.read() {
        if let CollisionEvent::Started(e1, e2, _flags) = event {
            let mut player : Option<&Entity> = None;
            let mut enemy : Option<&Entity> = None;
            if player_query.contains(*e1) {
                player = Some(e1);
            } else if player_query.contains(*e2) {
                player = Some(e2);
            }

            if enemy_query.contains(*e1) {
                enemy = Some(e1);
            } else if enemy_query.contains(*e2) {
                enemy = Some(e2);
            }

            if player.is_none() {
                continue;
            }

            let Some(player) = player else { return; };

            if enemy.is_none() {
                let mut world_transform : Option<&Transform> = None;
                if let Ok(t) = world_query.get(*e1) {
                    world_transform = Some(t);
                }else if let Ok(t) = world_query.get(*e2) {
                    world_transform = Some(t);
                }

                if let Some(_) = world_transform {
                    // Spawn dust particles
                    if let Ok(player_query) = player_query.get(*player) {
                        let (transform, _) = player_query;
                        particle_writer.write(BPSpawnDustMessage {
                            position: transform.translation.xy() - Vec2::new(0.0, 25.0), // Feet pos
                            count: 5,
                        });
                    }
                }
            }

            let Some(enemy) = enemy else { return; };

            if let Ok(mut player_query) = player_query.get_mut(*player) {
                if let Ok(enemy_transform) = enemy_query.get(*enemy) {
                    if player_query.0.translation.y > enemy_transform.translation.y + 20. {
                        //Enemy dead
                        commands.entity(*enemy).despawn();
                        player_query.1.impulse = Vec2::new(0., 1000000.);
                        next_state.set(BPGameState::Victory);
                    }
                    else {
                        commands.entity(*player).despawn();
                        next_state.set(BPGameState::GameOver);
                    }
                }
            }
        }
    }
}

fn procedural_animation_system(
    mut query: Query<(&mut Transform, &Velocity), With<BPPlayerMarker>>,
    time: Res<Time>,
) {
    for (mut transform, velocity) in &mut query {
        // 1. IS JUMPING? (High vertical speed) -> Stretch
        let is_airborne = velocity.linvel.y.abs() > 50.0;

        // 2. IS RUNNING? (High horizontal speed + On Ground)
        let is_running = velocity.linvel.x.abs() > 50.0 && !is_airborne;

        let target_scale;

        if is_airborne {
            // --- JUMP STRETCH (Existing Logic) ---
            let stretch_factor = 1.0 + (velocity.linvel.y.abs() / 2000.0);
            let clamped_y = stretch_factor.clamp(0.8, 1.5);
            let clamped_x = (1.0 / clamped_y).clamp(0.6, 1.2);
            target_scale = Vec3::new(clamped_x, clamped_y, 1.0);

        } else if is_running {
            // --- RUNNING WADDLE (New Logic) ---
            // We use sin() to create a rhythm.
            // * 20.0 = Speed of the waddle
            // * 0.1  = Strength of the waddle (10% squash/stretch)
            let waddle = (time.elapsed_secs() * 20.0).sin() * 0.1;

            // When X grows, Y shrinks (volume preservation)
            target_scale = Vec3::new(
                1.0 + waddle, // Wide
                1.0 - waddle, // Short
                1.0
            );
        } else {
            // --- IDLE (Reset) ---
            target_scale = Vec3::ONE;
        }

        // Apply with Lerp for smoothness
        transform.scale = transform.scale.lerp(target_scale, 15.0 * time.delta().as_secs_f32());

        // --- BONUS: TILT (The "Lean") ---
        // If you are a square, this looks great. If you are a circle, you won't see this
        // unless you add "eyes" or the waddle makes you oval enough to notice.
        let lean_angle = (-velocity.linvel.x / 5000.0).clamp(-0.2, 0.2);
        transform.rotation = transform.rotation.lerp(
            Quat::from_rotation_z(lean_angle),
            10.0 * time.delta().as_secs_f32()
        );
    }
}