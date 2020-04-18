use std::ops;

#[derive(Clone, Copy)]
pub struct Point {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

pub struct Ray {
    pub base: Point,
    pub direction: Point,
}

pub struct Sphere {
    pub center: Point,
    pub radius: f32,
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

impl ops::Div<f32> for Point {
    type Output = Point;

    fn div(self, num: f32) -> Point {
        Point { x: self.x / num, y: self.y / num, z: self.z / num }
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
        Point { x: self.x - other.x, y: self.y - other.y, z: self.z - other.z }
    }
}

impl Ray {
    pub fn new(base: Point, direction: Point) -> Ray {
        Ray { base, direction: direction / direction.norm() }
    }

    pub fn intersect(self, sphere: Sphere) -> Option<f32> {
        let hypo = sphere.center - self.base;
        let dot = self.direction * hypo;
        let root = dot * dot - hypo.norm_sqr() + sphere.radius * sphere.radius;
        if root < 0.0 {
            return None
        }
        let min = dot - root.sqrt();
        if min > 0.0 {
            return Some(min)
        }
        let max = dot + root.sqrt();
        if max > 0.0 {
            return Some(max)
        }
        None
    }
}

impl Sphere {
    pub fn new(center: Point, radius: f32) -> Sphere {
        Sphere { center, radius }
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
        assert_eq!(r.intersect(s), Some(0.5));

        let r = Ray::new(Point::new(0.0, 0.5, 0.0), Point::new(1.0, 0.0, 0.0));
        let s = Sphere::new(Point::new(1.0, 0.0, 0.0), 0.5);
        assert_eq!(r.intersect(s), Some(1.0));
    }

    #[test]
    fn ray_misses_sphere() {
        let r = Ray::new(Point::new(0.0, 0.0, 0.0), Point::new(1.0, 0.0, 0.0));
        let s = Sphere::new(Point::new(-1.0, 0.0, 0.0), 0.5);
        assert_eq!(r.intersect(s), None);

        let r = Ray::new(Point::new(0.0, 0.0, 0.0), Point::new(1.0, 1.0, 0.0));
        let s = Sphere::new(Point::new(1.0, 0.0, 0.0), 0.5);
        assert_eq!(r.intersect(s), None);
    }
}
