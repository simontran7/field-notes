# Atomics

## Performance Problems

Cache coherency mechanisms ([Multiprocessor Systems](../computer-organization/multiprocessor-systems.md#cache-coherence)) introduce two notable performance problems.

1. **False sharing**: occurs when different variables that happen to reside on the same cache line are modified by different processors. Since cache coherency operates at cache line granularity, modifications to logically independent variables trigger unnecessary invalidations across processors.

For example, if Thread A frequently updates `counter_a` and Thread B frequently updates `counter_b`, but both counters are adjacent in memory and share the same cache line, each write causes the cache line to bounce between processors even though the threads aren't actually sharing data. This ping-pong effect can cause severe performance degradation in multithreaded code.

2. **Write contention**: occurs when multiple processors genuinely write to the same or nearby shared data, triggering constant invalidation messages as cache coherency protocols work to maintain consistency. Each write broadcasts invalidations to every other cache holding that line, marking their copies Invalid while the writing processor marks its own Modified. As processors compete to write, the cache line repeatedly bounces between their caches, creating significant overhead from the constant coherency traffic. While false sharing produces artificial contention, write contention reflects a fundamental limitation: cache coherency protocols grow more expensive as more processors write to shared data. Minimizing it takes algorithmic approaches, like keeping per-thread local data merged only periodically, using lock-free data structures that reduce synchronization points, or redesigning algorithms to cut the frequency of shared writes.

## Costs at a Glance

Each of the three guarantees behind correct concurrent code (cache coherence, memory consistency, and atomicity) carries its own performance cost:

| Guarantee | Cost | How to avoid it |
| :---: | :---: | :---: |
| Coherence | The cache line moves between cores (false sharing, write contention) | Pad data or give each thread its own, so different cores don't write to the same line |
| Consistency | Fences drain the buffers, which costs hundreds of cycles | Use the weakest ordering that's still correct (`Relaxed` < `Acquire`/`Release` < `SeqCst`) |
| Atomicity | A contended atomic is a coherence cost: every atomic is a write, so the line moves on every attempt | Reduce sharing: count per thread and combine at the end; spin on loads, not on atomics |
