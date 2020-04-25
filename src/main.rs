extern crate image;

mod scene;
mod things;

use scene::*;
use things::*;

fn main() {
    let normal = Ray::new(Point::new(0.0, 0.0, -10.0), Point::new(0.0, 0.0, 5.0));
    let x = Point::new(0.7, 0.0, 0.0);
    let y = Point::new(0.0, 0.7, 0.0);
    let mut scene = Scene::new(Camera::new(normal, x, y, 2.0));

    let reddish = Material::new(0.5, 1.0, 0.0, Color::new(1.0, 0.0, 0.0));
    let greenish = Material::new(0.0, 1.0, 0.0, Color::new(0.5, 1.0, 0.5));
    let blueish = Material::new(0.0, 1.0, 0.0, Color::new(0.7, 0.7, 1.0));
    let grayish = Material::new(0.0, 1.0, 0.0, Color::new(0.99, 0.99, 0.99));
    let l = Material::new(0.0, 0.1, 1.0, Color::new(1.0, 1.0, 1.0));
    scene.add(Sphere::new(Point::new(-1.0, 0.0, 1.0), 0.1, grayish));
    scene.add(Sphere::new(Point::new(0.0, -1.0, 0.0), 0.3, grayish));
    scene.add(Sphere::new(Point::new(0.0, 0.0, 1.0), 0.3, reddish));
    scene.add(Sphere::new(Point::new(1.0, 0.0, 1.0), 0.5, grayish));
    scene.add(Sphere::new(Point::new(1.0, 1.0, 0.0), 0.3, grayish));
    scene.add(Sphere::new(Point::new(0.0, 1.0, 0.0), 0.3, grayish));
    scene.add(Sphere::new(Point::new(-1.0, 1.0, 0.0), 0.3, grayish));
    scene.add(Sphere::new(Point::new(-5.0, -5.0, -5.0), 5.0, l));
    scene.add(Rhomboid::new(
        Point::new(-4.0, 2.0, -1.0),
        Point::new(6.0, 0.0, 0.0),
        Point::new(0.0, 0.0, 4.0),
        greenish,
    ));
    scene.add(Rhomboid::new(
        Point::new(-4.0, 2.0, 3.0),
        Point::new(6.0, 0.0, 0.0),
        Point::new(0.0, -6.0, 0.0),
        blueish,
    ));
    scene.add(Rhomboid::new(
        Point::new(2.0, 2.0, 3.0),
        Point::new(0.0, 0.0, -4.0),
        Point::new(0.0, -6.0, 0.0),
        blueish,
    ));

    let samples = 1000;
    let bounces = 10;

    let width = 600;
    let height = 600;
    let mut imgbuf: image::RgbImage = image::ImageBuffer::new(width, height);

    imgbuf.enumerate_pixels_mut().for_each(|(x, y, pixel)| {
        *pixel = image::Rgb(scene.render(
            x as f32 / width as f32,
            y as f32 / height as f32,
            samples,
            bounces,
        ));
    });
    imgbuf.save("test.png").unwrap();
}
