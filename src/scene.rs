use crate::things::*;

pub struct Camera {
    /// The normal vector of the plain of the screen
    normal: Ray,
    /// Vector pointing along the width of the screen
    x: Point,
    /// Vector pointing along the height of the screen
    y: Point,
    /// The distance of the eye from the screen
    distance: f32,
}

impl Camera {
    pub fn new(normal: Ray, x: Point, y: Point, distance: f32) -> Self {
        Camera {
            normal,
            x,
            y,
            distance,
        }
    }

    /// Returns a ray for a given point of the screen
    ///
    /// # Arguments
    ///
    /// * `x` - the fractional position along the screen width
    /// * `y` - the fractional position along the screen height
    pub fn view(&self, x: f32, y: f32) -> Ray {
        let base = self.normal.base + (x - 0.5) * self.x + (y - 0.5) * self.y;
        let direction = base - self.normal.at(-self.distance);
        Ray::new(base, direction)
    }
}

pub struct Scene {
    camera: Camera,
    things: Vec<Box<dyn Thing>>,
}

impl Scene {
    pub fn new(camera: Camera) -> Self {
        Self {
            camera,
            things: Vec::new(),
        }
    }

    pub fn add<T>(&mut self, thing: T)
    where
        T: Thing + 'static,
    {
        self.things.push(Box::new(thing))
    }

    pub fn render(&self, x: f32, y: f32) -> [u8; 3] {
        let ray = self.camera.view(x, y);
        let hits: Vec<Option<f32>> = self.things.iter().map(|e| e.hit_by(&ray)).collect();
        let mut distances: Vec<f32> = hits.into_iter().flatten().collect();
        distances.sort_by(|a, b| a.partial_cmp(b).unwrap());

        if distances.is_empty() {
            return [0, 0, 0]
        }
        [255, 255, 255]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn camera_rays() {
        let normal = Ray::new(Point::new(0.0, 0.0, -1.0), Point::new(0.0, 0.0, 1.0));
        let x = Point::new(2.0, 0.0, 0.0);
        let y = Point::new(0.0, 2.0, 0.0);
        let c = Camera::new(normal, x, y, 2.0);

        let corner_ray = Ray::new(Point::new(1.0, 1.0, -1.0), Point::new(0.5, 0.5, 1.0));
        assert_eq!(c.view(1.0, 1.0), corner_ray);

        let edge_ray = Ray::new(Point::new(0.0, 1.0, -1.0), Point::new(0.0, 0.5, 1.0));
        assert_eq!(c.view(0.5, 1.0), edge_ray);
    }
}
