from .rayobject import ProceduralTerrain, Sphere, Square, Triangle
from .material import Dielectric, Diffuse, Metal, Mirror, HeightMap
from .scene import Scene


image_width = 720
image_height = 407
samples_per_pixel = 25
max_depth = 25
v_fov = 90
aperture = 0.01
focal_distance = 1.5
camera_pos = [0, 10.5, -14.0]
camera_look_at = [0, 0, 0]
camera_up = [0, 1, 0]
multithreading = True

scene = Scene(camera_pos, camera_look_at, camera_up, focal_distance, aperture, v_fov)

scene.add_light([-1.0, 1.5, -3.5])

colour_map = {
    0.1: Diffuse([0, 0, 0.2]),
    0.25: Diffuse([0, 0, 0.3]),
    0.35: Diffuse([0.0, 0.4, 0.0]),
    0.6: Diffuse([0.0, 0.6, 0.0]),
    0.85: Diffuse([0.8, 0.8, 0.8]),
    1.0: Diffuse([0.95, 0.95, 0.95]),
}

terrain = ProceduralTerrain([-10.0, 0.0, -10.0], [10.0, 0.0, 10.0], 100, HeightMap(colour_map))
terrain.perlin_heightmap([3, 6, 12, 24], 1, 6.0)

scene.add_object(terrain)
image = scene.raytrace(image_width, image_height, samples_per_pixel, max_depth, multithreading)

image.show()
image.save('img.png')
