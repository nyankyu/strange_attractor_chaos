use crate::AttractorParam;
use nannou::glam::const_vec3a;
use nannou::prelude::*;

const B: f32 = 0.11;

pub(crate) struct ThomasAttractor {}

impl AttractorParam for ThomasAttractor {
    const ANGLE_OF_VIEW: f32 = 90.0 * (PI / 180.0);

    const DELTA_T: f32 = 0.003;

    const CAMERA: Vec3A = const_vec3a!([-6.5, 0.0, 0.0]);
    const CENTER: Vec3A = const_vec3a!([0.0, 0.0, 0.0]);

    const DELTA_THETA: f32 = 0.0004;

    const ROTAION_X: f32 = 1.0;
    const ROTAION_Y: f32 = 4.9;
    const ROTAION_Z: f32 = 1.3;

    const LINES_LEN: usize = 5_000;

    const LINE_WEIGHT: f32 = 1.5;

    const COLOR_HUE1: f32 = 0.9;
    const INIT_START_END1: (Vec3A, Vec3A) = (
        const_vec3a!([0.0, 0.0, 1.0]),
        const_vec3a!([0.0, 2.0, 2.0]),
    );

    const COLOR_HUE2: f32 = 0.6;
    const INIT_START_END2: (Vec3A, Vec3A) = (
        const_vec3a!([2.0, 2.0, 1.0]),
        const_vec3a!([1.0, 1.0, 0.0]),
    );

    const RADIUS: f32 = 6.0;

    fn new() -> Self {
        ThomasAttractor {}
    }

    fn slope(p: Vec3A) -> Vec3A {
        let dx = p.y.sin() - B * p.x;
        let dy = p.z.sin() - B * p.y;
        let dz = p.x.sin() - B * p.z;
        vec3a(dx, dy, dz)
    }
}
