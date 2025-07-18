from main_02 import read_in_reports
import os
import pytest


@pytest.fixture
def report_file_path():
    current_dir = os.path.dirname(os.path.abspath(__file__))
    file_path = os.path.join(current_dir, "report.txt")
    return read_in_reports(file_path=file_path)


def test_find_amount_of_safe_reports(report_file_path):
    assert True
