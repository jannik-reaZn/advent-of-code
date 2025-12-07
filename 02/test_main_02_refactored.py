"""
Tests for the refactored Advent of Code 2024 Day 2 solution.
"""

import os
import pytest
from main_02_refactored import (
    read_reports,
    is_safe_report,
    find_safe_reports,
    count_safe_reports,
    is_safe_report_functional,
    is_safe_report_concise,
)


@pytest.fixture
def sample_reports():
    """Sample reports from the problem description."""
    return [
        [7, 6, 4, 2, 1],  # Safe: decreasing by 1-2
        [1, 2, 7, 8, 9],  # Unsafe: increase of 5
        [9, 7, 6, 2, 1],  # Unsafe: decrease of 4
        [1, 3, 2, 4, 5],  # Unsafe: mixed increase/decrease
        [8, 6, 4, 4, 1],  # Unsafe: no change (4 4)
        [1, 3, 6, 7, 9],  # Safe: increasing by 1-3
    ]


@pytest.fixture
def report_file_data():
    """Load reports from the test file."""
    current_dir = os.path.dirname(os.path.abspath(__file__))
    file_path = os.path.join(current_dir, "report.txt")
    return read_reports(file_path)


def test_is_safe_report_with_sample_data(sample_reports):
    """Test is_safe_report function with the sample data."""
    expected_results = [True, False, False, False, False, True]

    for report, expected in zip(sample_reports, expected_results):
        assert is_safe_report(report) == expected, f"Failed for report: {report}"


def test_find_safe_reports_count(report_file_data):
    """Test that we find exactly 2 safe reports from the file."""
    safe_reports = find_safe_reports(report_file_data)
    assert len(safe_reports) == 2
    assert safe_reports[0] == [7, 6, 4, 2, 1]
    assert safe_reports[1] == [1, 3, 6, 7, 9]


def test_count_safe_reports(report_file_data):
    """Test the count_safe_reports function."""
    assert count_safe_reports(report_file_data) == 2


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
def test_is_safe_report_parametrized(report, expected):
    """Parametrized tests for various report scenarios."""
    assert is_safe_report(report) == expected


def test_alternative_implementations_consistency(sample_reports):
    """Test that all three implementations give the same results."""
    for report in sample_reports:
        result_main = is_safe_report(report)
        result_functional = is_safe_report_functional(report)
        result_concise = is_safe_report_concise(report)

        assert result_main == result_functional == result_concise, (
            f"Inconsistent results for {report}: "
            f"main={result_main}, functional={result_functional}, concise={result_concise}"
        )


def test_edge_cases():
    """Test edge cases."""
    # Empty and single-element reports should be safe
    assert is_safe_report([]) == True
    assert is_safe_report([1]) == True

    # Two-element reports
    assert is_safe_report([1, 2]) == True  # Valid increase
    assert is_safe_report([2, 1]) == True  # Valid decrease
    assert is_safe_report([1, 5]) == False  # Too large increase
    assert is_safe_report([5, 1]) == False  # Too large decrease
    assert is_safe_report([1, 1]) == False  # No change


def test_performance_comparison(sample_reports):
    """Basic performance test - all implementations should handle sample data quickly."""
    import time

    implementations = [
        ("main", is_safe_report),
        ("functional", is_safe_report_functional),
        ("concise", is_safe_report_concise),
    ]

    # Test with larger dataset
    large_dataset = sample_reports * 1000

    for name, func in implementations:
        start_time = time.time()
        results = [func(report) for report in large_dataset]
        end_time = time.time()

        # Just ensure it completes without error
        assert len(results) == len(large_dataset)
        print(f"{name} implementation: {end_time - start_time:.4f}s")
