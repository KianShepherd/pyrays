import pytest

import pyrays


def test_mirror():
    x = pyrays.Mirror()
    assert isinstance(x, pyrays.Mirror)
    assert '["Mirror"]' == x._to_ron()


def test_dielectric():
    x = pyrays.Dielectric(1.0)
    assert isinstance(x, pyrays.Dielectric)
    assert '["Dielectric", "1.0"]' == x._to_ron()

    with pytest.raises(TypeError):
        pyrays.Dielectric("a")


def test_diffuse():
    x = pyrays.Diffuse([255, 255, 255])
    assert isinstance(x, pyrays.Diffuse)
    assert '["Lambertian", "255.0", "255.0", "255.0"]' == x._to_ron()
    x = pyrays.Diffuse([255.0, 255.0, 255.0])
    assert isinstance(x, pyrays.Diffuse)
    assert '["Lambertian", "255.0", "255.0", "255.0"]' == x._to_ron()

    with pytest.raises(TypeError):
        pyrays.Diffuse(["a", 255.0, 255.0])
    with pytest.raises(TypeError):
        pyrays.Diffuse(["a", 3.0, 255.0, 255.0])
    with pytest.raises(TypeError):
        pyrays.Diffuse("a")


def test_metal():
    x = pyrays.Metal([255, 255, 255], 0.5)
    assert isinstance(x, pyrays.Metal)
    assert '["Metal", "255.0", "255.0", "255.0", "0.5"]' == x._to_ron()
    x = pyrays.Metal([255.0, 255.0, 255.0], 1.0)
    assert isinstance(x, pyrays.Metal)
    assert '["Metal", "255.0", "255.0", "255.0", "1.0"]' == x._to_ron()

    with pytest.raises(TypeError):
        pyrays.Metal(["a", 255.0, 255.0], 1.0)
    with pytest.raises(TypeError):
        pyrays.Metal(["a", 3.0, 255.0, 255.0], 1.0)
    with pytest.raises(TypeError):
        pyrays.Metal("a", 1.0)
    with pytest.raises(TypeError):
        pyrays.Metal([255.0, 255.0, 255.0], "a")
