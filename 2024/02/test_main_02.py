from main_02 import (
    read_in_reports,
    find_save_reports,
    check_if_all_levels_decreasing,
    check_if_all_levels_increasing,
    is_report_save,
)
import os
import pytest


@pytest.fixture
def report_file_path():
    current_dir = os.path.dirname(os.path.abspath(__file__))
    file_path = os.path.join(current_dir, "report.txt")
    return read_in_reports(file_path=file_path)


@pytest.fixture
def reports():
    return [
        [7, 6, 4, 2, 1],  # Safe: decreasing by 1-2
        [1, 2, 7, 8, 9],  # Unsafe: increase of 5
        [9, 7, 6, 2, 1],  # Unsafe: decrease of 4
        [1, 3, 2, 4, 5],  # Unsafe: mixed increase/decrease
        [8, 6, 4, 4, 1],  # Unsafe: no change (4 4)
        [1, 3, 6, 7, 9],  # Safe: increasing by 1-3
    ]


def test_find_amount_of_safe_reports(report_file_path) -> None:
    saved_reports = find_save_reports(report_file_path)
    assert len(saved_reports) == 2
    assert saved_reports[0] == [7, 6, 4, 2, 1]
    assert saved_reports[1] == [1, 3, 6, 7, 9]


@pytest.mark.parametrize(
    "report, all_decreasing",
    [
        ([7, 6, 5, 4, 3, 2, 1], True),
        ([7, 6, 5, 4, 3, 2, 3], False),
        ([1, 2, 3, 4, 5, 6, 7], False),
    ],
)
def test_check_if_all_levels_decresing(report, all_decreasing):
    result = check_if_all_levels_decreasing(report)
    assert result == all_decreasing


@pytest.mark.parametrize(
    "report, all_increasing",
    [
        ([7, 6, 5, 4, 3, 2, 1], False),
        ([7, 6, 5, 4, 3, 2, 3], False),
        ([1, 2, 3, 4, 5, 6, 7], True),
    ],
)
def test_check_if_all_levels_increasing(report, all_increasing):
    result = check_if_all_levels_increasing(report)
    assert result == all_increasing


@pytest.mark.parametrize(
    "report, expected",
    [
        ([1, 2, 3, 4, 5], True),  # All increasing by 1
        ([5, 4, 3, 2, 1], True),  # All decreasing by 1
        ([1, 3, 5, 7, 9], True),  # All increasing by 2
        ([9, 7, 5, 3, 1], True),  # All decreasing by 2
        ([1, 4, 7, 10], True),  # All increasing by 3
        ([10, 7, 4, 1], True),  # All decreasing by 3
        ([1, 5], False),  # Increase by 4 (too much)
        ([5, 1], False),  # Decrease by 4 (too much)
        ([1, 1], False),  # No change
        ([1, 2, 1], False),  # Mixed direction
        ([1], True),  # Single element (edge case)
        ([], True),  # Empty list (edge case)
    ],
)
def test_is_report_save(report, expected) -> None:
    assert is_report_save(report) == expected
