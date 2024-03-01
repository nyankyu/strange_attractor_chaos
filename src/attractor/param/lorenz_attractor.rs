use crate::AttractorParam;
use nannou::glam::const_vec3a;
use nannou::prelude::*;

const SIGMA: f32 = 10.0;
const BETA: f32 = 8.0 / 3.0;
const RHO: f32 = 28.0;

pub(crate) struct LorenzAttractor {}

impl AttractorParam for LorenzAttractor {
    const ANGLE_OF_VIEW: f32 = 90.0 / 180.0 * PI;

    const LINES_LEN: usize = 5_000;
    const LINE_WEIGHT: f32 = 1.7;

    const DELTA_T: f32 = 0.001;

    const CAMERA: Vec3A = const_vec3a!([-50.0, 10.0, 20.0]);
    const CENTER: Vec3A = const_vec3a!([0.0, 10.0, 20.0]);

    const DELTA_THETA: f32 = 0.0003;

    const ROTAION_X: f32 = 7.9;
    const ROTAION_Y: f32 = 1.1;
    const ROTAION_Z: f32 = 1.5;

    const COLOR_HUE1: f32 = 0.0;
    const COLOR_HUE2: f32 = 0.5;

    const INIT_START_END1: (Vec3A, Vec3A) = (const_vec3a!([2.0, 2.0, 20.0]), const_vec3a!([1.0, 1.0, 10.0]));
    const INIT_START_END2: (Vec3A, Vec3A) = (const_vec3a!([1.0, -1.0, 20.0]), const_vec3a!([2.0, 2.0, 10.0]));

    const RADIUS: f32 = 45.0;

    fn new() -> Self {
        LorenzAttractor {}
    }

    fn slope(p: Vec3A) -> Vec3A {
        let dx = SIGMA * (p.y - p.x);
        let dy = p.x * (RHO - p.z) - p.y;
        let dz = p.x * p.y - BETA * p.z;
        vec3a(dx, dy, dz)
    }
}
