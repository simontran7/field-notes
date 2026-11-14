# In-Place Reversal

## Use Case

Reverse in-place the pointers in a linked list.

## Template

```python
def reverse_list(head):
    i = None
    j = head

    while j:
        temp = j.next
        j.next = i

        i = j
        j = temp

    return i
```
