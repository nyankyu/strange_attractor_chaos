mod param;
mod line;
mod math;

use nannou::{
    glam::{EulerRot, Vec3Swizzles},
    prelude::*,
};
use std::marker::PhantomData;

use crate::WINDOW_W;

#[allow(unused_imports)]
pub(crate) use param::lorenz_attractor::LorenzAttractor;
#[allow(unused_imports)]
pub(crate) use param::halvorsen_attractor::HalvorsenAttractor;
#[allow(unused_imports)]
pub(crate) use param::dadras_attractor::DadrasAttractor;
#[allow(unused_imports)]
pub(crate) use param::thomas_attractor::ThomasAttractor;
/*
#[allow(unused_imports)]
pub(crate) use param::langford_attractor::LangfordAttractor;
#[allow(unused_imports)]
pub(crate) use param::lorenz84_attractor::Lorenz84Attractor;
#[allow(unused_imports)]
pub(crate) use param::burke_shaw_attractor::BurkeShawAttractor;
*/

use line::Line;

pub(crate) trait AttractorParam {
    const ANGLE_OF_VIEW: f32;

    const LINES_LEN: usize;
    const LINE_WEIGHT: f32;

    const DELTA_T: f32;

    /// direction right: +y, left: -y, top: +z, bottom: -z, front: +x, back: -x
    const CAMERA: Vec3A;
    const CENTER: Vec3A;

    const DELTA_THETA: f32;

    const ROTAION_X: f32;
    const ROTAION_Y: f32;
    const ROTAION_Z: f32;

    const COLOR_HUE1: f32;
    const INIT_START_END1: (Vec3A, Vec3A);

    const COLOR_HUE2: f32;
    const INIT_START_END2: (Vec3A, Vec3A);

    const RADIUS: f32;

    fn new() -> Self;
    fn slope(p: Vec3A) -> Vec3A;
}

pub(crate) struct Attractor<Param: AttractorParam> {
    _param: PhantomData<fn() -> Param>,
    line1: Line<Param>,
    line2: Line<Param>,
    theta: f32,
    rotation: Quat,
    distance_screen: f32,
}

impl<Param: AttractorParam> Attractor<Param> {
    pub(crate) fn new() -> Self {
        Attractor {
            _param: PhantomData,
            line1: Line::new(1),
            line2: Line::new(2),
            theta: 0.0,
            rotation: Quat::IDENTITY,
            distance_screen: WINDOW_W as f32 * 0.5 / (0.5 * Param::ANGLE_OF_VIEW).tan(),
        }
    }

    pub(crate) fn update(&mut self) {
        self.line1.update();
        self.line2.update();
        self.theta += Param::DELTA_THETA;
        self.rotation = Quat::from_euler(
            EulerRot::ZYX,
            self.theta * Param::ROTAION_X,
            self.theta * Param::ROTAION_Y,
            self.theta * Param::ROTAION_Z,
        );
    }
    pub(crate) fn draw(&self, draw: &Draw) {
        self.line1.draw(draw, self.rotation, self.distance_screen);
        self.line2.draw(draw, self.rotation, self.distance_screen);
        if !crate::RECORDING {
            self.draw_axis(&draw);
        }
    }

    fn draw_axis(&self, draw: &Draw) {
        draw.line()
            .weight(5.0)
            .color(WHITE)
            .end(self.coordinate(Param::CENTER + Vec3A::AXES[0]))
            .start(self.coordinate(Param::CENTER));
        draw.line()
            .weight(5.0)
            .color(BLUE)
            .end(self.coordinate(Param::CENTER + Vec3A::AXES[1]))
            .start(self.coordinate(Param::CENTER));
        draw.line()
            .weight(5.0)
            .color(GREEN)
            .end(self.coordinate(Param::CENTER + Vec3A::AXES[2]))
            .start(self.coordinate(Param::CENTER));
        draw.ellipse()
            .radius(5.0)
            .color(WHITE)
            .xy(self.coordinate(Param::CENTER));
    }

    fn coordinate(&self, p: Vec3A) -> Vec2 {
        let rotated = self.rotation * (p - Param::CENTER) + Param::CENTER;
        let depth = rotated.x - Param::CAMERA.x;
        let scale = self.distance_screen / depth;
        (rotated.yz() - Param::CAMERA.yz()) * scale - Param::CAMERA.yz()
    }
}
