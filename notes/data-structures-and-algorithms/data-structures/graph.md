# Graph

## Properties

- A simple, undirected graph has $\binom{V}{2} = \frac{V(V-1)}{2} \in O(V^2)$ edges
- A tree has $V-1$ edges

## Abstract Data Type

|Operations|Description|
| :---: | :---: |
|`vertex_count()`|Returns the number of vertices in the graph.|
|`edge_count()`|Returns the number of edges in the graph.|
|`get_edge(u, v)`|Returns the edge from `u` to `v`, if one exists.|
|`endpoints(e)`|Returns the two endpoints of edge `e`.|
|`opposite_vertex(v, e)`|Returns the vertex opposite `v` on edge `e`.|
|`out_degree(v)`|Returns the number of outgoing edges from `v`.|
|`in_degree(v)`|Returns the number of incoming edges to `v`.|
|`add_vertex(x)`|Inserts a vertex storing element `x`.|
|`add_edge(u, v, x)`|Inserts an edge `(u, v)` storing element `x`.|
|`remove_vertex(v)`|Removes vertex `v` and all incident edges.|
|`remove_edge(e)`|Removes edge `e`.|
|`incoming_edges(v)`|Returns all incoming edges to `v`.|
|`outgoing_edges(v)`|Returns all outgoing edges from `v`.|
|`vertices()`|Returns all vertices in the graph.|
|`edges()`|Returns all edges in the graph.|

> [!TIP]
> - Adjacency lists are better suited if:
>     - You frequently need to add/remove vertices
>     - The graph has few edges
>     - Need to traverse the graph
> - Adjacency matrices are better suited if:
>     - you frequently need to add or remove edges, but _not_ vertices
>     - Check for the presence or absence of an edge between two vertices
>     - The matrix is small enough to fit in memory

## Adjacency List

```python
from collections import defaultdict

def from(edges):
    graph = defaultdict(list)
    for u, v in edges:
        graph[u].append(v)
        graph[v].append(u) # comment out this line if the input is a directed graph

    return graph
```

```python
from collections import defaultdict

def from(adjacency_matrix):
    graph = defaultdict(list)
    n = len(adjacency_matrix)

    for i in range(n):
        for j in range(i + 1, n):
            if adjacency_matrix[i][j]:
                graph[i].append(j)
                graph[j].append(i) # comment out this line if the input is a directed graph

    return graph
```

## Adjacency Matrix
