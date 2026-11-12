# Introduction to Operating Systems

## Definition

> [!DEFINITION]
> An **operating system** is system software that manages a computer's hardware resources (CPU, memory, storage, and I/O devices) by allocating them efficiently and safely among competing programs, while presenting users and applications with a simplified, consistent interface that hides the hardware's complexity.

For instance, it creates it provides the following abstractions: 

| Hardware | Abstraction |
| :---: | :---: |
| Processor | Thread |
| Memory | Address Space |
| Storage | Files |
| Networks | Sockets |
| Machines | Processes |

## POSIX, UNIX, Unix-like

Operating systems differ in how they implement the same basic functionality. A program that depends on one system's file handling, process creation, or shell behavior may break on another. **POSIX** is a set of IEEE standards that defines a common interface for system calls, the shell, and core utilities, so software written to it can be ported across systems with little or no change. But, conforming to POSIX is entirely optional. Linux and most BSDs follow it closely, but haven't paid for formal certification, which is expensive and gives most vendors little in return. **UNIX** is a trademark owned by The Open Group, granted to systems that pass the Single UNIX Specification test suite, which covers all of POSIX, plus additional interfaces. macOS holds this certification, while Linux is usually called **Unix-like**.

## Kernel

> [!DEFINITION]
> The operating system's core program that always runs is called the **kernel**.

The kernel communicates to the hardware directly, or through systems programs called **device drivers**.

## Privileges

The hardware is organized in terms of protection rings. It has at least two protection rings:
- Ring 0: Kernel/supervisor/priviledged mode
  - full access to all hardware
  - the kernel is the systems program that runs here
- Ring 3: User mode
  - restricts access on which instructions can be executed and which memory can be accessed.
  - regular applications programs run here

```tikz
\begin{tikzpicture}
  % Darker rings are more privileged.
  \foreach \r/\o in {4.2/0.04, 3.1/0.1, 2/0.2, 1/0.32} \draw[fill=black, fill opacity=\o] (0, 0) circle (\r);
  \foreach \y/\ring/\who in {3.6/3/Applications, 2.55/2/Device drivers, 1.5/1/Device drivers, 0.3/0/Kernel} {
    \node[font=\small] at (0, \y) {Ring \ring};
    \node[font=\small] at (0, -\y) {\who};
  }
  % Legend
  \node[font=\small] at (5.8, 2.6) {Least privileged};
  % Each swatch uses the combined opacity of the nested circles over that ring.
  \foreach \o [count=\k] in {0.04, 0.136, 0.309, 0.53} \draw[fill=black, fill opacity=\o] (5.45, 2.3 - 0.7*\k) rectangle ++(0.7, 0.7);
  \node[font=\small] at (5.8, -0.75) {Most privileged};
\end{tikzpicture}
```

This is enforced by a special register or a flag in the CPU, called a **mode bit**, that tracks which mode it's currently in. 
- `mode bit = 0`: kernel mode.
- `mode bit = 1`: user mode.

## System Call

In order for applications programs to perform privilege operations, it must request the kernel to carry it out on its behalf, and waits for the result. We call this request a **system call**. 

System calls vary in terms of the specific operation being requested (e.g., reading a file, writing to a socket, allocating memory, creating a process, etc.) and the kernel keeps track of and distinguishes between them by assigning each one a unique integer ID called a syscall number.

A system call work as follows:

1. The application program calls a specific system call via a small wrapper function (usually in libc). This wrapper function will:
   - Move the syscall number and the syscall arguments each into registers
   - Execute the `syscall` instruction, triggering a **trap**: a synchronous exception, reproducible at exactly the same point in the code on re-run.
3. The trap instruction will them perform two actions atomically:
    - Switch the CPU's privilege level from ring 3 to ring 0 by flipping the mode bit from 1 to 0
    - Jump to a fixed, kernel-controlled entry point
4. The kernel looks at a syscall number and arguments, and performs the operation itself.
5. The kernel executes a return-from-trap instruction which switches the CPU back to ring 3 and resumes the application program.

```tikz
\begin{tikzpicture}[box/.style={draw, rounded corners=3pt, minimum height=0.9cm, inner xsep=10pt}]
  \node[box] (exec) at (5, 0) {User process executing};
  \node[box] (call) at (5, -1.8) {Calls system call};
  \node[box] (sys) at (0, -3.6) {Execute system call};
  \node[box] (ret) at (5, -7.2) {Return from system call};
  \draw[->] (exec) -- (call);
  \draw[->] (call) |- node[pos=0.25, right, font=\small] {trap} (sys);
  \draw[->] (sys) |- node[pos=0.75, above, font=\small] {return} (5, -5.4) -- (ret);
  % Mode regions
  \draw[thin] (2.7, 1) rectangle (7.3, -8);
  \node[font=\small] at (5, 0.75) {User mode (mode bit $= 1$)};
  \draw[thin, fill=black, fill opacity=0.06] (-2.4, -2.6) rectangle (2.4, -4.6);
  \node[font=\small] at (0, -2.85) {Kernel mode (mode bit $= 0$)};
\end{tikzpicture}
```

## Kernel Architecture

Most kernels fall into one of two designs, depending on how much of the operating system runs inside the kernel itself.

> [!DEFINITION]
> A **monolithic kernel** runs the kernel and all of its services, such as filesystems, device drivers and the networking stack, as a single program in kernel mode.

> [!DEFINITION]
> A **microkernel** keeps only the bare minimum inside the kernel: scheduling, memory management and paging, and interprocess communication (IPC). Every other service runs as a separate program in user mode.

The pieces a microkernel keeps are the ones that every higher-level feature depends on. A filesystem needs to be scheduled, needs memory, and needs a way to talk to the programs that use it, so those three things have to exist before anything else can be built. The services built on top, like the networking stack, filesystems and device drivers, then run as ordinary programs that talk to the kernel and to each other through IPC, usually in the form of remote procedure calls (RPC).

In a monolithic kernel, a program that reads a file makes one system call, and the kernel calls into the filesystem code directly. In a microkernel, the same read becomes a message from the program to the kernel, which passes it on to the filesystem process, which sends its reply back through the kernel. Each of those hops is a switch between processes, so microkernels have traditionally been slower than monolithic kernels because of this IPC overhead.


