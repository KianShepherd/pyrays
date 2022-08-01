"""Material class wrapper.

Provides methods that affect how the light reacts to a collision.
Currently support Metal, Diffuse, Mirror, and Dielectric materials.
"""

from .util import is_vec3


class Material():
    """Base material object for all `Rust-Raytracing` materials."""

    def _to_ron(self):
        return self.ron_string


class Diffuse(Material):
    """Wrapper for the `Rust-Raytracing` lambertian type."""

    def __init__(self, colour):
        new_colour = is_vec3(colour)
        if new_colour == False:
            raise TypeError('Expected Vec3 object for Diffuse colour property.')
        self.colour = new_colour
        self.ron_string = f'["Lambertian", "{colour[0]}", "{colour[1]}", "{colour[2]}"]'


class Metal(Material):
    """Wrapper for the `Rust-Raytracing` metal type."""

    def __init__(self, colour, fuzz):
        new_colour = is_vec3(colour)
        if new_colour == False:
            raise TypeError('Expected Vec3 object for Metal colour property.')
        if not (type(fuzz) is float):
            raise TypeError('Expected float object in creation of Metal fuzz property.')
        self.colour = new_colour
        print(self.colour)
        self.fuzz = fuzz
        self.ron_string = f'["Metal", "{self.colour[0]}", "{self.colour[1]}", "{self.colour[2]}", "{fuzz}"]'


class Mirror(Material):
    """Wrapper for the `Rust-Raytracing` mirror type."""

    def __init__(self):
        self.ron_string = '["Mirror"]'


class Dielectric(Material):
    """Wrapper for the `Rust-Raytracing` dielectric type."""

    def __init__(self, refractive_index):
        if not (type(refractive_index) is float):
            raise TypeError('Expected float object in creation of Dielectric material.')
        self.ron_string = f'["Dielectric", "{refractive_index}"]'
