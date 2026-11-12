# Recursion

## Introduction

> [!DEFINITION]
> **Recursion** is a mechanism in which a problem is solved by defining a solution in terms of smaller instances of the same problem.

We can implement a recursive algorithm by modelling it as a **recurrence relation**:
- **Base Case(s)**: A simple occurrence that can be answered directly.
- **Recursive Case(s)**: A more complex occurrence of the problem that cannot be directly answered but can instead be described in terms of smaller occurrences of the same problem.

A recursive algorithm is implemented as a function that calls itself. Each call works on a simpler subproblem than the one before, and to guarantee termination, every recursive call must make progress toward a base case: a subproblem simple enough to solve directly, without recursing further. Some recursive algorithms have more than one base case or recursive case, but every recursive algorithm must have at least one of each.

What makes this work is the call stack, a region of memory that keeps track of the active function calls in a running thread. Each time a function is called, including when it calls itself, the program creates a new stack frame and pushes it onto the top of the call stack. The frame stores the function's parameters, local variables, and return address (the point in the calling code where execution resumes once the call completes). 

```tikz
\begin{tikzpicture}[frame/.style={draw, minimum width=6.8cm, minimum height=0.7cm, font=\ttfamily\small},
    code/.style={draw, font=\ttfamily\small, inner sep=6pt}, small/.style={font=\small}]
  \node[anchor=west, font=\large] at (0, 1.1) {Anatomy of a stack frame};
  % The frame on top of the stack, with its three parts.
  \draw (0, 0.4) rectangle (6.8, -3);
  \node[anchor=west, font=\ttfamily] at (0.2, 0) {factorial(2)};
  \node[anchor=east, small] at (6.6, 0) {top of stack};
  \foreach \y/\part/\value in {-0.8/Parameters/{n = 2}, -1.6/Local variables/{sub = (unassigned)},
      -2.4/Return address/{factorial(3), line 3}} {
    \draw[thin] (0.2, \y + 0.4) -- (6.6, \y + 0.4);
    \node[anchor=west, small] at (0.2, \y) {\part};
    \node[anchor=west, font=\ttfamily\small] (v) at (2.6, \y) {\value};
  }
  \coordinate (ret) at (6.8, -2.4);
  \node[frame, anchor=north west] at (0, -3.15) {factorial(3)};
  \node[frame, anchor=north west] at (0, -3.95) {main()};
  \draw (-0.2, -4.75) -- (7, -4.75);

  % The return address points back into the caller's code.
  \node[code, anchor=north west] (code) at (8.2, 0.4) {%
    \begin{tabular}{@{}r@{\quad}l@{}}
      1 & def factorial(n): \\
      2 & \ \ if n == 0: return 1 \\
      3 & \ \ sub = factorial(n - 1) \\
      4 & \ \ return n * sub
    \end{tabular}};
  \fill[black, fill opacity=0.12] ([yshift=-0.99cm]code.north west) rectangle ([yshift=-1.38cm]code.north east);
  \draw[->] (ret) to[out=0, in=180] ([yshift=-1.185cm]code.north west);
  \node[anchor=north west, small, text width=5.6cm] at ([yshift=-0.3cm]code.south west)
    {When \texttt{factorial(2)} returns, \texttt{factorial(3)} resumes at line 3 and assigns the result to \texttt{sub}.};
\end{tikzpicture}
```

The instruction pointer then jumps to the first instruction of the called function, and the new stack frame becomes the active one. This means that whenever the function reads or writes its parameters and local variables, it will use the copies stored in the active frame. Because every recursive call gets its own stack frame, each call has its own copies of these variables, so the calls don't interfere with one another. 

```tikz
\usepackage{amssymb}
\begin{tikzpicture}[frame/.style={draw, minimum width=6.2cm, minimum height=0.7cm, font=\ttfamily\small},
    code/.style={draw, font=\ttfamily\small, inner sep=6pt}, small/.style={font=\small}]
  \node[anchor=west, font=\large] at (0, 1.1) {Entering \texttt{factorial(2)}};
  \node[code, anchor=north west] (code) at (0, 0.4) {%
    \begin{tabular}{@{}r@{\quad}l@{}}
      1 & def factorial(n): \\
      2 & \ \ if n == 0: return 1 \\
      3 & \ \ sub = factorial(n - 1) \\
      4 & \ \ return n * sub
    \end{tabular}};
  % Line 2 is where the instruction pointer jumps to; line 3 (dashed) made the call.
  \fill[black, fill opacity=0.12] ([yshift=-0.6cm]code.north west) rectangle ([yshift=-0.99cm]code.north east);
  \draw[thin, dashed] ([yshift=-0.99cm]code.north west) rectangle ([yshift=-1.38cm]code.north east);
  \node[small] at ([shift={(-0.25, -0.795)}]code.north west) {$\blacktriangleright$};
  \draw[->] ([shift={(-0.1, -1.185)}]code.north east) to[out=0, in=0, looseness=2] ([shift={(-0.1, -0.795)}]code.north east);
  \node[anchor=north west, small, text width=5.8cm] at ([yshift=-0.3cm]code.south west)
    {$\blacktriangleright$ Instruction pointer. The call on line 3 (dashed) makes it jump to the first instruction, line 2.\\[4pt]
     While \texttt{factorial(2)} runs, every read or write of \texttt{n} and \texttt{sub} uses the copies in the active frame.
     The copies in \texttt{factorial(3)}'s frame stay untouched until it resumes.};

  % The stack: the active frame on top, the caller's frame below it.
  \foreach \y/\name/\note/\n/\s in {0.4/factorial(2)/active frame/2/hl, -2.2/factorial(3)/{inactive, untouched}/3/} {
    \draw[\s] (7.4, \y) rectangle (13.6, \y - 2.4);
    \node[anchor=west, font=\ttfamily] at (7.6, \y - 0.4) {\name};
    \node[anchor=east, small] at (13.4, \y - 0.4) {\note};
    \draw[thin] (7.6, \y - 0.8) -- (13.4, \y - 0.8);
    \node[anchor=west, small] at (7.6, \y - 1.2) {Parameter};
    \node[anchor=west, font=\ttfamily\small] at (9.8, \y - 1.2) {n = \n};
    \draw[thin] (7.6, \y - 1.6) -- (13.4, \y - 1.6);
    \node[anchor=west, small] at (7.6, \y - 2) {Local variable};
    \node[anchor=west, font=\ttfamily\small] at (9.8, \y - 2) {sub = (unassigned)};
  }
  \node[frame, anchor=north west] at (7.4, -4.75) {main()};
  \draw (7.2, -5.55) -- (13.8, -5.55);
  \draw[->] ([shift={(0.3, -0.795)}]code.north east) to[out=0, in=180] node[above, small, pos=0.4] {reads \texttt{n}} (7.4, -0.8);
\end{tikzpicture}
```

Stack frames keep piling up until a call's input satisfies the base case. 

```tikz
\begin{tikzpicture}[x=1cm, y=0.75cm, frame/.style={draw, minimum width=2.7cm, minimum height=0.6cm, font=\ttfamily\small},
    base/.style={hl, line width=1.6pt}, small/.style={font=\small}]
  % \stack{x}{frame/style, ... from the bottom}{caption}: one snapshot of the call stack
  \newcommand{\stack}[3]{
    \foreach \f/\s [count=\j from 0] in {#2} \node[frame, \s] at (#1, \j) {\f};
    \draw (#1 - 1.5, -0.6) -- (#1 + 1.5, -0.6);
    \node[align=center, small] at (#1, -1.4) {#3};
  }
  \node[anchor=west, font=\large] at (-1.35, 6) {Call stack growth: \texttt{factorial(3)}};
  \stack{0}{main()/hl}{Step 1\\\texttt{main()} starts}
  \stack{3.4}{main()/, factorial(3)/hl}{Step 2\\calls \texttt{factorial(3)}}
  \stack{6.8}{main()/, factorial(3)/, factorial(2)/hl}{Step 3\\calls \texttt{factorial(2)}}
  \stack{10.2}{main()/, factorial(3)/, factorial(2)/, factorial(1)/hl}{Step 4\\calls \texttt{factorial(1)}}
  \stack{13.6}{main()/, factorial(3)/, factorial(2)/, factorial(1)/, factorial(0)/base}{Step 5\\base case reached}
  \node at (1.7, 0) {$\rightarrow$};
  \node at (5.1, 0) {$\rightarrow$};
  \node at (8.5, 0) {$\rightarrow$};
  \node at (11.9, 0) {$\rightarrow$};
  \node[frame, hl, minimum width=0.3cm, minimum height=0.3cm, label=right:{\small currently executing}] at (-1.2, -2.6) {};
  \node[frame, base, minimum width=0.3cm, minimum height=0.3cm, label=right:{\small base case}] at (-1.2+4, -2.6) {};
  \node[frame, minimum width=0.3cm, minimum height=0.3cm, label=right:{\small waiting for a call to return}] at (-1.2+6.6, -2.6) {};
\end{tikzpicture}
```

That call returns without recursing further, and the stack begins to unwind. Each return pops a stack frame off the top of the stack, and control passes back to the caller, along with any return value, while the caller's frame becomes active again. Unwinding continues until control returns to the original call.

```tikz
\begin{tikzpicture}[x=1cm, y=0.75cm, frame/.style={draw, minimum width=2.7cm, minimum height=0.6cm, font=\ttfamily\small},
    base/.style={hl, line width=1.6pt}, small/.style={font=\small}]
  % \stack{x}{frame/style, ... from the bottom}{caption}: one snapshot of the call stack
  \newcommand{\stack}[3]{
    \foreach \f/\s [count=\j from 0] in {#2} \node[frame, \s] at (#1, \j) {\f};
    \draw (#1 - 1.5, -0.6) -- (#1 + 1.5, -0.6);
    \node[align=center, small] at (#1, -1.4) {#3};
  }
  \node[anchor=west, font=\large] at (-1.35, 6) {Call stack unwinding: \texttt{factorial(3)}};
  \stack{0}{main()/, factorial(3)/, factorial(2)/, factorial(1)/, factorial(0)/base}{Step 6\\\texttt{factorial(0)} returns}
  \stack{3.4}{main()/, factorial(3)/, factorial(2)/, factorial(1)/hl}{Step 7\\\texttt{factorial(1)} returns}
  \stack{6.8}{main()/, factorial(3)/, factorial(2)/hl}{Step 8\\\texttt{factorial(2)} returns}
  \stack{10.2}{main()/, factorial(3)/hl}{Step 9\\\texttt{factorial(3)} returns}
  \stack{13.6}{main()/hl}{Step 10\\back in \texttt{main()}}
  \foreach \x/\y/\note in {0/5/{returns $1$}, 3.4/4/{returns $1 \times 1 = 1$}, 6.8/3/{returns $2 \times 1 = 2$},
      10.2/2/{returns $3 \times 2 = 6$}, 13.6/1/{receives $6$}} \node[small] at (\x, \y - 0.1) {\note};
  \node at (1.7, 0) {$\rightarrow$};
  \node at (5.1, 0) {$\rightarrow$};
  \node at (8.5, 0) {$\rightarrow$};
  \node at (11.9, 0) {$\rightarrow$};
  \node[frame, hl, minimum width=0.3cm, minimum height=0.3cm, label=right:{\small returning frame}] at (-1.2, -2.6) {};
  \node[frame, base, minimum width=0.3cm, minimum height=0.3cm, label=right:{\small base case}] at (-1.2+4, -2.6) {};
  \node[frame, minimum width=0.3cm, minimum height=0.3cm, label=right:{\small waiting for a call to return}] at (-1.2+6.6, -2.6) {};
\end{tikzpicture}
```

## Template

To create recursive algorithms, ask the following questions:
1. What does `f()` mean? What does the current subproblem need to know that it can't figure out itself? These become the function's parameters. What useful information can this subproblem give back to its caller? These become the return values of the function. Do I need to maintain a global `result` variable across multiple recursive calls?
2. What is/are the base case(s)? For trees, it's often when the `root`/`node` is `None`, for arrays and strings, it's either when `i` is at `0` or at `array.count()`.
3.. What smaller version of the exact same problem can I ask recursion to solve? For trees, it's often recursing on the children nodes, for graphs, it's recursing on the neighbour vertices, for arrays, it's recursing the previous or next element, or for an integer $n$, it's recursing on a smaller $n$.
4. How do I use the recursive result(s) to produce the current result?

## From Recursion to Iteration

...

## Tail Recursion 

> [!DEFINITION] Tail recursion
> In a traditional recursive function, calculations happen after the recursive call returns. In a **tail-recursive function**, the recursive call is the absolute _last_ operation executed by the function.

The tail-recursive function lends itself to better memory usage, as in certain languages that support **Tail Call Optimization**, the compiler automatically optimizes tail-recursive functions into loops under the hood. Specifically, instead of adding a new stack frame for every call, it continuously reuses the same stack frame, preventing stack overflow errors. As a consequence, when the function recurses to the base case, the function can simply return the result to the original caller without going back to the previous function calls.

```tikz
\begin{tikzpicture}[x=1cm, y=0.75cm, frame/.style={draw, minimum width=2.7cm, minimum height=0.6cm, font=\ttfamily\small},
    base/.style={hl, line width=1.6pt}, small/.style={font=\small}]
  % \stack{x}{frame/style, ... from the bottom}{caption}: one snapshot of the call stack
  \newcommand{\stack}[3]{
    \foreach \f/\s [count=\j from 0] in {#2} \node[frame, \s] at (#1, \j) {\f};
    \draw (#1 - 1.5, -0.6) -- (#1 + 1.5, -0.6);
    \node[align=center, small] at (#1, -1.4) {#3};
  }
  \node[anchor=west, font=\large] at (-1.35, 6) {Tail-recursive \texttt{factorial(3)}};
  \node[anchor=west, small] at (-1.35, 5.3) {\texttt{factorial(n, acc)}: if \texttt{n == 0}, return \texttt{acc}; otherwise return \texttt{factorial(n - 1, n * acc)}};
  \node[anchor=west] at (-1.35, 4.3) {Without tail call optimization};
  \begin{scope}[yshift=-0.2cm]
    \stack{0}{main()/, {factorial(3, 1)}/hl}{2 frames}
    \stack{3.4}{main()/, {factorial(3, 1)}/, {factorial(2, 3)}/hl}{3 frames}
    \stack{6.8}{main()/, {factorial(3, 1)}/, {factorial(2, 3)}/, {factorial(1, 6)}/hl}{4 frames}
    \stack{10.2}{main()/, {factorial(3, 1)}/, {factorial(2, 3)}/, {factorial(1, 6)}/, {factorial(0, 6)}/base}{5 frames}
    \stack{13.6}{main()/hl}{after 4 returns}
    \node[small] at (13.6, 0.9) {receives $6$};
    \foreach \x in {1.7, 5.1, 8.5, 11.9} \node at (\x, 0) {$\rightarrow$};
  \end{scope}
  \draw[thin] (-1.35, -2.4) -- (15, -2.4);
  \node[anchor=west] at (-1.35, -3.2) {With tail call optimization};
  \begin{scope}[yshift=-4.5cm]
    \stack{0}{main()/, {factorial(3, 1)}/hl}{2 frames}
    \stack{3.4}{main()/, {factorial(2, 3)}/hl}{2 frames}
    \stack{6.8}{main()/, {factorial(1, 6)}/hl}{2 frames}
    \stack{10.2}{main()/, {factorial(0, 6)}/base}{2 frames}
    \stack{13.6}{main()/hl}{after 1 return}
    \foreach \x in {3.4, 6.8, 10.2} \node[small] at (\x, 1.9) {frame reused};
    \node[small] at (13.6, 0.9) {receives $6$};
    \foreach \x in {1.7, 5.1, 8.5, 11.9} \node at (\x, 0) {$\rightarrow$};
  \end{scope}
  \begin{scope}[yshift=-4.6cm]
  \node[frame, hl, minimum width=0.3cm, minimum height=0.3cm, label=right:{\small active frame}] at (-1.2, -2.6) {};
  \node[frame, base, minimum width=0.3cm, minimum height=0.3cm, label=right:{\small base case}] at (-1.2+4, -2.6) {};
  \node[frame, minimum width=0.3cm, minimum height=0.3cm, label=right:{\small waiting for a call to return}] at (-1.2+6.6, -2.6) {};
  \end{scope}
\end{tikzpicture}
```

To turn a standard recursive function into a tail-recursive function, there are two common approaches:
- **Accumulator**: Pass partial results forward using an accumulator argument, so there is no remaining work to perform after the recursive call returns.
- **Continuation (CPS)**: Pass the remaining work forward using a continuation, often named k. The continuation is a function that represents the work still to be performed and effectively models the pending computations of the call stack.

### Accumulator

1. Add an accumulator argument, usually named `acc`, to the recursive helper function.
2. Initialize the accumulator with the result corresponding to the base case.
3. Base case: Return the accumulator instead of returning the original base-case result.
4. Recursive case: Update the accumulator with the work that would have been performed after the recursive call.
5. Make the recursive call the final operation. 

#### Example 

```ocaml
let rec sum l =
  match l with
  | [] -> 0
  | h :: t -> h + sum t
```

turned into

```ocaml
let sum_tr l =
  let rec helper l acc =
    match l with
    | [] -> acc
    | h :: t -> helper t (acc + h)
  in
  helper l 0
```

### CPS 

1. Add a continuation higher-order function, usually named `k`, as an additional argument. 
2. Base case: Apply `k` to the base case's result.
3. Recursive case: Move all the work that previously happened after the recursive call into the continuation.
4. Make the recursive call the final operation.

#### Example 1

```ocaml
let rec append l1 l2 =
  match l1 with
  | [] -> l2
  | h :: t -> h :: append t l2
```

turned into 

```ocaml
let rec append_tr l1 l2 =
  let rec helper l1 l2 k =
    match l1 with
    | [] -> k l2
    | h :: t -> helper t l2 (fun r -> k (h :: r))
  in
  helper l1 l2 (fun r -> r)
```

where for example, `append_tr [1; 2] [3; 4]` executes as follows:

```
append_tr [1; 2] [3; 4]
=> helper [1; 2] [3; 4] (fun r -> r)
=> helper [2] [3; 4] (fun r1 -> (fun r -> r) (1 :: r1))
=> helper [] [3; 4] (fun r2 -> (fun r1 -> (fun r -> r) (1 :: r1)) (2 :: r2))
=> (fun r2 -> (fun r1 -> (fun r -> r) (1 :: r1)) (2 :: r2)) [3; 4]
=> (fun r1 -> (fun r -> r) (1 :: r1)) (2 :: [3; 4]))
=> (fun r -> r) (1 :: 2 :: [3; 4]) 
=> (1 :: 2 :: [3; 4])
=> [1; 2; 3; 4]
```

#### Example 2

For a binary tree

```ocaml
type 'a tree =
  | Empty
  | Node of 'a tree * 'a * 'a tree
```

We have the traditional `find_all` recursive function

```ocaml
let rec find_all predicate tree =
  match tree with
  | Empty -> []
  | Node (left, value, right) ->
      if predicate value then
        (find_all predicate left) @ (value :: (find_all predicate right))
      else
        (find_all predicate left) @ (find_all predicate right)
```

turned into the tail-recursive function via CPS

```ocaml
let rec find_all_tr predicate tree =
  let rec helper predicate tree k =
    match tree with
    | Empty -> k []
    | Node (left, value, right) ->
        helper predicate left
          (fun left_results ->
            helper predicate right
              (fun right_results ->
                if predicate value then
                  k (left_results @ (value :: right_results))
                else
                  k (left_results @ right_results)))
  in
  helper predicate tree (fun results -> results)
```

## When to Avoid Recursion

- Some programming languages impose a limit on the size of the call stack, and do not support tail-call optimization, which may lead to stack overflow.
- Some languages have higher function call overhead. 


