# Record Packing

## Background

Every type carries an **alignment requirement**, a number of bytes $N$ that determines which memory addresses its objects can occupy: if a type $T$ has an alignment requirement of $N$, every object of type $T$ must be placed at an address that's a multiple of $N$.

A primitive type has a **natural alignment requirement**, which is equal to its byte size.

The Rust compiler defaults to these alignment requirements:

| Type                              |               Typical alignment |
| :---: | :---: |
| `bool`                            |                               1 |
| `u8` / `i8`                       |                               1 |
| `u16` / `i16`                     |                               2 |
| `u32` / `i32`                     |                               4 |
| `u64` / `i64`                     |                               8 |
| `u128` / `i128`                   |                target-dependent |
| `usize` / `isize`                 |                target-dependent |
| `f32`                             |                               4 |
| `f64`                             |                               8 |
| `char`                            |                               4 |
| `()`                              |                               1 |
| Struct                            | maximum alignment of its fields |
| Enum                              |                target-dependent |
| Array `[T; N]`                    |                alignment of `T` |
| Slice `[T]`                       |                alignment of `T` |
| Reference `&T` / `&mut T`         |               pointer alignment |
| Raw pointer `*const T` / `*mut T` |               pointer alignment |

To satisfy alignment requirements, the compiler may insert **padding**, unused bytes holding no data, into record types.

There are two kinds of padding:
- **Internal padding**: unused bytes added after a field so the next field is aligned.
- **External padding**: unused bytes added after the last field so, in an array of structs, the next record object is aligned.

Since padding increases a record's size, it also increases memory consumption.

**Example (internal padding)**:

```rust
// offset
//   0    1        4          8
//   ↓    ↓        ↓          ↓
//   ┌────┬────────┬──────────┐
//   │ a  │   P    │    b     │
//   │1 B │ 3 bytes│  4 bytes │
//   └────┴────────┴──────────┘
//   └──────── 8 bytes ───────┘

#[repr(C)]
struct Foo {
    a: u8,
    b: u32,
}
```

**Example (external padding)**:

```rust
// offset
//   0                  8       12    14   15
//   ↓                  ↓        ↓     ↓    ↓
//   ┌──────────────────┬────────┬─────┬────┬─┐
//   │        a         │   b    │  c  │ d  │P│
//   │     8 bytes      │4 bytes │2 B  │1 B │1│
//   └──────────────────┴────────┴─────┴────┴─┘
//   └────────────── 16 bytes ────────────────┘

#[repr(C)]
struct Foo {
    a: u64,
    b: u32,
    c: u16,
    d: u8,
}
```

## Record Packing Tips

To shrink a record, reorder its fields:
1. Put fields with the larger alignment requirements first, then progressively smaller ones.
2. Group together fields of the same alignment
