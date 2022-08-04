"""
Wrapper for the various object types that can be used with the raytracer.

Base shapes are the sphere, triangle, and square.
"""

from .util import is_vec3
from .material import Material


class RayObject():
    """Base object for all `Rust-Raytracing` objects."""

    # def _to_ron(self):
    #     return self.ron_str


class Sphere(RayObject):
    """Wrapper for 3D sphere objects."""

    def __init__(self, position, radius, material):
        pos = is_vec3(position)
        if pos is False:
            raise TypeError('Expected Vec3 object for Sphere object position property.')
        try:
            radius = float(radius)
        except BaseException:
            raise TypeError('Expected a float object for Sphere object radius property.')
        if not issubclass(type(material), Material):
            raise TypeError('Expected a pyrays Material for the Sphere object material property.')
        self.position = pos
        self.radius = radius
        self.material = material

    def _to_ron(self):
        return (f'(objtype: "Sphere", vectors: [{self.position}], scalars: [{self.radius}], '
                f'material: {self.material._to_ron()})')


class Triangle(RayObject):
    """Wrapper for 2D triangle objects."""

    def __init__(self, p1, p2, p3, material, back_face_culling):
        x = is_vec3(p1)
        if x is False:
            raise TypeError('Expected Vec3 object for Triangle point one property.')
        y = is_vec3(p2)
        if y is False:
            raise TypeError('Expected Vec3 object for Triangle point two property.')
        z = is_vec3(p3)
        if z is False:
            raise TypeError('Expected Vec3 object for Triangle point three property.')
        if back_face_culling:
            self.cull = 1.0
        else:
            self.cull = 0.0
        if not issubclass(type(material), Material):
            raise TypeError('Expected a pyrays Material for the Sphere object material property.')
        self.p1 = x
        self.p2 = y
        self.p3 = z
        self.material = material

    def _to_ron(self):
        return (f'(objtype: "Triangle", vectors: [{str(self.p1)}, {str(self.p2)}, {str(self.p3)}],'
                f'scalars: [{self.cull}], material: {self.material._to_ron()})')


class Square(RayObject):
    """Wrapper for 2D triangle objects."""

    def __init__(self, p1, p2, p3, p4, material):
        x = is_vec3(p1)
        if x is False:
            raise TypeError('Expected Vec3 object for Triangle point one property.')
        y = is_vec3(p2)
        if y is False:
            raise TypeError('Expected Vec3 object for Triangle point two property.')
        z = is_vec3(p3)
        if z is False:
            raise TypeError('Expected Vec3 object for Triangle point three property.')
        w = is_vec3(p4)
        if w is False:
            raise TypeError('Expected Vec3 object for Triangle point four property.')
        if not issubclass(type(material), Material):
            raise TypeError('Expected a pyrays Material for the Sphere object material property.')
        self.p1 = x
        self.p2 = y
        self.p3 = z
        self.p4 = w
        self.material = material

    def _to_ron(self):
        t1 = Triangle(self.p1, self.p2, self.p3, self.material, True)
        t2 = Triangle(self.p1, self.p3, self.p4, self.material, True)
        return f'{t1._to_ron()}, {t2._to_ron()}'
