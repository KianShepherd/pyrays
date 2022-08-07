
import pytest

from PIL import Image
import imagehash

import pyrays


def test_scene_exceptions():
    with pytest.raises(TypeError):
        pyrays.Scene('a', [0.0, 0.0, 0.0], [0.0, 0.0, 0.0], 0.0, 0.0, 0.0)
    with pytest.raises(TypeError):
        pyrays.Scene([0.0, 0.0, 0.0], 'a', [0.0, 0.0, 0.0], 0.0, 0.0, 0.0)
    with pytest.raises(TypeError):
        pyrays.Scene([0.0, 0.0, 0.0], [0.0, 0.0, 0.0], 'a', 0.0, 0.0, 0.0)
    with pytest.raises(TypeError):
        pyrays.Scene([0.0, 0.0, 0.0], [0.0, 0.0, 0.0], [0.0, 0.0, 0.0], 'a', 0.0, 0.0)
    with pytest.raises(TypeError):
        pyrays.Scene([0.0, 0.0, 0.0], [0.0, 0.0, 0.0], [0.0, 0.0, 0.0], 0.0, 'a', 0.0)
    with pytest.raises(TypeError):
        pyrays.Scene([0.0, 0.0, 0.0], [0.0, 0.0, 0.0], [0.0, 0.0, 0.0], 0.0, 0.0, 'a')

    scene = pyrays.Scene([0.0, 0.0, 0.0], [0.0, 0.0, 0.0], [0.0, 0.0, 0.0], 0.0, 0.0, 0.0)
    with pytest.raises(TypeError):
        scene.add_light('a')
    with pytest.raises(TypeError):
        scene.add_object('a')

    with pytest.raises(TypeError):
        scene.raytrace('a', 0.0, 0.0, 0.0, False)
    with pytest.raises(TypeError):
        scene.raytrace(0.0, 'a', 0.0, 0.0, False)
    with pytest.raises(TypeError):
        scene.raytrace(0.0, 0.0, 'a', 0.0, False)
    with pytest.raises(TypeError):
        scene.raytrace(0.0, 0.0, 0.0, 'a', False)

def test_scene():
    image_width = 30
    image_height = 17
    samples_per_pixel = 25
    max_depth = 25

    scene = pyrays.Scene([0, 0, -3.5], [0, 0, 0], [0, 1, 0], 3.5, 0.01, 90)

    scene.add_light([-1.0, 1.5, -3.5])
    scene.add_light([-1.0, 1.5, -3.5])

    scene.add_object(pyrays.Sphere([0.6, 0.0, -1.5], 0.5, pyrays.Metal([0.7, 0.6, 0.2], 0.3)))
    scene.add_object(pyrays.Sphere([-0.7, -1.0, -1.5], 0.5, pyrays.Diffuse([0.85, 0.9, 0.75])))
    scene.add_object(pyrays.Sphere([0.1, 1.3, -1.0], 0.5, pyrays.Mirror()))
    scene.add_object(pyrays.Triangle(
        [0.0, 2.0, 3.0],
        [1.0, -1.0, 3.0],
        [-1.0, -1.0, 3.0],
        pyrays.Diffuse([0.0, 0.0, 0.0]),
        False)
    )
    image_ron = 'RaytracerScene(multithreading: false, aspect_ratio: 1.7647058823529411, image_width: 30, image_height: 17, samples_per_pixel: 25, max_depth: 25,v_fov: 90.0, aperture: 0.01, focal_distance: 3.5, camera_pos: [0.0, 0.0, -3.5], camera_dir: [0.0, 0.0, 0.0], camera_up: [0.0, 1.0, 0.0], objects: [(objtype: "Sphere", vectors: [[0.6, 0.0, -1.5]], scalars: [0.5], material: ["Metal", "0.7", "0.6", "0.2", "0.3"]), (objtype: "Sphere", vectors: [[-0.7, -1.0, -1.5]], scalars: [0.5], material: ["Lambertian", "0.85", "0.9", "0.75"]), (objtype: "Sphere", vectors: [[0.1, 1.3, -1.0]], scalars: [0.5], material: ["Mirror"]), (objtype: "Triangle", vectors: [[0.0, 2.0, 3.0], [1.0, -1.0, 3.0], [-1.0, -1.0, 3.0]],scalars: [0.0], material: ["Lambertian", "0.0", "0.0", "0.0"])], lights: [[-1.0, 1.5, -3.5], [-1.0, 1.5, -3.5]])'

    image = scene.raytrace(image_width, image_height, samples_per_pixel, max_depth, False, _debug=True)
    assert image == image_ron

def test_single_matches_multi():
    scenes = [
        ('tests/test_scenes/test_sphere/test_sphere_single.png',
         'tests/test_scenes/test_sphere/test_sphere_multi.png',
         'tests/test_scenes/comparison/test_sphere.png'),
        ('tests/test_scenes/test_triangle/test_triangle_single.png',
         'tests/test_scenes/test_triangle/test_triangle_multi.png',
         'tests/test_scenes/comparison/test_triangle.png'),
        ('tests/test_scenes/test_triangle/test_triangle_culled_single.png',
         'tests/test_scenes/test_triangle/test_triangle_culled_multi.png',
         'tests/test_scenes/comparison/test_triangle_culled.png')
    ]
    for (single, multi, comp) in scenes:
        single = Image.open(single).convert('RGB')
        multi = Image.open(multi).convert('RGB')
        known_good_image = Image.open(comp).convert('RGB')
        cutoff = 5  # maximum bits that could be different between the hashes. 

        assert imagehash.average_hash(single) - imagehash.average_hash(multi) < cutoff
        assert imagehash.average_hash(known_good_image) - imagehash.average_hash(multi) < cutoff
        assert imagehash.average_hash(single) - imagehash.average_hash(known_good_image) < cutoff
