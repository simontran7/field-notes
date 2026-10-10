# Math

## Prime Numbers

### Primality Check

#### Use Case

Check whether a number $n$ is a prime number or not.

#### Idea

If $n = a \cdot b$ with $a \le b$, then $a \le \sqrt{n}$. So, if there isn't an integer $k$ such that $2 \le k \le \sqrt{n}$, and $k$ divides $n$, then $n$ must be prime.

#### Code

```python
from math import isqrt


def is_prime(n):
    if n < 2:
        return False
    for k in range(2, isqrt(n) + 1):
        if n % k == 0:
            return False
    return True
```

### Sieve of Eratosthenes

TO LEARN

## Modular Arithmetic

### Use Case

Algorithms that involve `x mod m`, (`m` is typically $10^9 + 7$), where:

- Building `x` is expensive (repeated work), or
- `x` becomes too large

### Theorem

> [!FORMULA] Congruence modulo $m$
> $$
> \begin{aligned}
> a \equiv b \pmod{m}
> &\iff m \mid (a-b) \\
> &\iff a-b=km,\quad k\in\mathbb{Z}
> \end{aligned}
> $$
>
> In other words, $a$ and $b$ have the same remainder when divided by $m$.

### Properties

> [!FORMULA] Modular arithmetic properties
> We get the following properties from the above:
> $$
> \begin{aligned}
> (a+b)\bmod m &= \big((a\bmod m)+(b\bmod m)\big)\bmod m \\
> (a-b)\bmod m &= \big((a\bmod m)-(b\bmod m)\big)\bmod m \\
> (a \cdot b)\bmod m &= \big((a\bmod m)(b\bmod m)\big)\bmod m \\
> a^k \bmod m &= \big((a \bmod m)^k\big) \bmod m
> \end{aligned}
> $$

### Templates

#### Modular Sum

```python
def mod_sum(nums, mod):
    result = 0

    for x in nums:
        result = (result + x) % mod

    return result
```

#### Modular Subtraction

```python
def mod_sub(nums, mod):
    result = nums[0] % mod

    for x in nums[1:]:
        result = (result - x) % mod

    return result
```

#### Modular Product

```python
def mod_product(nums, mod):
    result = 1

    for x in nums:
        result = (result * x) % mod

    return result
```

#### Fast Modular Exponentiation

```python
def mod_exponentiation(base, exp, mod):
    result = 1
    base = base % mod

    while exp > 0:
        if exp % 2 == 1:  # odd exponent
            result = (result * base) % mod
        exp = exp >> 1  # divide by 2
        base = (base * base) % mod

    return result
```

#### Modular Number Construction

```python
def mod_subdigits(word, m):
    remainder = 0
    result = []

    for digit_char in word:
        digit = int(digit_char)
        remainder = (remainder * 10 + digit) % m

        if remainder == 0:
            result.append(1)  # divisible
        else:
            result.append(0)

    return result
```

#### Prefix Remainder

```python
def prefix_remainders(nums, mod):
    remainder = 0
    result = []

    for x in nums:
        remainder = (remainder + x) % mod
        result.append(remainder)

    return result
```
