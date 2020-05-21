extern crate rand;

use pathetic_derive::*;
use pyo3::prelude::*;
use rand::prelude::*;
use std::ops;

/// A point in space
#[pyclass]
#[text_signature = "(x, y, z)"]
#[derive(Clone, Copy, Debug, PartialEq, PartialOps)]
pub struct Point {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[pymethods]
impl Point {
    #[new]
    pub fn new(x: f32, y: f32, z: f32) -> Point {
        Point { x, y, z }
    }

    #[getter]
    pub fn get_x(&self) -> PyResult<f32> {
        Ok(self.x)
    }

    #[getter]
    pub fn get_y(&self) -> PyResult<f32> {
        Ok(self.y)
    }

    #[getter]
    pub fn get_z(&self) -> PyResult<f32> {
        Ok(self.z)
    }

    #[setter]
    pub fn set_x(&mut self, x: f32) -> PyResult<()> {
        self.x = x;
        Ok(())
    }

    #[setter]
    pub fn set_y(&mut self, y: f32) -> PyResult<()> {
        self.y = y;
        Ok(())
    }

    #[setter]
    pub fn set_z(&mut self, z: f32) -> PyResult<()> {
        self.z = z;
        Ok(())
    }
}

impl Point {
    pub fn cross(self, other: Point) -> Point {
        Point::new(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x,
        )
    }

    pub fn norm_sqr(self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn norm(self) -> f32 {
        self.norm_sqr().sqrt()
    }

    pub fn normalized(self) -> Point {
        self / self.norm()
    }

    /// Returns a Point that is perpendicular to the current one
    ///
    /// Constructed by zeroing the smallest component and swapping the remaining two components,
    /// normalizing afterwards.
    pub fn perpendicular(self) -> Point {
        if self.x.abs() <= self.y.abs() && self.x.abs() <= self.z.abs() {
            return Point::new(0.0, -self.z, self.y).normalized();
        } else if self.y.abs() <= self.x.abs() && self.y.abs() <= self.z.abs() {
            return Point::new(-self.z, 0.0, self.x).normalized();
        }
        Point::new(-self.y, self.x, 0.0).normalized()
    }

    /// Returns a point randomized in its hemisphere
    ///
    /// Uses simple rejection sampling to obtain a point on the hemisphere
    ///
    /// # Arguments
    ///
    /// * `rng` - The random number generator to use
    pub fn randomize(self, rng: &mut ThreadRng) -> Point {
        let a = self.perpendicular();
        let b = self.cross(a);
        let mut x = 2.0;
        let mut y = 2.0;
        let mut z = 2.0;
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
        self.x * other.x + self.y * other.y + self.z * other.z
    }
}

/// A color with components red, green, and blue
#[pyclass]
#[text_signature = "(r, g, b)"]
#[derive(Clone, Copy, Debug, PartialEq, PartialOps)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

#[pymethods]
impl Color {
    #[new]
    pub fn new(r: f32, g: f32, b: f32) -> Color {
        Color { r, g, b }
    }

    #[getter]
    pub fn get_r(&self) -> PyResult<f32> {
        Ok(self.r)
    }

    #[getter]
    pub fn get_g(&self) -> PyResult<f32> {
        Ok(self.g)
    }

    #[getter]
    pub fn get_b(&self) -> PyResult<f32> {
        Ok(self.b)
    }
}

impl ops::Mul<Color> for Color {
    type Output = Color;

    fn mul(self, other: Color) -> Color {
        Color::new(self.r * other.r, self.g * other.g, self.b * other.b)
    }
}

pub static ORIGIN: Point = Point {
    x: 0.0,
    y: 0.0,
    z: 0.0,
};
pub static UNIT_X: Point = Point {
    x: 1.0,
    y: 0.0,
    z: 0.0,
};
pub static UNIT_Y: Point = Point {
    x: 0.0,
    y: 1.0,
    z: 0.0,
};
pub static UNIT_Z: Point = Point {
    x: 0.0,
    y: 0.0,
    z: 1.0,
};

pub static BLACK: Color = Color {
    r: 0.0,
    g: 0.0,
    b: 0.0,
};

/// Properties of objects in a scene
#[pyclass]
#[derive(Clone, Copy)]
pub struct Material {
    pub specularity: f32,
    pub hardness: f32,
    pub diffusion: f32,
    pub refraction: f32,
    pub emittance: f32,
    pub color: Color,
}

#[pymethods]
impl Material {
    #[new]
    pub fn new(
        specularity: f32,
        hardness: f32,
        diffusion: f32,
        refraction: f32,
        emittance: f32,
        color: Color,
    ) -> Material {
        Material {
            specularity,
            hardness,
            diffusion,
            refraction,
            emittance,
            color,
        }
    }
}

#[pyclass]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Ray {
    pub base: Point,
    pub direction: Point,
}

#[pymethods]
impl Ray {
    #[new]
    pub fn new(base: Point, direction: Point) -> Ray {
        Ray {
            base,
            direction: direction / direction.norm(),
        }
    }
}

impl Ray {
    /// Returns a new point along the ray at distance `d` from the base
    pub fn at(&self, d: f32) -> Point {
        self.base + d * self.direction
    }

    pub fn intersect(
        &self,
        things: &[Box<dyn Thing + Sync>],
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

pub trait Thing {
    fn hit_by(&self, ray: &Ray) -> Option<f32>;
    fn material(&self) -> Material;
    fn normal(&self, position: &Point, direction: &Point) -> Point;
}

#[pyclass]
#[derive(Clone)]
pub struct Sphere {
    center: Point,
    radius: f32,
    material: Material,
}

#[pymethods]
impl Sphere {
    #[new]
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

    fn normal(&self, point: &Point, _direction: &Point) -> Point {
        let dist = *point - self.center;
        dist / dist.norm()
    }
}

#[pyclass]
#[derive(Clone)]
pub struct Triangle {
    base: Point,
    x: Point,
    y: Point,
    n: Point,
    width: f32,
    height: f32,
    material: Material,
}

#[pymethods]
impl Triangle {
    #[new]
    pub fn new(a: Point, b: Point, c: Point, material: Material) -> Triangle {
        let base = a;
        let x = b - a;
        let y = c - a;
        let normal = x.cross(y).normalized();
        let width = x.norm();
        let height = y.norm();
        Triangle {
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

impl Thing for Triangle {
    fn hit_by(&self, ray: &Ray) -> Option<f32> {
        let conn = self.base - ray.base;
        let norm = self.normal(&ORIGIN, &ray.direction);
        let t = conn * norm / (ray.direction * norm);
        if t < 0.0 {
            return None;
        }
        let in_plane = ray.at(t) - self.base;
        let along_x = (self.x * in_plane) / self.width;
        let along_y = (self.y * in_plane) / self.height;
        if (0.0..=1.0).contains(&(along_x + along_y)) {
            Some(t)
        } else {
            None
        }
    }

    fn material(&self) -> Material {
        self.material
    }

    fn normal(&self, _point: &Point, _direction: &Point) -> Point {
        self.n
    }
}

#[pyclass]
#[derive(Clone)]
pub struct Rhomboid {
    base: Point,
    x: Point,
    y: Point,
    n: Point,
    width: f32,
    height: f32,
    material: Material,
}

#[pymethods]
impl Rhomboid {
    #[new]
    pub fn new(base: Point, x: Point, y: Point, material: Material) -> Rhomboid {
        let normal = x.cross(y).normalized();
        let width = x.norm();
        let height = y.norm();
        Rhomboid {
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

impl Thing for Rhomboid {
    fn hit_by(&self, ray: &Ray) -> Option<f32> {
        let conn = self.base - ray.base;
        let norm = self.normal(&ORIGIN, &ray.direction);
        let t = conn * norm / (ray.direction * norm);
        if t < 0.0 {
            return None;
        }
        let in_plane = ray.at(t) - self.base;
        let along_x = self.x * in_plane;
        let along_y = self.y * in_plane;
        if (0.0..=self.width).contains(&along_x) && (0.0..=self.height).contains(&along_y) {
            Some(t)
        } else {
            None
        }
    }

    fn material(&self) -> Material {
        self.material
    }

    fn normal(&self, _point: &Point, _direction: &Point) -> Point {
        self.n
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn point_basics() {
        let p = Point::new(1.0, 2.0, 3.0);
        let q = Point::new(0.0, 1.0, 2.0);

        assert_eq!(p.get_x().unwrap(), 1.0);
        assert_eq!(p.get_y().unwrap(), 2.0);
        assert_eq!(p.get_z().unwrap(), 3.0);

        assert_eq!(p.norm_sqr(), 14.0);

        assert_eq!(p * q, 8.0);
    }

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
        let mut rng = rand::thread_rng();
        let p = Point::new(0.0, 0.0, 1.0);
        for _i in 0..10 {
            let r = p.randomize(&mut rng);
            assert!((-1.0..=1.0).contains(&r.x));
            assert!((-1.0..=1.0).contains(&r.y));
            assert!(r.z <= 1.0);
            assert!(r.z >= 0.0);
        }
    }

    #[test]
    fn ray_normalized() {
        let r = Ray::new(Point::new(0.0, 0.0, 0.0), Point::new(1.0, 2.0, 3.0));
        assert!((r.direction.norm() - 1.0).abs() < 1.0e-6);
    }

    #[test]
    fn ray_hits_sphere() {
        let m = Material::new(0.0, 0.0, 0.0, 0.0, 0.0, BLACK);
        let r = Ray::new(Point::new(0.0, 0.0, 0.0), Point::new(1.0, 0.0, 0.0));

        let s = Sphere::new(Point::new(1.0, 0.0, 0.0), 0.5, m);
        assert_eq!(s.hit_by(&r), Some(0.5));
        let n = s.normal(&r.at(0.5), &r.direction);
        assert_eq!(n, Point::new(-1.0, 0.0, 0.0));

        // Inside sphere
        let s = Sphere::new(Point::new(0.0, 0.0, 0.0), 0.5, m);
        assert_eq!(s.hit_by(&r), Some(0.5));
        let n = s.normal(&r.at(0.5), &r.direction);
        assert_eq!(n, Point::new(1.0, 0.0, 0.0));

        let r = Ray::new(Point::new(0.0, 0.5, 0.0), Point::new(1.0, 0.0, 0.0));
        let s = Sphere::new(Point::new(1.0, 0.0, 0.0), 0.5, m);
        assert_eq!(s.hit_by(&r), Some(1.0));
    }

    #[test]
    fn ray_misses_sphere() {
        let m = Material::new(0.0, 0.0, 0.0, 0.0, 0.0, BLACK);
        let r = Ray::new(Point::new(0.0, 0.0, 0.0), Point::new(1.0, 0.0, 0.0));
        let s = Sphere::new(Point::new(-1.0, 0.0, 0.0), 0.5, m);
        assert_eq!(s.hit_by(&r), None);

        let r = Ray::new(Point::new(0.0, 0.0, 0.0), Point::new(1.0, 1.0, 0.0));
        let s = Sphere::new(Point::new(1.0, 0.0, 0.0), 0.5, m);
        assert_eq!(s.hit_by(&r), None);
    }

    #[test]
    fn ray_hits_triangle() {
        let m = Material::new(0.0, 0.0, 0.0, 0.0, 0.0, BLACK);
        let r = Ray::new(-UNIT_X, UNIT_X);
        let a = Point::new(5.0, -1.0, -1.0);
        let t = Triangle::new(a, a + 2.0 * UNIT_Y, a + 2.0 * UNIT_Z, m);
        assert_eq!(t.hit_by(&r), Some(6.0));

        let a = Point::new(4.0, -0.1, -0.1);
        let t = Triangle::new(a, a + 2.0 * UNIT_Y, a + 2.0 * UNIT_Z, m);
        assert_eq!(t.hit_by(&r), Some(5.0));
    }

    #[test]
    fn ray_misses_triangle() {
        let m = Material::new(0.0, 0.0, 0.0, 0.0, 0.0, BLACK);
        let r = Ray::new(-UNIT_X, UNIT_X);
        let a = Point::new(5.0, -1.9, -1.9);
        let t = Triangle::new(a, a + 2.0 * UNIT_Y, a + 2.0 * UNIT_Z, m);
        assert_eq!(t.hit_by(&r), None);

        let r = Ray::new(-UNIT_X, Point::new(1.0, 0.1, 0.1));
        let a = Point::new(5.0, -1.0, -1.0);
        let t = Triangle::new(a, a + 2.0 * UNIT_Y, a + 2.0 * UNIT_Z, m);
        assert_eq!(t.hit_by(&r), None);
    }

    #[test]
    fn ray_hits_rectangle() {
        let m = Material::new(0.0, 0.0, 0.0, 0.0, 0.0, BLACK);
        let r = Ray::new(-UNIT_X, UNIT_X);
        let r2 = Rhomboid::new(Point::new(5.0, -1.0, -1.0), 2.0 * UNIT_Y, 2.0 * UNIT_Z, m);
        assert_eq!(r2.hit_by(&r), Some(6.0));

        let r2 = Rhomboid::new(Point::new(4.0, -0.1, -0.1), 2.0 * UNIT_Y, 2.0 * UNIT_Z, m);
        assert_eq!(r2.hit_by(&r), Some(5.0));

        let r2 = Rhomboid::new(Point::new(5.0, -1.9, -1.9), 2.0 * UNIT_Y, 2.0 * UNIT_Z, m);
        assert_eq!(r2.hit_by(&r), Some(6.0));

        let r = Ray::new(-UNIT_X, Point::new(1.0, 0.1, 0.1));
        let r2 = Rhomboid::new(Point::new(5.0, -1.0, -1.0), 2.0 * UNIT_Y, 2.0 * UNIT_Z, m);
        let t = r2.hit_by(&r).unwrap();
        assert!(t > 6.0);
        assert_eq!(r.at(t).x, 5.0);
    }

    #[test]
    fn ray_misses_rectangle() {
        let m = Material::new(0.0, 0.0, 0.0, 0.0, 0.0, BLACK);
        let r = Ray::new(ORIGIN, -UNIT_X);
        let r2 = Rhomboid::new(Point::new(5.0, -1.0, -1.0), 2.0 * UNIT_Y, 2.0 * UNIT_Z, m);
        assert_eq!(r2.hit_by(&r), None);

        let r = Ray::new(ORIGIN, UNIT_X);
        let r2 = Rhomboid::new(Point::new(5.0, 1.0, 1.0), 2.0 * UNIT_Y, 2.0 * UNIT_Z, m);
        assert_eq!(r2.hit_by(&r), None);

        let r = Ray::new(ORIGIN, UNIT_Z);
        let r2 = Rhomboid::new(Point::new(5.0, 1.0, 1.0), 2.0 * UNIT_Y, 2.0 * UNIT_Z, m);
        assert_eq!(r2.hit_by(&r), None);
    }

    #[test]
    fn ray_intersects() {
        let m = Material::new(0.0, 0.0, 0.0, 0.0, 0.0, BLACK);
        let r = Ray::new(ORIGIN, UNIT_Z);

        let things: Vec<Box<dyn Thing + Sync + 'static>> = vec![
            Box::new(Sphere::new(Point::new(0.0, 0.0, 1.0), 0.5, m)),
            Box::new(Sphere::new(Point::new(0.0, 0.0, 3.0), 0.5, m)),
            Box::new(Sphere::new(Point::new(0.0, 0.0, 2.0), 0.5, m)),
        ];

        let res = r.intersect(&things[..], Some(0));
        assert!(res.is_some());
        let (dist, item) = res.unwrap();
        assert_eq!(dist, 1.5);
        assert_eq!(item, 2);
    }

    #[test]
    fn normal_for_rectangle() {
        let m = Material::new(0.0, 0.0, 0.0, 0.0, 0.0, BLACK);
        let r = Rhomboid::new(ORIGIN, 1.0 * UNIT_Y, 1.0 * UNIT_Z, m);
        let n = r.normal(&ORIGIN, &UNIT_X);
        assert_eq!(n, UNIT_X);
    }

    #[test]
    fn color_getters() {
        let c = Color::new(1.0, 2.0, 3.0);
        let d = Color::new(1.0, 0.0, 3.0);

        assert_eq!(c.get_r().unwrap(), 1.0);
        assert_eq!(c.get_g().unwrap(), 2.0);
        assert_eq!(c.get_b().unwrap(), 3.0);

        assert_eq!(c * d, Color::new(1.0, 0.0, 9.0));
    }
}
