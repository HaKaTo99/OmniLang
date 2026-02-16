# OmniLang Specification: Basic Syntax

**Status**: Draft
**Version**: v0.1

## 1. Structure & Comments

OmniLang uses a C-family style syntax geared towards readability and modern tooling.

### Comments
```omni
// Single line comment
/* Multi-line
   comment */
/// Documentation comment (supports Markdown)
```

### Entry Point
Every executable program must have a `main` function.
```omni
fn main() {
    print("Hello, Distributed World!");
}
```

## 2. Variables & Constants

Variables are immutable by default (`let`). Mutable variables use `mut`.

```omni
let x = 42;             // Immutable int (inferred)
let y: string = "Hello"; // Explicit type
let mut z = 10;         // Mutable int
z = 20;

const MAX_CONNECTIONS = 100; // Compile-time constant
```

## 3. Functions

Functions are declared with `fn`. They support named arguments and default values.

```omni
fn add(a: int, b: int) -> int {
    return a + b;
}

// Short syntax for single expression
fn square(x: int) -> int = x * x;

// Named arguments
fn connect(host: string, port: int = 80) { ... }
connect(host: "localhost");
```

## 4. Control Flow

### If-Else
Parentheses around conditions are optional.
```omni
if x > 10 {
    print("Big");
} else if x == 10 {
    print("Ten");
} else {
    print("Small");
}
```

### Match (Pattern Matching)
Exhaustive pattern matching.
```omni
match status {
    200 => print("OK"),
    404 => print("Not Found"),
    _   => print("Unknown"),
}
```

### Loops
`for` is the universal loop construct.

```omni
// While-like
for x < 10 {
    x += 1;
}

// C-style
for let i = 0; i < 10; i++ { ... }

// Iterator
for item in collection { ... }

// Infinite loop
loop {
    if condition { break; }
}
```

## 5. Modules & Imports

File-based module system.

```omni
import std.io;
import network.http as net;

// Exporting symbols
pub fn publicFunction() { ... }
```
