# Topological Ordering

## Problem

Given a directed acyclic graph, produce a linear ordering of its vertices such that for every edge $(u, v)$, $u$ appears before $v$.

## Kahn's Algorithm

### Template

```python
from collections import deque, defaultdict

def kahns_algorithm(n, edges):
    indegrees = [0] * n
    graph = defaultdict(list)
    for u, v in edges:
        graph[u].append(v)
        indegrees[v] += 1

    queue = deque()
    for vertex in range(n):
        if indegrees[vertex] == 0:
            queue.append(vertex)

    result = []

    while queue:
        vertex = queue.popleft()
        result.append(vertex)
        for neighbour in graph[vertex]:
            indegrees[neighbour] -= 1
            if indegrees[neighbour] == 0:
                queue.append(neighbour)

    # if len(result) != n, there's a cycle
    return result if len(result) == n else []
```

### Complexity Analysis

Let $V$ be the number of vertices and $E$ be the number of edges. Then:
- Time: worst-case $O(V + E)$
- Auxiliary Space: worst-case $O(V + E)$
