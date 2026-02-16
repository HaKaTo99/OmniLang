# OmniLang Specification: Type System

**Status**: Draft
**Version**: v0.1

## 1. Primitive Types

OmniLang provides a set of standard primitive types.

| Type | Description |
|---|---|
| `int`, `int8`, `int16`, `int32`, `int64` | Signed integers. `int` is platform-dependent (usually 64-bit). |
| `uint`, `uint8`, `uint16`, `uint32`, `uint64` | Unsigned integers. |
| `float`, `float32`, `float64` | Floating-point numbers. `float` is `float64`. |
| `bool` | Boolean (`true`, `false`). |
| `char` | Unicode character (32-bit). |
| `string` | UTF-8 encoded immutable string. |
| `byte` | Alias for `uint8`. |

## 2. Composite Types

### Tuples
Fixed-size collection of heterogeneous types.
```omni
let coordinates: (int, int) = (10, 20);
let (x, y) = coordinates;
```

### Arrays
Fixed-size collection of homogeneous types.
```omni
let buffer: [byte; 1024];
let scores = [10, 20, 30]; // Inferred [int; 3]
```

### Slices
Dynamic view into an array.
```omni
let s: []int = scores[0..2];
```

### Structs
User-defined data structures.
```omni
struct Point {
    x: int,
    y: int,
}

let p = Point { x: 10, y: 20 };
```

### Enums (Algebraic Data Types)
Enums can hold data.
```omni
enum Shape {
    Circle(float),
    Rectangle(float, float),
}
```

## 3. Generics

Functions and structs can be generic.

```omni
struct Box<T> {
    value: T,
}

fn identity<T>(x: T) -> T {
    return x;
}
```

## 4. Traits/Interfaces

Traits define behavior.

```omni
trait Printable {
    fn toString() -> string;
}

impl Printable for Point {
    fn toString() -> string {
        return "Point(" + self.x + ", " + self.y + ")";
    }
}
```
