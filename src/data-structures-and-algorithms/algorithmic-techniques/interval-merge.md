# Interval Merge

## Use Case

Interval problems

> [!PROPOSITION] Overlap test (1D)
> Half-open 1D intervals $[a_s, a_e)$ and $[b_s, b_e)$ overlap if and only if $a_s < b_e$ and $b_s < a_e$.
>
> If the intervals are sorted so that $a_s \le b_s$, the test reduces to $b_s < a_e$.
>
> _Proof._ The intervals don't overlap exactly when one ends before the other starts: $(a_e \le b_s) \lor (b_e \le a_s)$. Negating with De Morgan's law, $\neg(\neg P \lor \neg Q) \equiv P \land Q$, i.e., turn each $\lor$ into $\land$ and each $\le$ into $<$, gives the test. ([source](https://zayenz.se/blog/post/how-to-check-for-overlapping-intervals/))

![](images/interval-merge-1.svg)

> [!COROLLARY] Overlap test (2D)
> Half-open 2D intervals $a = [a_{x_1}, a_{x_2}) \times [a_{y_1}, a_{y_2})$ and $b = [b_{x_1}, b_{x_2}) \times [b_{y_1}, b_{y_2})$ overlap if and only if
>
> $$(a_{x_1} < b_{x_2}) \land (b_{x_1} < a_{x_2}) \land (a_{y_1} < b_{y_2}) \land (b_{y_1} < a_{y_2})$$
>
> _Proof._ The 2D intervals don't overlap exactly when $b$ is left of, right of, above, or below $a$:
>
> $$(b_{x_2} \le a_{x_1}) \lor (a_{x_2} \le b_{x_1}) \lor (a_{y_2} \le b_{y_1}) \lor (b_{y_2} \le a_{y_1})$$
>
> Negating with De Morgan's law, $\neg(\neg P \lor \neg Q) \equiv P \land Q$, i.e., turn each $\lor$ into $\land$ and each $\le$ into $<$, gives the test.
>
> Alternatively, the 2D intervals overlap if and only if they overlap on both axes:
>
> $$\mathrm{overlap}(a_x, b_x) \land \mathrm{overlap}(a_y, b_y)$$
>
> _Proof._ Cases 1 and 2 mean $a_x$ and $b_x$ don't overlap. Cases 3 and 4 mean $a_y$ and $b_y$ don't overlap. So the 2D intervals don't overlap exactly when
>
> $$\neg\,\mathrm{overlap}(a_x, b_x) \lor \neg\,\mathrm{overlap}(a_y, b_y)$$
>
> Negating with De Morgan's law, $\neg(\neg P \lor \neg Q) \equiv P \land Q$, i.e., turn each $\lor$ into $\land$ and remove each $\neg$, gives the test.

> [!NOTE]
> For closed 1D and 2D intervals, replace each $<$ with $\le$.

![](images/interval-merge-2.svg)

## Template

```python
def merge(intervals):
    if not intervals:
        return []

    intervals.sort()

    result = [intervals[0]]

    for interval in intervals:
        start, end = interval
        prev_end = result[-1][1]
        if start <= prev_end:
            result[-1][1] = max(prev_end, end)
        else:
            result.append(interval)

    return result
```

