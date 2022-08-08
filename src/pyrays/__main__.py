from .rayobject import ProceduralTerrain, Sphere, Square, Triangle
from .material import Dielectric, Diffuse, Metal, Mirror
from .scene import Scene


image_width = 720
image_height = 407
samples_per_pixel = 25
max_depth = 25
v_fov = 90
aperture = 0.01
focal_distance = 1.5
camera_pos = [0, 12.0, -15.0]
camera_look_at = [0, 0, 0]
camera_up = [0, 1, 0]
multithreading = True

scene = Scene(camera_pos, camera_look_at, camera_up, focal_distance, aperture, v_fov)

scene.add_light([-1.0, 1.5, -3.5])

terrain = ProceduralTerrain([-10.0, 0.0, -10.0], [10.0, 0.0, 10.0], 50, Diffuse([0.7, 0, 0.8]))
terrain.perlin_heightmap([3, 6, 12, 24], 1, 6.0)

scene.add_object(terrain)
image = scene.raytrace(image_width, image_height, samples_per_pixel, max_depth, multithreading)

image.show()
