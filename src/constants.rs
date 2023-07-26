use bevy::prelude::Vec3;

pub const CAMERA_INIT: Vec3 = Vec3::new(-90., 500.0, 90.0);
pub const CAMERA_LOOK_AT: Vec3 = Vec3::new(0.0, 0.0, 0.0);

pub const CAMERA_TRANSLATION_SPEED: f32 = 10.0; // 1000.0
pub const CAMERA_DISTANCE_BEHIND: f32 = 10.0;

pub const CAR_SIZE: Vec3 = Vec3::new(0.5, 0.3, 0.935);
pub const CAR_SPEED: f32 = 5000.0; // 50000.0
pub const CAR_ROTATE_SPEED: f32 = 520.0; // 5200

pub const TORQUE_IMPULSE_COUP: Vec3 = Vec3::new(10., 0., 0.);

pub const SUSPENSION_STRENGTH: f32 = 1500.0; // 15000
pub const SUSPENSION_DAMPING: f32 = 1200.0;

pub const FORCE_TORQUE_ADD: f32 = 30.0;
