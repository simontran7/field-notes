# Rust

https://levin405.neocities.org/blog/2023-01-01-learning-rust/

## Alignment

On x86-64, the Rust compiler defaults to these [alignment requirements](../computer-systems/computer-organization/data-representation.md#alignment):

| Type                              |               Typical alignment |
| :---: | :---: |
| `bool`                            |                               1 |
| `u8` / `i8`                       |                               1 |
| `u16` / `i16`                     |                               2 |
| `u32` / `i32`                     |                               4 |
| `u64` / `i64`                     |                               8 |
| `u128` / `i128`                   |                              16 |
| `usize` / `isize`                 |                               8 |
| `f32`                             |                               4 |
| `f64`                             |                               8 |
| `char`                            |                               4 |
| `()`                              |                               1 |
| Struct                            | maximum alignment of its fields |
| Enum                              | maximum alignment of its variants' fields and tag |
| Array `[T; N]`                    |                alignment of `T` |
| Slice `[T]`                       |                alignment of `T` |
| Reference `&T` / `&mut T`         |                               8 |
| Raw pointer `*const T` / `*mut T` |                               8 |
