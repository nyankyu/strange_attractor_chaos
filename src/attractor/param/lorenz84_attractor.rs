use crate::AttractorParam;
use nannou::glam::const_vec3a;
use nannou::prelude::*;

const A: f32 = 0.25;
const B: f32 = 4.0;
const F: f32 = 8.0;
const G: f32 = 1.0;

pub(crate) struct Lorenz84Attractor {}

impl AttractorParam for Lorenz84Attractor {
    const ANGLE_OF_VIEW: f32 = 100.0 / 180.0 * PI;

    const DELTA_T: f32 = 0.0003;

    const CAMERA: Vec3A = const_vec3a!([-1.7, 0.0, 0.0]);
    const CENTER: Vec3A = const_vec3a!([1.0, 0.0, 0.0]);

    const DELTA_THETA: f32 = 0.0003;

    const ROTAION_X: f32 = -1.3;
    const ROTAION_Y: f32 = -4.9;
    const ROTAION_Z: f32 = 1.0;

    const LINES_LEN: usize = 5_000;

    const LINE_WEIGHT: f32 = 1.9;

    const COLOR_HUE1: f32 = 0.07;
    const INIT_START_END1: (Vec3A, Vec3A) = (const_vec3a!([1.0, 1.0, 1.0]), const_vec3a!([0.5, 0.5, 0.5]));

    const COLOR_HUE2: f32 = 0.5;
    const INIT_START_END2: (Vec3A, Vec3A) = (const_vec3a!([1.0, -1.0, 2.0]), const_vec3a!([-0.5, 1.5, 0.5]));

    const RADIUS: f32 = 2.5;

    fn new() -> Self {
        Lorenz84Attractor {}
    }

    fn slope(p: Vec3A) -> Vec3A {
        let dx = -A * p.x - p.y * p.y - p.z * p.z + A * F;
        let dy = -p.y + p.x * p.y - B * p.x * p.z + G;
        let dz = -p.z + B * p.x * p.y + p.x * p.z;
        vec3a(dx, dy, dz)
    }
}
