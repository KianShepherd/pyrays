"""Test rust functionality."""

import raspy


def test_sum_as_string():
    """Test sum as string rust func."""
    assert '3' == raspy.sum_as_string(1, 2)
