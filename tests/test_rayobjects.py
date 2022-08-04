import pytest

import pyrays


def test_sphere():
    x = pyrays.Mirror()
    y = pyrays.Sphere([1.0, 2, 3.5], 1, x)
    assert y._to_ron() == '(objtype: "Sphere", vectors: [[1.0, 2.0, 3.5]], scalars: [1.0], material: ["Mirror"])'
    x = pyrays.material.Diffuse([255, 0, 0])
    y = pyrays.Sphere([0.0, 0.0, 0.0], 0.5, x)
    assert y._to_ron() == '(objtype: "Sphere", vectors: [[0.0, 0.0, 0.0]], scalars: [0.5], material: ["Lambertian", "255.0", "0.0", "0.0"])'

    with pytest.raises(TypeError):
        pyrays.Sphere('foo', 0.5, pyrays.Mirror())
    with pytest.raises(TypeError):
        pyrays.Sphere([0, 0, 0], 'foo', pyrays.Mirror())
    with pytest.raises(TypeError):
        pyrays.Sphere([0, 0, 0], 0.5, 'foo')


def test_triangle():
    x = pyrays.Mirror()
    y = pyrays.Triangle([1, 1, 1], [2, 2, 2], [3, 3, 3], x, False)

    assert y._to_ron() == '(objtype: "Triangle", vectors: [[1.0, 1.0, 1.0], [2.0, 2.0, 2.0], [3.0, 3.0, 3.0]],scalars: [0.0], material: ["Mirror"])'

    y = pyrays.Triangle([1, 1, 1], [2, 2, 2], [3, 3, 3], x, True)
    assert y._to_ron() == '(objtype: "Triangle", vectors: [[1.0, 1.0, 1.0], [2.0, 2.0, 2.0], [3.0, 3.0, 3.0]],scalars: [1.0], material: ["Mirror"])'

    with pytest.raises(TypeError):
        pyrays.Triangle('a', [2, 2, 2], [3, 3, 3], x, False)
    with pytest.raises(TypeError):
        pyrays.Triangle([1, 1, 1], 'a', [3, 3, 3], x, False)
    with pytest.raises(TypeError):
        pyrays.Triangle([1, 1, 1], [2, 2, 2], 'a', x, False)
    with pytest.raises(TypeError):
        pyrays.Triangle([1, 1, 1], [2, 2, 2], [3, 3, 3], 'a', False)
