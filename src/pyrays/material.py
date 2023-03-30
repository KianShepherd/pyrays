"""Material class wrapper.

Provides methods that affect how the light reacts to a collision.
Currently support Metal, Diffuse, Mirror, and Dielectric materials.
"""

from random import random

from .util import is_vec3, typed_scaler


class Material():
    """Base material object for all `raytrace-rs` materials."""

    ron_string = ''

    def _to_ron(self):
        return self.ron_string


class Diffuse(Material):
    """Wrapper for the `raytrace-rs` lambertian type."""

    def __init__(self, colour):
        self.colour = is_vec3(colour, 'Diffuse colour property')

    def _to_ron(self):
        return f'["Lambertian", "{self.colour[0]}", "{self.colour[1]}", "{self.colour[2]}"]'


class Metal(Material):
    """Wrapper for the `raytrace-rs` metal type."""

    def __init__(self, colour, fuzz):
        self.colour = is_vec3(colour, 'Metal colour property')
        self.fuzz = typed_scaler(fuzz, float, 'creation of Metal Fuzz property')

    def _to_ron(self):
        return (f'["Metal", "{self.colour[0]}", "{self.colour[1]}", "{self.colour[2]}", '
                f'"{self.fuzz}"]')


class Mirror(Material):
    """Wrapper for the `raytrace-rs` mirror type."""

    def __init__(self):
        self.ron_string = '["Mirror"]'


class Dielectric(Material):
    """Wrapper for the `raytrace-rs` dielectric type."""

    def __init__(self, refractive_index):
        self.refractive_index = typed_scaler(refractive_index,
                                             float,
                                             'creation of Dielectric material'
        )

    def _to_ron(self):
        return f'["Dielectric", "{self.refractive_index}"]'


class HeightMap(Material):
    """HeightMap material to produce different material objects bashed on a height map."""

    def __init__(self, colour_map, fuzz=0.0):
        self.map = colour_map
        self.fuzz = fuzz

    def _to_ron(self, height):
        if self.fuzz != 0.0:
            height = min(1.0, height + random() * self.fuzz)
        for col in self.map.keys():
            if height <= col:
                return self.map[col]._to_ron()
        raise RuntimeError('Height too large for height map material.')
