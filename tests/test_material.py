import pyrays

def test_mirror():
    x = pyrays.Mirror()
    assert isinstance(x, pyrays.Mirror)
    assert '["Mirror"]' == x._to_ron()
