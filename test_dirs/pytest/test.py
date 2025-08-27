def recursive_sum(values: list[int], n: int):
    if n <= 0:
        return 0
    return recursive_sum(values, n - 1) + values[n - 1]

def sum_r(values: list[int]):
    return recursive_sum(values, len(values))

values = [1, 2, 3, 4, 5]
total = sum_r(values)
