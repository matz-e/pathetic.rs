extern crate image;
extern crate rand;
extern crate rand_xoshiro;
extern crate rayon;

use crate::things::*;
use pyo3::prelude::*;
use rand::prelude::*;
use rand_xoshiro::rand_core::SeedableRng;
use rand_xoshiro::Xoshiro256Plus;
use rayon::prelude::*;
use std::error::Error;

#[pyclass]
#[derive(Clone)]
pub struct Camera {
    /// The normal vector of the plain of the screen
    normal: Ray,
    /// Vector pointing along the width of the screen
    x: Point,
    /// Vector pointing along the height of the screen
    y: Point,
    /// The distance of the eye from the screen
    distance: f32,
    /// The radius of the aperture
    aperture: Option<f32>,
    /// The distance of the focal plane from the screen
    focal_length: Option<f32>,
}

#[pymethods]
impl Camera {
    #[new]
    pub fn new(
        normal: Ray,
        width: f32,
        height: f32,
        distance: f32,
        aperture: Option<f32>,
        focal_length: Option<f32>,
    ) -> Self {
        let x = -width * normal.direction.cross(UNIT_Y).normalized();
        let y = height * normal.direction.cross(UNIT_X).normalized();
        Camera {
            normal,
            x,
            y,
            distance,
            aperture,
            focal_length,
        }
    }
}

impl Camera {
    /// Returns a ray for a given point of the screen
    ///
    /// # Arguments
    ///
    /// * `x` - the fractional position along the screen width
    /// * `y` - the fractional position along the screen height
    /// * `rng` - the random number generator to use
    pub fn view(&self, x: f32, y: f32, rng: &mut dyn RngCore) -> Ray {
        let base = self.normal.base + (x - 0.5) * self.x + (y - 0.5) * self.y;
        let direction = base - self.normal.at(-self.distance);
        let ray = Ray::new(base, direction);
        if self.aperture.is_some() {
            let focal_point =
                ray.at(self.focal_length.unwrap() / (direction * self.normal.direction));
            let direction = focal_point - base;
            Ray::new(base, direction)
        } else {
            ray
        }
    }
}

pub struct Scene {
    camera: Camera,
    samples: usize,
    bounces: usize,
    things: Vec<Box<dyn Thing + Sync>>,
}

impl Scene {
    /// Returns a new scene object
    ///
    /// # Arguments
    ///
    /// * `camera` - the camera to use for rendering
    /// * `samples` - the number of rays to cast per pixel, defaults to 500
    /// * `bounces` - the maximum number of ray reflections, defaults to 6
    pub fn new(camera: Camera, samples: Option<usize>, bounces: Option<usize>) -> Self {
        Self {
            camera,
            samples: samples.unwrap_or(500),
            bounces: bounces.unwrap_or(6),
            things: Vec::new(),
        }
    }

    pub fn add<T>(&mut self, thing: T)
    where
        T: Thing + Sync + 'static,
    {
        self.things.push(Box::new(thing))
    }

    /// Schlick's approximation for the reflection coefficient
    fn reflect(n_frac: f32, cos_in: f32) -> f32 {
        let r0 = ((n_frac - 1.0) / (n_frac + 1.0)).powi(2);
        r0 + (1.0 - r0) * (1.0 - cos_in).powi(5)
    }

    fn bounce(
        &self,
        ray: &Ray,
        depth: usize,
        skip: Option<usize>,
        mut rng: &mut dyn RngCore,
    ) -> Color {
        if depth == 0 {
            return BLACK;
        }

        let hit = ray.intersect(&self.things, skip);
        if hit.is_none() {
            return BLACK;
        }

        let (distance, index) = hit.unwrap();
        let thing = &self.things[index];
        let material = thing.material();
        let impact = ray.at(distance);
        let normal = thing.normal(&impact, &ray.direction);

        let mut intensity = material.emittance * material.color;
        if material.specularity > 0.0 {
            let reflected = ray.direction - 2.0 * normal * (normal * ray.direction);
            let reflection = Ray::new(
                impact,
                (reflected + material.hardness * reflected.randomize(rng)).normalized(),
            );
            intensity +=
                material.specularity * self.bounce(&reflection, depth - 1, Some(index), rng);
        }

        if material.diffusion > 0.0 {
            let scatter = Ray::new(impact, normal.randomize(&mut rng));
            intensity += material.color
                * material.diffusion
                * self.bounce(&scatter, depth - 1, Some(index), rng);
        }

        if material.refraction > 0.0 {
            let cos_in = normal * ray.direction;
            let n_frac = if cos_in < 0.0 {
                1.0 / 1.5 // outside material
            } else {
                1.5
            };
            let cos_out_sqr = 1.0 - n_frac * n_frac * (1.0 - cos_in * cos_in);
            let reflection = Ray::new(impact, ray.direction - 2.0 * normal * cos_in);
            if cos_out_sqr < 0.0 {
                intensity += material.refraction
                    * self.bounce(&reflection, depth - 1, Some(index), &mut rng);
            } else {
                let in_plane = (ray.direction - normal * cos_in) * n_frac;
                let along_normal =
                    normal * 1.0f32.copysign(cos_in) * (1.0 - in_plane.norm_sqr()).sqrt();
                let transmission = Ray::new(impact, in_plane + along_normal);
                let refl = Scene::reflect(n_frac, cos_in.abs());
                let trans = 1.0 - refl;
                if self.bounces - depth < 2 {
                    intensity += material.refraction
                        * (refl * self.bounce(&reflection, depth - 1, Some(index), &mut rng)
                            + trans * self.bounce(&transmission, depth - 1, Some(index), &mut rng));
                } else {
                    let p = 0.25 + 0.5 * refl; // values: 0.25 - 0.75
                    let dist = rand::distributions::Uniform::new_inclusive(0.0, 1.0);
                    if rng.sample(dist) < p {
                        intensity += material.refraction
                            * refl
                            * self.bounce(&reflection, depth - 1, Some(index), &mut rng)
                            / p;
                    } else {
                        intensity += material.refraction
                            * trans
                            * self.bounce(&transmission, depth - 1, Some(index), &mut rng)
                            / (1.0 - p);
                    }
                }
            }
        }

        intensity
    }

    /// Render a point on the screen
    ///
    /// # Arguments
    ///
    /// * `x` - the fractional position along the width of the screen
    /// * `y` - the fractional position along the height of the screen
    /// * `rng` - the random number generator to use
    fn render_point(&self, x: f32, y: f32, mut rng: &mut dyn RngCore) -> [u8; 3] {
        let intensity = (0..self.samples).fold(BLACK, |sum, _i| {
            let ray = self.camera.view(x, y, &mut rng);
            sum + self.bounce(&ray, self.bounces, None, &mut rng)
        }) / self.samples as f32;

        [
            (255.0 * intensity.r) as u8,
            (255.0 * intensity.g) as u8,
            (255.0 * intensity.b) as u8,
        ]
    }

    /// Render the defined scene
    ///
    /// # Arguments
    ///
    /// * `filename` - the name to save the final image under
    /// * `dpi` - the scaling factor for the image resolution
    pub fn render(&self, filename: &str, dpi: u32) -> Result<(), Box<dyn Error>> {
        let width = (dpi as f32 * self.camera.x.norm()) as u32;
        let height = (dpi as f32 * self.camera.y.norm()) as u32;
        let mut imgbuf: image::RgbImage = image::ImageBuffer::new(width, height);
        imgbuf
            .enumerate_pixels_mut()
            .par_bridge()
            .for_each(|(x, y, pixel)| {
                let mut rng = Xoshiro256Plus::seed_from_u64((x as u64) << 32 | (y as u64 & 0xffffffff));
                *pixel = image::Rgb(
                    self.render_point(x as f32 / width as f32, y as f32 / height as f32, &mut rng),
                );
            });
        imgbuf.save(filename)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn camera_rays() {
        let normal = Ray::new(Point::new(0.0, 0.0, -1.0), Point::new(0.0, 0.0, 1.0));
        let c = Camera::new(normal, 2.0, 2.0, 2.0, None, None);

        let mut rng = thread_rng();

        let corner_ray = Ray::new(Point::new(1.0, 1.0, -1.0), Point::new(0.5, 0.5, 1.0));
        assert_eq!(c.view(1.0, 1.0, &mut rng), corner_ray);

        let edge_ray = Ray::new(Point::new(0.0, 1.0, -1.0), Point::new(0.0, 0.5, 1.0));
        assert_eq!(c.view(0.5, 1.0, &mut rng), edge_ray);
    }
}
