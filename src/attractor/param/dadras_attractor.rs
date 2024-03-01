use crate::AttractorParam;
use nannou::glam::const_vec3a;
use nannou::prelude::*;

const A: f32 = 3.0;
const B: f32 = 2.7;
const C: f32 = 1.7;
const D: f32 = 2.0;
const E: f32 = 9.0;

pub(crate) struct DadrasAttractor {}

impl AttractorParam for DadrasAttractor {
    const ANGLE_OF_VIEW: f32 = 90.0 * (PI / 180.0);

    const DELTA_T: f32 = 0.0004;

    const CAMERA: Vec3A = const_vec3a!([-10.0, 0.0, 0.0]);
    const CENTER: Vec3A = const_vec3a!([4.0, 0.0, 0.0]);

    const DELTA_THETA: f32 = 0.0003;

    const ROTAION_X: f32 = 1.0;
    const ROTAION_Y: f32 = 3.9;
    const ROTAION_Z: f32 = 2.9;

    const LINES_LEN: usize = 5_000;

    const LINE_WEIGHT: f32 = 1.7;

    const COLOR_HUE1: f32 = 0.3;
    const INIT_START_END1: (Vec3A, Vec3A) = (
        const_vec3a!([0.5, 0.5, 0.5]),
        const_vec3a!([1.0, 1.0, 1.0])
    );

    const COLOR_HUE2: f32 = 0.5;
    const INIT_START_END2: (Vec3A, Vec3A) = (
        const_vec3a!([-1.0, -1.0, 1.0]),
        const_vec3a!([-1.0, -1.0, 1.5]),
    );

    const RADIUS: f32 = 14.0;

    fn new() -> Self {
        DadrasAttractor {}
    }

    fn slope(p: Vec3A) -> Vec3A {
        let dx = p.y - A * p.x + B * p.y * p.z;
        let dy = C * p.y + p.z * (1.0 - p.x);
        let dz = D * p.x * p.y - E * p.z;
        vec3a(dx, dy, dz)
    }
}
