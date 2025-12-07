"""
Advent of Code 2024 Day 2 - Refactored Solution
Analyzes reactor safety reports to determine which are safe.

A report is safe if:
1. All levels are either increasing or decreasing
2. Adjacent levels differ by at least 1 and at most 3
"""

from typing import List


def read_reports(file_path: str) -> List[List[int]]:
    """Read and parse reports from file."""
    with open(file_path, "r") as file:
        return [list(map(int, line.strip().split())) for line in file]


def is_safe_report(report: List[int]) -> bool:
    """
    Check if a report is safe using a single-pass algorithm.
    
    A report is safe if:
    - All levels are either increasing or decreasing
    - Adjacent levels differ by 1-3
    """
    if len(report) < 2:
        return True
    
    # Determine direction from first two elements
    first_diff = report[1] - report[0]
    if abs(first_diff) < 1 or abs(first_diff) > 3:
        return False
    
    is_increasing = first_diff > 0
    
    # Check remaining adjacent pairs
    for i in range(2, len(report)):
        diff = report[i] - report[i - 1]
        
        # Check if difference is in valid range
        if abs(diff) < 1 or abs(diff) > 3:
            return False
        
        # Check if direction is consistent
        if (diff > 0) != is_increasing:
            return False
    
    return True


def find_safe_reports(reports: List[List[int]]) -> List[List[int]]:
    """Find all safe reports from the input list."""
    return [report for report in reports if is_safe_report(report)]


def count_safe_reports(reports: List[List[int]]) -> int:
    """Count the number of safe reports."""
    return sum(1 for report in reports if is_safe_report(report))


# Alternative implementation using more functional approach
def is_safe_report_functional(report: List[int]) -> bool:
    """Alternative implementation using more functional programming style."""
    if len(report) < 2:
        return True
    
    # Calculate all differences
    differences = [report[i] - report[i - 1] for i in range(1, len(report))]
    
    # Check if all differences are in valid range (1-3)
    if not all(1 <= abs(diff) <= 3 for diff in differences):
        return False
    
    # Check if all differences have the same sign (all increasing or all decreasing)
    return all(diff > 0 for diff in differences) or all(diff < 0 for diff in differences)


# Even more concise version
def is_safe_report_concise(report: List[int]) -> bool:
    """Most concise implementation."""
    if len(report) < 2:
        return True
    
    diffs = [b - a for a, b in zip(report, report[1:])]
    return all(1 <= abs(d) <= 3 for d in diffs) and (
        all(d > 0 for d in diffs) or all(d < 0 for d in diffs)
    )


if __name__ == "__main__":
    # Example usage
    import os
    
    current_dir = os.path.dirname(os.path.abspath(__file__))
    file_path = os.path.join(current_dir, "report.txt")
    
    reports = read_reports(file_path)
    safe_reports = find_safe_reports(reports)
    
    print(f"Total reports: {len(reports)}")
    print(f"Safe reports: {len(safe_reports)}")
    print(f"Safe report count: {count_safe_reports(reports)}")
    
    # Display safe reports
    for i, report in enumerate(safe_reports, 1):
        print(f"Safe report {i}: {report}")
