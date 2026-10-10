# Map and Set

## Abstract Data Type

### Map

| Operation | Description |
| :---: | :---: |
| `get(key)` | Returns the value associated with `key` |
| `add(key, value)` | Inserts or updates the value associated with `key` |
| `remove(key)` | Removes `key` from the map and returns its associated value |

### Set

| Operation | Description |
| :---: | :---: |
| `add(element)` | Inserts `element` into the set |
| `contains(element)` | Returns whether `element` exists in the set |
| `remove(element)` | Removes `element` from the set |

## Use Case

### Map

- Track elements seen so far for uniqueness (with any extra info stored as the value)
- Basic mapping (e.g., mapping every unique character to its frequency in a string)

### Set

- Track elements seen so far for uniqueness
- Store a chunk (or all) of the input for fast lookups

## Hash Table

<img src="images/hash_function_mapping.svg" width="500">

### Lookup

### Insertion

### Deletion

### Complexity Analysis

| Operation | Time Complexity |
| :---: | :---: |
| Lookup | worst-case $O(n)$, average $O(1)$ |
| Add | worst-case $O(n)$, amortized $O(1)$ |
| Remove | worst-case $O(n)$, average $O(1)$ |

## Bit Array

### Implementation

```rust
use std::ops::{
    BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not, Shl,
    Shr,
};

pub trait Word:
    Copy
    + Default
    + Eq
    + BitAnd<Output = Self>
    + BitAndAssign
    + BitOr<Output = Self>
    + BitOrAssign
    + BitXor<Output = Self>
    + BitXorAssign
    + Not<Output = Self>
    + Shl<u32, Output = Self>
    + Shr<u32, Output = Self>
{
    const BITS: usize;
    const ZERO: Self;
    const ONE: Self;
    const ONES: Self;

    fn count_ones(self) -> u32;
    fn trailing_zeros(self) -> u32;
    fn leading_zeros(self) -> u32;

    fn checked_shl(self, n: u32) -> Option<Self>;
    fn checked_shr(self, n: u32) -> Option<Self>;
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct StaticBitSet<W: Word, const WORDS: usize> {
    data: [W; WORDS],
}

pub struct StaticBitSetIterator<'a, W: Word, const WORDS: usize> {
    bitset: &'a StaticBitSet<W, WORDS>,
    prev: Option<usize>,
}

impl<W: Word, const WORDS: usize> StaticBitSet<W, WORDS> {
    /// Creates a new empty bit set.
    pub const fn new() -> Self {
        Self {
            data: [W::ZERO; WORDS],
        }
    }

    /// Creates a bit set from a slice `bits`
    pub fn from<const N: usize>(bits: &[usize; N]) -> Self {
        let mut data = [W::ZERO; WORDS];
        let mut i = 0;
        while i < N {
            let bit = bits[i];
            let word_index = bit / W::BITS;
            let bit_index = (bit % W::BITS) as u32;
            data[word_index] = data[word_index] | (W::ONE << bit_index);
            i += 1;
        }
        Self { data }
    }

    /// Returns whether `bit` is in the set.
    pub fn contains(&self, bit: usize) -> bool {
        let word_index = bit / W::BITS;
        let bit_index = (bit % W::BITS) as u32;
        (self.data[word_index] & (W::ONE << bit_index)) != W::ZERO
    }

    /// Adds `bit` to the set.
    pub fn add(&mut self, bit: usize) {
        let word_index = bit / W::BITS;
        let bit_index = (bit % W::BITS) as u32;
        self.data[word_index] |= W::ONE << bit_index;
    }

    /// Removes `bit` from the set.
    pub fn remove(&mut self, bit: usize) {
        let word_index = bit / W::BITS;
        let bit_index = (bit % W::BITS) as u32;
        self.data[word_index] &= !(W::ONE << bit_index);
    }

    /// Toggles `bit`.
    pub fn toggle(&mut self, bit: usize) {
        let word_index = bit / W::BITS;
        let bit_index = (bit % W::BITS) as u32;
        self.data[word_index] ^= W::ONE << bit_index;
    }

    /// Clears the set, removing all bits.
    pub fn clear(&mut self) {
        for w in self.data.iter_mut() {
            *w = W::ZERO;
        }
    }

    /// Fills the set, adding all possible bits.
    pub fn fill(&mut self) {
        for w in self.data.iter_mut() {
            *w = W::ONES;
        }
    }

    /// Returns the number of bits set.
    pub fn count(&self) -> usize {
        self.data.iter().map(|w| w.count_ones() as usize).sum()
    }

    /// Returns whether the set is empty.
    pub fn is_empty(&self) -> bool {
        self.data.iter().all(|&w| w == W::ZERO)
    }

    /// Returns whether the set is full.
    pub fn is_full(&self) -> bool {
        self.data.iter().all(|&w| w == W::ONES)
    }

    /// Returns the smallest bit in the set, or `None` if empty.
    pub fn first_set(&self) -> Option<usize> {
        for (word_index, word) in self.data.iter().enumerate() {
            let tz = word.trailing_zeros();
            if (tz as usize) < W::BITS {
                return Some(W::BITS * word_index + tz as usize);
            }
        }
        None
    }

    /// Returns the largest bit in the set, or `None` if empty.
    pub fn last_set(&self) -> Option<usize> {
        for (word_index, word) in self.data.iter().enumerate().rev() {
            let lz = word.leading_zeros();
            if (lz as usize) < W::BITS {
                let local_pos = W::BITS - 1 - lz as usize;
                return Some(W::BITS * word_index + local_pos);
            }
        }
        None
    }

    /// Returns the smallest bit strictly greater than `bit`, or `None`.
    pub fn first_set_after(&self, bit: usize) -> Option<usize> {
        let boundary_index = bit / W::BITS;
        let bit_index = (bit % W::BITS) as u32;

        if boundary_index >= WORDS {
            return None;
        }

        let mask = W::ONES.checked_shl(bit_index + 1).unwrap_or(W::ZERO);
        let masked = self.data[boundary_index] & mask;
        let tz = masked.trailing_zeros();
        if (tz as usize) < W::BITS {
            return Some(W::BITS * boundary_index + tz as usize);
        }

        for word_index in (boundary_index + 1)..WORDS {
            let tz = self.data[word_index].trailing_zeros();
            if (tz as usize) < W::BITS {
                return Some(W::BITS * word_index + tz as usize);
            }
        }

        None
    }

    /// Returns the largest bit strictly less than `bit`, or `None`.
    pub fn last_set_before(&self, bit: usize) -> Option<usize> {
        let boundary_index = (bit / W::BITS).min(WORDS);
        let bit_index = (bit % W::BITS) as u32;

        if boundary_index < WORDS {
            let mask = W::ONES
                .checked_shr(W::BITS as u32 - bit_index)
                .unwrap_or(W::ZERO);
            let masked = self.data[boundary_index] & mask;
            let lz = masked.leading_zeros();
            if (lz as usize) < W::BITS {
                let local_pos = W::BITS - 1 - lz as usize;
                return Some(W::BITS * boundary_index + local_pos);
            }
        }

        for word_index in (0..boundary_index).rev() {
            let lz = self.data[word_index].leading_zeros();
            if (lz as usize) < W::BITS {
                let local_pos = W::BITS - 1 - lz as usize;
                return Some(W::BITS * word_index + local_pos);
            }
        }

        None
    }

    /// Returns the smallest bit _not_ in the set, or `None` if full.
    pub fn first_unset(&self) -> Option<usize> {
        for (word_index, word) in self.data.iter().enumerate() {
            let tz = (!*word).trailing_zeros();
            if (tz as usize) < W::BITS {
                return Some(W::BITS * word_index + tz as usize);
            }
        }
        None
    }

    /// Returns the largest bit _not_ in the set, or `None` if full.
    pub fn last_unset(&self) -> Option<usize> {
        for (word_index, word) in self.data.iter().enumerate().rev() {
            let lz = (!*word).leading_zeros();
            if (lz as usize) < W::BITS {
                let local_pos = W::BITS - 1 - lz as usize;
                return Some(W::BITS * word_index + local_pos);
            }
        }
        None
    }

    /// Returns the smallest bit _not_ in the set, strictly greater than `bit`.
    pub fn first_unset_after(&self, bit: usize) -> Option<usize> {
        let boundary_index = bit / W::BITS;
        let bit_index = (bit % W::BITS) as u32;

        if boundary_index >= WORDS {
            return None;
        }

        let mask = W::ONES.checked_shl(bit_index + 1).unwrap_or(W::ZERO);
        let masked = (!self.data[boundary_index]) & mask;
        let tz = masked.trailing_zeros();
        if (tz as usize) < W::BITS {
            return Some(W::BITS * boundary_index + tz as usize);
        }

        for word_index in (boundary_index + 1)..WORDS {
            let tz = (!self.data[word_index]).trailing_zeros();
            if (tz as usize) < W::BITS {
                return Some(W::BITS * word_index + tz as usize);
            }
        }

        None
    }

    /// Returns the largest bit _not_ in the set, strictly less than `bit`.
    pub fn last_unset_before(&self, bit: usize) -> Option<usize> {
        let boundary_index = (bit / W::BITS).min(WORDS);
        let bit_index = (bit % W::BITS) as u32;

        if boundary_index < WORDS {
            let mask = W::ONES
                .checked_shr(W::BITS as u32 - bit_index)
                .unwrap_or(W::ZERO);
            let masked = (!self.data[boundary_index]) & mask;
            let lz = masked.leading_zeros();
            if (lz as usize) < W::BITS {
                let local_pos = W::BITS - 1 - lz as usize;
                return Some(W::BITS * boundary_index + local_pos);
            }
        }

        for word_index in (0..boundary_index).rev() {
            let lz = (!self.data[word_index]).leading_zeros();
            if (lz as usize) < W::BITS {
                let local_pos = W::BITS - 1 - lz as usize;
                return Some(W::BITS * word_index + local_pos);
            }
        }

        None
    }

    /// Returns a read-only view of the underlying words.
    pub fn words(&self) -> &[W] {
        &self.data
    }

    /// Returns a mutable view of the underlying words.
    pub fn words_mut(&mut self) -> &mut [W] {
        &mut self.data
    }

    /// Returns an iterator over the set bits, in ascending order.
    pub fn iter(&self) -> StaticBitSetIterator<'_, W, WORDS> {
        StaticBitSetIterator {
            bitset: self,
            prev: None,
        }
    }
}

macro_rules! impl_word {
    ($($t:ty),* $(,)?) => {
        $(
            impl Word for $t {
                const BITS: usize = <$t>::BITS as usize;
                const ZERO: Self = 0;
                const ONE: Self = 1;
                const ONES: Self = !0;

                #[inline] fn count_ones(self) -> u32 { <$t>::count_ones(self) }
                #[inline] fn trailing_zeros(self) -> u32 { <$t>::trailing_zeros(self) }
                #[inline] fn leading_zeros(self) -> u32 { <$t>::leading_zeros(self) }
                #[inline] fn checked_shl(self, n: u32) -> Option<Self> { <$t>::checked_shl(self, n) }
                #[inline] fn checked_shr(self, n: u32) -> Option<Self> { <$t>::checked_shr(self, n) }
            }
        )*
    };
}

impl_word!(u8, u16, u32, u64, u128, usize);

impl<'a, W: Word, const WORDS: usize> Iterator
    for StaticBitSetIterator<'a, W, WORDS>
{
    type Item = usize;

    fn next(&mut self) -> Option<usize> {
        let next_bit = match self.prev {
            None => self.bitset.first_set(),
            Some(prev) => self.bitset.first_set_after(prev),
        };
        self.prev = next_bit;
        next_bit
    }
}
impl<'a, W: Word, const WORDS: usize> IntoIterator
    for &'a StaticBitSet<W, WORDS>
{
    type Item = usize;
    type IntoIter = StaticBitSetIterator<'a, W, WORDS>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<W: Word, const WORDS: usize> Default for StaticBitSet<W, WORDS> {
    fn default() -> Self {
        Self::new()
    }
}

impl<W: Word, const WORDS: usize> BitOr for &StaticBitSet<W, WORDS> {
    type Output = StaticBitSet<W, WORDS>;
    fn bitor(self, rhs: Self) -> Self::Output {
        let mut result = StaticBitSet::new();
        for i in 0..WORDS {
            result.data[i] = self.data[i] | rhs.data[i];
        }
        result
    }
}
impl<W: Word, const WORDS: usize> BitAnd for &StaticBitSet<W, WORDS> {
    type Output = StaticBitSet<W, WORDS>;
    fn bitand(self, rhs: Self) -> Self::Output {
        let mut result = StaticBitSet::new();
        for i in 0..WORDS {
            result.data[i] = self.data[i] & rhs.data[i];
        }
        result
    }
}
impl<W: Word, const WORDS: usize> BitXor for &StaticBitSet<W, WORDS> {
    type Output = StaticBitSet<W, WORDS>;
    fn bitxor(self, rhs: Self) -> Self::Output {
        let mut result = StaticBitSet::new();
        for i in 0..WORDS {
            result.data[i] = self.data[i] ^ rhs.data[i];
        }
        result
    }
}
impl<W: Word, const WORDS: usize> Not for &StaticBitSet<W, WORDS> {
    type Output = StaticBitSet<W, WORDS>;
    fn not(self) -> Self::Output {
        let mut result = StaticBitSet::new();
        for i in 0..WORDS {
            result.data[i] = !self.data[i];
        }
        result
    }
}

impl<W: Word, const WORDS: usize> BitOrAssign<&Self>
    for StaticBitSet<W, WORDS>
{
    fn bitor_assign(&mut self, rhs: &Self) {
        for i in 0..WORDS {
            self.data[i] |= rhs.data[i];
        }
    }
}
impl<W: Word, const WORDS: usize> BitAndAssign<&Self>
    for StaticBitSet<W, WORDS>
{
    fn bitand_assign(&mut self, rhs: &Self) {
        for i in 0..WORDS {
            self.data[i] &= rhs.data[i];
        }
    }
}
impl<W: Word, const WORDS: usize> BitXorAssign<&Self>
    for StaticBitSet<W, WORDS>
{
    fn bitxor_assign(&mut self, rhs: &Self) {
        for i in 0..WORDS {
            self.data[i] ^= rhs.data[i];
        }
    }
}

impl<W: Word, const WORDS: usize> BitOr for StaticBitSet<W, WORDS> {
    type Output = StaticBitSet<W, WORDS>;
    fn bitor(self, rhs: Self) -> Self::Output {
        &self | &rhs
    }
}
impl<W: Word, const WORDS: usize> BitAnd for StaticBitSet<W, WORDS> {
    type Output = StaticBitSet<W, WORDS>;
    fn bitand(self, rhs: Self) -> Self::Output {
        &self & &rhs
    }
}
impl<W: Word, const WORDS: usize> BitXor for StaticBitSet<W, WORDS> {
    type Output = StaticBitSet<W, WORDS>;
    fn bitxor(self, rhs: Self) -> Self::Output {
        &self ^ &rhs
    }
}
impl<W: Word, const WORDS: usize> Not for StaticBitSet<W, WORDS> {
    type Output = StaticBitSet<W, WORDS>;
    fn not(self) -> Self::Output {
        !&self
    }
}

impl<W: Word, const WORDS: usize> BitOrAssign for StaticBitSet<W, WORDS> {
    fn bitor_assign(&mut self, rhs: Self) {
        *self |= &rhs;
    }
}
impl<W: Word, const WORDS: usize> BitAndAssign for StaticBitSet<W, WORDS> {
    fn bitand_assign(&mut self, rhs: Self) {
        *self &= &rhs;
    }
}
impl<W: Word, const WORDS: usize> BitXorAssign for StaticBitSet<W, WORDS> {
    fn bitxor_assign(&mut self, rhs: Self) {
        *self ^= &rhs;
    }
}
```
