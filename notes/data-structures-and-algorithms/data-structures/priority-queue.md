# Priority Queue

## Abstract Data Type

| Operation    | Description                                                    |
| :---: | :---: |
| `top()`          | Returns the element at the top of the priority queue               |  
| `add(e)`         | Inserts element `e` in the correct priority in the priority queue  |
| `remove_top()`   | Removes element at in the priority queue                           |

## Use Case

- Repeatedly find the maximum or minimum element
- Get the "top" $k$ elements

```python
import heapq

def top_k(array, k):
    # use a min pq to keep the largest `k` elements,
    # or a max pq to keep the smallest `k`
    pq = []

    for num in array:
        # some logic to add an element according to problem's criteria
        heapq.heappush(pq, (<criteria as key>, num))
        if len(pq) > k:
            heapq.heappop(pq)

    result = []
    while pq:
        result.append(heapq.heappop(pq)[1])

    return result
```

- Find a running/streaming median

```python
import heapq


class MedianFinder:
    def __init__(self):
        self.min_pq = []
        self.max_pq = []

    def add_num(self, num):
        heapq.heappush_max(self.max_pq, num)
        heapq.heappush(self.min_pq, heapq.heappop_max(self.max_pq))
        if len(self.min_pq) > len(self.max_pq):
            heapq.heappush_max(self.max_pq, heapq.heappop(self.min_pq))

    def find_median(self):
        if len(self.max_pq) == len(self.min_pq):
            return (self.max_pq[0] + self.min_pq[0]) / 2.0
        return self.max_pq[0]
```

## Binary Heap

> [!DEFINITION]
> A **binary heap** is a complete, binary tree satisfying the **heap property**:
> - Largest element is stored at the root (max-heap) or the smallest element is stored at the root (min-heap)
> - Excluding the root, every parent node is $\ge$ its children (max-heap) or $\le$ its children (min-heap).

Since the tree is always complete, it can be stored implicitly in an array such that for a node at index $i$:
- left child: $2i + 1$ 
- right child: $2i + 2$
- parent: $\lfloor (i - 1) / 2 \rfloor$

### Sift Down

1. Compare the node against its children
2. If it violates the heap property, swap it with the larger child (max-heap) or smaller child (min-heap)
3. Repeat from the new position once the heap property is respected, or it reaches a leaf node.

### Sift Up

1. Compare the node against its parent
2. If it violates the heap property, swap the positions with its parent node
3. Repeat once the heap property is respected, or it reaches the root.

### Heapify

1. Find the index of the last non-leaf node: $\lfloor n / 2 \rfloor - 1$ (the parent of the last element).
2. Iterate from that index down to the root (index $0$) (i.e., bottom-up, right-to-left).
3. At each index, perform the sift down operation.

### Lookup min/max

Index into the $0^{th}$ element of the backing array.

### Insertion

1. Append the new node at the end of the array (the next open leaf)
2. Starting at the rightmost node in the last level, perform the sift up operation.

### Deletion

1. Swap the root with the last element in the array
2. Remove the last element
3. Starting at the root node, perform the sift down operation.

### Complexity Analysis

| Operation | Time Complexity |
| :---: | :---: |
| Heapify | worst-case $O(n)$ |
| Lookup (top) | worst-case $O(1)$ |
| Insertion | worst-case $O(\log n)$ |
| Deletion | worst-case $O(\log n)$ |
