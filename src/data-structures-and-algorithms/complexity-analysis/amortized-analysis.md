# Amortized Analysis

1. Identify the resource whose movement drives the inner loop's cost (e.g., a pointer, a counter, a stack's size).
2. Verify the resource is **monotonic** (i.e., resource never gets refilled, or each step refills at most a constant amount) and **globally bounded** (i.e., cannot exceed some value $B$ over the entire run).
3. Conclude that the inner loop body executes at most $B$ times _in total across all outer iterations_.
