use rand::distributions::Uniform;
use rand::Rng;

pub const PI: f32 = std::f32::consts::PI;
pub const INFINITY: f32 = std::f32::INFINITY;

pub fn degrees_to_radians(degrees: f32) -> f32 {
    degrees * std::f32::consts::PI / 180.0
}

pub fn random() -> f32 {
    rand::thread_rng().sample(Uniform::new(0.0, 1.0))
}

pub fn random_range(min: f32, max: f32) -> f32 {
    rand::thread_rng().sample(Uniform::new(min, max))
}

pub fn clamp(value: f32, min: f32, max: f32) -> f32 {
    if value < min {
        min
    } else if value > max {
        max
    } else {
        value
    }
}
