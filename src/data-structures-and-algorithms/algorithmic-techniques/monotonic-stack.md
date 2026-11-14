# Monotonic Stack

## Use Case

- Looking for the next/previous greater/smaller element
- Spans, ranges, or boundaries

## Template

```python
def monotonic_decreasing_stack(array):
    mono_stack = []
    result = <initial value>

    for i in range(len(array)):
        while mono_stack and array[mono_stack[-1]] < array[i]:
            mono_stack.pop()
            # Utilize the popped index in result
            # e.g. result[mono_stack[-1]] = <something involving i>
            # e.g. result += mono_stack[-1]
            # (read mono_stack[-1] before pop() if you need the popped value)

        mono_stack.append(i)

    return result
```
