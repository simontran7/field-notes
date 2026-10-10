# Monotonic Deque

## Use Case

Maximum or minimum values in a sliding window or some ranges.

## Template

```python
from collections import deque

def monotonic_non_increasing_deque(array, k):
    mono_deque = deque()
    result = []

    for i in range(len(array)):
        while mono_deque and array[mono_deque[-1]] < array[i]:
            mono_deque.pop()

        mono_deque.append(i)

        if mono_deque[0] <= i - k:
            mono_deque.popleft()

        if i >= k - 1:
            result.append(array[mono_deque[0]])

    return result
```
