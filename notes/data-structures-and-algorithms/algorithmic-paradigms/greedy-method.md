# Greedy Method

## Use Case

A greedy algorithm can compute an optimal solution if we identify:
- a greedy choice
- an optimal substructure 

The problem typically asks:
- To maximize/minimize an amount
- You repeatedly have a choice of “what should I take/use/do next?”
- There is an obvious best choice at each step.
- You don't need to reconsider previous choices.
- Taking the best-looking option now doesn't make a future solution worse.

> [!TIP]
> To pick between a greedy algorithm and a dynamic programming algorithm (assuming optimal substructure):
> 1. Try to break greedy by coming up with small counterexamples. 
> 2. If greedy survives, look for the exchange argument.
> 3. If there is an exchange argument, then you may use greedy. Otherwise, consider DP.

## Template

1. Determine if you should greedily pick the minimum or the maximum at each step.
2. Sort the input array as needed (often you may need to sort based on the frequency of each element).
3. Iterate over the sorted array and increment the result, making use of a priority queue as needed.

