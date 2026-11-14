# Dynamic Programming

## Use Case

An optimization problem, a counting problem, or a feasability problem with the following properties:
- **Overlapping subproblems**: the same subproblems are encountered and solved multiple times
- **Optimal substructure**: the solution to the whole problem can be built from solutions to its subproblems

## Template 

## Step 0: Draw the recursion tree for a few small, varied inputs

1. Draw the root node and label it with everything that describes the leftover problem. Start with the obvious quantity that shrinks or moves as choices are made, such as an index or a remaining amount. Common examples include an input index $i$, a range $[i, j]$, a remaining budget, or a boolean status.
2. List every kind of choice the problem allows at a step. This list is the same for every node.
3. At the current node, draw an outgoing edge for each choice that is valid there. Each edge points to a child node labeled with the leftover problem after making that choice. Not every node gets an edge for every choice.
    - If you need information that isn't in the label to draw an edge, such as remaining fuel or whether you're holding something, add it to every node's label.
4. Repeat step 3 for each child until every branch ends at a leaf: a node where no choices remain because the leftover problem is trivially solved or impossible.

## Step 1: Define the subproblem

A subproblem definition is a function $dp$ that describes a family of related subproblems:

$$
dp(x_1,\dots,x_k)=\text{output}
$$

Use the recursion trees from Step 0 to decide what each subproblem represents.

- **Inputs $x_1,\dots,x_k$ (state variables)**: Find the smallest set of variables that fully determines the remaining problem at a node.
    - Use the quantities in the node labels from Step 0 as the state variables.

- **Output (subproblem meaning)**: Define the quantity the function computes for a given state.
    - Look at the question being asked at each node: what are you trying to determine about the remaining problem?
    - Express the answer in terms of the state variables and the problem's fixed input data, without referring to $dp$ itself.
    - Common examples include a minimum, maximum, count, cost, or boolean indicating whether a solution exists.

> [!DEFINITION]
> A **state** is a particular tuple $(x_1,\dots,x_k)$ of values for the state variables. Each state identifies one subproblem.

> [!DEFINITION]
> The **dimensionality** of a dynamic programming problem is the number of state variables. A problem is $1D$ when the state has one variable and $2D$ when it has two.

### Step 2: Check for Conditions

Dynamic programming relies on optimal substructure to be correct and on overlapping subproblems to be worthwhile. Check both using the recursion trees from Step 0:

- Optimal Substructure: Suppose you already know the answers to the child nodes' subproblems. Check whether you can combine those answers to get the answer for the current node. Identify how the children's answers combine, and explain why the combined result is the correct answer for the current node.
- Overlapping Subproblems: Look for two or more nodes with the same state, meaning the same label and the same values for any state variables added in Step 1. Those nodes are the same subproblem. If any subproblem repeats, dynamic programming can solve it once and reuse its answer. If none repeats, dynamic programming may offer no benefit over ordinary recursion.

### Step 3: Determine the Base Cases

The base cases come from the leaf nodes of the recursion trees in Step 0:

1. **Find the leaf states**: For each leaf node, read its label along with the values of any state variables added in Step 1. Together these give the state at which the recursion stops.
2. **Answer each leaf state**: For each leaf state, ask "What is the answer to the subproblem this state represents?" No choices remain at a leaf, so the answer can be stated directly. The leftover problem is either trivially solved or impossible.
3. **Write each base case using $dp$**: Use the leaf state as the input and its answer as the output, in the form $dp(\text{leaf state}) = \text{answer}$.

### Step 4: Determine the Recurrence Relation

Look at how each internal node's answer is computed from its children's answers, then generalize that pattern to any state:

$$
dp(x_1, \dots, x_k) = \underset{\text{each choice } c}{\text{combine}} \Big( \text{contribution}(c) + dp(x_1', \dots, x_k') \Big)
$$

- **Contribution:** what choice $c$ itself adds. It may be 0.
- **Update:** the general formula for how choice $c$ changes the state inputs, producing $(x_1', \dots, x_k')$. It must move the state closer to a base case.
- **Combine:** $\min$ or $\max$ for optimization problems, $\sum$ for counting problems, $\lor$ for feasibility problems.

### Step 5: Identify the Answer State

Determine which state holds the answer to the original problem. This is usually the state that covers the _entire_ problem. Also, the answer state sits at the opposite end from the base cases. 

> [!NOTE]
> Whenever the state definition pins the output to a _specific_ position, then the final answer is an aggregate over several states. 

### Step 4: Combine the recurrence relation and the answer state

#### Memoization

```python
memo = dict()

def dp(<state definition inputs>):
    if <state definition inputs> == <base case state>:
        return <base case value>

    if (<state definition inputs>) in memo:
        return memo[(<state definition inputs>)]

    result = <recurrence>  

    memo[(<state definition inputs>)] = result

    return result

return dp(<answer state>)
```

#### Tabulation

1. Initialize an array `dp` that is sized according to the subproblem variables largest values. In particular, whenever you have a base case of the form `if i >= n: return <base case value>`, and somewhere else in your algorithm you have `dp(i + x)`, then your array must have `n + x` rows.
2. For every base case `if <state variable> == <value>: return <base case value>`, explicitly set them in the lookup table `dp` i.e., `dp[<state variable>][...] = <base case value>` or implicitly through the initial values of the lookup table (e.g., `dp = [0] * (n + 1)` zero-initializes).
3. Write for-loop(s) that will iterate over your state variables, such that the outermost for loop iterates the first state variable in the order of the `dp()` parameters, and such that the `range()` bounds should begin from the first _non-base-case_ state variables problems, and end at the final result state variables. For boolean state variables, the range should be from $[0, 2)$. However, for certain matrix problems (e.g., Unique Paths, Minimum Path Sum) where the calculation for a cell `(row, col)` depends only on the results from cells that are above it `(row - 1, col)`, to its left `(row, col - 1)`, or both `(row - 1, col - 1)`, you can iterate through all the rows and columns from top-to-bottom, left-to-right. Within the loop, you use `continue` to skip the base case cells because their values have already been correctly initialized.
4. Under the inner-most for loop, copy-paste _only_ the recurrence logic from your memoization function.
5. Change every `dp(<state variable 1>, <state variable 2>, <...>)` function calls and `result` to array accesses `dp[<state variable 1>][<state variable 2>][<...>]`. For boolean state variables, represent `True` as `1` and `False` as `0`.

> [!NOTE]
> If you merge every set of nodes that share the same state into a single node, the recursion tree becomes a **directed acyclic graph (DAG)** of subproblems. Edges point from each subproblem to the smaller subproblems it depends on, and some nodes now have more than one parent. Dynamic programming solves each node once and reuses its answer for every parent. When no states repeat, nothing merges and the tree stays a tree, and so plain recursion is already efficient, since every node is a different subproblem and there’s nothing to reuse.

> [!TIP]
> We can reduce the space complexity of a bottom-up dynamic programming algorithm whenever the recurrence is static (i.e., it doesn't change between inputs and it only cares about a static number of previous states). Simply replace the lookup table with variables to keep track of those previous states, one rolling variable per dp index you look back at. Additionally, add an aggregation `result` variable (initialized to the first valid base case value) if the answer is the maximum over all states rather than just the final state `dp[n - 1]`. This happens when the recurrence can produce values smaller than a previous state, meaning the peak may not be at the end.

> [!TIP]
> When the problem asks for the solution itself, not just its value, perform a **traceback**:
>
> 1. **Record the choice:** Inside `dp`, whenever a state's result is computed, store which transition produced it.
> 2. **Follow the choices:** Once the initial `dp` call returns, start at the answer state, look up its recorded choice, and move to the state that choice leads to. Repeat until you reach a base case.
>
> For example:
>
> ```python
> import math
>
>
> class Solution:
>     def coinChange(self, coins: List[int], amount: int) -> List[int]:
>         memo = dict()
>         choice = (
>             dict()
>         )  # choice[<amount>] = coin that achieved the minimum for <amount>
>
>         def dp(amount):
>             if amount == 0:
>                 return 0
>
>             if amount in memo:
>                 return memo[amount]
>
>             result = math.inf
>             for coin in coins:
>                 if amount - coin >= 0:
>                     candidate = 1 + dp(amount - coin)
>                     if candidate < result:
>                         result = candidate
>                         choice[amount] = coin  # record the winning transition
>
>             memo[amount] = result
>             return memo[amount]
>
>         # Traceback: follow recorded choices from the answer state to the base case
>         coins_used = []
>
>         if dp(amount) == math.inf:
>             return coins_used  # no solution, so nothing to trace back
>
>         while amount > 0:
>             coins_used.append(choice[amount])
>             amount -= choice[amount]
>         return coins_used
> ```

## Complexity Analysis

Let $x_i$ be $i$-th state definition's input (out of $k$ total state definition inputs), and $C$ the cost of processing a single subproblem. Then:
- Time Complexity: worst-case $O(\prod_{i=1}^{k} x_i \cdot C)$
- Space Complexity:
    - Top-down (memoization): worst-case $O(\prod_{i=1}^{k} x_i)$
    - Bottom-up (tabulation): worst-case $O(\prod_{i=1}^{k} x_i)$ when using a lookup table _or_ $O(1)$ when the recurrence only depends on a static number of previous states and forego a lookup table (see note above)
