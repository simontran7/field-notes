# Sliding Window

## Variable 

### Use Case

You are looking for the longest/smallest subarray/substring that satisfies a certain constraint.

### Templates

```python
def variable_sliding_window_max(array):
    left = 0
    current = <bookkeeping to track the window>
    result = 0

    for right in range(len(array)):
        # 1. unconditionally add the new element to the window 
        <update state for array[right]>

        # conditionally update `current` bookkeeping 

        # while the invariant is violated, restore the sliding window.
        while <window is broken condition>:
            <update state for array[left]>   
            left += 1

        # record the window length now that the window is valid 
        result = max(result, right - left + 1)

    return result
```

```python
def variable_sliding_window_min(array):
    left = 0
    current = <bookkeeping to track the window>
    result = float("inf")

    for right in range(len(array)):
        # unconditonally extend add the new element to the window.
        <update `current` for `array[right]`>

        # conditionally update `current` bookkeeping 

        # while the invariant holds, shrink the sliding window.
        while <window is valid condition>:
            result = min(result, right - left + 1)
            <update `current` for `array[left]`>
            left += 1

    return result if result != float("inf") else -1
```

## Fixed 

### Use Case

You are looking for a subarray/substring of some length `k` that satisfies a certain constraint.

### Template

```python
def fixed_sliding_window(array, k):
    current = <data to track the window>
    result = <initial value>

    # 1. seed the first window of size `k`.
    for i in range(k):
        <update `current` for array[i]>

    <update result from initial window>

    # 2. slide the window: add `right`, remove `left`, update `result`.
    for right in range(k, len(array)):
        <update `current` for `array[right]`>
        <undo `current` for `array[right - k]`>
        <update `result`>

    return result
```

> [!WARNING]
> The constraint must be preserved as the sliding window shrinks. If shrinking the window can break the constraint, consider using the _prefix state_ technique instead.

> [!NOTE]
> The formula for the length of any window is `right - left + 1` .
