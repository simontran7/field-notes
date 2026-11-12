# Physical Memory

## Memory Hierarchy

The memory hierarchy is typically divided into two broad categories:
- **Primary storage**: Devices the CPU can access directly using its instruction set, including registers, caches, and main memory. These devices are **volatile**, meaning they do _not_ retain data when power is lost.
- **Secondary storage**: Devices that the CPU cannot access directly, requiring data to first be copied into primary storage before the CPU can operate on it. These devices are **persistent**, meaning they retain data when there are power outages.

Memory performance is evaluated primarily along three dimensions:
- **Memory Latency**: the time required for a device to deliver requested data after being instructed to do so, measured in time units (nanoseconds, milliseconds) or CPU cycles
- **Memory Throughput**: the amount of data that can be transferred between the device and main memory per unit time, typically measured in bytes per second.
- **Memory Bandwidth**: The upper bound on the amount of data that can be read per unit time, typically measured in bytes per second.

These dimensions are shaped by two key factors:
- The distance from the CPU plays a crucial role, as devices closer to the CPU's processing units can deliver data more quickly.
- The underlying technology of the storage medium heavily influences performance. Registers and caches rely on extremely simple, compact circuits made of only a few logic gates, allowing signals to propagate almost instantly, while mechanical hard drives suffer from delays of 5-15 milliseconds due to the physical need to rotate and align read/write heads.

```tikz
\begin{tikzpicture}[level/.style={draw, align=center, minimum height=1cm}, unit/.style={font=\small, right}]
  % Each level is wider than the one above it: larger, slower, and cheaper per byte.
  \foreach \k/\name/\desc in {0/Registers/{}, 1/L1 cache/{SRAM, per core}, 2/L2 cache/{SRAM, on-chip},
      3/L3 cache/{SRAM, on-chip, shared}, 4/Main memory/DRAM, 5/Local secondary storage/{SSDs, hard disks},
      6/Remote secondary storage/{Network file systems, web servers}}
    \node[level, minimum width=4cm + 1.1cm*\k] (L\k) at (0, -1.6*\k) {L\k: \name \\ {\small \desc}};
  % Each level holds copies of data from the level below, moved in these units.
  \foreach \k/\unit in {1/words, 2/cache lines, 3/cache lines, 4/cache lines, 5/disk blocks, 6/files} {
    \pgfmathtruncatemacro{\above}{\k - 1}
    \draw[->, thin] (L\k.north) -- node[unit] {\unit} (L\above.south);
  }
  \draw[->] (-6.6, -9.6) -- node[sloped, above, font=\small] {smaller, faster, costlier per byte} (-6.6, 0);
  \draw[->] (6.6, 0) -- node[sloped, above, font=\small] {larger, slower, cheaper per byte} (6.6, -9.6);
\end{tikzpicture}
```

### Primary Storage

Primary storage devices, notably CPU registers, CPU caches, and main memory, consist of **random access memory (RAM)**, meaning access time stays constant regardless of where the data sits in the device.

Primary storage devices follow two main cell memory designs:
- **Static RAM (SRAM)**: stores data in small electrical circuits and represents the fastest type of memory used to build registers and caches
- **Dynamic RAM (DRAM)**: stores data using capacitors that hold electrical charges and must frequently refresh these charges to maintain stored values, making it ideal for implementing main memory.

### Secondary Storage

The two most common secondary storage devices today are **hard disk drives** (HDDs) and flash-based **solid-state drives** (SSDs).

A hard drive consists of several flat, circular **platters**, each with an upper surface and a lower surface. Organized in concentric rings called **tracks**, each surface breaks into **sectors** separated by **gaps**, and tracks aligned across platters form a **cylinder**. At the centre of each platter sits a **spindle**.

```tikz
\begin{tikzpicture}[note/.style={font=\small}]
  % A surface holds concentric tracks around the spindle.
  \foreach \r in {1.25, 1.5, 1.75, 2, 2.25, 2.5} \draw[thin] (0, 0) circle (\r);
  \draw (0, 0) circle (2.75);
  \draw[hl] (0, 0) circle (0.85);
  \node at (0, 0) {spindle};
  \node[note] (surface) at (2.7, 2.9) {surface};
  \draw[->, thin] (surface) -- (1.75, 1.75);
  \node[note] (tracks) at (-2.6, 2.9) {tracks};
  \draw[->, thin] (tracks) -- (-1.2, 1.55);
  \draw[->, thin] (tracks) -- (-1.38, 1.15);
  \draw[->, line width=1.5pt] (2.9, 0) -- (4.1, 0);
  % One track is divided into sectors separated by gaps.
  \foreach \a in {0, 30, ..., 330} \draw[line width=2.4pt] (6, 0) ++(\a + 4:1.45) arc[start angle=\a + 4, end angle=\a + 26, radius=1.45];
  \node[note] at (5.8, 1.85) {track $k$};
  \node[note] (gaps) at (7.7, 1.9) {gaps};
  \draw[->, thin] (gaps) -- ($(6, 0) + (60:1.5)$);
  \draw[->, thin] (gaps) -- ($(6, 0) + (30:1.5)$);
  \node[note] (sectors) at (6, -2.3) {sectors};
  \draw[->, thin] (sectors) -- ($(6, 0) + (-105:1.5)$);
  \draw[->, thin] (sectors) -- ($(6, 0) + (-75:1.5)$);
\end{tikzpicture}
```

```tikz
\begin{tikzpicture}[note/.style={font=\small}]
  % Three platters on one spindle, seen from slightly above. Platter i sits at height -1.7i.
  \def\R{2.8}\def\r{0.45}\def\k{1.4}
  % Spindle: only the parts not hidden behind a platter (each platter's front edge is \r below its centre).
  \foreach \lo/\hi in {0/1.1, -1.7/-0.45, -3.4/-2.15, -6/-3.85} {
    \fill[black, fill opacity=0.12] (-0.4, \lo) rectangle (0.4, \hi);
    \draw (-0.4, \lo) -- (-0.4, \hi) (0.4, \lo) -- (0.4, \hi);
  }
  \draw (0, 1.1) ellipse (0.4 and 0.08);
  \draw (-0.4, -6) arc[start angle=180, end angle=360, x radius=0.4, y radius=0.08];
  \foreach \y in {0, -1.7, -3.4} \draw[thin] (-0.4, \y) arc[start angle=180, end angle=360, x radius=0.4, y radius=0.08];
  % Platters, each with the track that belongs to cylinder k.
  \foreach \i in {0, 1, 2} {
    \pgfmathsetmacro{\y}{-1.7*\i}
    \draw (0, \y) ellipse (\R cm and \r cm);
    \draw[thin] (0, \y) ellipse (\k cm and 0.22cm);
    \node[note, anchor=west] at (6, \y) {platter \i};
  }
  \foreach \x in {-\k, \k} \draw[thin, dotted] (\x, 0) -- (\x, -3.4);
  % Surfaces: the upper and lower side of each platter.
  \foreach \i in {0, 1, 2} {
    \pgfmathsetmacro{\y}{-1.7*\i}
    \pgfmathtruncatemacro{\up}{2*\i}\pgfmathtruncatemacro{\down}{2*\i + 1}
    \draw[<-, thin] (-1.9, \y + 0.18) -- (-3.4, \y + 0.45) node[left, note] {surface \up};
    \draw[<-, thin] (-2.3, \y - 0.27) -- (-3.4, \y - 0.5) node[left, note] {surface \down};
  }
  % Actuator: one column whose arms move every read/write head together.
  \draw[hl] (4.1, 0.5) -- (4.1, -4.4) arc[start angle=180, end angle=360, x radius=0.35, y radius=0.1] -- (4.8, 0.5);
  \draw (4.45, 0.5) ellipse (0.35 and 0.1);
  \foreach \i in {0, 1, 2} {
    \pgfmathsetmacro{\y}{-1.7*\i}
    \draw (4.1, \y + 0.42) -- (\k + 0.12, \y + 0.08) -- (\k + 0.12, \y - 0.02) -- (4.1, \y + 0.22);
    \fill (\k + 0.02, \y - 0.06) rectangle (\k + 0.22, \y + 0.12);
  }
  \node[note] (cyl) at (0.6, 2.1) {cylinder $k$};
  \draw[->, thin] (cyl) -- (\k, 0.12);
  \node[note] (arm) at (3.2, 1.5) {arm};
  \draw[->, thin] (arm) -- (3.1, 0.33);
  \node[note] at (4.45, -4.95) {actuator};
  \node[note] at (0, -6.45) {spindle};
\end{tikzpicture}
```

To read or write data, mechanical **arms** attached to a single **actuator** move in unison, extending or retracting across the platter. Each arm carries a **read/write head** at its tip to position over the cylinder containing holding the desired data. The platters then spin counter-clockwise until the target sector sits under the head. Finally, the data is read.

There are three performance metrics when reading or writing data from a persistent storage:
- **Seek time**:
- **Rotational Latency**: $\frac{1}{2} \times \frac{1}{\text{RPM}} \times \frac{60 \text{ seconds}}{1 \text{ min}}$
- **Transfer Time**: $\frac{1}{\text{RPM}} \times \frac{1}{\text{average sectors per track}} \times \frac{60 \text{ seconds}}{1 \text{ min}}$

## Caching

> [!DEFINITION]
> A **cache** is a small, fast memory bank built into the CPU that stores copies of frequently or recently used instructions and data from main memory.

According to **Moore's law**, the number of transistors on microchips, and therefore processor performance, doubles every two years, but memory bus bandwidth improves far more slowly. The widening gap starves increasingly powerful processors of data, since they can request information faster than the memory system can deliver it. Caches exist to bridge that gap.

### Locality of Reference

> [!DEFINITION]
> A program's memory access pattern is called its **locality of reference**. It comes in two types: **temporal locality** and **spatial locality**.

- **Temporal locality**: the tendency for a recently accessed memory location to be accessed again in the near future. 
    - Loops: the same instructions execute repeatedly, and the same variables are updated each iteration.
    - Function calls: a function's code may run many times; its stack frame and local variables are reused throughout its lifetime.
    - Hot code paths: frequently executed conditionals and critical sections repeatedly touch the same data structures.

- **Spatial locality**: describes the tendency for memory locations _near_ a recently accessed address to be accessed in the near future. 
    - Array traversals: sequential accesses like `a[i]` followed by `a[i + 1]`
    - Struct field access: fields that are adjacent in memory tend to be read or written together.

By improving locality of reference, a program makes better use of the CPU cache and, as a result, runs faster. 

Conversely, poor locality leaves the cache under-utilized:
- Random access over large datasets: jumping around memory whose working set exceeds the cache size leads to frequent cache misses.
- Pointer chasing: traversing linked lists or graphs with scattered heap nodes defeats hardware prefetching and wastes cache lines.
- Cache thrashing: access patterns that repeatedly evict still-useful data, preventing the cache from ever warming up.

### Mechanics

#### Cache Addressing

A cache is made of small chunks of memory copied from main memory, each called a **cache line** and typically **64 bytes**. The cache can only load and store data in multiples of a cache line, its basic unit of transfer.

Each cache line consists of three sections: the **valid bit**, the **tag**, and the **data block**.

- **Valid bit**: Indicates whether the cache line contains valid, up-to-date data corresponding to some address in main memory.
- **Tag**: Stores the high-order bits of the memory address of the data currently cached.
- **Data block**: Contains the actual data copied from main memory.

```tikz
\begin{tikzpicture}[line/.style={draw, hl, minimum width=2cm, minimum height=0.6cm}, set/.style={draw, thin, dashed}]
  % S sets, each with E lines; each line holds a valid bit, a tag, and a block of B bytes.
  \foreach \y in {0, -1, -2, -3.8} {
    \draw[set] (-0.2, \y + 0.45) rectangle (7.4, \y - 0.45);
    \node[line] (a\y) at (1, \y) {};
    \node[line] (b\y) at (3.2, \y) {};
    \node at (4.7, \y) {$\cdots$};
    \node[line] (c\y) at (6.2, \y) {};
  }
  \node at (3.6, -2.9) {$\vdots$};
  \draw[thin] (7.4, 0.3) -- (8, 0.3) node[right] {set};
  \draw[thin] (c0.east) -- (8, 0) node[right] {line};
  % Zoom into one line.
  \draw[thin] (2.2, -4.1) -- (0.3, -5.2);
  \draw[thin] (4.2, -4.1) -- (6.7, -5.2);
  \draw (0.3, -5.2) rectangle (6.7, -6.2);
  \node[draw, minimum size=0.6cm] at (0.8, -5.7) {$v$};
  \node[draw, minimum width=1.4cm, minimum height=0.6cm] at (2.05, -5.7) {tag};
  \foreach \b [count=\i from 0] in {0, 1, 2} \node[draw, minimum size=0.6cm] at (3.3 + 0.6*\i, -5.7) {\b};
  \node[draw, minimum width=0.8cm, minimum height=0.6cm] at (5.2, -5.7) {$\cdots$};
  \node[draw, minimum width=0.9cm, minimum height=0.6cm, inner sep=1pt] at (6.05, -5.7) {$B{-}1$};
\end{tikzpicture}
```

> [!NOTE]
> For caches using the write-back write hit policy, the cache line also contains a **dirtiness bit**: $1$ when dirty, $0$ when clean.

A cache memory address is split into three fields:
- **Block offset**: Tells _which_ starting byte inside a data block the CPU wants. The number of bits $b$ allocated to the block offset field is $\log_2 (B)$, where $B$ is the data block size.
- **Set index**: Tells _which_ set in the cache to look in. The number of bits $s$ allocated to the set index field is $\log_2 (S)$, where $S$ is the total sets.
- **Tag**: Tells _which_ specific data block the CPU wants to access in the cache. The number of bits $t$ allocated to the tag field is $w - (s + b)$, where $w$ is word size.

```tikz
\begin{tikzpicture}[field/.style={draw, minimum height=0.8cm, anchor=west}]
  % An address splits into three fields, most significant on the left.
  \node[field, hl, minimum width=4cm] (t) at (0, 0) {$t$ bits};
  \node[field, minimum width=2.4cm] (s) at (t.east) {$s$ bits};
  \node[field, minimum width=1.6cm] (b) at (s.east) {$b$ bits};
  \foreach \f/\name in {t/tag, s/set index, b/block offset} {
    \draw[thin] ([shift={(0.06, -0.15)}]\f.south west) -- ++(0, -0.15) -| ([shift={(-0.06, -0.15)}]\f.south east);
    \node[below, font=\small] at ([yshift=-0.3cm]\f.south) {\name};
  }
\end{tikzpicture}
```

#### Accessing the Cache

1. **Locate the set**

    The cache uses the index bits from the memory address to identify which set in the cache might contain the desired block.

2. **Check for a matching tag**

    The cache compares the tag bits from the memory address against the stored tag in each line of that set. A match with the line's valid bit set to 1 is a **cache hit**.

    No match, or a valid bit of 0, is a **cache miss**.

    Cache misses fall into three categories. A **cold (compulsory) miss** happens when data is requested for the first time and isn't yet cached. A **capacity miss** happens when the cache is too small to hold a program's active data, its **working set**, forcing old data out to make room for new. A **conflict miss** happens when a memory address maps to a cache location already occupied by a different line, even though free space exists elsewhere; the existing line is evicted and replaced by the one holding the requested address.

    When a miss occurs, the controller first looks for an invalid line to hold the new data. If every line in the set is valid, one must be replaced: a **replacement policy** picks the line in set-associative and fully associative caches, while a direct-mapped cache automatically selects the single line each block maps to. Once chosen, a dirty line in a write-back cache is written back to the lower memory device first. The controller then fetches the requested block from the next lower memory level, stores it in the selected line, and updates the line's valid bit and tag. Finally, it extracts the requested bytes and delivers them to the processor.

```tikz
\begin{tikzpicture}[blk/.style={draw, minimum width=1.1cm, minimum height=0.55cm}]
  % The request asks for block 14, which is already in the cache.
  \draw[<->, line width=1.5pt] (2.3, 2.2) -- node[right, font=\small] {Request: 14} (2.3, 1);
  \node[left] at (-0.3, 0.4) {Cache};
  \draw (-0.15, 0.85) rectangle (4.75, -0.05);
  \foreach \b/\s [count=\i from 0] in {8/, 9/, 14/hl, 3/} \node[blk, \s] at (0.55 + 1.2*\i, 0.4) {\b};
  \draw[<->, line width=1.5pt] (2.3, -0.2) -- (2.3, -1.4);
  \node[left] at (-0.3, -2.5) {Memory};
  \draw (-0.15, -1.55) rectangle (4.75, -4.6);
  \foreach \r in {0, ..., 3} \foreach \c in {0, ..., 3} {
    \pgfmathtruncatemacro{\b}{4*\r + \c}
    \node[blk] at (0.55 + 1.2*\c, -1.95 - 0.62*\r) {\b};
  }
  \node at (2.3, -4.3) {$\cdots$};
  \node[anchor=west, align=left, font=\small] at (5.4, 1.6) {\textit{Data in block $b$ is needed}};
  \node[anchor=west, align=left, font=\small] at (5.4, 0.4) {\textit{Block $b$ is in cache:} \\ \textbf{Hit!}};
\end{tikzpicture}
```

5. **Perform the access (read or write)**

    On a **read**, the CPU retrieves the data directly from the cache line's data block, beginning at the **block offset** from the memory address.

    On a **write hit** (the CPU writes to an address whose block is already cached), it updates the data in the line. What happens next depends on the **write-hit policy**:
    - **Write-through**: The write is immediately propagated to the lower memory to keep all levels synchronized.
    - **Write-back**: The write is deferred, where the cache line is marked **dirty** and only written back to lower memory when evicted or explicitly flushed.

    On a **write miss** (the CPU writes to an address whose block is _not_ cached), behaviour depends on the **write-miss policy**:
    - **Write-allocate (fetch-on-write)**: The missing block is first loaded from main memory into the cache, and then the write is performed. Typically paired with write-back, since once the block is cached, multiple writes can be performed locally without repeatedly accessing main memory.
    - **No-write-allocate (write-around)**: The data is written directly to main memory without loading the block into the cache. Typically paired with write-through, since write-through already ensures main memory stays up to date. If the CPU later _reads_ that address, it's a cache miss, and the block is fetched then.

### Issues

Inefficient access patterns can undo the speedup that caches provide by keeping frequently used data close to the CPU.

**Cache pollution** occurs when the cache is filled with data that is unlikely to be reused soon, displacing more valuable data that would have benefited from being cached. This often happens in workloads with poor temporal locality.

> [!DEFINITION]
> **Cache thrashing** is severe performance degradation caused by cache lines being repeatedly evicted and reloaded, forcing the system to spend more time fetching from slow main memory than executing instructions. It happens when a working set exceeds cache capacity, or when access patterns repeatedly conflict.

### Cache Hierarchy

Modern processors organize caches into a three-tier hierarchy:
- **L1 cache**: the smallest and fastest cache, located directly on each CPU core for immediate access.
- **L2 cache**: larger and a bit slower than L1, usually dedicated to a single core but still very close to it.
- **L3 cache**: the largest and slowest of the three, typically shared among all cores on the processor to coordinate data efficiently.

| Level | Common Size(s) |
| :---: | :---: |
| L1    | 32 KB        |
| L2    | 256 KB       |
| L3    | 8 MB or more |

```tikz
\begin{tikzpicture}[box/.style={draw, align=center, minimum height=0.8cm}, l1/.style={box, minimum width=1.25cm, font=\small}]
  % Each core has private L1 caches (split into instructions and data) and an L2; all cores share the L3.
  \foreach \c in {0, ..., 3} {
    \begin{scope}[xshift=3.2cm*\c]
      \node[box, hl, minimum width=2.7cm] (core\c) at (0, 0) {Core};
      \node[l1] (i\c) at (-0.725, -1.4) {L1 \\ (instr)};
      \node[l1] (d\c) at (0.725, -1.4) {L1 \\ (data)};
      \node[box, minimum width=2.7cm] (l2\c) at (0, -2.8) {L2};
      \draw[->, thin] (core\c.south -| i\c) -- (i\c);
      \draw[->, thin] (core\c.south -| d\c) -- (d\c);
      \draw[->, thin] (i\c) -- (l2\c.north -| i\c);
      \draw[->, thin] (d\c) -- (l2\c.north -| d\c);
      \draw[->, thin] (l2\c) -- (0, -3.85);
    \end{scope}
  }
  \node[box, minimum width=12.3cm, minimum height=1cm] at (4.8, -4.35) {L3 (shared)};
\end{tikzpicture}
```

#### Instruction Cache and Data Cache

L1 caches are typically split into **instruction caches (I-cache)** and **data caches (D-cache)**, following the **Harvard Architecture** design principle.

Separating instruction and data caches eliminates contention between instruction fetches and data accesses. A CPU needs to fetch instructions every cycle to keep the pipeline fed, while also loading and storing data for instructions already in execution. With a unified cache, fetches and accesses in the same cycle would compete for the same **cache port**, the read/write path into the cache's memory array. That competition creates a **structural hazard**, where two operations contend for the same hardware resource and one must stall until it frees up. Splitting the L1 cache lets fetches and accesses proceed simultaneously, keeping the pipeline busier and raising **instructions per cycle (IPC)**, and with it, processor performance.

Separate caches also add design flexibility. Instruction streams follow mostly sequential, highly predictable patterns and rarely self-modify, while data accesses are more random and frequently involve writes. This difference lets hardware designers optimize each cache type:
- **I-cache optimization**: Can be designed as read-only with sequential access patterns, enabling aggressive prefetching of subsequent instruction lines while avoiding the complexity and cost of write-back circuits, cache invalidation logic, and coherency protocols.
- **D-cache optimization**: Must handle the full complexity of read/write operations, cache coherency, and the unpredictable access patterns typical of data operations.

### Cache Associativity

> [!DEFINITION]
> A **fully-associative cache** is a cache where memory blocks can be stored _anywhere_ in the cache. Each access checks _every_ line in parallel, which is still expensive for reasonably sized caches.

To address this, caches restrict where specific memory blocks can reside, at a cost: since the cache is far smaller than main memory, multiple addresses inevitably map to the same cache location, a phenomenon called **cache aliasing**.

> [!DEFINITION]
> A **set-associative cache** is a cache where a memory block maps to a specific **set** (i.e., a compartment of the cache), according to the index field encoded in the memory block's address. Each set contains $N$ cache lines, so we typically specify a cache as **$N$-way set-associative**. The cache hardware uses the index field to select a set, then searches all $N$ lines in that set in parallel.

> [!DEFINITION]
> A **direct-mapped cache** maps each memory block to exactly one specific cache line based on the index field encoded in the memory block's address. No searching is needed, as the hardware directly computes which single cache line to check.

> [!PROPOSITION] Cache size
> $$
> \text{cache size} = \text{total sets} \times \text{cache lines per set} \times \text{data block size}
> $$
