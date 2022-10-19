"""
Wrapper for the various object types that can be used with the raytracer.

Base shapes are the sphere, triangle, and square.
"""
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
        self.p1 = is_vec3(p1, 'ProceduralTerrain point one')
        self.p2 = is_vec3(p2, 'ProceduralTerrain point two')
        if self.p1[1] != self.p2[1]:
            raise TypeError('y value of the plane points must be equal.')
        self.ppa = typed_scaler(points_per_axis, int, 'points per axis property')
        if self.ppa < 2:
            raise TypeError('points per axis must be at least 2')
        if not issubclass(type(material), Material):
            raise TypeError('Expected a pyrays Material for the Sphere object material property.')
        x_diff = self.p2[0] - self.p1[0]
        z_diff = self.p2[2] - self.p1[2]
        self.points = [
            [
                [
                    self.p1[0] + (x_diff * (x / (self.ppa - 1))),
                    self.p1[1],
                    self.p1[2] + (z_diff * (z / (self.ppa - 1)))
                ] for x in range(self.ppa)
            ] for z in range(self.ppa)
        ]
        self.material = material

    def _parse_octaves(self, octa):
        if type(octa) is int or type(octa) is float:
            octa = [int(octa)]
        elif type(octa) is list:
            try:
                for i in range(len(octa)):
                    octa[i] = int(octa[i])
            except BaseException:
                raise TypeError('Expected int or list of ints for octaves property.')
        else:
            raise TypeError('Expected int or list of ints for octaves property.')
        return octa

    def perlin_heightmap(self, octa, seed, magnitude):
        """Apply a heightmap to the terrain using perlin noise."""
        octa = self._parse_octaves(octa)
        seed = typed_scaler(seed, int, 'seed property')
        magnitude = typed_scaler(magnitude, float, 'magnitude property')
        self.magnitude = magnitude
        noises = []
        alpha = 1.0
        for oc in octa:
            noises.append(PerlinNoise(octaves=oc, seed=seed))

        max_noise = -1000000.0
        min_noise = 1000000.0
        for i in range(self.ppa):
            for j in range(self.ppa):
                noise_val = 0.0
                alpha = 1.0
                for noise in noises:
                    noise_val += alpha * noise([i / self.ppa, j / self.ppa])
                    alpha /= 2.0
                if noise_val < min_noise:
                    min_noise = noise_val
                if noise_val > max_noise:
                    max_noise = noise_val
                self.points[i][j][1] = noise_val

        for i in range(self.ppa):
            for j in range(self.ppa):
                p = self.points[i][j][1]
                self.points[i][j][1] = ((p + (-min_noise)) / (max_noise + (-min_noise))) * magnitude

    def _to_ron(self):
        triangles = []
        for y in range(self.ppa - 1):
            for x in range(self.ppa - 1):
                p1 = [
                    self.points[y][x][0],
                    self.points[y][x][1],
                    self.points[y][x][2]
                ]
                p2 = [
                    self.points[y][x + 1][0],
                    self.points[y][x + 1][1],
                    self.points[y][x + 1][2]
                ]
                p3 = [
                    self.points[y + 1][x + 1][0],
                    self.points[y + 1][x + 1][1],
                    self.points[y + 1][x + 1][2]
                ]
                p4 = [
                    self.points[y + 1][x][0],
                    self.points[y + 1][x][1],
                    self.points[y + 1][x][2]
                ]
                h1 = ((p1[1] + p2[1] + p3[1]) / 3.0) / self.magnitude
                h2 = ((p1[1] + p3[1] + p4[1]) / 3.0) / self.magnitude
                triangles.append(Triangle(p1, p3, p2, self.material, True, height=h1))
                triangles.append(Triangle(p1, p4, p3, self.material, True, height=h2))
        ron_str = ''
        for i in range(len(triangles)):
            ron_str += triangles[i]._to_ron()
            if i != (len(triangles) - 1):
                ron_str += ', '
        return ron_str
