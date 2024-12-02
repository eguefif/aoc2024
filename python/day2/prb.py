import itertools

# with open("../../inputs/d2") as f:
with open("../../inputs/d2") as f:
    lines = f.readlines()

ans = 0


def is_safe(nums):
    diffs = [b - a for a, b in itertools.zip_longest(nums, nums[1:]) if b is not None]
    return all([a >= 1 and a <= 3 for a in diffs]) or all(
        [a <= -1 and a >= -3 for a in diffs]
    )


def is_safe_p2(nums):
    if is_safe(nums):
        return True

    for n in range(0, len(nums)):
        tmp = nums.copy()
        tmp.pop(n)
        if is_safe(tmp):
            return True
    return False


ans2 = 0

for line in lines:
    nums = [int(entry) for entry in line.strip().split()]
    if is_safe(nums):
        ans += 1
    if is_safe_p2(nums):
        ans2 += 1


assert ans == 299


assert ans2 == 364
