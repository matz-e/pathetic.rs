from pathetic import Point, Color, Ray, Material, Camera, Rhomboid, Sphere, render

normal = Ray(Point(0.0, 0.0, -10.0), Point(0.0, 0.0, 5.0))
x = Point(0.7, 0.0, 0.0)
y = Point(0.0, 0.7, 0.0)
camera = Camera(normal, x, y, 2.0)

red = Material(0.5, 0.0, 1.0, 0.0, 0.0, Color(1.0, 0.0, 0.0))
green = Material(0.0, 0.0, 1.0, 0.0, 0.0, Color(0.5, 1.0, 0.5))
blue = Material(0.0, 0.0, 1.0, 0.0, 0.0, Color(0.7, 0.7, 1.0))
gray = Material(0.1, 1.0, 1.0, 0.0, 0.0, Color(0.99, 0.99, 0.99))
glass = Material(0.0, 0.0, 0.0, 1.0, 0.0, Color(1.0, 1.0, 1.0))
light = Material(0.0, 0.0, 0.1, 0.0, 1.0, Color(1.0, 1.0, 1.0))

objects = [
    Sphere(Point(-1.0, 0.0, 1.0), 0.1, gray),
    Sphere(Point(0.0, -1.0, 0.0), 0.3, gray),
    Sphere(Point(0.0, 0.0, 1.0), 0.3, red),
    Sphere(Point(1.0, 0.0, 1.0), 0.5, gray),
    Sphere(Point(1.0, 1.0, 0.0), 0.3, gray),
    Sphere(Point(0.0, 1.0, 0.0), 0.3, gray),
    Sphere(Point(0.5, 0.5, -1.5), 0.5, glass),
    Sphere(Point(-1.0, 1.0, 0.0), 0.3, gray),
    Sphere(Point(-5.0, -5.0, -5.0), 5.0, light),
    Rhomboid(
        Point(-4.0, 2.0, -11.0), Point(6.0, 0.0, 0.0), Point(0.0, 0.0, 14.0), green,
    ),
    Rhomboid(Point(-4.0, 2.0, 3.0), Point(6.0, 0.0, 0.0), Point(0.0, -6.0, 0.0), blue,),
    Rhomboid(
        Point(2.0, 2.0, 3.0), Point(0.0, 0.0, -14.0), Point(0.0, -6.0, 0.0), blue,
    ),
]

dpi = int(600 / 0.7)
samples = 600
bounces = 6

render(camera, objects, "example.jpg", dpi, samples, bounces)
