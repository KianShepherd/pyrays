from .rayobject import ProceduralTerrain, Sphere, Square, Triangle
from .material import Dielectric, Diffuse, Metal, Mirror
from .scene import Scene


image_width = 300
image_height = 169
samples_per_pixel = 25
max_depth = 25
v_fov = 90
aperture = 0.01
focal_distance = 1.5
camera_pos = [0, 1.0, -1.5]
camera_look_at = [0, 0, 0]
camera_up = [0, 1, 0]
multithreading = True

scene = Scene(camera_pos, camera_look_at, camera_up, focal_distance, aperture, v_fov)

scene.add_light([-1.0, 1.5, -3.5])


scene.add_object(ProceduralTerrain([-1.0, 0.0, -1.0], [1.0, 0.0, 1.0], 50, Diffuse([0.7, 0, 0.8])))
image = scene.raytrace(image_width, image_height, samples_per_pixel, max_depth, multithreading)

image.show()
