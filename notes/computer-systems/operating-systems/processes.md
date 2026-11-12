# Processes

## Concurrency vs Parallelism

```tikz
\begin{tikzpicture}[x=0.6cm, y=0.6cm, slot/.style={draw, minimum width=0.6cm, minimum height=0.5cm}]
  % Concurrency: one core alternates between the tasks.
  \node at (2.5, 2.4) {Concurrency (single core)};
  \node[left, font=\small] at (-0.1, 1) {Task A};
  \node[left, font=\small] at (-0.1, 0) {Task B};
  \foreach \i in {0, ..., 4} {
    \pgfmathsetmacro{\a}{mod(\i, 2) == 0 ? "hl" : "dashed"}
    \pgfmathsetmacro{\b}{mod(\i, 2) == 0 ? "dashed" : "hl"}
    \node[slot, \a] at (\i + 0.5, 1) {};
    \node[slot, \b] at (\i + 0.5, 0) {};
  }
  \draw[->, thin] (0, -0.8) -- (5.4, -0.8) node[right, font=\small] {time};

  % Parallelism: each task has its own core.
  \begin{scope}[xshift=6cm]
    \node at (2.5, 2.4) {Parallelism (multi-core)};
    \node[left, font=\small] at (-0.1, 1) {Task A};
    \node[left, font=\small] at (-0.1, 0) {Task B};
    \node[slot, hl, minimum width=3cm] at (2.5, 1) {};
    \node[slot, hl, minimum width=3cm] at (2.5, 0) {};
    \draw[->, thin] (0, -0.8) -- (5.4, -0.8) node[right, font=\small] {time};
  \end{scope}
  \node[slot, hl, minimum width=0.4cm, minimum height=0.4cm, label=right:{\small running}] at (0.3, -2) {};
  \node[slot, dashed, minimum width=0.4cm, minimum height=0.4cm, label=right:{\small waiting (context switched out)}] at (4.3, -2) {};
\end{tikzpicture}
```

...

```tikz
\begin{tikzpicture}[call/.style={draw, minimum width=2cm, minimum height=0.9cm, font=\ttfamily}]
  \node[font=\small] at (0, 0) {Parent};
  \node[font=\small] at (0, -2) {Child};
  \node[call] (fork) at (2.6, 0) {fork()};
  \node[call] (wait) at (7.6, 0) {wait()};
  \node[call, hl] (exec) at (5.1, -2) {exec()};
  \draw[->] (fork) -- (wait);
  \draw[->] (wait) -- ++(1.8, 0);
  \draw[->] (fork.south) to[out=-90, in=180] (exec.west);
  \draw[->] (exec.east) to[out=0, in=-90] (wait.south);
\end{tikzpicture}
```

```tikz
\usetikzlibrary{shapes.geometric}
\begin{tikzpicture}[x=1.9cm, y=-1.35cm, op/.style={draw, circle, minimum size=1.15cm, inner sep=0pt, font=\ttfamily\scriptsize},
    end/.style={op, regular polygon, regular polygon sides=8, hl}]
  % The parent (left column) forks three times; child k sleeps k seconds, prints k, and exits.
  % Rows run down in time order, so the prints appear in the order 1, 2, 3, 4.
  \foreach \k/\y in {1/0, 2/1, 3/2} \node[op] (fork\k) at (0, \y) {fork()};
  \node[op] (s1) at (0, 3) {sleep(1)};
  \node[op] (p1) at (0, 4) {print(1)};
  \node[end] (e1) at (0, 5) {exit()};
  \foreach \a/\b in {fork1/fork2, fork2/fork3, fork3/s1, s1/p1, p1/e1} \draw[->, thin] (\a) -- (\b);
  \foreach \k/\x/\start in {4/3/1, 2/2/2, 3/1/3} {
    \node[op] (s\k) at (\x, \start) {sleep(\k)};
    \node[op] (p\k) at (\x, \k + 3) {print(\k)};
    \node[end] (e\k) at (\x, \k + 4) {exit()};
    \draw[->, thin] (s\k) -- (p\k);
    \draw[->, thin] (p\k) -- (e\k);
  }
  \draw[->, thin] (fork1) to[out=0, in=180] (s4);
  \draw[->, thin] (fork2) to[out=0, in=180] (s2);
  \draw[->, thin] (fork3) to[out=0, in=180] (s3);
\end{tikzpicture}
```

```tikz
\begin{tikzpicture}[region/.style={draw, minimum width=5cm}]
  % High addresses at the top; the stack grows down and the heap grows up.
  \node[region, minimum height=2.6cm] (stack) at (0, 0) {};
  \draw[->, thin] (0, 1) -- (0, -1);
  \node[region, minimum height=2.6cm] (heap) at (0, -4) {};
  \draw[->, thin] (0, -5) -- (0, -3);
  \node[region, minimum height=0.65cm, below=0.3cm of heap] (data) {};
  \node[region, minimum height=0.65cm, below=0.1cm of data] (text) {};
  \foreach \r/\name in {stack/Stack, heap/Heap, data/Data, text/Text}
    \draw[thin, dashed] ([xshift=0.15cm]\r.east) -- ++(0.5, 0) node[right] {\name};
\end{tikzpicture}
```

```tikz
\begin{tikzpicture}[state/.style={draw, rounded corners=3pt, minimum height=0.8cm, minimum width=1.6cm},
    tag/.style={font=\small, inner sep=2pt}]
  \fill (3, 1.6) circle (4pt);
  \node[state] (new) at (3, 0) {New};
  \node[state] (ready) at (3, -2.2) {Ready};
  \node[state, hl] (run) at (1.4, -4.6) {Running};
  \node[state] (term) at (0, -7) {Terminated};
  \node[state] (wait) at (4.4, -7) {Waiting};
  \draw[->] (3, 1.45) -- (new);
  \draw[->] (new) -- node[tag, right] {admitted} (ready);
  \draw[->] (ready) to[bend right=25] node[tag, left] {dispatch} (run);
  \draw[->] (run) to[bend right=25] node[tag, right] {interrupt} (ready);
  \draw[->] (run) -- node[tag, left] {exit} (term);
  \draw[->] (run) -- node[tag, sloped, above, pos=0.6] {I/O or event wait} (wait);
  \draw[->] (wait) to[out=60, in=0] node[tag, right] {I/O or event completion} (ready);
  \draw[->] (term) -- (0, -8.4);
  \draw (0, -8.6) circle (4pt);
  \fill (0, -8.6) circle (2pt);
\end{tikzpicture}
```
