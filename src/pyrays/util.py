"""Utility functions for pyrays."""


def is_vec3(x, err_str):
    """Util func to check if an arg is a valic vec3."""
    if not (type(x) is list or type(x) is tuple):
        raise TypeError(f'Expected Vec3 object for {err_str}.')
    if len(x) != 3:
        raise TypeError(f'Expected Vec3 object for {err_str}.')
    try:
        return [float(y) for y in x]
    except BaseException:
        raise TypeError(f'Expected Vec3 object for {err_str}.')


def typed_scaler(x, ty, err_str):
    """Util func to type check a scalar value."""
    try:
        return ty(x)
    except BaseException:
        raise TypeError(f'Expected type {str(ty)} for {err_str} got {type(x)}.')
