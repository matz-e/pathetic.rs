use std::ops;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Point {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Point {
    pub fn new(x: f32, y: f32, z: f32) -> Point {
        Point { x, y, z }
    }

    pub fn norm_sqr(self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn norm(self) -> f32 {
        self.norm_sqr().sqrt()
    }
}

impl ops::Add<Point> for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl ops::Div<f32> for Point {
    type Output = Point;

    fn div(self, num: f32) -> Point {
        Point {
            x: self.x / num,
            y: self.y / num,
            z: self.z / num,
        }
    }
}

impl ops::Mul<Point> for f32 {
    type Output = Point;

    fn mul(self, other: Point) -> Point {
        Point {
            x: self * other.x,
            y: self * other.y,
            z: self * other.z,
        }
    }
}

impl ops::Mul<Point> for Point {
    type Output = f32;

    fn mul(self, other: Point) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }
}

impl ops::Sub<Point> for Point {
    type Output = Point;

    fn sub(self, other: Point) -> Point {
        Point {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

pub trait Thing {
    fn hit_by(&self, ray: &Ray) -> Option<f32>;
    fn normal(&self, position: &Point) -> Point;
}

pub trait BoxedThing: Thing {
    fn rebox(&self) -> Box<dyn BoxedThing>;
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

    pub fn intersect<'a>(
        &self,
        things: &'a Vec<Box<dyn BoxedThing>>,
    ) -> Option<(f32, &'a Box<dyn BoxedThing>)> {
        things.iter().fold(None, |min, e| {
            let hit = e.hit_by(&self);
            match hit {
                None => min,
                Some(d) => match min {
                    None => Some((d, e)),
                    Some(m) => {
                        if m.0 < d {
                            min
                        } else {
                            Some((d, e))
                        }
                    }
                },
            }
        })
    }
}

#[derive(Clone)]
pub struct Material {
    specularity: f32,
    diffusion: f32,
    ambience: f32,
    shininess: f32,
}

#[derive(Clone)]
pub struct Sphere {
    pub center: Point,
    pub radius: f32,
}

impl Sphere {
    pub fn new(center: Point, radius: f32) -> Sphere {
        Sphere { center, radius }
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

    fn normal(&self, point: &Point) -> Point {
        let dist = *point - self.center;
        dist / dist.norm()
    }
}

impl<T: Thing + Clone + 'static> BoxedThing for T {
    fn rebox(&self) -> Box<dyn BoxedThing> {
        Box::new(self.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ray_normalized() {
        let r = Ray::new(Point::new(0.0, 0.0, 0.0), Point::new(1.0, 2.0, 3.0));
        assert!((r.direction.norm() - 1.0).abs() < 1.0e-6);
    }

    #[test]
    fn ray_hits_sphere() {
        let r = Ray::new(Point::new(0.0, 0.0, 0.0), Point::new(1.0, 0.0, 0.0));
        let s = Sphere::new(Point::new(1.0, 0.0, 0.0), 0.5);
        assert_eq!(s.hit_by(&r), Some(0.5));

        let r = Ray::new(Point::new(0.0, 0.5, 0.0), Point::new(1.0, 0.0, 0.0));
        let s = Sphere::new(Point::new(1.0, 0.0, 0.0), 0.5);
        assert_eq!(s.hit_by(&r), Some(1.0));
    }

    #[test]
    fn ray_misses_sphere() {
        let r = Ray::new(Point::new(0.0, 0.0, 0.0), Point::new(1.0, 0.0, 0.0));
        let s = Sphere::new(Point::new(-1.0, 0.0, 0.0), 0.5);
        assert_eq!(s.hit_by(&r), None);

        let r = Ray::new(Point::new(0.0, 0.0, 0.0), Point::new(1.0, 1.0, 0.0));
        let s = Sphere::new(Point::new(1.0, 0.0, 0.0), 0.5);
        assert_eq!(s.hit_by(&r), None);
    }
}
