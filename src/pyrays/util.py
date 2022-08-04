"""Utility functions for pyrays."""


def is_vec3(x):
    """Util func to check if an arg is a valic vec3."""
    if not (type(x) is list or type(x) is tuple):
        return False
    if len(x) != 3:
        return False
    try:
        return [float(y) for y in x]
    except BaseException:
        return False
