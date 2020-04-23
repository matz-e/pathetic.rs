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
    /// Returns a new scene object
    ///
    /// # Arguments
    ///
    /// * `camera` - the camera to use for rendering
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

    fn bounce(&self, ray: &Ray, depth: usize, skip: Option<usize>) -> Color {
        if depth == 0 {
            return ORIGIN;
        }

        let hit = ray.intersect(&self.things, skip);
        if hit.is_none() {
            return ORIGIN;
        }

        let (distance, index) = hit.unwrap();
        let thing = &self.things[index];
        let material = thing.material();
        let impact = ray.at(distance);
        let normal = thing.normal(&impact, &ray.direction);

        let mut intensity = material.emittance * material.color;
        if material.specularity > 0.0 {
            let reflection = Ray::new(
                impact,
                ray.direction - 2.0 * normal * (normal * ray.direction),
            );
            intensity += material
                .color
                .mul(material.specularity * self.bounce(&reflection, depth - 1, Some(index)));
        }

        if material.diffusion > 0.0 {
            let scatter = Ray::new(impact, normal.randomize());
            intensity += material
                .color
                .mul(material.diffusion * self.bounce(&scatter, depth - 1, Some(index)));
        }

        intensity
    }

    /// Render a point on the screen
    ///
    /// # Arguments
    ///
    /// * `x` - the fractional position along the width of the screen
    /// * `y` - the fractional position along the height of the screen
    /// * `samples` - the number of rays to cast
    /// * `bounces` - the maximum number of scatterings of each ray
    pub fn render(&self, x: f32, y: f32, samples: usize, bounces: usize) -> [u8; 3] {
        let ray = self.camera.view(x, y);

        let intensity = (0..samples).fold(ORIGIN, |sum, _i| sum + self.bounce(&ray, bounces, None))
            / samples as f32;

        [
            (255.0 * intensity.x) as u8,
            (255.0 * intensity.y) as u8,
            (255.0 * intensity.z) as u8,
        ]
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
