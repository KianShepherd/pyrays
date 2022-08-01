"""
Wrapper for the various object types that can be used with the raytracer.

Base shapes are the sphere, triangle, and square.
"""


class RayObject():
    """Base object for all `Rust-Raytracing` objects."""

    def _to_ron(self):
        return self.ron_str


class Sphere(RayObject):
    """Wrapper for 3D sphere objects."""

    def __init__(self):
        pass


class Triangle(RayObject):
    """Wrapper for 2D triangle objects."""

    def __init__(self):
        pass


class Square(RayObject):
    """Wrapper for 2D square objects."""

    def __init__(self):
        pass
