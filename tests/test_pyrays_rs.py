"""Test rust functionality."""

import pyrays


def test_sum_as_string():
    """Test sum as string rust func."""
    assert '3' == pyrays.sum_as_string(1, 2)
