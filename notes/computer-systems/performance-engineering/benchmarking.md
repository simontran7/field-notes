# Benchmarking

## Micro-benchmarking

### Step 1: Pick one question

Write the question down before any code. Good questions compare two things:

- "Is `sum` on `u32` faster than on `u64`?"
- "Is `Vec::insert` in the middle faster than `LinkedList` up to N = 100k?"
- "Does `chunks_exact` make `find2` vectorize?"

**Rule**: always have a baseline (naive version, std version, or the previous commit).

### Step 2: Generate the input

#### Step 2a: Use a tiny, seeded, hand-written PRNG


```rust
fn random_numbers() -> impl Iterator<Item = u32> {
    let mut random = 92u32; // fixed seed
    println!("");
    std::iter::repeat_with(move || {
        random ^= random << 13;
        random ^= random >> 17;
        random ^= random << 5;
        random
    })
}
```

Benefits:
- **Deterministic**: same input every run, so runs are comparable.
- **No dependency**: no `rand` crate version drift.
- **Cheap**: generation cost never leaks into the timing.

### Step 2b: Generate a fixed number of _bytes_, then reinterpret

```rust
let data: Vec<u8> = random_numbers()
    .flat_map(|it| it.to_le_bytes())
    .take(64 * 1024 * 1024) // 64 MiB, a round size
    .collect();
```

Choosing the size in bytes makes it easy to reason about cache
fit (L1 ~ 32–128 KiB, L2 ~ 1–16 MiB, L3 ~ tens of MiB, then RAM).

### Step 2c: Build the input _outside_ the timed region

In matklad's `sort` and `blake3`, input is even written to a file once
(`input.data`), and each implementation reads the same file. In divan, the
equivalent is: build it in the bench fn body, before `bencher.bench(...)`.

```rust
#[divan::bench(args = [1_000, 100_000, 10_000_000])]
fn sum(bencher: Bencher, n: usize) {
    let v: Vec<u32> = random_vec(n);          // NOT timed
    bencher.bench(|| benchplay::sum(black_box(&v)))  // timed
}
```

If the function **mutates** its input (sort, insert), use `with_inputs` so each
iteration gets a fresh copy and the copy is not timed:

```rust
bencher
    .with_inputs(|| v.clone())
    .bench_local_refs(|v| v.sort_unstable());
```

### Step 2d: Shape the data like reality (distribution matters)

Uniform random is the default, but ask: does real data look like this?

- `find2`: haystack is a repeated English sentence, needle placed **at the very
  end**. Forces a full scan, so it measures the worst case on purpose.
- `big-hash-map-small-hash-map`: keys are **Pareto** (half the accounts are
  one id, a quarter another, ...), then shuffled. Hot keys stay in cache, so a
  uniform distribution would give a misleading answer.
- `list-vs-vec`: always insert at the **middle**. Picks the case where the two
  data structures actually differ.

Rule: pick the distribution that answers *your* question, and write that choice
in a comment.

### Step 2e: Sweep sizes by powers of ten

`list-vs-vec` prints results at 1, 10, 100, ... 100 000. Crossover points and
cache cliffs only show up across sizes. In divan:

```rust
#[divan::bench(args = [1_000, 100_000, 10_000_000])]
```

Pick sizes that land in L1, L2/L3, and RAM.

### Step 3: Stop the compiler from optimizing away

Two failure modes:

| Problem | Symptom | Fix |
| :---: | :---: | :---: |
| Input known at compile time | constant-folded, ~0 ns | `black_box(input)` |
| Output unused | whole call deleted, ~0 ns | return the value (divan black-boxes it) or `black_box(out)` |

Your `range_const` vs `range_blackbox` benches demonstrate the first row.

matklad's non-divan version of the same idea:
- `#[inline(never)]` on the functions under test.
- Print a **fingerprint/checksum** of the result (`fingerprint=`, `checksum=`),
  so the work cannot be eliminated.

### Step 4: Check correctness in the same run

`find2` does this before printing times:

```rust
assert_eq!(r_slow, r_loop);
assert_eq!(r_slow, r_fast);
```

Check that every variant computes the correct result. In divan, add a plain `#[test]` or an
`assert_eq!` in the bench body (outside `bench(...)`) comparing the variants.

### Step 5: Report throughput alongside time

Time per call is hard to compare across sizes. Add a counter:

```rust
bencher.counter(ItemsCount::new(n))      // items/s
bencher.counter(BytesCount::of_slice(&v)) // bytes/s: compare against memory bandwidth
```

matklad's `sort` does the manual version: "64MiB round = 42ms".

### Step 6: Run it right

```bash
cargo bench                          # release mode, always
cargo bench -- sum                   # filter to one bench
RUSTFLAGS="-C target-cpu=native" cargo bench   # if you want SIMD (find2 does this)
```

- Close heavy apps. Plug in the laptop.
- Run twice. If numbers move more than ~5%, the environment is noisy.
- Repeat attempts: matklad runs 5–10 attempts and shows every attempt along with the mean.

## Step 7: Write down the result

Each matklad benchmark has a README with: the question, the command, the raw
output. Do the same, even as a comment at the top of the bench file. Without
it you will re-run the benchmark in a month and not remember what it meant.

## Macro-Benchmarking

TO LEARN
