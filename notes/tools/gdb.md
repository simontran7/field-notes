# GDB

> [!IMPORTANT]
> You need to compile your program with debug symbols.

- Start GDB: `gdb <file>`
- Start (▶️): `run`
- Restart (🔄): `run`
- Set a breakpoint (🔴): `break <file>:<line number>`
- Step Over: `next`
- Step Into (⬇): `step`
- Step Out (⬆): `finish`
- Continue: `continue`
- Hover over a variable: `print <variable>`
- Variable panel: `display <variable>` (add once, and it'll update every step)
- Call Stack panel: `backtrace`
- Watch panel: `display <expression>`
- Stop GDB: `quit`