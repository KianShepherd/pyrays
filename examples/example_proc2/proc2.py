"""Example procedural generation 2."""

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
                0.82: Diffuse([0.45, 0.45, 0.45]),
                0.60: Diffuse([0.30, 0.60, 0.00]),
                0.25: Diffuse([0.0, 0.6, 0.0]),
                0.00: Diffuse([0.0, 0.4, 0.0]),
            },
            fuzz=0.15
        )
    ).perlin_heightmap(
        octa=12,
        seed=15,
        magnitude=0.55 * resolution,
        frequency=1.5 / _resolution,
        lacunarity=2.5,
        persistence=0.4,
        erosion_factor=0.5
    )
).add_light(
    [1.1 * resolution, 1.25 * resolution, -1.1 * resolution]
).raytrace(
    image_width,
    image_height,
    samples_per_pixel,
    max_depth,
    multithreading
)

image.show()
image.save('img.png')
