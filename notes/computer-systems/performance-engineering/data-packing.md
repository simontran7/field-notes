# Data Packing

**Data packing** shrinks data so that more of it fits in each cache line, which cuts memory use and the number of trips to memory. It builds on [alignment and padding](../computer-organization/data-representation.md#alignment) and [bit masks](../computer-organization/data-representation.md#bit-masks).

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

Record packing works in whole bytes. **Bit packing** goes further by storing several small fields in the bits of a single integer, using shifts to place each field and [masks](../computer-organization/data-representation.md#bit-masks) to read it back.

For example, an RGB colour with 8 bits per channel fits in one `u32` instead of three separate fields:

```rust
fn pack(r: u8, g: u8, b: u8) -> u32 {
    ((r as u32) << 16) | ((g as u32) << 8) | (b as u32)
}

fn red(color: u32) -> u8 {
    ((color >> 16) & 0xFF) as u8
}
```

**Flags** are the extreme case: each boolean takes a single bit, so one `u32` holds 32 of them instead of 32 one-byte `bool`s.

```rust
const VISIBLE: u32 = 1 << 0;
const SELECTED: u32 = 1 << 1;
const LOCKED: u32 = 1 << 2;

let mut flags = VISIBLE | LOCKED;
flags |= SELECTED;                     // set
flags &= !LOCKED;                      // clear
let is_visible = flags & VISIBLE != 0; // check
```

C also has **bit-fields**, which let the compiler do the shifting and masking for you:

```c
struct Color {
    unsigned r : 8;
    unsigned g : 8;
    unsigned b : 8;
};
```

> [!WARNING]
> The layout of C bit-fields (bit order, padding between them) is implementation-defined, so don't rely on it for file formats or network protocols. You also can't take the address of a bit-field.

### Trade-Off

Packed data is smaller, but every read and write now costs extra shift and mask instructions. Bit packing pays off when there are many objects and memory traffic is the bottleneck (e.g., large arrays scanned in a loop), and is usually not worth it for a handful of objects.
