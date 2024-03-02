use crate::AttractorParam;
use nannou::glam::const_vec3a;
use nannou::prelude::*;

const S: f32 = 10.0;
const V: f32 = 4.272;

pub(crate) struct BurkeShawAttractor {}

impl AttractorParam for BurkeShawAttractor {
    const ANGLE_OF_VIEW: f32 = 90.0 / 180.0 * PI;

    const DELTA_T: f32 = 0.0003;

    const CAMERA: Vec3A = const_vec3a!([-2.0, 0.0, 0.0]);
    const CENTER: Vec3A = const_vec3a!([0.0, 0.0, 0.0]);

    const DELTA_THETA: f32 = 0.0003;

    const ROTAION_X: f32 = -1.3;
    const ROTAION_Y: f32 = -7.9;
    const ROTAION_Z: f32 = 1.0;

    const LINES_LEN: usize = 5_000;

    const LINE_WEIGHT: f32 = 1.9;

    const COLOR_HUE1: f32 = 0.9;
    const INIT_START_END1: (Vec3A, Vec3A) = (const_vec3a!([0.5, 0.5, 0.5]), const_vec3a!([0.5, 0.4, 0.1]));

    const COLOR_HUE2: f32 = 0.7;
    const INIT_START_END2: (Vec3A, Vec3A) = (const_vec3a!([0.0, 0.9, 0.5]), const_vec3a!([0.5, 0.0, 0.1]));

    const RADIUS: f32 = 2.0;

    fn new() -> Self {
        BurkeShawAttractor {}
    }

    fn slope(p: Vec3A) -> Vec3A {
        let dx = - S * (p.x + p.y);
        let dy = - p.y - S * p.x * p.z;
        let dz = S * p.x * p.y + V;
        vec3a(dx, dy, dz)
    }
}
