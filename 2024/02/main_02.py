def read_in_reports(file_path: str) -> list[list[int]]:
    with open(file_path, "r") as file:
        return [list(map(int, line.strip().split())) for line in file.readlines()]


def check_if_all_levels_increasing(report: list[int]) -> bool:
    for idx, level in enumerate(report):
        if idx == 0:
            continue

        # n-th level is greater, but should have been smaller
        if level < report[idx - 1]:
            return False
    return True


def check_if_all_levels_decreasing(report: list[int]) -> bool:
    for idx, level in enumerate(report):
        if idx == 0:
            continue

        # n-th level is smaller, but should have been greater
        if level > report[idx - 1]:
            return False
    return True


def check_difference_of_adjacent_levels(first_level: int, seconds_level: int) -> bool:
    difference = abs(first_level - seconds_level)
    if difference > 0 and difference < 4:
        return True
    return False


def is_report_save(report: list[int]) -> bool:
    if len(report) < 2:
        return True

    first_difference = report[0] - report[1]
    if abs(first_difference) < 1 or abs(first_difference) > 3:
        return False

    is_increasing = first_difference < 0

    for level_idx in range(2, len(report)):
        difference = report[level_idx - 1] - report[level_idx]

        # check if difference is in valid range
        if abs(difference) < 1 or abs(difference) > 3:
            return False

        if (difference < 0) != is_increasing:
            return False

    return True


def find_save_reports(reports: list[list[int]]) -> list[list[int]]:
    return [report for report in reports if is_report_save(report)]
