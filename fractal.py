import numpy as np
import random

from pathetic import Point, Color, Ray, Material, Camera, Triangle, Sphere, render


def elevate(roughness, iterations):
    size = 2**iterations + 1
    elevation = np.zeros(shape=(size, size), dtype=float)

    for n in range(iterations, 0, -1):
        step = 2**n
        offset = step // 2
        magnitude = 2**((n - iterations) / roughness)
        # Center points of the subdivisions
        for x in range(offset, size, step):
            for y in range(offset, size, step):
                elevation[x, y] = 0.25 * (
                    elevation[x - offset, y] +
                    elevation[x + offset, y] +
                    elevation[x, y - offset] +
                    elevation[x, y + offset]
                ) + random.gauss(0, 1) * magnitude
        # Intermediate row/column points
        for x in range(0, size, step):
            for y in range(offset, size, step):
                avg = elevation[x, y - offset] + elevation[x, y + offset]
                count = 2
                if x > 0:
                    avg += elevation[x - offset, y]
                    count += 1
                if x < size - 1:
                    avg += elevation[x + offset, y]
                    count += 1
                elevation[x, y] = 1.0 * avg / count + random.gauss(0, 1) * magnitude
        for x in range(offset, size, step):
            for y in range(0, size, step):
                avg = elevation[x - offset, y] + elevation[x + offset, y]
                count = 2
                if y > 0:
                    avg += elevation[x, y - offset]
                    count += 1
                if y < size - 1:
                    avg += elevation[x, y + offset]
                    count += 1
                elevation[x, y] = 1.0 * avg / count + random.gauss(0, 1) * magnitude
    return elevation


def triangulate(elevation, scale, *args):
    scale_x = 2.0 / (elevation.shape[0] - 1)
    scale_y = 2.0 / (elevation.shape[1] - 1)
    for x in range(elevation.shape[0] - 1):
        for y in range(elevation.shape[1] - 1):
            a = Point(
                -1 + scale_x * x,
                -1 + scale_y * y,
                elevation[x, y]
            )
            b = Point(
                -1 + scale_x * (x + 1),
                -1 + scale_y * y,
                elevation[x + 1, y]
            )
            c = Point(
                -1 + scale_x * x,
                -1 + scale_y * (y + 1),
                elevation[x, y + 1]
            )
            d = Point(
                -1 + scale_x * (x + 1),
                -1 + scale_y * (y + 1),
                elevation[x + 1, y + 1]
            )
            yield Triangle(a, b, c, *args)
            yield Triangle(b, c, d, *args)


roughness = 0.25
iterations = 4
elevation = 0.2 * elevate(roughness, iterations)

gray = Material(0.1, 1.0, 1.0, 0.2, 0.0, Color(0.99, 0.99, 0.99))
triangles = list(triangulate(elevation, 1.0, gray))

normal = Ray(Point(-2.0, -2.0, -1.0), Point(1.0, 1.0, 0.5))
camera = Camera(normal, 1.25, 0.75, 2)

light = Material(0.0, 0.0, 0.1, 0.0, 1.0, Color(1.0, 1.0, 1.0))
objects = triangles + [
    Sphere(Point(-100.0, -60.0, -60.0), 90.0, light),
]

dpi = int(300)
samples = 200
bounces = 4

render(camera, objects, "example.jpg", dpi, samples, bounces)
