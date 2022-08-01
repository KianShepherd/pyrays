"""Wrapper for the scene to be raytraced."""

from .rayobject import *


class Scene:
    """Base scene object to be ratraced."""

    def __init__(self, camera_pos, camera_direction, camera_up):
        self.camera_pos = camera_pos
        self.camera_dir = camera_direction
        self.camera_up = camera_up
        self.lights = []
        self.objects = []

    def add_light(self, location):
        """Add a light to the scene."""
        self.lights.append(location)

    def add_object(self, obj):
        """Add an object to the scene."""
        if not issubclass(type(obj), RayObject):
            raise TypeError(f'Expected a pyrays RayObject type. Found {type(obj)}')
        self.objects.append(obj)

    def raytrace(self):
        """Raytrace the scene."""
        pass
