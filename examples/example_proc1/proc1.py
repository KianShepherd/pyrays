"""Example procedural generation 1."""

from pyrays import ProceduralTerrain, Diffuse, HeightMap, Scene


image_width = 7680
aspect_ratio = 9.0 / 16.0
image_height = int(aspect_ratio * image_width)
samples_per_pixel = 250
resolution = 6000
_resolution = resolution
resolution *= 4
max_depth = 200
v_fov = 90
aperture = 0.01
focal_distance = 1.5
camera_pos = [0, resolution * 1.35, -1.1 * resolution]
camera_look_at = [0, 0, 0]
camera_up = [0, 1, 0]
multithreading = True

image = Scene(
    camera_pos,
    camera_look_at,
    camera_up,
    focal_distance,
    aperture,
    v_fov
).add_object(
    ProceduralTerrain(
        [-1.0 * resolution, 0.0, -1.0 * resolution],
        [1.0 * resolution, 0.0, 1.0 * resolution],
        _resolution,
        HeightMap(
            {
                0.87: Diffuse([0.95, 0.95, 0.95]),
                0.60: Diffuse([0.45, 0.45, 0.45]),
                0.39: Diffuse([0.0, 0.6, 0.0]),
                0.25: Diffuse([0.0, 0.4, 0.0]),
                0.10: Diffuse([0, 0, 0.3]),
                0.00: Diffuse([0, 0, 0.2]),
            },
            fuzz=0.07
        )
    ).perlin_heightmap(
        octa=14,
        seed=121,
        magnitude=0.6 * resolution,
        frequency=3 / _resolution,
        lacunarity=2,
        persistence=0.5
    )
).add_light(
    [-1.7 * resolution, 2.5 * resolution, -1.5 * resolution]
).raytrace(
    image_width,
    image_height,
    samples_per_pixel,
    max_depth,
    multithreading
)

image.show()
image.save('img.png')
