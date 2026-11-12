# Interval Merge

## Use Case

Interval problems

> [!PROPOSITION] Overlap test (1D)
> Half-open 1D intervals $[a_s, a_e)$ and $[b_s, b_e)$ overlap if and only if $a_s < b_e$ and $b_s < a_e$.
>
> If the intervals are sorted so that $a_s \le b_s$, the test reduces to $b_s < a_e$.
>
> _Proof._ The intervals don't overlap exactly when one ends before the other starts: $(a_e \le b_s) \lor (b_e \le a_s)$. Negating with De Morgan's law, $\neg(\neg P \lor \neg Q) \equiv P \land Q$, i.e., turn each $\lor$ into $\land$ and each $\le$ into $<$, gives the test. ([source](https://zayenz.se/blog/post/how-to-check-for-overlapping-intervals/))

```tikz
\begin{tikzpicture}[thick, x=0.55cm, y=0.55cm]
  \foreach \dx/\as/\ae/\bs/\be/\txt in {
      0/2/4/5/7/{Case 1: $a$ ends before $b$ starts},
      10/5/7/2/4/{Case 2: $b$ ends before $a$ starts}} {
    \begin{scope}[shift={(\dx,0)}]
      \draw[thin] (1,0) -- (8,0);
      \foreach \t in {1,...,8} \draw[thin] (\t,0) -- (\t,-0.2);
      \foreach \t in {1,3,5,7} \node[below, font=\footnotesize] at (\t,-0.2) {\t};
      \draw (\as,1.7) rectangle node {$a$} (\ae,2.5);
      \draw (\bs,0.5) rectangle node {$b$} (\be,1.3);
      \node[below] at (4.5,-1.1) {\txt};
    \end{scope}
  }
\end{tikzpicture}
```

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

```tikz
\begin{tikzpicture}[thick, x=0.4cm, y=0.4cm]
  \foreach \dx/\dy/\ax/\aX/\ay/\aY/\bx/\bX/\by/\bY/\txt in {
      0/0/5/8/2/7/1/4/3/8/{Case 1: $b$ is left of $a$},
      13/0/1/4/2/7/5/8/3/8/{Case 2: $b$ is right of $a$},
      0/-12/2/7/1/4/3/8/5/8/{Case 3: $b$ is above $a$},
      13/-12/2/7/5/8/3/8/1/4/{Case 4: $b$ is below $a$}} {
    \begin{scope}[shift={(\dx,\dy)}]
      \draw[thin] (0,9) -- (0,0) -- (9,0);
      \draw (\ax,\ay) rectangle node {$a$} (\aX,\aY);
      \draw (\bx,\by) rectangle node {$b$} (\bX,\bY);
      \node[below] at (4.5,-0.4) {\txt};
    \end{scope}
  }
\end{tikzpicture}
```

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

