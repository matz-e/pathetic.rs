extern crate rand;

use self::rand::prelude::*;
use std::ops;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Point(f32, f32, f32);

impl Point {
    pub fn new(x: f32, y: f32, z: f32) -> Point {
        Point(x, y, z)
    }

    #[inline]
    pub fn x(&self) -> f32 {
        self.0
    }

    #[inline]
    pub fn y(&self) -> f32 {
        self.1
    }

    #[inline]
    pub fn z(&self) -> f32 {
        self.2
    }

    pub fn cross(self, other: Point) -> Point {
        Point::new(
            self.1 * other.2 - self.2 * other.1,
            self.2 * other.0 - self.0 * other.2,
            self.0 * other.1 - self.1 * other.0,
        )
    }

    pub fn norm_sqr(self) -> f32 {
        self.0 * self.0 + self.1 * self.1 + self.2 * self.2
    }

    pub fn norm(self) -> f32 {
        self.norm_sqr().sqrt()
    }

    pub fn normalized(self) -> Point {
        self / self.norm()
    }

    pub fn perpendicular(self) -> Point {
        if self.0.abs() <= self.1.abs() && self.0.abs() <= self.2.abs() {
            return Point::new(0.0, -self.2, self.1).normalized();
        } else if self.1.abs() <= self.0.abs() && self.1.abs() <= self.2.abs() {
            return Point::new(-self.2, 0.0, self.0).normalized();
        }
        Point::new(-self.1, self.0, 0.0).normalized()
    }

    /// Returns a point randomized in its hemisphere
    ///
    /// Uses simple rejection sampling to obtain a point on the hemisphere
    pub fn randomize(self) -> Point {
        let a = self.perpendicular();
        let b = self.cross(a);
        let mut x = 2.0;
        let mut y = 2.0;
        let mut z = 2.0;
        let mut rng = rand::thread_rng();
        let full_dist = rand::distributions::Uniform::new_inclusive(-1.0, 1.0);
        let part_dist = rand::distributions::Uniform::new_inclusive(0.0, 1.0);
        while x * x + y * y + z * z > 1.0 {
            x = rng.sample(part_dist);
            y = rng.sample(full_dist);
            z = rng.sample(full_dist);
        }
        (x * self + y * a + z * b).normalized()
    }
}

impl ops::Mul<Point> for Point {
    type Output = f32;

    fn mul(self, other: Point) -> f32 {
        self.0 * other.0 + self.1 * other.1 + self.2 * other.2
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Color(f32, f32, f32);

impl Color {
    pub fn new(r: f32, g: f32, b: f32) -> Color {
        Color(r, g, b)
    }

    #[inline]
    pub fn r(&self) -> f32 {
        self.0
    }

    #[inline]
    pub fn g(&self) -> f32 {
        self.1
    }

    #[inline]
    pub fn b(&self) -> f32 {
        self.2
    }
}

impl ops::Mul<Color> for Color {
    type Output = Color;

    fn mul(self, other: Color) -> Color {
        Color(self.0 * &other.0, self.1 * &other.1, self.2 * &other.2)
    }
}

macro_rules! operations {
    ($t:ident) => {
        impl ops::Add<$t> for $t {
            type Output = $t;

            fn add(self, other: $t) -> $t {
                $t(self.0 + other.0, self.1 + other.1, self.2 + other.2)
            }
        }

        impl ops::AddAssign<$t> for $t {
            fn add_assign(&mut self, other: $t) {
                *self = $t(self.0 + other.0, self.1 + other.1, self.2 + other.2)
            }
        }

        impl ops::Div<f32> for $t {
            type Output = $t;

            fn div(self, num: f32) -> $t {
                $t(self.0 / num, self.1 / num, self.2 / num)
            }
        }

        impl ops::Mul<$t> for f32 {
            type Output = $t;

            fn mul(self, other: $t) -> $t {
                $t(self * other.0, self * other.1, self * other.2)
            }
        }

        impl ops::Mul<f32> for $t {
            type Output = $t;

            fn mul(self, other: f32) -> $t {
                $t(other * self.0, other * self.1, other * self.2)
            }
        }

        impl ops::Neg for $t {
            type Output = $t;

            fn neg(self) -> $t {
                $t(-self.0, -self.1, -self.2)
            }
        }

        impl ops::Sub<$t> for $t {
            type Output = $t;

            fn sub(self, other: $t) -> $t {
                $t(self.0 - other.0, self.1 - other.1, self.2 - other.2)
            }
        }
    };
}

operations![Color];
operations![Point];

pub static ORIGIN: Point = Point(0.0, 0.0, 0.0);
pub static UNIT_X: Point = Point(1.0, 0.0, 0.0);
pub static UNIT_Y: Point = Point(0.0, 1.0, 0.0);
pub static UNIT_Z: Point = Point(0.0, 0.0, 1.0);

pub static BLACK: Color = Color(0.0, 0.0, 0.0);

#[derive(Clone, Copy)]
pub struct Material {
    pub specularity: f32,
    pub diffusion: f32,
    pub emittance: f32,
    pub color: Color,
}

impl Material {
    pub fn new(specularity: f32, diffusion: f32, emittance: f32, color: Color) -> Material {
        Material {
            specularity,
            diffusion,
            emittance,
            color,
        }
    }
}

pub trait Thing {
    fn hit_by(&self, ray: &Ray) -> Option<f32>;
    fn material(&self) -> Material;
    fn normal(&self, position: &Point, direction: &Point) -> Point;
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Ray {
    pub base: Point,
    pub direction: Point,
}

impl Ray {
    pub fn new(base: Point, direction: Point) -> Ray {
        Ray {
            base,
            direction: direction / direction.norm(),
        }
    }

    /// Returns a new point along the ray at distance `d` from the base
    pub fn at(&self, d: f32) -> Point {
        self.base + d * self.direction
    }

    pub fn intersect(
        &self,
        things: &[Box<dyn Thing>],
        skip: Option<usize>,
    ) -> Option<(f32, usize)> {
        things.iter().enumerate().fold(None, |min, (n, e)| {
            if skip.is_some() && skip.unwrap() == n {
                return min;
            }
            let hit = e.hit_by(&self);
            match hit {
                None => min,
                Some(d) => match min {
                    None => Some((d, n)),
                    Some(m) => {
                        if m.0 < d {
                            min
                        } else {
                            Some((d, n))
                        }
                    }
                },
            }
        })
    }
}

#[derive(Clone)]
pub struct Sphere {
    center: Point,
    radius: f32,
    material: Material,
}

impl Sphere {
    pub fn new(center: Point, radius: f32, material: Material) -> Sphere {
        Sphere {
            center,
            radius,
            material,
        }
    }
}

impl Thing for Sphere {
    fn hit_by(&self, ray: &Ray) -> Option<f32> {
        let hypo = self.center - ray.base;
        let dot = ray.direction * hypo;
        let root = dot * dot - hypo.norm_sqr() + self.radius * self.radius;
        if root < 0.0 {
            return None;
        }
        let min = dot - root.sqrt();
        if min > 0.0 {
            return Some(min);
        }
        let max = dot + root.sqrt();
        if max > 0.0 {
            return Some(max);
        }
        None
    }

    fn material(&self) -> Material {
        self.material
    }

    fn normal(&self, point: &Point, direction: &Point) -> Point {
        let dist = *point - self.center;
        let n = dist / dist.norm();
        if n * *direction > 0.0 {
            -n
        } else {
            n
        }
    }
}

pub struct Rectangle {
    base: Point,
    x: Point,
    y: Point,
    n: Point,
    width: f32,
    height: f32,
    material: Material,
}

impl Rectangle {
    pub fn new(base: Point, x: Point, y: Point, material: Material) -> Rectangle {
        let normal = x.cross(y).normalized();
        let width = x.norm();
        let height = y.norm();
        Rectangle {
            base,
            x: x / width,
            y: y / height,
            n: normal,
            width,
            height,
            material,
        }
    }
}

impl Thing for Rectangle {
    fn hit_by(&self, ray: &Ray) -> Option<f32> {
        let conn = self.base - ray.base;
        let in_plane = self.n * conn * self.n - conn;
        let norm = self.normal(&ORIGIN, &ray.direction);
        let t = conn * norm / (ray.direction * norm);
        if t < 0.0 {
            return None;
        }
        let along_x = self.x * in_plane;
        let along_y = self.y * in_plane;
        if (0.0..=self.width).contains(&along_x) && (0.0..=self.height).contains(&along_y) {
            // println!("{}", t);
            Some(t)
        } else {
            None
        }
    }

    fn material(&self) -> Material {
        self.material
    }

    fn normal(&self, _point: &Point, direction: &Point) -> Point {
        if self.n * *direction > 0.0 {
            -self.n
        } else {
            self.n
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn point_rotations() {
        let p = Point::new(1.0, 0.0, 0.0);
        let o = p.perpendicular();
        assert!((o.norm() - 1.0).abs() < 1.0e-6);
        assert!((p * o).abs() < 1.0e-6);

        let p = Point::new(0.0, 0.5, 0.3).normalized();
        let o = p.perpendicular();
        assert!((o.norm() - 1.0).abs() < 1.0e-6);
        assert!((p * o).abs() < 1.0e-6);

        let p = Point::new(1.0, 0.5, 0.0).normalized();
        let o = p.perpendicular();
        assert!((o.norm() - 1.0).abs() < 1.0e-6);
        assert!((p * o).abs() < 1.0e-6);

        let p = Point::new(1.0, 0.5, 5.0).normalized();
        let o = p.perpendicular();
        assert!((o.norm() - 1.0).abs() < 1.0e-6);
        assert!((p * o).abs() < 1.0e-6);

        let p = Point::new(-0.0, -0.0, -5.0).normalized();
        let o = p.perpendicular();
        assert!((o.norm() - 1.0).abs() < 1.0e-6);
        assert!((p * o).abs() < 1.0e-6);
    }

    #[test]
    fn point_crossed() {
        let p = Point::new(1.0, 0.1, 0.2).normalized();
        let o = p.perpendicular();
        let q = p.cross(o);

        assert!((q.norm() - 1.0).abs() < 1.0e-6);
        assert!((p * o).abs() < 1.0e-6);
        assert!((p * q).abs() < 1.0e-6);
        assert!((o * q).abs() < 1.0e-6);
    }

    #[test]
    fn point_randomized() {
        let p = Point::new(0.0, 0.0, 1.0);
        for _i in 0..10 {
            let r = p.randomize();
            assert!((-1.0..=1.0).contains(&r.x()));
            assert!((-1.0..=1.0).contains(&r.y()));
            assert!(r.z() <= 1.0);
            assert!(r.z() >= 0.0);
        }
    }

    #[test]
    fn ray_normalized() {
        let r = Ray::new(Point::new(0.0, 0.0, 0.0), Point::new(1.0, 2.0, 3.0));
        assert!((r.direction.norm() - 1.0).abs() < 1.0e-6);
    }

    #[test]
    fn ray_hits_sphere() {
        let m = Material::new(0.0, 0.0, 0.0, BLACK);
        let r = Ray::new(Point::new(0.0, 0.0, 0.0), Point::new(1.0, 0.0, 0.0));
        let s = Sphere::new(Point::new(1.0, 0.0, 0.0), 0.5, m);
        assert_eq!(s.hit_by(&r), Some(0.5));

        let r = Ray::new(Point::new(0.0, 0.5, 0.0), Point::new(1.0, 0.0, 0.0));
        let s = Sphere::new(Point::new(1.0, 0.0, 0.0), 0.5, m);
        assert_eq!(s.hit_by(&r), Some(1.0));
    }

    #[test]
    fn ray_misses_sphere() {
        let m = Material::new(0.0, 0.0, 0.0, BLACK);
        let r = Ray::new(Point::new(0.0, 0.0, 0.0), Point::new(1.0, 0.0, 0.0));
        let s = Sphere::new(Point::new(-1.0, 0.0, 0.0), 0.5, m);
        assert_eq!(s.hit_by(&r), None);

        let r = Ray::new(Point::new(0.0, 0.0, 0.0), Point::new(1.0, 1.0, 0.0));
        let s = Sphere::new(Point::new(1.0, 0.0, 0.0), 0.5, m);
        assert_eq!(s.hit_by(&r), None);
    }

    #[test]
    fn ray_hits_rectangle() {
        let m = Material::new(0.0, 0.0, 0.0, BLACK);
        let r = Ray::new(-UNIT_X, UNIT_X);
        let r2 = Rectangle::new(Point::new(5.0, -1.0, -1.0), 2.0 * UNIT_Y, 2.0 * UNIT_Z, m);
        assert_eq!(r2.hit_by(&r), Some(6.0));

        let r2 = Rectangle::new(Point::new(4.0, -0.1, -0.1), 2.0 * UNIT_Y, 2.0 * UNIT_Z, m);
        assert_eq!(r2.hit_by(&r), Some(5.0));

        let r2 = Rectangle::new(Point::new(5.0, -1.9, -1.9), 2.0 * UNIT_Y, 2.0 * UNIT_Z, m);
        assert_eq!(r2.hit_by(&r), Some(6.0));

        let r = Ray::new(-UNIT_X, Point::new(1.0, 0.1, 0.1));
        let r2 = Rectangle::new(Point::new(5.0, -1.0, -1.0), 2.0 * UNIT_Y, 2.0 * UNIT_Z, m);
        let t = r2.hit_by(&r).unwrap();
        assert!(t > 6.0);
        assert_eq!(r.at(t).x(), 5.0);
    }

    #[test]
    fn ray_misses_rectangle() {
        let m = Material::new(0.0, 0.0, 0.0, BLACK);
        let r = Ray::new(ORIGIN, -UNIT_X);
        let r2 = Rectangle::new(Point::new(5.0, -1.0, -1.0), 2.0 * UNIT_Y, 2.0 * UNIT_Z, m);
        assert_eq!(r2.hit_by(&r), None);

        let r = Ray::new(ORIGIN, UNIT_X);
        let r2 = Rectangle::new(Point::new(5.0, 1.0, 1.0), 2.0 * UNIT_Y, 2.0 * UNIT_Z, m);
        assert_eq!(r2.hit_by(&r), None);

        let r = Ray::new(ORIGIN, UNIT_Z);
        let r2 = Rectangle::new(Point::new(5.0, 1.0, 1.0), 2.0 * UNIT_Y, 2.0 * UNIT_Z, m);
        assert_eq!(r2.hit_by(&r), None);
    }

    #[test]
    fn normal_for_rectangle() {
        let m = Material::new(0.0, 0.0, 0.0, BLACK);
        let r = Rectangle::new(ORIGIN, 1.0 * UNIT_Y, 1.0 * UNIT_Z, m);
        let n = r.normal(&ORIGIN, &UNIT_X);
        assert_eq!(n, -UNIT_X);
    }
}
