import pytest
from main_1 import calulate_total_distance_between_lists


@pytest.mark.parametrize(
    "list1, list2, expected_distance",
    [
        ([3, 4, 2, 1, 3, 3], [4, 3, 5, 3, 9, 3], 11),
    ],
)
def test_total_distance_between_lists(list1, list2, expected_distance):
    distance = calulate_total_distance_between_lists(list1, list2)
    assert distance == expected_distance
