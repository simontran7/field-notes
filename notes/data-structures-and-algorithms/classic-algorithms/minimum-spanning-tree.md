# Minimum Spanning Tree

## Problem

Given a connected, undirected, weighted graph, find a subset of edges that connects all vertices with no cycles and minimum total edge weight.

## Kruskal's Algorithm

### Template

```python
def kruskal(n, edges):
    ds = DisjointSet(n)
    mst_cost = 0
    edges_used = 0

    # edges as [weight, u, v] so sorting orders by weight
    edges.sort()

    for weight, u, v in edges:
        if ds.union(u, v):
            mst_cost += weight
            edges_used += 1
            if edges_used == n - 1:
                break

    return mst_cost
```

### Complexity Analysis

Let $V$ be number of vertices, and $E$ be the number of edges. Then:
- Time Complexity: worst-case $O(E \log E) + O(E \alpha(V)) = O(E \log E)$
- Space Complexity: worst-case $O(V)$

## Prim's Algorithm

### Template

```python
import heapq

def prim(n, graph):
    visited = [False] * n
    visited_count = 0

    pq = [(0, 0)]  # (weight, vertex)

    mst_cost = 0

    while pq and visited_count < n:
        weight, vertex = heapq.heappop(pq)
        if visited[vertex]:
            continue
        mst_cost += weight
        visited[vertex] = True
        visited_count += 1
        for neighbour_weight, neighbour in graph[vertex]:
            if not visited[neighbour]:
                heapq.heappush(pq, (neighbour_weight, neighbour))

    return mst_cost
```

### Complexity Analysis

Let $V$ be number of vertices, and $E$ be the number of edges. Then:
- Time Complexity: worst-case $O(V + E) \cdot O(\log V) = O(E \log V)$
- Space Complexity: worst-case $O(V + E)$