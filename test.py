from pathetic import Point as P, Color as C, Ray, Material, Camera, Sphere, render

camera = Camera(Ray(P(0, 0, -10), P(0, 0, 5)), P(0.7, 0, 0), P(0, 0.7, 0), 2.0)

red = Material(0.5, 0.0, 1.0, 0.0, 0.0, C(1, 0, 0))
light = Material(0.0, 0.0, 0.1, 0.0, 1.0, C(1, 1, 1))

render(
    camera,
    [Sphere(P(0, 0, 1), 0.3, red), Sphere(P(-5, -5, -5), 5, light),],
    "python.jpg",
    300,
    500,
    6,
)
