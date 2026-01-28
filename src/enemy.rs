use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use crate::physic::{INVISIBLE_WALL_GROUP, PAWN_GROUP, PLAYER_GROUP, WORLD_GROUP};
use crate::utils::BPGameCleanupMarker;

pub struct BPEnemyPlugin;

impl Plugin for BPEnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, enemy_movement_system);
    }
}

#[derive(Component)]
#[require(BPGameCleanupMarker)]
pub struct BPEnemyMarker;

#[derive(Component)]
pub struct BPEnemyDirection(pub f32);

#[derive(Bundle)]
pub struct BPEnemyBundle {
    // Markers & Logic
    pub marker: BPEnemyMarker,
    pub direction: BPEnemyDirection,

    // Physics
    pub rigid_body: RigidBody,
    pub locked_axes: LockedAxes,
    pub collider: Collider,
    pub external_force: ExternalForce,
    pub collision_groups: CollisionGroups,
    pub damping: Damping,

    // Visuals
    pub mesh: Mesh2d,
    pub material: MeshMaterial2d<ColorMaterial>,
    pub transform: Transform,
}

impl BPEnemyBundle {
    pub fn new(meshes: &mut ResMut<Assets<Mesh>>,
               materials: &mut ResMut<Assets<ColorMaterial>>) -> Self {

        let mut enemy_damping = Damping::default();
        enemy_damping.linear_damping = 5.;

        Self {
            direction: BPEnemyDirection(1.),
            marker: BPEnemyMarker {},
            mesh: Mesh2d(meshes.add(Rectangle::new(50., 50.))),
            material: MeshMaterial2d(materials.add(Color::srgb(8.25, 2.4, 2.1))), // RGB values exceed 1 to achieve a bright color for the bloom effect
            transform: Transform::from_xyz(0., -150., 2.),
            rigid_body: RigidBody::Dynamic,
            locked_axes: LockedAxes::ROTATION_LOCKED,
            collider: Collider::ball(25.),
            external_force: ExternalForce::default(),
            collision_groups: CollisionGroups::new(
                PAWN_GROUP,
                PLAYER_GROUP | WORLD_GROUP | PAWN_GROUP | INVISIBLE_WALL_GROUP,
            ),
            damping: enemy_damping
        }
    }
}

fn enemy_movement_system(mut query: Query<(&Transform, &mut ExternalForce, &mut BPEnemyDirection), With<BPEnemyMarker>>,
                         rapier_context: ReadRapierContext){

    let Ok(rapier_context) = rapier_context.single() else {
        return;
    };

    for(transform, mut external_force, mut direction) in &mut query {
        let ray_pos = Vec2::new(transform.translation.x, transform.translation.y);
        let ray_dir = Vec2::new(direction.0, 0.);
        let max_toi: bevy_rapier2d::prelude::Real = 60.0;
        let solid = true;
        let filter = QueryFilter::default()
            .groups(CollisionGroups::new(PAWN_GROUP, WORLD_GROUP | INVISIBLE_WALL_GROUP));

        if let Some((_entity, toi)) = rapier_context.cast_ray(ray_pos, ray_dir, max_toi, solid, filter) {
            // If we hit something close, reverse direction
            let toi: f32 = toi.into();
            if toi < 50.0 {
                direction.0 *= -1.;
            }
        }

        external_force.force = Vec2::new(direction.0 * 4000000., 0.);
    }
}