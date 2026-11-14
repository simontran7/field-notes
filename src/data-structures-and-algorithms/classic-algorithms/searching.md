# Searching

## Problem

Given a collection of candidates and a criterion, find an element that satisfies the criterion (or determine that none does).

## Linear Search

### Use Case

For simple searches.

### Template

```python
def linear_search(array, target):
    for i in range(len(array)):
        if array[i] == target:
            return i

    return -1
```

![](images/searching-1.svg)

#### Complexity Analysis

Let $n$ be the length of the array. Then:
- Time Complexity: worst-case $O(n)$
- Space Complexity: worst-case $O(1)$

## Binary Search

### Exact Match On an Array

#### Use Case

For some input array, `array`, and a desired element `target`, `array` must be sorted, and you want to find the index of `target` if it is in `array`, or otherwise return `-1`.

#### Template

```python
def binary_search(array, target):
    low = 0
    high = len(array) - 1

    while low <= high:
        mid = low + (high - low) // 2

        if <found condition>:
            return mid

        if <go right condition>:
            low = mid + 1
        else:
            high = mid - 1

    return -1
```

![](images/searching-2.svg)

#### Complexity Analysis

Let $n$ be the length of the array. Then:
- Time Complexity: worst-case $O(\log n)$
- Space Complexity: worst-case $O(1)$

### Partition Point On an Array

#### Use Case

You want the first index where a monotonic condition _flips_ in a sorted array

#### Template

```python
def lower_bound(array, target):
    """Returns the index of the leftmost value >= `target`"""
    low = 0
    high = len(array)

    while low < high:
        mid = low + (high - low) // 2
        if array[mid] >= target:
            high = mid
        else:
            low = mid + 1

    return low
```

![](images/searching-3.svg)

```python
def upper_bound(array, target):
    """Returns the index of the leftmost value > `target`"""
    low = 0
    high = len(array)

    while low < high:
        mid = low + (high - low) // 2
        if array[mid] > target:
            high = mid
        else:
            low = mid + 1

    return low
```

![](images/searching-4.svg)

#### Complexity Analysis

Let $n$ be the length of the array. Then:
- Time Complexity: worst-case $O(\log n)$
- Space Complexity: worst-case $O(1)$

> [!TIP]
> To locate a neighbouring _value_ to a target value, you may use `upper_bound()`'s or `lower_bound()`'s result as follows:

```python
def find_lt(array, target):
    """Returns the rightmost value < `target`"""
    i = lower_bound(array, target)
    if i > 0:
        return array[i - 1]
    raise ValueError("no value < target")


def find_le(array, target):
    """Returns the rightmost value <= `target`"""
    i = upper_bound(array, target)
    if i > 0:
        return array[i - 1]
    raise ValueError("no value <= target")


def find_gt(array, target):
    """Returns the leftmost value > `target`"""
    i = upper_bound(array, target)
    if i != len(array):
        return array[i]
    raise ValueError("no value > target")


def find_ge(array, target):
    """Returns the leftmost value >= `target`"""
    i = lower_bound(array, target)
    if i != len(array):
        return array[i]
    raise ValueError("no value >= target")
```

> [!WARNING]
> `binary_search()` doesn't work if `array` contains duplicates, but `lower_bound()` and `upper_bound()` allows duplicates in `array`.

> [!TIP]
> If the input array is sorted in descending order, simply invert the inequality in the if condition.

### Partition Point On a Solution Space

#### Use Case

You're trying to find a **maximum** or **minimum** value, and:

- You can verify (usually with a greedy algorithm) in $O(n)$ time (or faster) whether a given candidate `x` is a valid solution
- The predicate (i.e., the yes/no question to ask the current candidate at `mid`) is monotonic, and it flips exactly once.
    - Finding the minimum means finding the leftmost `True` in a sequence like `F F F T T T`
    - Finding the maximum means finding the rightmost `T` in a sequence like `T T T F F F`.

#### Template

```python
def binary_search_minimum(array):
    def is_valid(x):
        # Some O(n) algorithm (usually also a greedy algorithm)
        return <boolean>

    low = <minimum possible answer>
    high = <maximum possible answer>

    while low <= high:
        mid = low + (high - low) // 2
        if is_valid(mid):
            high = mid - 1
        else:
            low = mid + 1

    return low
```

```python
def binary_search_maximum(array):
    def is_valid(x):
        # Some O(n) algorithm (usually also a greedy algorithm)
        return <boolean>

    low = <minimum possible answer>
    high = <maximum possible answer>

    while low <= high:
        mid = low + (high - low) // 2
        if is_valid(mid):
            low = mid + 1
        else:
            high = mid - 1

    return high
```

#### Complexity Analysis

Let $k$ be the size of the solution space (`high - low`) and $f(n)$ be the time complexity of `is_valid()`. Then:
- Time Complexity: worst-case $O(f(n) \log k)$
- Space Complexity: worst-case $O(1)$, and whatever `is_valid()` itself uses
