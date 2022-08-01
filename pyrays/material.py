"""Material class wrapper.

Provides methods that affect how the light reacts to a collision.
Currently support Metal, Diffuse, Mirror, and Dielectric materials.
"""


class Material():
    """Base material object for all `Rust-Raytracing` materials."""

    def _to_ron(self):
        return self.ron_string


class Diffuse(Material):
    """Wrapper for the `Rust-Raytracing` lambertian type."""

    def __init__(self, colour):
        self.colour = colour
        self.ron_string = f'["Lambertian", "{colour[0]}", "{colour[1]}", "{colour[2]}"]'


class Metal(Material):
    """Wrapper for the `Rust-Raytracing` metal type."""

    def __init__(self, colour, fuzz):
        self.colour = colour
        self.fuzz = fuzz
        self.ron_string = f'["Metal", "{colour[0]}", "{colour[1]}", "{colour[2]}", "{fuzz}"]'


class Mirror(Material):
    """Wrapper for the `Rust-Raytracing` mirror type."""

    def __init__(self):
        self.ron_string = '["Mirror"]'


class Dielectric(Material):
    """Wrapper for the `Rust-Raytracing` dielectric type."""

    def __init__(self, refractive_index):
        self.ron_string = f'["Dielectric", "{refractive_index}"]'
