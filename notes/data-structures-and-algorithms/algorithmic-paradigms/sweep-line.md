# Sweep Line

## Use Case

Problem involves intervals of events (i.e., each event happens over a continuous number line of time or position) represented as a 2D array where each element represents an event `[left, right, value]`.

## Examples

### [LeetCode 1094. Car Pooling](https://leetcode.com/problems/car-pooling/description/)

```python
class Solution:
    def carPooling(self, trips: List[List[int]], capacity: int) -> bool:
        last_end = max(to for _, _, to in trips)
        diff_array = [0] * (last_end + 1)
        for passengers, start, end in trips:
            diff_array[start] += passengers
            diff_array[end] -= passengers

        passenger_count = 0
        for change in diff_array:
            passenger_count += change
            if passenger_count > capacity:
                return False

        return True
```

### [LeetCode 2021. Brightest Position on Street](https://leetcode.com/problems/brightest-position-on-street/description/)

```python
class Solution:
    def brightestPosition(self, lights: List[List[int]]) -> int:
        diff_array = []
        for position, radius in lights:
            diff_array.append((position - radius, 1))
            diff_array.append((position + radius + 1, -1))

        diff_array.sort()
        result = 0
        curr_brightness = 0
        max_brightness = 0
        for position, change in diff_array:
            curr_brightness += change
            if curr_brightness > max_brightness:
                max_brightness = curr_brightness
                result = position

        return result
```
