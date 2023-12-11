"""
Wrapper for the various object types that can be used with the raytracer.

Base shapes are the sphere, triangle, and square.
"""
import sys
import time

from perlin_noise import PerlinNoise

from .util import is_vec3, typed_scaler
from .material import HeightMap, Material


class RayObject():
    """Base object for all `raytrace-rs` objects."""

    # def _to_ron(self):
    #     return self.ron_str


class Sphere(RayObject):
    """Wrapper for 3D sphere objects."""

    def __init__(self, position, radius, material):
        if not issubclass(type(material), Material):
            raise TypeError('Expected a pyrays Material for the Sphere object material property.')
        self.position = is_vec3(position, 'Sphere position property')
        self.radius = typed_scaler(radius, float, 'Sphere object radius property')
        self.material = material

    def _to_ron(self):
        return (f'(objtype: "Sphere", vectors: [{self.position}], scalars: [{self.radius}], '
                f'material: {self.material._to_ron()})')


class Triangle(RayObject):
    """Wrapper for 2D triangle objects."""

    def __init__(self, p1, p2, p3, material, back_face_culling, *, height=0.0):
        if back_face_culling:
            self.cull = 1.0
        else:
            self.cull = 0.0
        if not issubclass(type(material), Material):
            raise TypeError('Expected a pyrays Material for the Sphere object material property.')
        self.p1 = is_vec3(p1, 'Triangle point one property')
        self.p2 = is_vec3(p2, 'Triangle point two property')
        self.p3 = is_vec3(p3, 'Triangle point three property')
        self.height = height
        self.material = material

    def _to_ron(self):
        if isinstance(self.material, HeightMap):
            return (f'(objtype: "Triangle", vectors: [{str(self.p1)}, {str(self.p2)}, '
                    f'{str(self.p3)}],'
                    f'scalars: [{self.cull}], material: {self.material._to_ron(self.height)})')
        else:
            return (f'(objtype: "Triangle", vectors: [{str(self.p1)}, {str(self.p2)}, '
                    f'{str(self.p3)}],'
                    f'scalars: [{self.cull}], material: {self.material._to_ron()})')


class Square(RayObject):
    """Wrapper for 2D triangle objects."""

    def __init__(self, p1, p2, p3, p4, material):
        if not issubclass(type(material), Material):
            raise TypeError('Expected a pyrays Material for the Sphere object material property.')
        self.p1 = is_vec3(p1, 'Square point one property')
        self.p2 = is_vec3(p2, 'Square point two property')
        self.p3 = is_vec3(p3, 'Square point three property')
        self.p4 = is_vec3(p4, 'Square point four property')
        self.material = material

    def _to_ron(self):
        t1 = Triangle(self.p1, self.p2, self.p3, self.material, True)
        t2 = Triangle(self.p1, self.p3, self.p4, self.material, True)
        return f'{t1._to_ron()}, {t2._to_ron()}'


class ProceduralTerrain(RayObject):
    """Wrapper for a procedurally generated plane."""

    def __init__(self, p1, p2, points_per_axis, material):
        print('Creating procedural terrain.', file=sys.stderr)
        self.p1 = is_vec3(p1, 'ProceduralTerrain point one')
        self.p2 = is_vec3(p2, 'ProceduralTerrain point two')
        if self.p1[1] != self.p2[1]:
            raise TypeError('y value of the plane points must be equal.')
        self.ppa = typed_scaler(points_per_axis, int, 'points per axis property')
        if self.ppa < 2:
            raise TypeError('points per axis must be at least 2')
        if not issubclass(type(material), Material):
            raise TypeError('Expected a pyrays Material for the Sphere object material property.')
        self.material = material
        print('Created procedural terrain.\n', file=sys.stderr)

    def perlin_heightmap(self, octa, seed, magnitude, frequency, lacunarity, persistence):
        """Apply a heightmap to the terrain using perlin noise."""
        octa = typed_scaler(octa, int, 'octaves property')
        seed = typed_scaler(seed, int, 'seed property')
        magnitude = typed_scaler(magnitude, float, 'magnitude property')
        self.octave = octa
        self.magnitude = magnitude
        self.frequency = frequency
        self.seed = seed
        self.lacunarity = lacunarity
        self.persistence = persistence
        return self
