extern crate image;

mod scene;
mod things;

use scene::*;
use things::*;

fn main() {
    let normal = Ray::new(Point::new(0.0, 0.0, -1.0), Point::new(0.0, 0.0, 1.0));
    let x = Point::new(2.0, 0.0, 0.0);
    let y = Point::new(0.0, 2.0, 0.0);
    let mut scene = Scene::new(
        Camera::new(normal, x, y, 2.0),
        vec![Light::new(Point::new(-2.0, -5.0, -5.0))],
    );

    scene.add(Sphere::new(Point::new(-1.0, 0.0, 1.0), 0.1));
    scene.add(Sphere::new(Point::new(0.0, -1.0, 0.0), 0.3));
    scene.add(Sphere::new(Point::new(0.0, 0.0, 1.0), 0.3));
    scene.add(Sphere::new(Point::new(1.0, 0.0, 1.0), 0.5));

    let width = 800;
    let height = 800;
    let mut imgbuf: image::RgbImage = image::ImageBuffer::new(width, height);

    imgbuf.enumerate_pixels_mut().for_each(|(x, y, pixel)| {
        *pixel = image::Rgb(scene.render(x as f32 / width as f32, y as f32 / height as f32));
    });
    imgbuf.save("test.png").unwrap();
}
