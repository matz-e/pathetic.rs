mod scene;
mod things;

use scene::*;
use things::*;

fn main() {
    let normal = Ray::new(Point::new(0.0, 0.0, -10.0), Point::new(0.0, 0.0, 5.0));
    let x = Point::new(0.7, 0.0, 0.0);
    let y = Point::new(0.0, 0.7, 0.0);
    let mut scene = Scene::new(Camera::new(normal, x, y, 2.0));

    let red = Material::new(0.5, 0.0, 1.0, 0.0, 0.0, Color::new(1.0, 0.0, 0.0));
    let green = Material::new(0.0, 0.0, 1.0, 0.0, 0.0, Color::new(0.5, 1.0, 0.5));
    let blue = Material::new(0.0, 0.0, 1.0, 0.0, 0.0, Color::new(0.7, 0.7, 1.0));
    let gray = Material::new(0.1, 1.0, 1.0, 0.0, 0.0, Color::new(0.99, 0.99, 0.99));
    let glass = Material::new(0.0, 0.0, 0.0, 1.0, 0.0, Color::new(1.0, 1.0, 1.0));
    let light = Material::new(0.0, 0.0, 0.1, 0.0, 1.0, Color::new(1.0, 1.0, 1.0));
    scene.add(Sphere::new(Point::new(-1.0, 0.0, 1.0), 0.1, gray));
    scene.add(Sphere::new(Point::new(0.0, -1.0, 0.0), 0.3, gray));
    scene.add(Sphere::new(Point::new(0.0, 0.0, 1.0), 0.3, red));
    scene.add(Sphere::new(Point::new(1.0, 0.0, 1.0), 0.5, gray));
    scene.add(Sphere::new(Point::new(1.0, 1.0, 0.0), 0.3, gray));
    scene.add(Sphere::new(Point::new(0.0, 1.0, 0.0), 0.3, gray));
    scene.add(Sphere::new(Point::new(0.5, 0.5, -1.5), 0.5, glass));
    scene.add(Sphere::new(Point::new(-1.0, 1.0, 0.0), 0.3, gray));
    scene.add(Sphere::new(Point::new(-5.0, -5.0, -5.0), 5.0, light));
    scene.add(Rhomboid::new(
        Point::new(-4.0, 2.0, -11.0),
        Point::new(6.0, 0.0, 0.0),
        Point::new(0.0, 0.0, 14.0),
        green,
    ));
    scene.add(Rhomboid::new(
        Point::new(-4.0, 2.0, 3.0),
        Point::new(6.0, 0.0, 0.0),
        Point::new(0.0, -6.0, 0.0),
        blue,
    ));
    scene.add(Rhomboid::new(
        Point::new(2.0, 2.0, 3.0),
        Point::new(0.0, 0.0, -14.0),
        Point::new(0.0, -6.0, 0.0),
        blue,
    ));

    let dpi = 300;
    let samples = 500;
    let bounces = 6;

    scene.render("example.jpg", dpi, samples, bounces).unwrap();
}
