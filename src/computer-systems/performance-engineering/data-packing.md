# Data Packing

**Data packing** shrinks data so that more of it fits in each cache line, which cuts memory use and the number of trips to memory.

## Record Packing

The compiler inserts [padding](../computer-organization/data-representation.md#padding) between and after fields to keep them aligned, but it lays fields out in the order you declare them (with `#[repr(C)]` in Rust, and always in C). So the declaration order decides how much padding a record needs.

To shrink a record, reorder its fields:
1. Put fields with the larger alignment requirements first, then progressively smaller ones.
2. Group together fields of the same alignment

**Example:**

```rust
// 24 bytes: 7 bytes of padding after `a`, 7 after `c`
#[repr(C)]
struct Before {
    a: u8,
    b: u64,
    c: u8,
}

// 16 bytes: `a` and `c` share the space after `b`, then 6 bytes of external padding
#[repr(C)]
struct After {
    b: u64,
    a: u8,
    c: u8,
}
```

> [!NOTE]
> Without `#[repr(C)]`, the Rust compiler is free to reorder fields itself, and usually does this for you.

## Bit Packing

### Bit Fields

For example, an RGB colour with 8 bits per channel fits in one `u32` instead of three separate fields:

```rust
fn pack(r: u8, g: u8, b: u8) -> u32 {
    ((r as u32) << 16) | ((g as u32) << 8) | (b as u32)
}

fn red(color: u32) -> u8 {
    ((color >> 16) & 0xFF) as u8
}
```

### Bit Flags

Each boolean takes a single bit, so one `u32` holds 32 of them instead of 32 one-byte `bool`s.

```rust
const VISIBLE: u32 = 1 << 0;
const SELECTED: u32 = 1 << 1;
const LOCKED: u32 = 1 << 2;

let mut flags = VISIBLE | LOCKED;
flags |= SELECTED;                     // set
flags &= !LOCKED;                      // clear
let is_visible = flags & VISIBLE != 0; // check
```

