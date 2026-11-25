# Two Pointers

## General Use Case

When the loop indices of your brute-force nested loops algorithm never needs to move backwards, but only forward.

> [!NOTE]
> The following templates are not comprehensive.

## Indexing Tips

- Finding `end` in `for i in range(end)`:
    1. Find the last _valid_ value of $i$ by solving for $i$ the following inequality: $$\text{highest} \ i \ \text{the loop body touches} \le n - 1$$
    2. Let $l$ be the $\text{RHS}$ of step 1. Then, $\text{end} = l + 1$.
- Use half-open intervals $[\text{low}, \text{high})$ everywhere, where the length is $\text{high} - \text{low}$ and the empty range is $\text{low} = \text{high}$, since adjacent ranges join with no gap.
- Run the loop mentally with $n = 0$, $n = 1$, $n = 2$, and the case where the window equals the whole input. 
- The element at index $k$ is the $(k + 1)^{th}$ element
- Prefer iterators over indices
    - `for x in a`
    - `for i, x in enumerate(a)`
    - `for x in zip(a, b)`
    - `for x in zip(a, a[1:])`
- Fence-post problem
    ```python
    for i, x in enumerate(items):
        if i > 0:
            print(",", end="")
        print(x, end="")
    ``` 

## Converge

### Use Case

The list input is sorted, and you are looking for a pair (or triplet) that meet a condition.

### Template

```python
def converge_two_pointers(array):
    left = 0
    right = len(array) - 1
    result = 0

    while left < right:
        # process `array[left]` and/or `array[right]`

        if <condition to move only `left`>:
            left += 1
        elif <condition to move only `right`>:
            right -= 1
        else:
            # both pointers satisfy/violate the condition together
            left += 1
            right -= 1

    return result
```

## Read/Write

### Use Case

The problem asks you to modify an array in-place and return its new count.

### Template

```python
def read_write_two_pointers(array):
    write = 0

    for read in range(len(array)):
        if <condition to keep array[read]>:
            array[write] = array[read]  # optionally transform
            write += 1

    return write  # new length
```

## Parallel

### Use Case

You have two separate lists (often sorted) and want to combine, compare, or synchronize them in order.

### Template

```python
def parallel_two_pointers(list1, list2):
    i = 0
    j = 0

    while i < len(list1) and j < len(list2):
        # compare `list1[i]` and `list2[j]`

        if <condition to advance only `i`>:
            i += 1
        elif <condition to advance only `j`>:
            j += 1
        else:
            # both elements satisfy the condition together (e.g. equal, or matched pair)
            i += 1
            j += 1

    # drain whichever list wasn't exhausted )only one of these runs)
    while i < len(list1):
        # process `list1[i]`
        i += 1

    while j < len(list2):
        # process `list2[j]`
        j += 1
```

## Three-Way Partition

### Use Case

Group values in a list into 3 categories.

### Template

```python
def three_way_partition_two_pointers(array):
    low = 0
    mid = 0
    high = len(array) - 1
    while mid <= high:
        if arr[mid] < pivot:
            arr[low], arr[mid] = arr[mid], arr[low]
            low += 1
            mid += 1
        elif arr[mid] > pivot:
            arr[mid], arr[high] = arr[high], arr[mid]
            high -= 1 # don't advance mid: the swapped-in value is unchecked
        else:
            mid += 1
```