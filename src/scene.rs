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

pub struct Light {
    position: Point,
}

impl Light {
    pub fn new(position: Point) -> Self {
        Light { position }
    }
}

pub struct Scene {
    camera: Camera,
    lights: Vec<Light>,
    things: Vec<Box<dyn Thing>>,
}

fn identical<T>(a: &T, b: &T) -> bool {
    a as *const T == b as *const T
}

impl Scene {
    pub fn new(camera: Camera, lights: Vec<Light>) -> Self {
        Self {
            camera,
            lights,
            things: Vec::new(),
        }
    }

    pub fn add<T>(&mut self, thing: T)
    where
        T: Thing + 'static,
    {
        self.things.push(Box::new(thing))
    }

    fn brightness(&self, ray: &Ray, distance: f32, thing: &Box<dyn Thing>) -> f32 {
        let hit = ray.at(distance);
        let norm = thing.normal(&hit);
        self.lights
            .iter()
            .map(|l| {
                let n = l.position - hit;
                let r = Ray::new(hit, n);
                let filtered = &self
                    .things
                    .iter()
                    .map(|e| {
                        if identical(e, &thing) {
                            None
                        } else {
                            Some(e.rebox())
                        }
                    })
                    .flatten()
                    .collect();
                if r.intersect(filtered).is_some() {
                    return 0.0;
                }
                let i = (n / n.norm()) * norm;
                if i < 0.0 {
                    0.0
                } else {
                    i
                }
            })
            .sum()
    }

    pub fn render(&self, x: f32, y: f32) -> [u8; 3] {
        let ray = self.camera.view(x, y);
        let hit = ray.intersect(&self.things);

        match hit {
            None => [0, 0, 0],
            Some((dist, elem)) => {
                let c = (255.0 * self.brightness(&ray, dist, elem)) as u8;
                [c, c, c]
            }
        }
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
