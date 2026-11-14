# Assembly

> [!NOTE]
> AT&T (a.k.a. GNU Assembler) assembly's syntax, emitted by GCC and LLVM, has a few differences:
> - Instructions are formated as `<instruction> <source> <destination>`.
> - Immediate values and registers are prefixed with sigils (`$` and `%`, respectively)
> - Instructions are _suffixed_ with a letter to indicate the byte size they operate on:
>    - `<...>b`: applies to a 1 byte value
>    - `<...>w`: applies to a 2 byte value
>    - `<...>l`: applies to a 4 byte value
>    - `<...>q`: applies to an 8 byte value

## Code 

### Instruction Layout

```
<instruction> <destination> <source>
```

> [!NOTE]
> The convention is indenting instructions by 4 spaces.

### Data Layout

```
<label> <directive> <content>
```

> [!NOTE]
> Data is placed back-to-back with no [padding](data-representation.md#padding), so aligning it is up to you. The `align <N>` directive pads with bytes until the next address is a multiple of `N` (see [Alignment](data-representation.md#alignment)). The stack must also be 16-byte aligned at every `call` (System V).
>
> ```asm
> section .data
>     flag  db 1
>     align 8
>     count dq 0 ; 8-byte aligned, after 7 bytes of padding
> ```

## File Sections 

```asm
section .data
    ; this section is optional

section .text
    global _start 

_start:
    ; ...
```

## Comments

```asm
; this is a comment
```

## Labels

- Declare a label: `label:`
- Reference a label: `label`

## Syscalls

1. Select the system call to invoke by moving its identifier in `rax`
2. Pass arguments to the syscall by populating appropriate registers
3. Use the `syscall` instruction to fire the system call

## Registers

| Register | Higher byte | Lower byte | Lower 2 bytes¹ | Lower 4 bytes² | Description |
| :---: | :---: | :---: | :---: | :---: | :---: |
| `rax` | `ah` | `al` | `ax` | `eax` | General purpose; conventionally accumulator, used for return values and arithmetic |
| `rcx` | `ch` | `cl` | `cx` | `ecx` | General purpose; conventionally counter, used in loops and shift/rotate (implicit in `shl`/`shr`) |
| `rbx` | `bh` | `bl` | `bx` | `ebx` | General purpose; callee-saved by convention |
| `rdx` | `dh` | `dl` | `dx` | `edx` | General purpose; used with `rax` in `mul`/`div`, and for I/O |
| `rsp` |  | `spl` | `sp` | `esp` | **Special**: stack pointer, used implicitly by `push`/`pop`/`call`/`ret` |
| `rsi` |  | `sil` | `si` | `esi` | General purpose; conventionally source index for string ops, 2nd integer arg (System V) |
| `rdi` |  | `dil` | `di` | `edi` | General purpose; conventionally destination index for string ops, 1st integer arg (System V) |
| `rbp` |  | `bpl` | `bp` | `ebp` | General purpose; conventionally frame/base pointer |
| `r8` |  | `r8b` | `r8w` | `r8d` | General purpose; 5th integer arg (System V) |
| `r9` |  | `r9b` | `r9w` | `r9d` | General purpose; 6th integer arg (System V) |
| `r10` |  | `r10b` | `r10w` | `r10d` | General purpose; often used as a scratch register |
| `r11` |  | `r11b` | `r11w` | `r11d` | General purpose; often used as a scratch register |
| `r12` |  | `r12b` | `r12w` | `r12d` | General purpose; callee-saved by convention |
| `r13` |  | `r13b` | `r13w` | `r13d` | General purpose; callee-saved by convention |
| `r14` |  | `r14b` | `r14w` | `r14d` | General purpose; callee-saved by convention |
| `r15` |  | `r15b` | `r15w` | `r15d` | General purpose; callee-saved by convention |
| `rip` |  |  |  | `eip` | **Special**: instruction pointer, holds address of next instruction; not directly accessible as an operand in most instructions |

## Bitwise Instructions

Each [bitwise operator](data-representation.md#bitwise-operations) is a single instruction, which stores its result in the destination operand.

| Instruction | C equivalent | Description |
| :---: | :---: | :---: |
| `and rax, rbx` | `rax &= rbx` | Bitwise AND |
| `or rax, rbx` | `rax \|= rbx` | Bitwise OR |
| `xor rax, rbx` | `rax ^= rbx` | Bitwise XOR |
| `not rax` | `rax = ~rax` | Bitwise NOT |
| `shl rax, k` | `rax <<= k` | Left shift |
| `shr rax, k` | `rax >>= k` (unsigned) | Logical right shift, fills with `0`s |
| `sar rax, k` | `rax >>= k` (signed) | Arithmetic right shift, fills with copies of the sign bit |
| `rol rax, k` / `ror rax, k` | | [Rotate](data-representation.md#rotates-bits) left / right |
| `popcnt rax, rbx` | `rax = stdc_count_ones(rbx)` | [Count set bits](data-representation.md#count-the-number-of-set-bits) |
| `lzcnt rax, rbx` / `tzcnt rax, rbx` | `rax = stdc_leading_zeros(rbx)` / `stdc_trailing_zeros(rbx)` | Count [leading](data-representation.md#count-the-amount-of-leading-zeroes) / [trailing](data-representation.md#count-the-amount-of-trailing-zeroes) zeroes |
| `bswap rax` | | Reverse the bytes, converting between [byte orders](data-representation.md#byte-order) |

> [!NOTE]
> The shift amount `k` is either an immediate or the `cl` register (e.g., `shl rax, cl`), and only its low 6 bits are used (5 bits for 32-bit operands). Shifting by the full width is therefore well-defined here, unlike in [C](../../tools/c.md#bitwise).

> [!TIP]
> `xor rax, rax` is the conventional way to set a register to `0`, since `x ^ x = 0` and the instruction is shorter than `mov rax, 0`.

## Hello, World!

```asm
section .data
    ; We define the constant `msg`, the string to print.
    ; We use the `db` (define byte) directive to define 
    ; constants of one or more bytes.
    msg db `Hello, World!\n`
    ; ...
    len equ $ - msg

section .text
    global _start

_start:
    ; We call the `sys_write` syscall to print on the 
    ; screen. Syscalls  are invoked with `syscall` and
    ; they are identified by a number.
    ; The identifier for sys_write is 1. The `syscall`
    ; intruction will look for the instruction id in the
    ; register `rax`. So we move 1 in there.
    mov rax, 1
    ; The system call to invoke has the signature:
    ;   `size_t sys_write(uint fd, const char* buf, size_t count)`
    ; The `syscall` instruction wants the first argument
    ; in `rdi`. In this case, the first argument is the
    ; file descriptor of where to write the output.
    ; We will use 1, which identifies the standard output.
    mov rdi, 1
    ; The second argument is expected to be in `rsi` and
    ; the signature tells us that it the string we want
    ; to print. We defined the buffer in the .data section
    ; (it's `msg`), so we just need to move it to `rsi`
    mov rsi, msg
    ; `rdx` is the register for the third argument. 
    ; We see in the signature that this is the count of
    ; characters we want to print.
    ; NOTE: the string is null-terminated, that is, there
    ; is an ending "ghost" character we need to account for.
    mov rdx, len
    ; We're now finally issuing the syscall
    syscall
  
    
    ; Time to exit the program.
    ; Same as above, we invoke `syscall` with 60, which is
    ; `exit` and has the following signature:
    ;   `void exit(int status)`
    ; Once again, `syscall` looks for the identifier in 
    ; `rax`. We move 60, the identifier for `exit`, there
    mov rax, 60
    ; Again, `rdi` is where the first argument is expected
    ; to be found.
    ; The first argument is the status code (see signature)
    ; so we put 0 for a clean exit.
    mov rdi, 0
    ; Issuing the `exit` syscall and cleanly exiting 
    ; the program
    syscall
```


