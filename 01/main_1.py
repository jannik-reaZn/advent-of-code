def calulate_total_distance_between_lists(list1: list[int], list2: list[int]) -> int:
    """Calculate the total distance between two lists of integers.
    Within each pair, figure out how far apart the two numbers are
    """
    sorted_list1 = sorted(list1)
    sorted_list2 = sorted(list2)

    total_distance = 0

    for left, right in zip(sorted_list1, sorted_list2):
        distance_between_pair = abs(left - right)
        total_distance += distance_between_pair

    return total_distance
