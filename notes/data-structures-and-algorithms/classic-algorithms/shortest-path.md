# Shortest Path Algorithms

## Problem

Given a weighted graph and a source vertex, find the minimum total edge weight to reach a destination vertex (or every other vertex).

> [!TIP]
> You can use breadth-first-search instead of the following algorithms to find the shortest path on an unweighted graph:
> 1. Store the distance/path length as part of the queue's state:
>     - If the problem asks for distance (number of moves/steps), the `distance` queue's state variable should be initialized to 0.
>     - If the problem asks for path length (number of cells in the path), the `path_length` queue's state variable should be initialized to 1.
> 2. After the line that pops a vertex from the queue, add the following terminating condition:
> ```python
>  if vertex == destination: 
>      return <distance or path length>
> ```

## Dijkstra's Algorithm

### Use Case

Finding the single source shortest path on graphs with non-negative edge weights.

### Template

```python
import heapq
from collections import defaultdict

def dijkstras(edges, n, source):
    graph = defaultdict(list)
    for u, v, w in edges:
        graph[u].append((v, w))

    distances = [float("inf")] * n
    distances[source] = 0

    pq = [(0, source)]

    while pq:
        distance, node = heapq.heappop(pq)

        if distance > distances[node]:
            continue

        for neighbour, weight in graph[node]:
            new_distance = distance + weight
            if new_distance < distances[neighbour]:
                distances[neighbour] = new_distance
                heapq.heappush(pq, (new_distance, neighbour))

    return distances
```

### Complexity Analysis

Let $V$ be the number of vertices and $E$ be the number of edges. Then:
- Time Complexity: worst-case $O((V + E) \log V)$
- Space Complexity: worst-case $O(V + E)$


