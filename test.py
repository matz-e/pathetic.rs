from pathetic import Point as P, Color as C, Ray, Material, Camera, Sphere, render

camera = Camera(Ray(P(0, 0, -10), P(0, 0, 7.5)), P(1, 0, 0), P(0, 1, 0), 2.0)

sky = Material(0.5, 2.0, 1.0, 0.0, 0.1, C(0.8, 0.8, 1.0))
red = Material(0.5, 0.0, 1.0, 0.0, 0.0, C(1, 0, 0))
light = Material(0.0, 0.0, 0.1, 0.0, 1.0, C(1, 1, 1))

spheres = [Sphere(P(x, 0, 1), 0.3, red) for x in range(-3, 4)]
lights = [Sphere(P(-10, -10, -10), 5, light)]
world = [Sphere(P(0, 0, 0), 20, sky)]

render(
    camera,
    spheres + lights + world,
    filename="python.jpg",
    dpi=600,
    samples=50,
    bounces=7,
)
