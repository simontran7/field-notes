# Introduction to Complexity Analysis

## Best, Worst, and Average Case

Runtime is usually analyzed based on three different scenarios:
- **Best-Case:** $T_{best}(n) = \min\left\lbrace t(x) \;\middle|\; x \text{ is an input of size } n \right\rbrace$
- **Worst-Case:** $T_{worst}(n) = \max\left\lbrace t(x) \;\middle|\; x \text{ is an input of size } n \right\rbrace$
- **Average-Case:** $T_{average}(n) = \sum_{x} t(x) \cdot \Pr[x]$

> [!NOTE]
> Best case is is rarely studied since we're interested in the runtime for _all_ inputs!

## Asymptotic Notation

The asymptotic notation tells you how tightly/loosely you're bounding that scenario's growth.

### Definition (big O)

$O(f(n))$ is the set of functions that grow _at most_ as fast as $c \dot f(n)$, for some constant $c$. 

If $g \in O(f)$, then we say $g$ is asymptotically **upper bounded** by $f$.

### Definition (big Omega)

$\Omega(f(n))$ is the set of functions that grow _at least_ as fast as $c \dot f(n)$, for some constant $c$. 

If $g \in \omega(f)$, then we say $g$ is asymptotically **lower bounded** by $f$.

### Definition (big Theta)

$\Theta(f(n)))$ is the set of functions that grow _as fast_ as $f(n)$. 

If $g \in O(f)$ and $g \in \omega(f)$, then $g \in \Theta(f)$, and we say $g$ is asymptotically **tight bounded** by $f$.

> [!IMPORTANT]
> The case and the asymptotic notation are independent choices!

### Simplification Rules

1. Drop constant factors (since we only care about the rate of growth)

$$
O(c \cdot f(n)) \rightarrow O(f(n))
$$

3. Drop lower-order terms (since we only care about large inputs)

$$
O(f(n) + g(n)) \rightarrow O(max(f, g))
$$

### Laws

> [!formula] Law of Addition
> $$
> O(f(n)) + O(g(n)) \rightarrow O(f(n) + g(n))
> $$

> [!formula] Law of Multiplication
> $$
> O(f(n)) \cdot O(g(n)) \rightarrow O(f(n) \cdot g(n))
> $$

### Common Orders of Growth

| Growth type  | Function       | Nature of algorithm |
| :---: | :---: | :---: |
| Constant     | $1$            | A direct formula |
| Logarithmic  | $\log n$       | Halves the problem at each step |
| Square root  | $\sqrt n$      | Splits into $\sqrt{n}$ blocks, or loops up to $\sqrt{n}$ |
| Linear       | $n$            | Performs a constant number of passes over the input |
| Linearithmic | $n \log n$     | Involves sorting, and/or a data structure with operations in $O(\log n)$ time |
| Quadratic    | $n^2$          | Involves two nested loops |
| Cubic        | $n^3$          | Involves three nested loops |
| Exponential  | $2^n$          | Iterates over all subsets of the input |
| Factorial    | $n!$           | Iterates through all permutations of the input |

> [!definition]
> An algorithm is said to be **polynomial** if its time complexity is at most $O(n^k)$ where $k$ is a constant. A polynomial algorithm can typically process large inputs.

> [!NOTE]
> Asymptotic notation does not tell the whole story about how fast a program runs in real life! In real-world applications, constant factor matters, hardware matters, and implementation matters!

