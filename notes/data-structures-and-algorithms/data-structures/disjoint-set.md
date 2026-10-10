# Disjoint Set (Union-Find)

## Abstract Data Type

| Operation | Description |
| :---: | :---: |
| `new(n)` | Initializes a disjoint set with `n` elements, each in its own set |
| `union(x, y)` | Merges the sets containing `x` and `y`, and returns whether a merge occurred |
| `find(x)` | Returns the representative (root) of the set containing `x` |

## Use Case

- Detect and track connected components 
- Detect cycles in undirected graphs
- Enforce grouping constraints 

## Forest (with Path Compression & Union by Rank)

### Insertion

1. Make `x` its own representative, since the initial set only has `x`.
4. Set `x`'s rank to 0, since its tree has height 0.

### Lookup

1. Starting at `x`, follow parent pointers until you reach the representative (i.e., the node that is its own parent). 
2. While walking up, point each visited node to its grandparent 
3. Return the root.

### Merge

1. Look up the roots of $x$ and $y$.
2. Reject the merge if both roots are the same, since $x$ and $y$ are already in the same set.
3. Attach the root with the lower rank under the root with the higher rank (i.e., **union by rank**), so the tree height does not grow.
4. If both ranks are equal, attach either root under the other, then increment the new root's rank by $1$.

### Implementation

```
record DisjointSet[T: Hash + Eq] {
    index: HashMap[T, UInt], 
    data: ArrayList[T],      
    parent: ArrayList[UInt],
    rank: ArrayList[UInt8],
}

impl DisjointSet {
    pub func new() -> This {
        
    }

    pub func make_set(this, x: T) {
        let i = this.data.count();
        this.index[x] = i;
        this.data.push(x);
    
        this.parent.push(i);
    
        this.rank.push(0);
    }
    
    pub func find(this, x: T) -> T {
        func find_rec(this, i: UInt) -> UInt {
            if this.parent[i] == i {
                return i;
            }
            this.parent[i] = this.find_rec(this.parent[i]);
            return this.parent[i];
        }
        return this.data[find_rec(this.index[x])];
    }
    
    pub func union(this, x: UInt, y: UInt) -> Bool {
        let x_root = this.find(x);
        let y_root = this.find(y);
        
        if x_root == y_root {
            return False;
        }
    
        if this.rank[x_root] < this.rank[y_root] {
            this.parent[x_root] = y_root;
        } else if this.rank[x_root] > this.rank[y_root] {
            this.parent[y_root] = x_root;
        } else {
            this.parent[y_root] = x_root;
            this.rank[x_root] += 1;
        }
        True
    }
```

### Complexity Analysis

| Operation | Time Complexity |
| :---: | :---: |
| Initialize | worst-case $O(n)$ |
| Find | worst-case $O(\log n)$, but amortized $O(\alpha(n))$ |
| Union | worst-case $O(\log n)$, but amortized $O(\alpha(n))$ |

