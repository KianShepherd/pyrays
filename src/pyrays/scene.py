"""Wrapper for the scene to be raytraced."""

import sys

from .pyrays_rs import create_scene
from .rayobject import RayObject, ProceduralTerrain
from .util import is_vec3, typed_scaler

from PIL import Image


class Scene:
    """Base scene object to be ratraced."""

    def __init__(self, camera_pos, camera_direction, camera_up, focal_distance, aperture, v_fov):
        self.camera_pos = is_vec3(camera_pos, 'Camera Position property')
        self.camera_dir = is_vec3(camera_direction, 'Camera Direction property')
        self.camera_up = is_vec3(camera_up, 'Camera Up property')
        self.focal_distance = typed_scaler(focal_distance, float, 'focal distance property')
        self.aperture = typed_scaler(aperture, float, 'aperture property')
        self.v_fov = typed_scaler(v_fov, float, 'visual field of view property')
        self.lights = []
        self.objects = []

    def add_light(self, location):
        """Add a light to the scene."""
        self.lights.append(is_vec3(location, 'Light Location property'))
        return self

    def add_object(self, obj):
        """Add an object to the scene."""
        if not issubclass(type(obj), RayObject):
            raise TypeError(f'Expected a pyrays RayObject type. Found {type(obj)}')
        self.objects.append(obj)
        return self

    def _to_ron(self, image_meta):
        res = ('RaytracerScene(multithreading: '
               f'{"true" if image_meta["multithreading"] else "false"}, aspect_ratio: '
               f'{image_meta["image_width"] / image_meta["image_height"]},'
               f' image_width: {image_meta["image_width"]}, image_height: '
               f'{image_meta["image_height"]}, samples_per_pixel: '
               f'{image_meta["samples_per_pixel"]}, max_depth: {image_meta["max_depth"]},'
               f'v_fov: {self.v_fov}, aperture: {self.aperture}, focal_distance: '
               f'{self.focal_distance}, camera_pos: {self.camera_pos}, camera_dir: '
               f'{self.camera_dir}, camera_up: {self.camera_up}, objects: ['
        )
        has_terrain = False
        terrain = None
        for i in range(len(self.objects)):
            if isinstance(self.objects[i], ProceduralTerrain):
                has_terrain = True
                terrain = self.objects[i]
            else:
                res += self.objects[i]._to_ron()
                if i != len(self.objects) - 1:
                    res += ', '
        res += '], lights: ['
        for i in range(len(self.lights)):
            res += str(self.lights[i])
            if i != len(self.lights) - 1:
                res += ', '
        res += '],'
        res += f'has_terrain: {"1" if has_terrain else "0"}, '
        if has_terrain:
            res += (
                f'terrain: (p1: {terrain.p1}, p2: {terrain.p2}, resolution: {terrain.ppa}, '
                f'octaves: {terrain.octave}, frequency: {terrain.frequency}, lacunarity: '
                f'{terrain.lacunarity}, seed_value: {terrain.seed}, magnitude: {terrain.magnitude}, '
                f'persistence: {terrain.persistence}, fuzz: {terrain.material.fuzz}, map_cutoff: '
                f'{list(terrain.material.map.keys())}, map_value: '
                f'{[x.colour for x in list(terrain.material.map.values())]})'
            )
        else:
            res += (
                'terrain: (p1: [0.0, 0.0, 0.0], p2: [0.0, 0.0, 0.0], resolution: 0, octaves: 0, '
                'frequency: 0.0, lacunarity: 0.0, seed_value: 0, magnitude: 0, persistence: 0.0, '
                'fuzz: 0.0, map_cutoff: [0.0], map_value: [[0.0, 0.0, 0.0]])'
            )
        res += ')'
        return res

    def raytrace(self,
                 image_width,
                 image_height,
                 samples_per_pixel,
                 max_depth,
                 multithreading,
                 *,
                 _debug=False
    ) -> Image.Image:
        """Raytrace the scene."""
        image_meta = {
            'image_width': typed_scaler(image_width, int, 'image width'),
            'image_height': typed_scaler(image_height, int, 'image height'),
            'samples_per_pixel': typed_scaler(samples_per_pixel, int, 'samples per pixel'),
            'max_depth': typed_scaler(max_depth, int, 'max ray depth'),
            'multithreading': multithreading
        }
        pil_image = Image.new('RGB', (image_width, image_height))
        print('Creating raytracer scene config.', file=sys.stderr)
        ron_str = self._to_ron(image_meta)
        print('Created scene config.\n', file=sys.stderr)
        if _debug:
            print(ron_str)
            return pil_image

        print('Loading Scene Data.', file=sys.stderr)
        image = create_scene(ron_str)

        for y in range(image_height):
            for x in range(image_width):
                pil_image.putpixel((x, y), (image[y][x][0], image[y][x][1], image[y][x][2]))

        return pil_image
