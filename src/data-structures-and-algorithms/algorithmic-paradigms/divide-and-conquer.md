# Divide and Conquer

## Use Case

The problem has a natural partition where it can be split into independent subproblems of the same type, solved separately, then combined.

## Template

```python
def divide_and_conquer(s):
    # Divide
    subproblems = divide(s)

    # Conquer
    subresults = [divide_and_conquer(subproblem) for subproblem in subproblems]

    # Combine
    return combine(subresults)
```

## Complexity Analysis

For divide and conquer algorithms of the recurrence $T(n) = aT(n/b) + f(n)$, compare $f(n)$ to $n^{\log_b a}$ to determine the worst-case time complexity:

1. **$f(n)$ is smaller**: $T(n) = \Theta(n^{\log_b a})$ (subproblems dominate)
2. **$f(n)$ is equal**: $T(n) = \Theta(n^{\log_b a} \log n)$ (balanced)
3. **$f(n)$ is larger and $af(n/b) \le cf(n)$ for some constant $c < 1$ and sufficiently large $n$**: $T(n) = \Theta(f(n))$ (work dominates)