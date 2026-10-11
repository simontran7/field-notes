# Bit Manipulation

## Bitwise Operators 

### AND Operator

#### Behaviour

The AND of 2 bits is 1 if both bits are 1, and 0 otherwise.

| a | b | output |
| :---: | :---: | :---: |
| 0 | 0 | 0      |
| 0 | 1 | 0      |
| 1 | 0 | 0      |
| 1 | 1 | 1      |

#### Properties

- **Identity:** `x & 1 = x`, i.e., AND with 1 lets the bit through unchanged.
- **Annihilator:** `x & 0 = 0`, i.e., AND with 0 forces the bit to 0.

#### Syntax

```cpp
a &b
```

### OR Operator

#### Behaviour

The OR of 2 bits is 1 if either (or both) bits is 1, and 0 otherwise.

| a | b | output |
| :---: | :---: | :---: |
| 0 | 0 | 0      |
| 0 | 1 | 1      |
| 1 | 0 | 1      |
| 1 | 1 | 1      |

#### Properties

- **Identity:** `x | 0 = x`, i.e., OR with 0 leaves the bit alone.
- **Annihilator:** `x | 1 = 1`, i.e., OR with 1 forces the bit to 1.

#### Syntax

```cpp
a | b
```

### NOT Operator

#### Behaviour

The NOT of a bit is 1 if the bit is 0, and 0 otherwise.

| a | output |
| :---: | :---: |
| 0 | 1      |
| 1 | 0      |

#### Syntax

```cpp
~a
```

> [!NOTE]
> `~x` flips every bit in the width of `x`'s type. For unsigned types, `~x == MAX - x` (e.g. `~0u == 4294967295` for 32 bits). For signed types (two's complement, guaranteed since C++20), `~x == -x - 1`.

### XOR Operator

#### Behaviour

The XOR of 2 bits is 1 if _exactly_ one of the bits is 1, and 0 otherwise.

| a | b | output |
| :---: | :---: | :---: |
| 0 | 0 | 0      |
| 0 | 1 | 1      |
| 1 | 0 | 1      |
| 1 | 1 | 0      |

#### Properties

- **Identity:** `x ^ 0 = x`, so XOR with 0 leaves the bit alone.
- **Negation:** `x ^ 1 = ~x`, so XOR with 1 flips the bit.

#### Syntax

```cpp
a ^ b
```

> [!NOTE]
> When bitwise operators are applied to an integer (i.e., multiple bits), the operator is applied to the corresponding bits!

### Left Shift Operator

#### Behaviour

Moves all bits in `n` over by `k` places to the left. Bits shifted past the left of the type are discarded. 

#### Syntax

```cpp
n << k
```

> [!PROPOSITION] Left shift multiplies
> `x << n` is equivalent to $x \cdot 2^n$, as long as no set bits fall off the top.

### Right Shift Operator

#### Behaviour

Moves all bits in `n` over by `k` places to the right. Bits shifted past the right of the type are discarded. What fills in from the left depends on the type:
- Unsigned Integers (**logical right shift**): fills `0`s from the left.
- Signed Integers (**arithmetic right shift**): fills copies of the sign bit from the left (sign extension). 

#### Syntax

```cpp
n >> k
```

> [!PROPOSITION] Right shift floor-divides
> `x >> n` is equivalent to $\lfloor \frac{x}{2^n} \rfloor$.

> [!WARNING]
> In C/C++, bitwise operators have low precedence, and therefore happen later in evaluation order. So, make sure to use parentheses to clearly define your intended grouping.

> [!WARNING]
> In C/C++, a shift happens in the type of its _left_ operand, where types narrower than `std::uint32_t` are promoted to signed `std::uint32_t` before shifting. Additionally, for a left operand $n$ that is $w$ bits wide, the shift amount `k` in `n <shift> k` must be in the range of $[0, w - 1]$, so shifting outside of this range is undefined behaviour. Lastly, when the left operand is a signed integer, shifting on it will produce unexpected results since its most significant bit acts as a sign bit. To avoid all these pitfalls, explicitly cast the _left_ operand via `static_cast<TYPE>(LEFT_OPERAND)` to an fixed-width integer that is wide enough, and that is unsigned.

## Bit Mask

### Use Case

Isolate specific bit(s) in an integer.

### Common Masks

```cpp
// single bit k
std::uint32_t mask = 1u << k;

// lowest k bits (k < 32; for k == 32 use ~0u)
std::uint32_t mask = (1u << k) - 1;

// contiguous range, bits i..j inclusive (j - i + 1 < 32)
std::uint32_t mask = ((1u << (j - i + 1)) - 1) << i;

// arbitrary bits (e.g. 3 and 5)
std::uint32_t mask = (1u << 5) | (1u << 3);

// everything except bit k
std::uint32_t mask = ~(1u << k);
```

## Common Bit Operations

Almost every bit manipulation operation on an integer `x` follows two steps:
1. Build a bitmask using shift operators.
2. Combine the bitmask with `x` using AND, OR, XOR, or NOT operator.

### Check Bit

Uses AND's identity on bit `k` and annihilator everywhere else, leaving only bit `k`.

```cpp
auto check_bit(std::uint32_t x, std::uint32_t k) -> bool {
    return (x & (1u << k)) != 0;
}
```

### Set Bit

Uses OR's annihilator on bit `k` (forces it to 1) and identity everywhere else.

```cpp
auto set_bit(std::uint32_t x, unsigned k) -> std::uint32_t {
    return x | (1u << k);
}
```

### Clear Bit

Uses AND's annihilator on bit `k` (forces it to 0) and identity everywhere else.

```cpp
auto clear_bit(std::uint32_t x, unsigned k) -> std::uint32_t {
    return x & ~(1u << k);
}
```

### Toggle Bit

Uses XOR's negation on bit `k` (flips it) and identity everywhere else.

```cpp
auto toggle_bit(std::uint32_t x, unsigned k) -> std::uint32_t {
    return x ^ (1u << k);
}
```

### Count the number of Set Bits

`x & (x - 1)` clears the lowest set bit of `x`, so we count how many times we can do that before reaching `0`.

```cpp
auto count_ones(std::uint32_t x) -> std::uint32_t {
    int count = 0;
    while (x != 0) {
        count += 1;
        x &= x - 1;
    }

    return count;
}
```

> [!TIP]
> C++20's `<bit>` header provides `std::popcount(x)` for this, which compiles to a single instruction on most CPUs.

### Count the Amount of Leading Zeroes

...

### Count the Amount of Trailing Zeroes

...

### Find the Index of the Most Significant Bit

The index of the most significant bit of some integer `x > 0` is $\lfloor \log_2(x) \rfloor$.

That is, how many times do I need to divide $x$ by 2 to get to 1.

```cpp
auto most_significant_bit(std::uint32_t x) -> std::uint32_t {
    std::uint32_t bit_index =
        -1; // so we can get 0-based index (and -1 for `x` == 0)
    while (x != 0) {
        bit_index += 1;
        x >>= 1;
    }

    return bit_index;
}
```

> [!WARNING]
> We take `x` as **unsigned** so that `>>=` is a logical shift. Right-shifting a negative signed `x` sign-extends indefinitely and will never reach `0` (leading to an infinite loop). Don't fix this with `std::abs` like you might in Python since `std::abs(INT_MIN)` overflows, which is undefined behaviour.

> [!TIP]
> C++20's `<bit>` header provides `std::bit_width(x)`, which equals `most_significant_bit(x) + 1`.

### Check if the Number is a Power of 2

...

### Rotates Bits

...

### Reverse Bits

...