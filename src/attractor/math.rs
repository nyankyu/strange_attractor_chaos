use nannou::prelude::*;

fn solution_equation_2d(a: f32, b: f32, c: f32) -> Option<(f32, f32)> {
    if a.is_zero() {
        return None;
    }

    let root = (b * b - 4.0 * a * c).sqrt();
    if root.is_nan() {
        return None;
    }

    Some(((-b + root) / 2.0 / a, (-b - root) / 2.0 / a))
}

pub fn cross_points(p1: Vec3A, p2: Vec3A, c: Vec3A, r: f32) -> Option<(Vec3A, Vec3A)> {
    //return Some((p1, p2));
    let (t1, t2) = solution_equation_2d(
        p1.distance_squared(p2),
        2.0 * ((p1.x - c.x) * (p2.x - p1.x)
            + (p1.y - c.y) * (p2.y - p1.y)
            + (p1.z - c.z) * (p2.z - p1.z)),
        p1.distance_squared(c) - r * r,
    )?;

    Some((t1 * (p2 -p1) + p1, t2 * (p2 - p1) + p1))
}
