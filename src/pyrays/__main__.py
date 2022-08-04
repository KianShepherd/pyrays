from .rayobject import Sphere, Square, Triangle
from .material import Dielectric, Diffuse, Metal, Mirror
from .scene import Scene


image_width = 300
image_height = 169
samples_per_pixel = 25
max_depth = 25

scene = Scene([0, 0, -3.5], [0, 0, 0], [0, 1, 0], 3.5, 0.01, 90)

scene.add_light([-1.0, 1.5, -3.5])

scene.add_object(Sphere([0.6, 0.0, -1.5], 0.5, Metal([0.7, 0.6, 0.2], 0.3)))
scene.add_object(Sphere([-0.7, -1.0, -1.5], 0.5, Diffuse([0.85, 0.9, 0.75])))
scene.add_object(Sphere([0.1, 1.3, -1.0], 0.5, Mirror()))
scene.add_object(Square([-1.0, 1.0, -1.0],
                        [1.0, 1.0, -1.0],
                        [1.0, -1.0, -1.0],
                        [-1.0, -1.0, -1.0],
                        Diffuse([200, 230, 0])))

image = scene.raytrace(image_width, image_height, samples_per_pixel, max_depth, True)

image.show()
