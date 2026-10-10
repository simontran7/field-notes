# Sorting

## Problem

Given a sequence of $n$ elements $A = \langle a_1, a_2, \ldots, a_n \rangle$, rearrange the elements into nondecreasing order, so that

$$
a_1 \leq a_2 \leq \cdots \leq a_n.
$$

## Bubble Sort

### Template

```
func bubble_sort[E: Ord](array: &mut Slice[E]) {
    let n: UInt = array.count();

    for i in 0..n {
        let swapped: Bool = False;

        for j in 0..(n - 1 - i) {
            if array[j] > array[j + 1] {
                array[j].swap(array[j + 1]);
                swapped = True;
            }
        }

        if not swapped {
            break;
        }
    }
}
```

### Complexity Analysis

Let $n$ be the number of elements in the input sequence. Then:
- Time: worst-case $O(n^2)$
- Auxiliary Space: worst-case $O(1)$

## Selection Sort

## Insertion Sort

## Heap Sort

## Merge Sort

## Quick Sort

## Counting Sort

## Radix Sort

## Bucket Sort
