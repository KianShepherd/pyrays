"""Wrapper for the scene to be raytraced."""

from .pyrays_rs import create_scene
from .rayobject import RayObject
from .util import is_vec3

from PIL import Image


class Scene:
    """Base scene object to be ratraced."""

    def __init__(self, camera_pos, camera_direction, camera_up, focal_distance, aperture, v_fov):
        camera_pos = is_vec3(camera_pos)
        if camera_pos is False:
            raise TypeError('Expected Vec3 object for camera position property.')
        camera_direction = is_vec3(camera_direction)
        if camera_direction is False:
            raise TypeError('Expected Vec3 object for camera direction property.')
        camera_up = is_vec3(camera_up)
        if camera_up is False:
            raise TypeError('Expected Vec3 object for camera up property.')
        try:
            focal_distance = float(focal_distance)
        except BaseException:
            raise TypeError('Expected float object for focal distance property.')
        try:
            aperture = float(aperture)
        except BaseException:
            raise TypeError('Expected float object for aperture property.')
        try:
            v_fov = float(v_fov)
        except BaseException:
            raise TypeError('Expected float object for visual field of view property.')
        self.camera_pos = camera_pos
        self.camera_dir = camera_direction
        self.camera_up = camera_up
        self.focal_distance = focal_distance
        self.aperture = aperture
        self.v_fov = v_fov
        self.lights = []
        self.objects = []

    def add_light(self, location):
        """Add a light to the scene."""
        loc = is_vec3(location)
        if loc is False:
            raise TypeError('Expected Vec3 object for light location property.')
        self.lights.append(loc)

    def add_object(self, obj):
        """Add an object to the scene."""
        if not issubclass(type(obj), RayObject):
            raise TypeError(f'Expected a pyrays RayObject type. Found {type(obj)}')
        self.objects.append(obj)

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
        for i in range(len(self.objects)):
            res += self.objects[i]._to_ron()
            if i != len(self.objects) - 1:
                res += ', '
        res += '], lights: ['
        for i in range(len(self.lights)):
            res += str(self.lights[i])
            if i != len(self.lights) - 1:
                res += ', '
        res += '])'
        return res

    def raytrace(self, image_width, image_height, samples_per_pixel, max_depth, multithreading, *, _debug=False): # noqa
        """Raytrace the scene."""
        try:
            image_width = int(image_width)
        except BaseException:
            raise TypeError('Expected int type object for image width.')
        try:
            image_height = int(image_height)
        except BaseException:
            raise TypeError('Expected int type object for image height.')
        try:
            samples_per_pixel = int(samples_per_pixel)
        except BaseException:
            raise TypeError('Expected int type object for samples per pixel.')
        try:
            max_depth = int(max_depth)
        except BaseException:
            raise TypeError('Expected int type object for max_depth.')
        multithreading = bool(multithreading)
        image_meta = {
            'image_width': image_width,
            'image_height': image_height,
            'samples_per_pixel': samples_per_pixel,
            'max_depth': max_depth,
            'multithreading': multithreading
        }

        ron_str = self._to_ron(image_meta)
        if _debug:
            return ron_str
        flat_rgb = create_scene(ron_str)
        image = [flat_rgb[x:x + image_width] for x in range(0, len(flat_rgb), image_width)]
        pil_image = Image.new('RGB', (image_width, image_height))

        for y in range(image_height):
            for x in range(image_width):
                pil_image.putpixel((x, y), (image[y][x][0], image[y][x][1], image[y][x][2]))

        return pil_image
