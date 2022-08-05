"""Material class wrapper.

Provides methods that affect how the light reacts to a collision.
Currently support Metal, Diffuse, Mirror, and Dielectric materials.
"""

from .util import is_vec3, typed_scaler


class Material():
    """Base material object for all `Rust-Raytracing` materials."""

    ron_string = ''

    def _to_ron(self):
        return self.ron_string


class Diffuse(Material):
    """Wrapper for the `Rust-Raytracing` lambertian type."""

    def __init__(self, colour):
        self.colour = is_vec3(colour, 'Diffuse colour property')

    def _to_ron(self):
        return f'["Lambertian", "{self.colour[0]}", "{self.colour[1]}", "{self.colour[2]}"]'


class Metal(Material):
    """Wrapper for the `Rust-Raytracing` metal type."""

    def __init__(self, colour, fuzz):
        self.colour = is_vec3(colour, 'Metal colour property')
        self.fuzz = typed_scaler(fuzz, float, 'creation of Metal Fuzz property')

    def _to_ron(self):
        return (f'["Metal", "{self.colour[0]}", "{self.colour[1]}", "{self.colour[2]}", '
                f'"{self.fuzz}"]')


class Mirror(Material):
    """Wrapper for the `Rust-Raytracing` mirror type."""

    def __init__(self):
        self.ron_string = '["Mirror"]'


class Dielectric(Material):
    """Wrapper for the `Rust-Raytracing` dielectric type."""

    def __init__(self, refractive_index):
        self.refractive_index = typed_scaler(refractive_index,
                                             float,
                                             'creation of Dielectric material'
        )

    def _to_ron(self):
        return f'["Dielectric", "{self.refractive_index}"]'
