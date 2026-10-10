# Fast and Slow Pointers

## Dynamic

### Use Case

Detect cycles in a linked list.

### Template

```python
def dynamic_fast_and_slow_pointers(head):
    slow = head
    fast = head
    result = <initial value>

    while fast and fast.next:
        slow = slow.next
        fast = fast.next.next
        # Optionally to detect cycle:
        # if fast == slow: break

    # `slow` is at midpoint of the list

    return result
```

## Fixed

### Use Case

Find the $k^\text{th}$ node from the end of a linked list.

### Template

```python
def fixed_gap_fast_and_slow_pointers(head, k):
   dummy = ListNode(0, head)
   slow = dummy
   fast = dummy

    # Advance fast by `k` steps
    # so that when `fast` is None,
    # `slow` will be `k` step(s) away from None,
    # i.e., the `k`th last node
    for _ in range(k):
        fast = fast.next

    while fast:
        slow = slow.next
        fast = fast.next

    # ... involving `slow` as the `k`th last node
```
