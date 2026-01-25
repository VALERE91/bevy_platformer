use bevy::post_process::bloom::Bloom;
use bevy::prelude::*;
use crate::player::BPPlayerMarker;

pub struct BPCameraPlugin;

impl Plugin for BPCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, camera_follow);
    }
}

#[derive(Component)]
pub struct BPCameraMarker;

#[derive(Component)]
pub struct BPCameraSpeed(pub f32);

#[derive(Bundle)]
pub struct BPCameraBundle {
    pub marker: BPCameraMarker,
    pub speed: BPCameraSpeed,
    pub camera: Camera2d,
    pub bloom: Bloom,
}

impl BPCameraBundle {
    pub fn new(speed: f32) -> Self {
        use bevy::post_process::bloom::Bloom;

        Self {
            marker: BPCameraMarker {},
            speed: BPCameraSpeed(speed),
            camera: Camera2d::default(),
            bloom: Bloom::NATURAL,
        }
    }
}

fn camera_follow(player_pos: Query<&Transform, With<BPPlayerMarker>>,
                 mut camera_query: Query<(&mut Transform, &BPCameraSpeed), (With<BPCameraMarker>, Without<BPPlayerMarker>)>,
                 time: Res<Time>) {
    if let Ok(player_pos) = player_pos.single() {
        if let Ok(mut camera_query) = camera_query.single_mut() {
            let target = Vec3::new(player_pos.translation.x,
                                   player_pos.translation.y,
                                   camera_query.0.translation.z);
            let cam_pos_2d = Vec2::new(camera_query.0.translation.x, camera_query.0.translation.y);
            let target_2d = Vec2::new(player_pos.translation.x, player_pos.translation.y);
            let alpha = cam_pos_2d.distance(target_2d) * camera_query.1.0 * time.delta().as_secs_f32();
            camera_query.0.translation = camera_query.0.translation.lerp(target, alpha);
        }
    }
}