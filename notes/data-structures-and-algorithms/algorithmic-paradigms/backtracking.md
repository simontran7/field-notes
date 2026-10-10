# Backtracking (Complete Search)

## Use Case

Generate **all** valid configurations (e.g., subsets, permutations, combinations,
partitions, etc.)

## Template

```python
def is_valid(candidate):
    # check if candidate is valid or not

def backtrack(path, <other state variables>):
    if <base case>:
        # update global result (i.e., append the path to the global result or increment the global result)
        return

    result = <initial value>
    for candidate in <input>:
        if not is_valid(candidate):
            continue
        # modify `path`
        result += backtrack(path, <other state variables>)
        # undo the modification to `path`

    return result
```

> [!NOTE]
> Backtracking can be visualized as a tree, where each node represents the current state of the path during a recursive function call. Every `backtrack()` call explores a different branch of the tree, building potential solutions along the way. The leaves of the tree correspond to base cases, often representing complete solutions (but not in every problem).

## Complexity Analysis

Let $S$ be number of possible states explored and $C$ the cost of processing a single state. Then:
- Time Complexity: worst-case $O(S \cdot C)$
- Space Complexity: worst-case $O(d)$ for the recursion stack, where $d$ is the maximum depth of the recursion (i.e., the length of a complete `path`), excluding the space to store `path` itself or the accumulated results

> [!NOTE]
> Common values of $S$:
> - Combinations of $k$ from $n$: $\binom{n}{k}$
> - Permutations of from $n$: $\frac{n!}{(n - k)!}$
> - Generic tree with branching factor $b$ and depth $d$: $b^d$
