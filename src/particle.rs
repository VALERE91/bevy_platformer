use bevy::prelude::*;
use rand::Rng;

// 1. The Resource to hold shared assets
#[derive(Resource)]
pub struct BPDustAssets {
    pub mesh: Handle<Mesh>,
    pub material: Handle<ColorMaterial>,
}

#[derive(Message)]
pub struct BPSpawnDustMessage {
    pub position: Vec2,
    pub count: usize,
}

#[derive(Component)]
pub struct BPDustParticle {
    pub velocity: Vec2,
    pub lifetime: Timer,
    pub shrink_speed: f32,
}

pub struct BPParticlePlugin;

impl Plugin for BPParticlePlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<BPSpawnDustMessage>()
            // Load assets at startup
            .add_systems(Startup, setup_dust_assets)
            .add_systems(Update, (spawn_dust_listener, update_particles));
    }
}

fn setup_dust_assets(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>
) {
    // Create the mesh and material ONCE
    commands.insert_resource(BPDustAssets {
        // We make a 1x1 rectangle so we can easily scale it later
        mesh: meshes.add(Rectangle::new(1.0, 1.0)),
        // White with transparency
        material: materials.add(Color::WHITE.with_alpha(0.5)),
    });
}

fn spawn_dust_listener(
    mut commands: Commands,
    mut events: MessageReader<BPSpawnDustMessage>,
    dust_assets: Res<BPDustAssets>, // <--- Read the cached assets
) {
    let mut rng = rand::rng();

    for event in events.read() {
        for _ in 0..event.count {
            let vx = rng.random_range(-50.0..50.0);
            let vy = rng.random_range(10.0..60.0);
            // Random size between 3px and 8px
            let size = rng.random_range(3.0..8.0);

            commands.spawn((
                BPDustParticle {
                    velocity: Vec2::new(vx, vy),
                    lifetime: Timer::from_seconds(0.5, TimerMode::Once),
                    shrink_speed: 2.0,
                },
                // Use the shared mesh!
                Mesh2d(dust_assets.mesh.clone()),
                // Use the shared material!
                MeshMaterial2d(dust_assets.material.clone()),
                // Scale the TRANSFORM to get the size we want
                Transform::from_xyz(event.position.x, event.position.y, 3.)
                    .with_scale(Vec3::splat(size)),
            ));
        }
    }
}

fn update_particles(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut Transform, &mut BPDustParticle)>,
) {
    for (entity, mut transform, mut particle) in &mut query {
        // 1. Move
        transform.translation.x += particle.velocity.x * time.delta_secs();
        transform.translation.y += particle.velocity.y * time.delta_secs();

        // 2. Shrink
        let scale_sub = particle.shrink_speed * time.delta_secs();
        transform.scale -= Vec3::splat(scale_sub);

        // 3. Age & Die
        particle.lifetime.tick(time.delta());
        if particle.lifetime.is_finished() || transform.scale.x <= 0.0 {
            commands.entity(entity).despawn();
        }
    }
}