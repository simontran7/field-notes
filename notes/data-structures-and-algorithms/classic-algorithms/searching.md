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

```tikz
\begin{tikzpicture}[x=0.9cm, y=0.9cm, cell/.style={draw, minimum size=0.9cm},
    found/.style={hl, line width=1.6pt}]
  % \row{y}{value/style, ...}: one array, with each index below its cell
  \newcommand{\row}[2]{\foreach \v/\s [count=\i from 0] in {#2} {
    \node[cell, \s] at (\i, #1) {\v};
    \node[font=\scriptsize] at (\i, #1 - 0.75) {\i};
  }}
  \row{0}{1/,2/,3/,4/,5/,6/,7/,8/}
  \foreach \i in {0,...,5} \draw[->, thin] (\i, -1.6) to[bend left=60] (\i+1, -1.6);
  \row{-2.1}{1/,2/,3/,4/,5/,6/,7/found,8/}
\end{tikzpicture}
```

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

```tikz
\begin{tikzpicture}[x=0.9cm, y=0.9cm, cell/.style={draw, minimum size=0.9cm},
    found/.style={hl, line width=1.6pt}]
  % \row{y}{value/style, ...}: one array, with each index below its cell
  \newcommand{\row}[2]{\foreach \v/\s [count=\i from 0] in {#2} {
    \node[cell, \s] at (\i, #1) {\v};
    \node[font=\scriptsize] at (\i, #1 - 0.75) {\i};
  }}
  \node[right, font=\small] at (-0.45, -0.0 + 0.8) {$\mathit{low} = 0$, $\mathit{high} = 7$, $\mathit{mid} = 3$: $18 < 40$, search right half};
  \row{-0.0}{3/,7/,12/,18/hl,24/,31/,40/,52/}
  \node[right, font=\small] at (-0.45, -2.4 + 0.8) {$\mathit{low} = 4$, $\mathit{high} = 7$, $\mathit{mid} = 5$: $31 < 40$, search right half};
  \row{-2.4}{3/,7/,12/,18/,24/,31/hl,40/,52/}
  \node[right, font=\small] at (-0.45, -4.8 + 0.8) {$\mathit{low} = 6$, $\mathit{high} = 7$, $\mathit{mid} = 6$: $40$ is the target, found at index $6$};
  \row{-4.8}{3/,7/,12/,18/,24/,31/,40/found,52/}
\end{tikzpicture}
```

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

```tikz
\begin{tikzpicture}[x=0.9cm, y=0.9cm, cell/.style={draw, minimum size=0.9cm},
    found/.style={hl, line width=1.6pt}]
  % \row{y}{value/style, ...}: one array, with each index below its cell
  \newcommand{\row}[2]{\foreach \v/\s [count=\i from 0] in {#2} {
    \node[cell, \s] at (\i, #1) {\v};
    \node[font=\scriptsize] at (\i, #1 - 0.75) {\i};
  }}
  \node[right, font=\small] at (-0.45, -0.0 + 0.8) {$\mathit{low} = 0$, $\mathit{high} = 6$, $\mathit{mid} = 3$: $2$ is not less than the target, search left half};
  \row{-0.0}{1/,2/,2/,2/hl,4/,7/}
  \node[right, font=\small] at (-0.45, -2.4 + 0.8) {$\mathit{low} = 0$, $\mathit{high} = 3$, $\mathit{mid} = 1$: $2$ is not less than the target, search left half};
  \row{-2.4}{1/,2/hl,2/,2/,4/,7/}
  \node[right, font=\small] at (-0.45, -4.8 + 0.8) {$\mathit{low} = 0$, $\mathit{high} = 1$, $\mathit{mid} = 0$: $1$ is less than the target, search right half};
  \row{-4.8}{1/hl,2/,2/,2/,4/,7/}
  \node[right, font=\small] at (-0.45, -7.199999999999999 + 0.8) {$\mathit{low} = \mathit{high} = 1$: the first index with a value $\ge$ the target};
  \row{-7.199999999999999}{1/,2/found,2/,2/,4/,7/}
\end{tikzpicture}
```

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

```tikz
\begin{tikzpicture}[x=0.9cm, y=0.9cm, cell/.style={draw, minimum size=0.9cm},
    found/.style={hl, line width=1.6pt}]
  % \row{y}{value/style, ...}: one array, with each index below its cell
  \newcommand{\row}[2]{\foreach \v/\s [count=\i from 0] in {#2} {
    \node[cell, \s] at (\i, #1) {\v};
    \node[font=\scriptsize] at (\i, #1 - 0.75) {\i};
  }}
  \node[right, font=\small] at (-0.45, -0.0 + 0.8) {$\mathit{low} = 0$, $\mathit{high} = 6$, $\mathit{mid} = 3$: $2$ is at most the target, search right half};
  \row{-0.0}{1/,2/,2/,2/hl,4/,7/}
  \node[right, font=\small] at (-0.45, -2.4 + 0.8) {$\mathit{low} = 4$, $\mathit{high} = 6$, $\mathit{mid} = 5$: $7$ is greater than the target, search left half};
  \row{-2.4}{1/,2/,2/,2/,4/,7/hl}
  \node[right, font=\small] at (-0.45, -4.8 + 0.8) {$\mathit{low} = 4$, $\mathit{high} = 5$, $\mathit{mid} = 4$: $4$ is greater than the target, search left half};
  \row{-4.8}{1/,2/,2/,2/,4/hl,7/}
  \node[right, font=\small] at (-0.45, -7.199999999999999 + 0.8) {$\mathit{low} = \mathit{high} = 4$: the first index with a value $>$ the target};
  \row{-7.199999999999999}{1/,2/,2/,2/,4/found,7/}
\end{tikzpicture}
```

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
