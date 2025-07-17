# my_r_num

A Rust numeric type library that supports automatic optimization for various integer/float types, special values (NaN, positive/negative infinity), and implements common arithmetic operations and type conversions.

## Features
- Smartly selects the most efficient integer/float type to save memory
- Supports arithmetic operators (+ - * / %) for addition, subtraction, multiplication, division, and remainder
- Supports in-place operators (+= -= *= /= %=)
- Implements common traits: PartialEq, PartialOrd, Display
- Supports NaN, positive infinity, negative infinity
- Supports string parsing and type name query
- Generic From implementation for automatic conversion from native types

## Usage

### Dependency

Add to your `Cargo.toml`:
```toml
[dependencies]
my_r_num = { path = "." }
```

### Basic Example
```rust
use my_r_num::Number;

fn main() {
    let a = Number::from(10i8);
    let b = Number::from(3.14f64);
    let c = Number::from(20000i16);
    let d = Number::parse("inf").unwrap();

    println!("a = {} (type: {})", a, a.type_name());
    println!("b = {} (type: {})", b, b.type_name());
    println!("c = {} (type: {})", c, c.type_name());
    println!("d = {} (type: {})", d, d.type_name());

    let mut x = a + c;
    x += Number::from(5);
    println!("x = {}", x);

    let y = b * Number::from(2);
    println!("y = {}", y);

    // Special value operations
    println!("d + a = {}", d + a);
    println!("d - d = {}", d - d);
}
```

### String Parsing
```rust
let n = Number::parse("32767").unwrap();
assert_eq!(n.type_name(), "Integer16");
```

### Special Value Check
```rust
let nan = Number::parse("nan").unwrap();
assert!(nan.is_nan());
```

## Testing

Run all unit tests:
```sh
cargo test
```

## Use Cases
- Unified handling of various integer/float types
- Need to support special values (NaN, infinity)
- Memory optimization and type-safe numeric computation

---

Feel free to open an issue for suggestions or questions! 