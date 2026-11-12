# List

## Abstract Data Type

| Operation | Description |
| :---: | :---: |
| `get_first()` | Returns the first element in the list. |
| `get_last()` | Returns the last element in the list. |
| `get(index)` | Returns the element at the specified index. |
| `add_first(element)` | Inserts the element at the beginning of the list. |
| `add_last(element)` | Appends the element to the end of the list. |
| `add(index, element)` | Inserts the element at the specified index, shifting subsequent elements. |
| `set(index, element)` | Replaces the element at the specified index with the given element. |
| `remove_first()` | Removes and returns the first element in the list. |
| `remove_last()` | Removes and returns the last element in the list. |
| `remove(index)` | Removes and returns the element at the specified index, shifting subsequent elements. |

## Use Case

You need an ordered sequence of elements where each element has an index.

## Dynamic Array


### Lookup

Index into the `array` at the desired index $i$ if it is within bounds $[0, N)$.

### Insertion

1. Reject the insertion if the desired index $i$ is less than $0$.
2. Resize the current array if the number of elements in the current array is equal to the capacity $N$ of the array by allocating a new array of capacity $N * 1.5$ (i.e., **geometric resizing**), then copying all elements from the current array to the new array.
3. If inserting at index $N - 1$, skip to step 4. Otherwise, shift all elements from $[i + 1, N)$ to the right to make way for the new element.
4. Write the new element at index $i$.

```tikz
\begin{tikzpicture}[x=0.9cm, y=0.9cm, cell/.style={draw, minimum size=0.9cm}, empty/.style={dashed}]
  % \row{y}{value/style, ...}: one array, with each index below its cell
  \newcommand{\row}[2]{\foreach \v/\s [count=\i from 0] in {#2} {
    \node[cell, \s] at (\i, #1) {\v};
    \node[font=\scriptsize] at (\i, #1 - 0.75) {\i};
  }}
  \node[draw] (new) at (0, 1.4) {8};
  \draw[->] (new) -- (0, 0.5);
  \row{0}{1/,2/,3/,4/,5/,6/,7/,/empty}
  \foreach \i in {0,1,2,3,4,5,6} \draw[->, thin] (\i, -2.1+0.5) to[bend left=60] (\i+1, -2.1+0.5);
  \row{-2.1}{1/,2/,3/,4/,5/,6/,7/,/empty}
  \row{-4.2}{8/hl,1/,2/,3/,4/,5/,6/,7/}
\end{tikzpicture}
```

```tikz
\begin{tikzpicture}[x=0.9cm, y=0.9cm, cell/.style={draw, minimum size=0.9cm}, empty/.style={dashed}]
  % \row{y}{value/style, ...}: one array, with each index below its cell
  \newcommand{\row}[2]{\foreach \v/\s [count=\i from 0] in {#2} {
    \node[cell, \s] at (\i, #1) {\v};
    \node[font=\scriptsize] at (\i, #1 - 0.75) {\i};
  }}
  \node[draw] (new) at (4, 1.4) {8};
  \draw[->] (new) -- (4, 0.5);
  \row{0}{1/,2/,3/,4/,5/,6/,7/,/empty}
  \foreach \i in {4,5,6} \draw[->, thin] (\i, -2.1+0.5) to[bend left=60] (\i+1, -2.1+0.5);
  \row{-2.1}{1/,2/,3/,4/,5/,6/,7/,/empty}
  \row{-4.2}{1/,2/,3/,4/,8/hl,5/,6/,7/}
\end{tikzpicture}
```

```tikz
\begin{tikzpicture}[x=0.9cm, y=0.9cm, cell/.style={draw, minimum size=0.9cm}, empty/.style={dashed}]
  % \row{y}{value/style, ...}: one array, with each index below its cell
  \newcommand{\row}[2]{\foreach \v/\s [count=\i from 0] in {#2} {
    \node[cell, \s] at (\i, #1) {\v};
    \node[font=\scriptsize] at (\i, #1 - 0.75) {\i};
  }}
  \node[draw] (new) at (7, 1.4) {8};
  \draw[->] (new) -- (7, 0.5);
  \row{0}{1/,2/,3/,4/,5/,6/,7/,/empty}
  \row{-2.1}{1/,2/,3/,4/,5/,6/,7/,8/hl}
\end{tikzpicture}
```

### Deletion

1. Starting at index $i$, up till and including $N - 2$, replace its element with the element to its left.
2. (Optional) Zero out the last element at index $N - 1$.

```tikz
\begin{tikzpicture}[x=0.9cm, y=0.9cm, cell/.style={draw, minimum size=0.9cm}, empty/.style={dashed}]
  % \row{y}{value/style, ...}: one array, with each index below its cell
  \newcommand{\row}[2]{\foreach \v/\s [count=\i from 0] in {#2} {
    \node[cell, \s] at (\i, #1) {\v};
    \node[font=\scriptsize] at (\i, #1 - 0.75) {\i};
  }}
  \row{0}{1/hl,2/,3/,4/,5/,6/,7/,8/}
  \foreach \i in {0,1,2,3,4,5,6} \draw[->, thin] (\i+1, -2.1+0.5) to[bend right=60] (\i, -2.1+0.5);
  \row{-2.1}{/empty,2/,3/,4/,5/,6/,7/,8/}
  \row{-4.2}{2/,3/,4/,5/,6/,7/,8/,/empty}
\end{tikzpicture}
```

```tikz
\begin{tikzpicture}[x=0.9cm, y=0.9cm, cell/.style={draw, minimum size=0.9cm}, empty/.style={dashed}]
  % \row{y}{value/style, ...}: one array, with each index below its cell
  \newcommand{\row}[2]{\foreach \v/\s [count=\i from 0] in {#2} {
    \node[cell, \s] at (\i, #1) {\v};
    \node[font=\scriptsize] at (\i, #1 - 0.75) {\i};
  }}
  \row{0}{1/,2/,3/,4/,5/hl,6/,7/,8/}
  \foreach \i in {4,5,6} \draw[->, thin] (\i+1, -2.1+0.5) to[bend right=60] (\i, -2.1+0.5);
  \row{-2.1}{1/,2/,3/,4/,/empty,6/,7/,8/}
  \row{-4.2}{1/,2/,3/,4/,6/,7/,8/,/empty}
\end{tikzpicture}
```

```tikz
\begin{tikzpicture}[x=0.9cm, y=0.9cm, cell/.style={draw, minimum size=0.9cm}, empty/.style={dashed}]
  % \row{y}{value/style, ...}: one array, with each index below its cell
  \newcommand{\row}[2]{\foreach \v/\s [count=\i from 0] in {#2} {
    \node[cell, \s] at (\i, #1) {\v};
    \node[font=\scriptsize] at (\i, #1 - 0.75) {\i};
  }}
  \row{0}{1/,2/,3/,4/,5/,6/,7/,8/hl}
  \row{-2.1}{1/,2/,3/,4/,5/,6/,7/,/empty}
\end{tikzpicture}
```

### Implementation

```
pub record ArrayList[E] {
    data: RawMutPointer[E],
    capacity: UInt,
    count: UInt,
}

impl ArrayList {
    pub func get(this, i: UInt) -> Result[E, ListError] {
        if i >= this.count {
            return Error(ListError::IndexOutOfBounds);
        } 
        Ok(this.data[i])
    }

    pub func add(this, i: UInt, e: E) -> Result[(), ListError] {
        if i > this.count {
            return Error(ListError::IndexOutOfBounds);
        }
        if this.count == this.data.capacity() {
            let new_capacity = if this.count == 0 { 1 } else { (this.count.to_float() * 1.5).ceil() as UInt };
            try this.resize(new_capacity);
        }
        for j in (i..this.count).rev() {
            this.data[j + 1] = this.data[j];
        }
        this.data[i] = e; 
        this.count += 1;
        Ok(())
    }

    pub func remove(this, i: UInt) -> Result[E, ListError] {
        if i >= this.count { 
            return Error(ListError::IndexOutOfBounds); // also prevents `this.count - 1` underflow when `this.count` is 0
        }
        let old_element = this.data[i];
        for j in i..this.count - 1 { 
            this.data[j] = this.data[j + 1];
        }
        this.data[this.count - 1] = 0; 
        this.count -= 1;
        Ok(old_element)
    }
    
    func resize(this, new_capacity: UInt) -> Result[(), ListError] {
        let new_array = Array::with_capacity(new_capacity);
        for i in 0..this.count {
            new_array[i] = this.data[i];
        }
        this.data = new_array;
        Ok(())
    }
}
```

### Complexity Analysis

| Operation | Time Complexity |
| :---: | :---: |
| Add at the last position | worst-case $O(n)$, but amortized $O(1)$  |
| Add at the first position | worst-case $O(n)$ |
| Add in the middle position | worst-case $O(n)$ |
| Lookup by index | worst-case $O(1)$ |
| Remove at the last position | worst-case $O(1)$ |
| Remove at the first position | worst-case $O(n)$ |
| Remove in the middle position | worst-case $O(n)$ |

> [!TIP]
> For a 2D list stored in a single flat list (row-major order), to convert between rows & columns and an index:
> - from rows & columns to index: `row * COLUMN_COUNT + column`
> - from index to rows & columns: `row, column = divmod(index, COLUMN_COUNT)`

## Singly Linked List

### Lookup

TO DO

### Insertion

TO DO

### Deletion

TO DO

### Complexity Analysis

| Operation | Time Complexity |
| :---: | :---: |
| Add at the head | worst-case $O(1)$ |
| Add at the tail | worst-case $O(n)$, or $O(1)$ with a maintained tail pointer |
| Add in the middle | worst-case $O(n)$ |
| Lookup by index | worst-case $O(n)$ |
| Remove at the head | worst-case $O(1)$ |
| Remove at the tail | worst-case $O(n)$ |
| Remove in the middle | worst-case $O(n)$ |

## Doubly Linked List

### Lookup

TO DO

### Insertion

TO DO

### Deletion

TO DO

### Complexity Analysis

| Operation | Time Complexity |
| :---: | :---: |
| Add at the head | worst-case $O(1)$ |
| Add at the tail | worst-case $O(1)$ (with a maintained tail pointer) |
| Add in the middle | worst-case $O(n)$ to find the position, $O(1)$ to link |
| Lookup | worst-case $O(n)$ |
| Remove at the head | worst-case $O(1)$ |
| Remove at the tail | worst-case $O(1)$ (with a maintained tail pointer) |
| Remove in the middle | worst-case $O(n)$ to find the node, $O(1)$ to unlink |

> [!TIP]
> In order to reduce edge cases, and when your algorithm might modify the head, create a **sentinel head** as well as a **sentinel tail** for a doubly linked list. These are extra nodes storing no real data which is placed immediately before the head or immediately after the tail. We create this so that insertions and removals at a boundary no longer need to be different than the middle of the linked list logic since modifications at the head or tail typically require a predecessor/successor, yet the head/tail has none.
