from .rayobject import ProceduralTerrain, Sphere, Square, Triangle
from .material import Dielectric, Diffuse, HeightMap, Metal, Mirror
from .scene import Scene


image_width = 400
aspect_ratio = 9.0 / 16.0
image_height = int(aspect_ratio * image_width)
samples_per_pixel = 50
resolution = 500
_resolution = resolution
resolution *= 4
max_depth = 50
v_fov = 90
aperture = 0.01
focal_distance = 1.5
camera_pos = [0, resolution * 1.2, -12.0 * (resolution / 10.0)]
camera_look_at = [0, 0, 0]
camera_up = [0, 1, 0]
multithreading = True

scene = Scene(
    camera_pos,
    camera_look_at,
    camera_up,
    focal_distance,
    aperture,
    v_fov
)

scene.add_light(
    [-1.0 * resolution, 1.5 * resolution, -2.5 * resolution]
)

colour_map = {
    0.1: Diffuse([0, 0, 0.2]),
    0.25: Diffuse([0, 0, 0.3]),
    0.4: Diffuse([0.0, 0.4, 0.0]),
    0.67: Diffuse([0.0, 0.6, 0.0]),
    0.90: Diffuse([0.45, 0.45, 0.45]),
    1.0: Diffuse([0.95, 0.95, 0.95]),
}

terrain = ProceduralTerrain(
    [-1.0 * resolution, 0.0, -1.0 * resolution],
    [1.0 * resolution, 0.0, 1.0 * resolution],
    _resolution,
    HeightMap(colour_map, fuzz=0.05)
)
terrain.perlin_heightmap([3, 6, 12, 24, 48, 96], 1, 0.6 * resolution)

scene.add_object(terrain)
image = scene.raytrace(
    image_width,
    image_height,
    samples_per_pixel,
    max_depth,
    multithreading
)

image.show()
image.save('img.png')
