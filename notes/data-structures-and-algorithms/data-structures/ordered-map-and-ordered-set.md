# Ordered Map and Ordered Set

## Abstract Data Type

### Ordered Map

| Operation          | Description                                                                           |
| :---: | :---: |
| `get(key)`         | Returns the value corresponding to a given key in the ordered map.                 |
| `add(key, value)`  | Inserts a new key-value pair into the ordered map.                                    |
| `remove(key)`      | Removes the key-value pair with the given key from the ordered map.                |
| `min()`            | Returns the minimum key-value pair in the ordered map.                                |
| `max()`            | Returns the maximum key-value pair from the ordered map.                              |
| `successor(key)`   | Returns the smallest key-value pair with a key greater than `key` in the ordered map. |
| `predecessor(key)` | Returns the largest key-value pair with a key less than `key` in the ordered map.     |

### Ordered Set

| Operation          | Description                                                                           |
| :---: | :---: |
| `contains(<element>)`    | Returns whether `element` exists in the ordered set.                           |
| `add(<element>)`  | Inserts an element into the ordered set.                                    |
| `remove(<element>)`      | Removes `element` from the ordered set.                |
| `min()`            | Returns the minimum element in the ordered set.                                |
| `max()`            | Returns the maximum element in the ordered set.                              |
| `successor(<element>)`   | Returns the smallest element greater than `element` in the ordered set. |
| `predecessor(<element>)` | Returns the largest element less than `element` in the ordered set.     |

## Use Case

- Iterating over keys in sorted order without sorting them yourself
- Range queries

## Red-Black Tree

### Tree Properties

- In a binary tree, a level $d$ has at most $2^d$ nodes.
- A binary tree with $n$ levels has $2^n - 1$ nodes.
- A binary tree with height $h$ has $2^{h + 1} - 1$ nodes.
- A binary tree with $n$ nodes has $n$ - 1 edges.
- A complete binary tree has at most $\lceil{\frac{n}{2}}\rceil$ leaves.
- The height $h$ of a complete binary tree is $\lfloor{\log_{2} n}\rfloor$.
- The height $h$ of a balanced binary search tree is $\log n$.
- A full binary tree has $\text{internal nodes} + 1$ leaves.
- A full binary tree has $2 \cdot \text{internal nodes} + 1$ nodes, or equivalently, $2 \cdot \text{internal leaves} + 1$

### Lookup

### Insertion

### Deletion

### Complexity Analysis

| Operation | Time Complexity   |
| :---: | :---: |
| Lookup    | worst-case $O(\log n)$ |
| Insertion | worst-case $O(\log n)$ |
| Deletion  | worst-case $O(\log n)$ |

## Prefix Tree (Trie)

### Lookup

1. Start a cursor at the root.
2. For each character $c$ in the word, move the cursor to its child for $c$. If that child does not exist, the word is not in the trie, so return nothing.
3. Once every character is consumed, return the cursor's data. It is empty if the word is only a prefix of another word.

### Insertion

1. Start a cursor at the root.
2. For each character $c$ in the word, create an empty child for $c$ if one does not exist, then move the cursor to that child.
3. Once every character is consumed, write the value into the cursor's data, overwriting any existing value.

### Deletion

1. Recursively walk down from the root, one character at a time. If a character's child does not exist, the word is not in the trie, so stop.
2. At the node for the last character, reject the deletion if it holds no data. Otherwise, clear its data.
3. While unwinding back up, remove the child you came from if it now has no data and no children (i.e., it is a dead branch).
4. Stop pruning at the first ancestor that still has data or other children, since it belongs to another word.

### Implementation

```
pub record TrieMap[V] {
    root: ref TrieNode[V],
}

record TrieNode[V] {
    children: HashMap[Char, ref TrieNode],
    data: Option[V],
}

impl TrieMap {
    pub func get(this, word: String) -> Option[V] {
        let cursor = this.root;
        for c in word {
            if not cursor.children.contains(c) {
                return None;
            }
            cursor = cursor.children[c];
        }
        cursor.data
    }
    
    pub func add(this, word: String, value: V) {
        let cursor = this.root;
        for c in word {
            if not cursor.children.contains(c) {
                cursor.children[c] = TrieNode::new();
            }
            cursor = cursor.children[c];
        }
        cursor.data = Some(value);
    }
    
    pub func remove(this, word: String) -> Bool {
        func remove_rec[V](node: TrieNode[V], word: String, i: UInt) -> Bool {
            if i == word.count() {
                if node.data == None {
                    return False;
                }
                node.data = None;
                return node.children.is_empty();
            }
            let c = word[i];
            if not node.children.contains(c) {
                return False;
            }
            if remove_rec(node.children[c], word, i + 1) {
                node.children.remove(c);
                return node.children.is_empty() and node.data == None;
            }
            False
        }
    
        remove_rec(this.root, word, 0)
    }
}
```

### Complexity Analysis

| Operation | Time Complexity |
| :---: | :---: |
| Lookup | worst-case $O(m)$ |
| Insertion | worst-case $O(m)$ |
| Deletion | worst-case $O(m)$ |

