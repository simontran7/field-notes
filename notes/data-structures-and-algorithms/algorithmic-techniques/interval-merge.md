# Interval Merge

## Use Case

Interval problems

## Template

```python
def merge(intervals):
    if not intervals:
        return []

    intervals.sort()

    result = [intervals[0]]

    for interval in intervals:
        start, end = interval
        prev_end = result[-1][1]
        if start <= prev_end:
            result[-1][1] = max(prev_end, end)
        else:
            result.append(interval)

    return result
```

