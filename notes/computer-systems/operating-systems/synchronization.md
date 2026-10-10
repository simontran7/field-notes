# Synchronization

As long as no thread is writing, multiple threads can read shared data simultaneously; the moment one of them writes, it opens the door to a class of concurrency bugs called **data races**.

A data race occurs when:
- Two or more threads concurrently access the same memory location
- At least one access is a write
- The accesses are not synchronized

**Race conditions** form a broader class of concurrency bugs, arising whenever a program's correctness depends on the timing or ordering of events, like thread scheduling, where different orderings produce different results.

To prevent both data races and race conditions, we introduce **synchronization primitives**.

## Mutex Lock

...

## Readers-writer Lock

When reads are frequent and writes are rare, a **readers-writer lock** ([`std::sync::RwLock`](https://doc.rust-lang.org/std/sync/struct.RwLock.html)) becomes the synchronization primitive of choice, allowing multiple readers _or_ a single writer to access shared mutable state at a time.

Its two core methods are:
- `read()`: Acquires a read lock, and returns `RwLockReadGuard<T>`, or blocks if a write lock is held.
- `write()`: Acquires a write lock, and returns `RwLockWriteGuard<T>`, or blocks if read or write locks are held.

## Condition Variable

...

## Semaphore

...


