# Stack

## Abstract Data Type

| Operation | Description                               |
| :---: | :---: |
| `top()`      | Returns the element at the top of the stack |
| `push(e)`  | Inserts element `e` at the top of the stack  |
| `pop()`   | Removes the element at the stack of the stack |

## Space Optimization

> [!TIP]
> If every element pushed onto the stack is the same, only the stack's size matters. Replace the stack with a counter: `push` increments it, `pop` decrements it, and an empty stack is a count of $0$. Space drops from $O(n)$ to $O(1)$.

For example, to find the length of the longest subsequence of matching brackets in a string of `(` and `)`, a stack would only ever hold `(`, so a counter of unmatched `(` replaces it:

```python
def longest_matching_subsequence(s: str) -> int:
    unmatched = 0  # stands in for a stack of "("
    pairs = 0
    for c in s:
        if c == "(":
            unmatched += 1  # push
        elif unmatched > 0:  # ")" with a "(" to match
            unmatched -= 1  # pop
            pairs += 1
    return 2 * pairs
```
