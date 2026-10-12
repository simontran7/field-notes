# C

## Strings

A **string** is a sequence of `char`s ending at (and including) the first `\0`.
It's a property of the data in memory, not of the syntax used to create it.

- **String literal** = the double-quoted source syntax (`"hello"`). It creates an
  unnamed static array of type `char[n+1]` with the `\0` added automatically,
  even if one is already there: `"hello\0"` is 7 bytes (`hello\0\0`).
- A string always lives in an array (declared, a literal's hidden array, a
  compound literal, or `malloc`'d memory). A `char *` only *points* to one.

### LHS: what you declare

| Declaration      | What it is                                   | Writable? | Lifetime of the chars |
|------------------|----------------------------------------------|-----------|-----------------------|
| `char *s = "hi"` | pointer to the literal's hidden array        | No (UB, often crashes) | static (whole program) |
| `char s[] = "hi"`| your own array, a *copy* of the literal; size inferred (3) | Yes | automatic in a function, static at file scope or with `static` |
| `char s[n] = "hi"` | your own array of exactly n                | Yes       | same as above |

- Prefer `const char *s = "hi";` so the compiler rejects writes.
- `char s[n]` with explicit size:
  - n > length: copies the chars, the rest (including the terminator) is zero-filled
  - n == length (no room for `\0`): **legal in C, no terminator** → not a string. Error in C++.
  - n < length: excess-elements warning/error.

### RHS: where the chars come from

- **String literal** `"hello"`: auto `\0`. Works with `char *`, `char[]`, `char[n]`.
- **Brace list** `{'h','e','l','l','o','\0'}`: no auto `\0`, you must add it.
  Only for arrays. `char *s = {...}` is wrong (treats `'h'` as an address).
- **Compound literal** (C99) `char *s = (char[]){'h','i','\0'};`: unnamed, modifiable
  array; automatic lifetime in a function, so don't return it.
- **At runtime**: fill an array and set `'\0'`, or `strcpy`, `snprintf`, `fgets`, ...
- **`malloc`**: `char *s = malloc(n);` heap memory, lives until `free`. You must
  write the chars and the `\0` yourself. Only works with a pointer.

### Gotchas

- `= "..."` / `= {...}` on an array only works at **initialization**.
  `char s[6]; s = "hello";` is an error (arrays aren't assignable) → use `strcpy`.
- `sizeof` vs `strlen`: for `char s[] = "hello"`, `sizeof s == 6`, `strlen(s) == 5`.
  For `char *p = "hello"`, `sizeof p` is the pointer size (8 on 64-bit).
  `sizeof("hello") == 6` (the literal is an array).
- Arrays **decay** to a pointer to their first element when passed to functions,
  which is why string functions take `char *` / `const char *`.
- A char array without `\0` is just a byte buffer; passing it to string functions
  is UB (they read past the end). Track the length yourself (`printf("%.*s", n, buf)`).
- Rust analogy: `const char *p = "hi";` ≈ `let p: &'static str = "hi";`, except
  `&str` is a fat pointer (address + length) and isn't null-terminated.

## ISO C Standard Library

[https://beej.us/guide/bgclr/html/](https://beej.us/guide/bgclr/html/)

> [!WARNING]
> ISO C: the language and its library. Works everywhere. (You’ve got this.)
POSIX: the standard for Unix-like OSes. Adds files and file descriptors (open, read, write), processes (fork, exec, wait), threads (pthread_*), signals, sockets, pipes. Works on Linux, macOS, BSDs.
Extensions (non-standard, OS- or library-specific): Linux things like epoll, GNU things like asprintf, macOS things like kqueue. Use them when you need them and know you’re giving up portability. Some stuff are library functions, `glibc` is an impl of a c stdlib, covering iso c, posix, and linux extensions

## Encapsulation

C has no `private` keyword. To hide a struct's fields, a common C idiom is to create a header that declares the struct without defining it, which makes it an **incomplete type**, or **opaque type**. Other files can hold pointers to it and pass them to the library's functions, and only the library's own `.c` file can see the fields inside. 

For example, let's build a fixed-capacity stack of `int`s across three files.

The header is the public interface. `struct stack;` tells callers that the type exists and says nothing about its fields. Every operation goes through a function that takes a `struct stack *`. 

```c
// stack.h
#ifndef STACK_H
#define STACK_H

#include <stddef.h>

struct stack;

[[nodiscard]] struct stack *stack_new(size_t capacity);
void stack_free(struct stack *s);
[[nodiscard]] bool stack_push(struct stack *s, int value);
[[nodiscard]] bool stack_pop(struct stack *s, int *out);
[[nodiscard]] size_t stack_count(const struct stack *s);

#endif
```

The full definition lives _only_ in `stack.c`, so this is the one file that can read or change `count`, `capacity` and `data`. Additionally, callers can't see the struct's size, so the library has to allocate it for them. 

```c
// stack.c
#include "stack.h"

#include <stdckdint.h>
#include <stdlib.h>

struct stack {
    size_t count;
    size_t capacity;
    int *data;
};

struct stack *stack_new(size_t capacity) {
    size_t items_size;
    if (ckd_mul(&items_size, capacity, sizeof(int))) {
        goto out;
    }

    struct stack *s = malloc(sizeof *s);
    if (s == nullptr) {
        goto out;
    }

    s->data = malloc(items_size);
    if (s->data == nullptr) {
        goto free_stack;
    }

    s->count = 0;
    s->capacity = capacity;
    return s;

free_stack:
    free(s);
out:
    return nullptr;
}

void stack_free(struct stack *s) {
    if (s == nullptr) {
        return;
    }
    free(s->items);
    free(s);
}

bool stack_push(struct stack *s, int value) {
    if (s->count == s->capacity) {
        return false;
    }

    s->data[s->len++] = value;

    return true;
}

bool stack_pop(struct stack *s, int *out) {
    if (s->count == 0) {
        return false;
    }

    *out = s->data[--s->count];

    return true;
}

size_t stack_count(const struct stack *s) {
    return s->count;
}
```

`main.c` includes only the header:

```c
// main.c
#include <stdio.h>

#include "stack.h"

int main(void) {
    constexpr size_t capacity = 2;
    struct stack *s = stack_new(capacity);
    if (s == nullptr) {
        return 1;
    }

    for (int v = 10; v <= 30; v += 10) {
        if (!stack_push(s, v)) {
            printf("push %d: stack is full\n", v);
        }
    }
    printf("size: %zu\n", stack_size(s));

    int value;
    while (stack_pop(s, &value)) {
        printf("popped %d\n", value);
    }

    stack_free(s);
    return 0;
}
```

Compile `stack.c` once into an object file, then link it with `main.c`.

```console
$ gcc -std=c23 -Wall -Wextra -c stack.c
$ gcc -std=c23 -Wall -Wextra -o main main.c stack.o
$ ./main
push 30: stack is full
size: 2
popped 20
popped 10
```

To see the encapsulation at work, here is a program `peek.c` that tries to read `len` directly.

```c
// peek.c
#include <stdio.h>

#include "stack.h"

int main(void) {
    struct stack *s = stack_new(4);
    printf("%zu\n", s->count);
    stack_free(s);
    return 0;
}
```

The compiler rejects it, because `peek.c` only knows the incomplete type from the header.

```console
$ gcc -std=c23 -Wall -Wextra -c peek.c
peek.c: In function ‘main’:
peek.c:9:22: error: invalid use of undefined type ‘struct stack’
    9 |     printf("%zu\n", s->len);
      |                      ^~
```

Hiding the fields has two costs. Callers can't declare a `struct stack` on the stack or embed one in their own structs, so every stack lives on the heap. Every access also goes through a function call in another file, which the compiler can only inline when link-time optimization (`-flto`) is enabled.

## Generics

https://youtu.be/3IAlJSIjvH0?t=541

## Deferred Cleanup

C has no `defer` statement, so a function that acquires several resources needs another way to release them on every exit path. The common idiom uses `goto` to jump forward to a chain of cleanup labels at the end of the function.

This example opens a file, allocates a buffer, and reads into it. Each of those steps can fail. The buffer size is a `constexpr` constant, so `malloc` and `fread` always agree on it.

```c
#include <stdio.h>
#include <stdlib.h>

constexpr size_t buf_size = 4096;
```

`result` starts as the failure value and becomes `0` only when every step succeeds, so each early exit returns an error without any extra code.

```c
static int process(const char *path) {
    int result = -1;
```

When a step fails, jump to the label that releases everything acquired _before_ it. A failed `fopen` has nothing to release yet, so it jumps straight to `out`.

```c
    FILE *f = fopen(path, "r");
    if (f == nullptr) {
        goto out;
    }

    char *buf = malloc(buf_size);
    if (buf == nullptr) {
        goto close_file;
    }

    size_t n = fread(buf, 1, buf_size, f);
    if (ferror(f)) {
        goto free_buf;
    }

    printf("read %zu bytes from %s\n", n, path);
    result = 0;
```

The labels appear in the reverse order of acquisition. The success path falls through all of them, and each failure enters partway down, so every resource is released exactly once.

```c
free_buf:
    free(buf);
close_file:
    fclose(f);
out:
    return result;
}
```

```c
int main(int argc, char **argv) {
    if (argc < 2) {
        fprintf(stderr, "usage: %s FILE\n", argv[0]);
        return 1;
    }
    return process(argv[1]) == 0 ? 0 : 1;
}
```

Run it on its own source file, then on a file that doesn't exist. The second run takes the `goto out` path and exits with status `1`.

```console
$ gcc -std=c23 -Wall -Wextra -o cleanup cleanup.c
$ ./cleanup cleanup.c
read 720 bytes from cleanup.c
$ ./cleanup missing.txt; echo $?
1
```

> [!WARNING]
> **`goto` pitfalls**:
> - `goto` can't jump forward past a variable-length array, because its memory is allocated only when execution reaches the declaration.
> - Variable initializations skipped by a `goto` never run, so reading those variables is undefined behavior. In the example above, `goto out` skips the initialization of `buf`, which is safe only because nothing after `out` reads `buf`.

## Tagged Unions

A union stores one of several types in the same memory, but it has no way of recording which member currently holds a value. A **tagged union** pairs the union with an enum, the tag, that records which member is valid. Code that writes the union also sets the tag, and code that reads it checks the tag first.

This example builds a `value` that holds an `int`, a `double`, or a string. The tag lists the possible kinds.

```c
#include <stdio.h>

enum value_kind {
    VALUE_INT,
    VALUE_DOUBLE,
    VALUE_STRING,
};
```

The struct keeps the tag next to an anonymous union. Since C11, the members of an anonymous union are accessed directly through the struct, so `v.i` works without naming the union.

```c
struct value {
    enum value_kind kind;
    union {
        int i;
        double d;
        const char *s;
    };
};
```

Each constructor sets the tag and the matching member in one compound literal, so a value with a mismatched tag can't be created through them.

```c
static struct value value_int(int i) {
    return (struct value){ .kind = VALUE_INT, .i = i };
}

static struct value value_double(double d) {
    return (struct value){ .kind = VALUE_DOUBLE, .d = d };
}

static struct value value_string(const char *s) {
    return (struct value){ .kind = VALUE_STRING, .s = s };
}
```

Reading a value means switching on the tag and touching only the member that case allows. With `-Wall`, the compiler warns when a `switch` over an enum leaves out one of its values and has no `default`, so adding a new kind later flags every `switch` that needs a new case.

```c
static void value_print(struct value v) {
    switch (v.kind) {
    case VALUE_INT:
        printf("int     %d\n", v.i);
        break;
    case VALUE_DOUBLE:
        printf("double  %g\n", v.d);
        break;
    case VALUE_STRING:
        printf("string  \"%s\"\n", v.s);
        break;
    }
}
```

Since every element has the same type, values of different kinds fit in one array.

```c
int main(void) {
    struct value values[] = {
        value_int(42),
        value_double(3.5),
        value_string("hello"),
    };

    for (size_t i = 0; i < sizeof values / sizeof values[0]; i++) {
        value_print(values[i]);
    }
    printf("sizeof(struct value) = %zu\n", sizeof(struct value));
    return 0;
}
```

The union is as large as its largest member (8 bytes for the `double` and the pointer), and the 4-byte tag is padded to 8 bytes so the union stays aligned, which makes 16 bytes on x86-64.

```console
$ gcc -std=c23 -Wall -Wextra -o value value.c && ./value
int     42
double  3.5
string  "hello"
sizeof(struct value) = 16
```

> [!WARNING]
> Nothing stops code from reading a member the tag doesn't allow. Reading `v.d` when `v.kind` is `VALUE_INT` reinterprets the `int`'s bytes as a `double` and quietly produces a garbage number, so every read must go through a check of the tag.

## Inheritance and Intrusive Data Structures with `container_of`

The `container_of` macro takes a pointer to a struct member and returns a pointer to the struct that contains it. The Linux kernel uses it everywhere, because it gives C two features the language lacks:
- generic intrusive data structures
- object-oriented inheritance

The examples below build up to both, one program at a time.

### Step 1: Understand `offsetof`

`offsetof(type, member)`, from `<stddef.h>`, gives the byte distance from the start of a struct to one of its members.

```c
#include <stdio.h>
#include <stddef.h>
```

`tag` is a single `char`, and `x` needs 4-byte alignment, so the compiler inserts 3 bytes of padding after `tag`. Because of padding, a member's offset can be larger than the sum of the sizes of the members before it (see [Record Packing](../computer-systems/performance-engineering/record-packing.md)).

```c
struct point {
    char tag;
    int x;
    int y;
};
```

In C23, `static_assert` is a keyword and its message is optional. It turns the padding claim into a compile-time check, so the build fails on any platform where `x` lands somewhere else.

```c
static_assert(offsetof(struct point, x) == 4);
```

```c
int main(void) {
    printf("tag: %zu\n", offsetof(struct point, tag));
    printf("x:   %zu\n", offsetof(struct point, x));
    printf("y:   %zu\n", offsetof(struct point, y));
    return 0;
}
```

Adding a member's offset to the struct's address gives the member's address. `container_of` runs this calculation in reverse.

```console
$ gcc -std=c23 -Wall -Wextra -o offsetof offsetof.c && ./offsetof
tag: 0
x:   4
y:   8
```

### Step 2: Go backwards, then wrap it in a macro

Given a pointer to a member, subtracting that member's offset recovers the address of the whole struct.

```c
#include <stdio.h>
#include <stddef.h>
```

The cast to `char *` makes the subtraction count in bytes. Subtracting from an `int *` would move in steps of `sizeof(int)`.

The `1 ? (ptr) : &((type *)0)->member` part is a type check. The conditional operator requires compatible pointer types in both branches, so passing a `ptr` of the wrong type produces a compiler diagnostic. The condition is the constant `1`, so the null-pointer branch is never evaluated.

```c
#define container_of(ptr, type, member) \
    ((type *)((char *)(1 ? (ptr) : &((type *)0)->member) - \
              offsetof(type, member)))
```

```c
struct point {
    char tag;
    int x;
    int y;
};
```

Starting from a pointer to `y`, the calculation written out by hand and the macro both land on `p`.

```c
int main(void) {
    struct point p = { 'A', 10, 20 };
    int *py = &p.y;

    struct point *by_hand =
        (struct point *)((char *)py - offsetof(struct point, y));
    struct point *by_macro = container_of(py, struct point, y);

    printf("by hand:  %s\n", by_hand == &p ? "&p" : "wrong");
    printf("by macro: %s\n", by_macro == &p ? "&p" : "wrong");
    printf("tag:      %c\n", by_macro->tag);
    return 0;
}
```

```console
$ gcc -std=c23 -Wall -Wextra -o container_of container_of.c && ./container_of
by hand:  &p
by macro: &p
tag:      A
```

### Step 3: An intrusive linked list

In an intrusive list, the list node lives inside your data. The list code works only with nodes, so it never needs to know your type, and adding an object to a list needs no `void *` and no extra allocation.

```c
#include <stdio.h>
#include <stddef.h>

#define container_of(ptr, type, member) \
    ((type *)((char *)(1 ? (ptr) : &((type *)0)->member) - \
              offsetof(type, member)))
```

The generic part is a circular doubly linked list with a sentinel head, the same design as the Linux kernel's `list_head`. An empty list is a head that points to itself.

```c
struct list_node {
    struct list_node *prev, *next;
};

static void list_init(struct list_node *head) {
    head->prev = head->next = head;
}

static void list_add_tail(struct list_node *head, struct list_node *n) {
    n->prev = head->prev;
    n->next = head;
    head->prev->next = n;
    head->prev = n;
}
```

`list_del` unlinks a node by connecting its neighbors to each other, then points the node at itself.

```c
static void list_del(struct list_node *n) {
    n->prev->next = n->next;
    n->next->prev = n->prev;
    n->next = n->prev = n;
}
```

An `employee` embeds two hooks, so the same object can sit in two lists at once: one for the whole company and one for a team. The hooks can go anywhere in the struct, since `container_of` subtracts each hook's real offset.

```c
struct employee {
    const char *name;
    int salary;
    struct list_node all_staff;
    struct list_node team;
};
```

Everyone joins the company list, and Carol and Bob join the team.

```c
int main(void) {
    struct list_node staff, team;
    list_init(&staff);
    list_init(&team);

    struct employee a = { .name = "Alice", .salary = 5000 };
    struct employee b = { .name = "Bob",   .salary = 4200 };
    struct employee c = { .name = "Carol", .salary = 6100 };

    list_add_tail(&staff, &a.all_staff);
    list_add_tail(&staff, &b.all_staff);
    list_add_tail(&staff, &c.all_staff);

    list_add_tail(&team, &c.team);
    list_add_tail(&team, &b.team);
```

Bob then leaves the team. `list_del` only touches his `team` hook, so he stays on the company list.

```c
    list_del(&b.team);
```

Walking a list means following `next` until you arrive back at the head. `container_of` takes each node back to its `employee`, and the member name tells it which hook the node came from.

```c
    printf("staff:\n");
    for (struct list_node *p = staff.next; p != &staff; p = p->next) {
        struct employee *e = container_of(p, struct employee, all_staff);
        printf("  %s earns %d\n", e->name, e->salary);
    }

    printf("team:\n");
    for (struct list_node *p = team.next; p != &team; p = p->next) {
        struct employee *e = container_of(p, struct employee, team);
        printf("  %s\n", e->name);
    }
    return 0;
}
```

With a non-intrusive list, the same setup would need two separately allocated nodes per employee, each holding a pointer back to its employee.

```console
$ gcc -std=c23 -Wall -Wextra -o list list.c && ./list
staff:
  Alice earns 5000
  Bob earns 4200
  Carol earns 6100
team:
  Carol
```

### Step 4: Polymorphism with a vtable

A base type holds a pointer to a table of function pointers, called a **vtable**. Each derived type embeds the base, and its methods use `container_of` to get from the base back to the derived type.

```c
#include <stdio.h>
#include <stddef.h>

#define container_of(ptr, type, member) \
    ((type *)((char *)(1 ? (ptr) : &((type *)0)->member) - \
              offsetof(type, member)))
```

```c
struct shape;

struct shape_ops {
    double (*area)(struct shape *s);
    const char *(*name)(struct shape *s);
};

struct shape {
    const struct shape_ops *ops;
};
```

Each circle method receives a `struct shape *` and recovers its `struct circle` with `container_of`. `circle_name` doesn't use its parameter, so it leaves it unnamed, which C23 allows. All circles share one `static const` ops table, filled in with designated initializers, so each object stores only a single pointer to it. The Linux kernel's `struct file_operations` works the same way.

```c
constexpr double pi = 3.14159265358979;

struct circle {
    double r;
    struct shape base;
};

static double circle_area(struct shape *s) {
    struct circle *c = container_of(s, struct circle, base);
    return pi * c->r * c->r;
}

static const char *circle_name(struct shape *) {
    return "circle";
}

static const struct shape_ops circle_ops = { .area = circle_area,
                                             .name = circle_name };

static void circle_init(struct circle *c, double r) {
    c->r = r;
    c->base.ops = &circle_ops;
}
```

A rectangle follows the same pattern.

```c
struct rect {
    double w, h;
    struct shape base;
};

static double rect_area(struct shape *s) {
    struct rect *r = container_of(s, struct rect, base);
    return r->w * r->h;
}

static const char *rect_name(struct shape *) {
    return "rectangle";
}

static const struct shape_ops rect_ops = { .area = rect_area,
                                           .name = rect_name };

static void rect_init(struct rect *r, double w, double h) {
    r->w = w;
    r->h = h;
    r->base.ops = &rect_ops;
}
```

Generic code sees only `struct shape *` and calls through the vtable, which is how C++ virtual methods work underneath.

```c
static void print_shape(struct shape *s) {
    printf("%s with area %.2f\n", s->ops->name(s), s->ops->area(s));
}

int main(void) {
    struct circle c;
    struct rect r;

    circle_init(&c, 1.0);
    rect_init(&r, 3.0, 4.0);

    struct shape *shapes[] = { &c.base, &r.base };

    for (size_t i = 0; i < sizeof shapes / sizeof shapes[0]; i++) {
        print_shape(shapes[i]);
    }
    return 0;
}
```

```console
$ gcc -std=c23 -Wall -Wextra -o vtable vtable.c && ./vtable
circle with area 3.14
rectangle with area 12.00
```

> [!WARNING]
> `container_of` is valid only when the pointer really points to that member inside a real object of the containing type. The macro checks that the pointer's type matches the member's type, and it has no way to check which object the pointer actually came from. For example, if `circle_area` receives the `struct shape` embedded in a rectangle, `container_of` computes the address of a circle that doesn't exist, and using that address is undefined behavior. The safest approach is to make this mistake impossible by design. Since only `circle_init` installs `circle_ops`, the only way to reach `circle_area` is through a real circle. Keeping each ops table `static` to its source file ensures that no other code can attach it to the wrong kind of object.

### Step 5: Putting it together, a list of polymorphic shapes

Give the base `struct shape` a list hook, and any shape can live in a generic list. Walking that list takes two `container_of` hops: from the list node to the `shape` in the loop, then from the `shape` to the `circle` or `rect` inside each method.

```c
#include <stdio.h>
#include <stddef.h>

#define container_of(ptr, type, member) \
    ((type *)((char *)(1 ? (ptr) : &((type *)0)->member) - \
              offsetof(type, member)))
```

The list is the one from Step 3, with a `list_for_each` macro to shorten the loop.

```c
struct list_node {
    struct list_node *prev, *next;
};

static void list_init(struct list_node *head) {
    head->prev = head->next = head;
}

static void list_add_tail(struct list_node *head, struct list_node *n) {
    n->prev = head->prev;
    n->next = head;
    head->prev->next = n;
    head->prev = n;
}

#define list_for_each(pos, head) \
    for ((pos) = (head)->next; (pos) != (head); (pos) = (pos)->next)
```

The base type gains a `link` hook, so every shape can live in a list.

```c
struct shape;

struct shape_ops {
    double (*area)(struct shape *s);
    const char *(*name)(struct shape *s);
};

struct shape {
    const struct shape_ops *ops;
    struct list_node link;
};
```

The circle and rectangle code is unchanged from Step 4.

```c
constexpr double pi = 3.14159265358979;

struct circle {
    double r;
    struct shape base;
};

static double circle_area(struct shape *s) {
    struct circle *c = container_of(s, struct circle, base);
    return pi * c->r * c->r;
}

static const char *circle_name(struct shape *) {
    return "circle";
}

static const struct shape_ops circle_ops = { .area = circle_area,
                                             .name = circle_name };

static void circle_init(struct circle *c, double r) {
    c->r = r;
    c->base.ops = &circle_ops;
}

struct rect {
    double w, h;
    struct shape base;
};

static double rect_area(struct shape *s) {
    struct rect *r = container_of(s, struct rect, base);
    return r->w * r->h;
}

static const char *rect_name(struct shape *) {
    return "rectangle";
}

static const struct shape_ops rect_ops = { .area = rect_area,
                                           .name = rect_name };

static void rect_init(struct rect *r, double w, double h) {
    r->w = w;
    r->h = h;
    r->base.ops = &rect_ops;
}
```

`scene_print` knows only shapes and lists. Adding a triangle means writing a new struct, two methods and an ops table, and this loop stays the same.

```c
static double scene_print(struct list_node *scene) {
    struct list_node *p;
    double total = 0.0;

    list_for_each(p, scene) {
        struct shape *s = container_of(p, struct shape, link);
        double a = s->ops->area(s);
        printf("%-10s %8.2f\n", s->ops->name(s), a);
        total += a;
    }
    return total;
}
```

Nothing here is heap-allocated. The shapes, their vtable pointers and their list hooks all live inside the objects themselves.

```c
int main(void) {
    struct list_node scene;
    struct circle c1, c2;
    struct rect r1;

    list_init(&scene);

    circle_init(&c1, 1.0);
    rect_init(&r1, 3.0, 4.0);
    circle_init(&c2, 2.5);

    list_add_tail(&scene, &c1.base.link);
    list_add_tail(&scene, &r1.base.link);
    list_add_tail(&scene, &c2.base.link);

    printf("%-10s %8.2f\n", "total", scene_print(&scene));
    return 0;
}
```

```console
$ gcc -std=c23 -Wall -Wextra -o shapes shapes.c && ./shapes
circle         3.14
rectangle     12.00
circle        19.63
total         34.78
```

`offsetof` accepts nested members, so when you know the concrete type, you can jump straight from the list node to it with `container_of(p, struct circle, base.link)`.
