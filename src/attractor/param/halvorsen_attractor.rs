use crate::AttractorParam;
use nannou::glam::const_mat3a;
use nannou::glam::const_vec3a;
use nannou::prelude::*;

const A: f32 = 1.3;
const MAT: Mat3A = const_mat3a!([-A, -4.0, -4.0], [-4.0, -A, -4.0], [-4.0, -4.0, -A]);

pub(crate) struct HalvorsenAttractor {}

impl AttractorParam for HalvorsenAttractor {
    const ANGLE_OF_VIEW: f32 = 95.0 * (PI / 180.0);

    const DELTA_T: f32 = 0.0002;

    const CAMERA: Vec3A = const_vec3a!([-18.0, -6.0, -6.0]);
    const CENTER: Vec3A = const_vec3a!([0.0, -6.0, -6.0]);

    const DELTA_THETA: f32 = 0.0003;

    const ROTAION_X: f32 = -7.0;
    const ROTAION_Y: f32 = 3.3;
    const ROTAION_Z: f32 = 1.3;

    const LINES_LEN: usize = 8_000;

    const LINE_WEIGHT: f32 = 1.7;

    const COLOR_HUE1: f32 = 0.4;
    const INIT_START_END1: (Vec3A, Vec3A) = (
        const_vec3a!([-1.0, 1.0, 0.0]),
        const_vec3a!([1.0, 0.0, 1.0]),
    );

    const COLOR_HUE2: f32 = 0.7;
    const INIT_START_END2: (Vec3A, Vec3A) = (
        const_vec3a!([1.0, 0.0, 0.0]),
        const_vec3a!([1.0, -1.0, -1.0]),
    );

    const RADIUS: f32 = 15.0;

    fn new() -> Self {
        HalvorsenAttractor {}
    }

    fn slope(p: Vec3A) -> Vec3A {
        MAT * p - vec3a(p.y * p.y, p.z * p.z, p.x * p.x)
    }
}
