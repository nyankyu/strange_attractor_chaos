use std::{collections::VecDeque, marker::PhantomData};

use nannou::{glam::Vec3Swizzles, prelude::*};

use super::math::*;
use super::AttractorParam;

pub(super) struct Line<Param: AttractorParam> {
    _param: PhantomData<fn() -> Param>,
    starts: VecDeque<Vec3A>,
    ends: VecDeque<Vec3A>,
    hue_mini: f32,
}

impl<Param: AttractorParam> Line<Param> {
    pub(super) fn new(i: usize) -> Self {
        let (init_start, init_end) = match i {
            1 => Param::INIT_START_END1,
            2 => Param::INIT_START_END2,
            _ => (Vec3A::ZERO, Vec3A::ZERO),
        };

        let mut starts = VecDeque::with_capacity(Param::LINES_LEN + 1);
        let mut ends = VecDeque::with_capacity(Param::LINES_LEN + 1);

        starts.push_back(init_start);
        ends.push_back(init_end);

        for _ in 1..Param::LINES_LEN {
            starts.push_back(Self::runge_kutta4(*starts.back().unwrap()));
            ends.push_back(Self::runge_kutta4(*ends.back().unwrap()));
        }

        Line {
            _param: PhantomData,
            starts,
            ends,
            hue_mini: match i {
                1 => Param::COLOR_HUE1,
                2 => Param::COLOR_HUE2,
                _ => 0.0,
            },
        }
    }

    pub(super) fn update(&mut self) {
        for _ in 0..10 {
            let next_start = Self::runge_kutta4(*self.starts.back().unwrap());
            self.starts.push_back(next_start);

            let next_end = Self::runge_kutta4(*self.ends.back().unwrap());
            self.ends.push_back(next_end);

            if self.starts.len() > Param::LINES_LEN {
                self.starts.pop_front();
                self.ends.pop_front();
            }
        }
    }

    pub(super) fn draw(&self, draw: &Draw, rotation: Quat, distance_screen: f32) {
        for i in 0..Param::LINES_LEN {
            let hue = map_range(i, 0, Param::LINES_LEN, self.hue_mini, self.hue_mini + 0.19);
            let alpha = if i < Param::LINES_LEN / 2 {
                map_range(i, 0, Param::LINES_LEN / 2, 0.0, 0.01)
            } else {
                map_range(i, Param::LINES_LEN / 2, Param::LINES_LEN, 0.01, 0.0)
            };

            match cross_points(self.starts[i], self.ends[i], Param::CENTER, Param::RADIUS) {
                Some((start, end)) => {
                    draw.line()
                        .start(Self::coordinate(start, rotation, distance_screen))
                        .end(Self::coordinate(end, rotation, distance_screen))
                        .weight(Param::LINE_WEIGHT)
                        .color(hsla(hue, 1.0, 0.5, alpha));
                }
                None => continue,
            }
        }
    }

    fn coordinate(p: Vec3A, rotation: Quat, distance_screen: f32) -> Vec2 {
        let rotated = rotation * (p - Param::CENTER) + Param::CENTER;
        let depth = rotated.x - Param::CAMERA.x;
        let scale = distance_screen / depth;
        (rotated.yz() - Param::CAMERA.yz()) * scale - Param::CAMERA.yz()
    }

    fn runge_kutta4(p: Vec3A) -> Vec3A {
        let k1 = Param::DELTA_T * Param::slope(p);
        let k2 = Param::DELTA_T * Param::slope(p + 0.5 * k1);
        let k3 = Param::DELTA_T * Param::slope(p + 0.5 * k2);
        let k4 = Param::DELTA_T * Param::slope(p + k3);
        p + (k1 + k2 + k2 + k3 + k3 + k4) / 6.0
    }
}
