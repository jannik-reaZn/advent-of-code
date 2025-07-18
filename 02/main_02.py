def read_in_reports(file_path: str) -> list[list[int]]:
    with open(file_path, "r") as file:
        return [line for line in file.read()]


def find_amount_of_safe_reports(reports: list[list[int]]) -> int:
    return 0
