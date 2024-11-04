# simple-counter

Provides a single macro for generating thread-local global counters by 
creating a new module with a thread-local static `Cell`. Currently intended 
to be used with integer types. Useful for basic ID generation.

## Usage

Add the following dependency to your Cargo.toml file:

```toml
[dependencies]
simple-counter = "0.1.0"
```

And make sure to use the `#[macro_use]` annotation when importing:

```rust
#[macro_use]
extern crate simple_counter;

generate_counter!(Counter, usize);

fn main() {

  // Starts at 0 by default
  assert_eq!(Counter::next(), 0);
  assert_eq!(Counter::next(), 1);
  assert_eq!(Counter::next(), 2);

  // Can be set to arbitrary value
  Counter::set(1000);
  assert_eq!(Counter::next(), 1000);
  assert_eq!(Counter::next(), 1001);
  assert_eq!(Counter::next(), 1002);

  // Or reset to 0
  Counter::reset();
  assert_eq!(Counter::next(), 0);
}
```

## Example

Here's a simple unique temp generator for a compiler:

```rust
#[macro_use]
extern crate simple_counter;

generate_counter!(TempID, usize);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Temp {
    id: usize,
    name: String,
}

impl Temp {
    pub fn from_str(name: &'static str) -> Self {
        Temp {
            id: TempID::next(),
            name: name.to_string(),
        }
    }
}
```
