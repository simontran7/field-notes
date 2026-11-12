# Tree and Graph Traversals

## Problem

Given a graph or n-ary tree, systematically visit every relevant node, exactly once, while doing some operation at each node.

> [!TIP]
> For graph traversal problems, it's useful to rephrase the goal in terms of its inverse.
> - [Example #1](https://leetcode.com/problems/minimum-number-of-vertices-to-reach-all-nodes/): the problem description asks us to find the smallest set of vertices from which all nodes in the graph are reachable. Instead, we can rephrase the problem description in terms of its inverse: find the smallest set of nodes that _cannot_ be reached from other nodes, since if a node can be reached from another node, then we would rather just include the pointer rather than the pointee in our set.
> - [Example #2](https://leetcode.com/problems/01-matrix/description/): the brute force solution would be to perform BFS for each cell with a 1, but instead, we can perform a multi-source BFS by performing starting from all cells with a 0 (if we have a cell `x` with value 1 and its nearest cell y has value 0, then it doesn't make a difference if we traverse from `x -> y` or `y -> x`; both give the same distance).

## Depth-First Search

### On Trees

```tikz
\begin{tikzpicture}[tn/.style={draw, circle, minimum size=0.8cm, inner sep=0pt},
    cell/.style={draw, minimum size=0.75cm}]
  % The same binary tree is used for every traversal on this page.
  \foreach \n/\x/\y in {F/0/0, B/-2.4/-1.4, G/2.4/-1.4, A/-3.6/-2.8, D/-1.2/-2.8, I/3.6/-2.8, C/-1.9/-4.2, E/-0.5/-4.2, H/2.9/-4.2}
    \node[tn] (\n) at (\x, \y) {\n};
  \foreach \a/\b in {F/B, F/G, B/A, B/D, D/C, D/E, G/I, I/H} \draw[->] (\a) -- (\b);

  % \order{y}{name}{nodes}: the order in which a traversal visits the nodes
  \newcommand{\order}[3]{
    \node[anchor=east] at (-3.4, #1) {#2};
    \foreach \v [count=\i from 0] in {#3} \node[cell] at (-3 + 0.75*\i, #1) {\v};
  }
  \order{-5.6}{Preorder}{F, B, A, D, C, E, G, I, H}
  \order{-6.6}{Inorder}{A, B, C, D, E, F, G, H, I}
  \order{-7.6}{Postorder}{A, C, E, D, B, H, I, G, F}
\end{tikzpicture}
```

#### Use Case

- Pre-order traversal (visit the root, then traverse the left subtree, then traverse the right subtree)
- In-order traversal (traverse the left subtree, then visit the root, then traverse the right subtree)
- Post-order traversal (traverse the left subtree, then traverse the right subtree, then visit the root).

#### Template

```python
def recursive_preorder_dfs(root):
    if not root:
        return

    # process `root`

    recursive_preorder_dfs(root.left)

    recursive_preorder_dfs(root.right)
```

```python
def iterative_preorder_dfs(root):
    if not root:
        return

    stack = [root]

    while stack:
        node = stack.pop()

		# process `current` node

	    if node.right:
            stack.append(node.right)
        if node.left:
            stack.append(node.left)
```

```python
def recursive_inorder_dfs(root):
    if not root:
        return

    recursive_inorder_dfs(root.left)

    # process `root`

    recursive_inorder_dfs(root.right)
```

```python
def iterative_inorder_dfs(root):
    stack = []
    current = root

    while stack or current:
        if current:
            stack.append(current)
            current = current.left
        else:
            current = stack.pop()

            # process `current`

            current = current.right
```

```python
def recursive_postorder_dfs(root):
    if not root:
        return

    recursive_postorder_dfs(root.left)

    recursive_postorder_dfs(root.right)

    # process `root`
```

```python
def iterative_postorder_dfs(root):
    stack = []
    current = root
    last_visited = None
    result = []

    while stack or current:
        if current:
            stack.append(current)
            current = current.left
        else:
            if stack[-1].right and last_visited != stack[-1].right:
                current = stack[-1].right
            else:
                # process `stack[-1]`

                last_visited = stack.pop()

    return result
```


#### Complexity Analysis

Let $n$ be the number of nodes in the tree, $C_{node}$ be the cost of processing a single node, and $h$ the height of the tree. Then:
- Time Complexity: worst-case $O(n \cdot C_{node})$
- Space Complexity: worst-case $O(h)$

### On Graphs

#### Use Case

Most graph problems.

#### Templates

```python
def adjacency_list_recursive_dfs(vertex):
    visited.add(vertex)
    for neighbour in graph[vertex]:
        if neighbour not in visited:
            adjacency_list_recursive_dfs(neighbour)
```

```python
def adjacency_list_iterative_dfs(vertex):
    stack = [vertex]
    visited.add(vertex)

    while stack:
        vertex = stack.pop()

        for neighbour in graph[vertex]:
            if neighbour not in visited:
                visited.add(neighbour)
                stack.append(neighbour)
```

```python
def grid_recursive_dfs(row, col):
	visited.add((row, col))

	for dr, dc in <directions>: # where (change in row, change in col)
		neighbour_row = row + dr
		neighbour_col = col + dc
		if (0 <= neighbour_row < ROW_COUNT and 0 <= neighbour_col < COLUMN_COUNT) and (neighbour_row, neighbour_col) not in visited:
			dfs(neighbour_row, neighbour_col)
```

```python
def grid_iterative_dfs(row, col):
	stack = [(row, col)]

	while stack:
		row, col = stack.pop()

		for dr, dc in <directions>: # where (change in row, change in col)
			neighbour_row = row + dr
			neighbour_col = col + dc
			if 0 <= neighbour_row < ROW_COUNT and 0 <= neighbour_col < COLUMN_COUNT and (neighbour_row, neighbour_col) not in visited:
				visited.add((row, col))
				stack.append((neighbour_row, neighbour_col))		
```

#### Complexity Analysis

Let $S$ be the number of reachable states (product of the ranges of each state variable), $C_{state}$ the cost of processing a single state, $T$ the number of transitions (the transitions per state times $S$), $C_{transition}$ the cost of processing a single transition, $V$ be the number of vertices and $E$ be the number of edges. Then:

- Time Complexity: worst-case $O(S \cdot C_{state} + T \cdot C_{transition})$
- Space Complexity:
	- Adjacency List: worst-case $O(V + E)$
	- Adjacency Matrix: worst-case $O(R \cdot C)$

## Breadth-First Search

### On Trees

```tikz
\begin{tikzpicture}[tn/.style={draw, circle, minimum size=0.8cm, inner sep=0pt},
    cell/.style={draw, minimum size=0.75cm}]
  % The same binary tree is used for every traversal on this page.
  \foreach \n/\x/\y in {F/0/0, B/-2.4/-1.4, G/2.4/-1.4, A/-3.6/-2.8, D/-1.2/-2.8, I/3.6/-2.8, C/-1.9/-4.2, E/-0.5/-4.2, H/2.9/-4.2}
    \node[tn] (\n) at (\x, \y) {\n};
  \foreach \a/\b in {F/B, F/G, B/A, B/D, D/C, D/E, G/I, I/H} \draw[->] (\a) -- (\b);

  % Each level is one batch of the queue, visited left to right.
  \foreach \lvl/\y/\nodes in {0/0/{F}, 1/-1.4/{B, G}, 2/-2.8/{A, D, I}, 3/-4.2/{C, E, H}} {
    \draw[thin, dashed] (-4.4, \y - 0.6) -- (4.4, \y - 0.6);
    \node[anchor=west, font=\small] at (4.6, \y) {Level \lvl: \nodes};
  }
  \node[anchor=west] at (-4.4, -5.4) {Level-order traversal: \texttt{[[F], [B, G], [A, D, I], [C, E, H]]}};
\end{tikzpicture}
```

#### Use Case

Level-order traversal.

#### Template

```python
from collections import deque


def bfs(root):
    if not root:
        return

    queue = deque([root])

    while queue:
        for _ in range(len(queue)):
            node = queue.popleft()

            # process `node`

            if node.left:
                queue.append(node.left)
            if node.right:
                queue.append(node.right)
```

#### Complexity Analysis 

Let $n$ be the number of nodes in the tree, $C_{node}$ be the cost of processing a single node. Then:
- Time: worst-case $O(n \cdot C_{node})$
- Space: worst-case $O(n)$

### On Graphs

#### Use Case

Determine the shortest path in an _unweighted_ graph.

#### Template

```python
from collections import deque


def adjacency_list_bfs(source):
    queue = deque([source])
    visited = set([source])

    while queue:
        vertex = queue.popleft()

        for neighbour in graph[vertex]:
            if neighbour not in visited:
                visited.add(neighbour)
                queue.append(neighbour)

            # process `neighbour`'s state
```

```python
from collections import deque

def grid_bfs(source_row, source_col):
	queue = deque([(source_row, source_col)])
	visited.set([(row, col)])

	while queue:
		row, col = queue.popleft()

		for dr, dc in <directions>: # where (change in row, change in col)
			neighbour_row = row + dr
			neighbour_col = col + dc
			if (0 <= neighbour_row < ROW_COUNT and 0 <= neighbour_col < COLUMN_COUNT) and (neighbour_row, neighbour_col) not in visited:
				visited.add((neighbour_row, neighbour_col))
				queue.append((neighbour_row, neighbour_col))

			# process `neighbour`'s state
```

#### Complexity Analysis

Let $S$ be the number of reachable states (product of the ranges of each state variable), $C_{state}$ the cost of processing a single state, $T$ the number of transitions (the transitions per state times $S$), $C_{transition}$ the cost of processing a single transition, $V$ be the number of vertices and $E$ be the number of edges. Then:

- Time Complexity: worst-case $O(S \cdot C_{state} + T \cdot C_{transition})$
- Space Complexity:
	- Adjacency List: worst-case $O(V + E)$
	- Adjacency Matrix: worst-case $O(R \cdot C)$

> [!TIP]
> In grid problems, the direction vectors for horizontal and vertical moves are `[(-1, 0), (1, 0), (0, -1), (0, 1)]`. If horizontal, vertical, _and_ diagonal moves are allowed, the complete direction vectors are `[(-1, 0), (1, 0), (0, -1), (0, 1), (-1, -1), (-1, 1), (1, 1), (1, -1)]`.

```tikz
\begin{tikzpicture}[x=2.2cm, y=1.3cm, cell/.style={draw, minimum width=2.2cm, minimum height=1.3cm}]
  % Row r grows downward and column c grows rightward, as in the direction vectors.
  \foreach \r in {-1, 0, 1} \foreach \c in {-1, 0, 1} \node[cell] at (\c, -\r) {};
  \node[cell, hl] at (0, 0) {};
  \node (center) at (0, 0) {$(r, c)$};
  \foreach \dr/\dc/\label in {-1/0/{$(r-1, c)$}, 1/0/{$(r+1, c)$}, 0/-1/{$(r, c-1)$}, 0/1/{$(r, c+1)$}} {
    \node (n) at (\dc, -\dr) {\label};
    \draw[->, thin, shorten <=2pt, shorten >=2pt] (center) -- (n);
  }
\end{tikzpicture}
```

```tikz
\begin{tikzpicture}[x=2.2cm, y=1.3cm, cell/.style={draw, minimum width=2.2cm, minimum height=1.3cm}]
  % Row r grows downward and column c grows rightward, as in the direction vectors.
  \foreach \r in {-1, 0, 1} \foreach \c in {-1, 0, 1} \node[cell] at (\c, -\r) {};
  \node[cell, hl] at (0, 0) {};
  \node (center) at (0, 0) {$(r, c)$};
  \foreach \dr/\dc/\label in {-1/0/{$(r-1, c)$}, 1/0/{$(r+1, c)$}, 0/-1/{$(r, c-1)$}, 0/1/{$(r, c+1)$}, -1/-1/{$(r-1, c-1)$}, -1/1/{$(r-1, c+1)$}, 1/1/{$(r+1, c+1)$}, 1/-1/{$(r+1, c-1)$}} {
    \node (n) at (\dc, -\dr) {\label};
    \draw[->, thin, shorten <=2pt, shorten >=2pt] (center) -- (n);
  }
\end{tikzpicture}
```

> [!TIP]
> BST problems typically use DFS traversal. Common techniques include:
> - Checking if the current node's value is within bounds (e.g., `low <= node.val <= high`)
> - Leveraging the BST property to prune subtrees: if `node.val < low` , skip the left subtree; if `node.val > high` , skip the right subtree (where `low` or `high` can also just be a target value)
> - Using inorder traversal to collect values in sorted order for problems requiring sorted data without explicit sorting.

> [!TIP]
> When you need to traverse every connected component of a graph, use an outer loop to find an unvisited vertex and start a new DFS/BFS from it.
> ```python
> def main(adjacency_list):
>     result = <initial value>
>     visited = set()
>     for vertex in graph:
>         if vertex not in visited:
>             <dfs(vertex) or bfs(vertex)> 
>             # update `result` 
>     return result
> ```
> or for grid problems
> ```python
> def main(grid):
>     visited = set()
>     result = <initial value>
>     for row in range(len(grid)):
>         for col in range(len(grid[0])):
>             if (row, col) not in visited:
>                 <dfs(row, col) or bfs(row, col)>
>                 # update `result`
>     return result
> ```

> [!TIP]
> When using BFS to find shortest paths, store the distance/path length as part of the queue's state.
> - If the problem asks for distance (number of moves/steps), initialize source vertices with `distance = 0`
> - If the problem asks for path length (number of cells in the path), initialize source vertices with `path_length = 1`
> Then, after the line that pops a vertex from the queue, add an if expression that early returns the distance/path length.

> [!TIP]
> For a multi-source BFS, create a for loop that visits all source nodes and appends them to the queue for the BFS.

