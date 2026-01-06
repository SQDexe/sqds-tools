/**
Shorthand for *"robust"* `if`-`else`, or `match` expression,
therefore this can be interpreted as a `ternary operator`.  

This macro expands into a `match` on a bool variable. 

# Examples

```rust
# use sqds_tools::select;
#
/* Example variables */
let day_of_week = 6;
let is_weekend = (6 ..= 7).contains(&day_of_week);

/* Select the motd */
let motd = select!(is_weekend,
    "It's the weekend!",
    "Only a bit until the weekend"
    );

/* Check whether it's  correct */
assert_eq!(motd, "It's the weekend!");
```
*/
#[macro_export]
macro_rules! select {
    ($bool:expr, $truthy:expr, $falsy:expr) => {
        match $bool {
            true => $truthy,
            false => $falsy
            }
        };
    }



/**
Custom, batch assertion on a number of expressions.

Asserts whether each one is equal to `true` at runtime.

# Panics

This will invoke the [`panic!`] macro,
if the provided expression cannot be evaluated to `true` at runtime.

# Examples

```rust
# use sqds_tools::batch_assert;
#
/* Example variable */
let value = 10;

/* Set conditions */
let is_within_range = (0 .. 100).contains(&value);
let is_even = value % 2 == 0;
let is_five_mutiple = value % 5 == 0;

/* Assert! */
batch_assert!(
    is_within_range,    
    is_even,
    is_five_mutiple
    );
```

```rust,should_panic
# use sqds_tools::batch_assert;
#
/* Example variable */
let value = 7;

/* Set conditions */
let is_within_range = (0 .. 100).contains(&value);
let is_even = value % 2 == 0;

/* Assert! */
batch_assert!(
    is_within_range,    
    is_even
    );
```

[`panic!`]: https://doc.rust-lang.org/std/macro.panic.html
*/
#[macro_export]
macro_rules! batch_assert {
    ( $($boolean:expr),+ ) => {
        $(
        if ! ($boolean) {
            ::core::panic!(::core::concat!("assertion failed: ", ::core::stringify!($boolean)));
            }
        )+
        };
    }



/**
Declares the body of a trait for a bunch of types.

Useful for adding traits for multiple types,
when using generics is cumbersome, or outright impossible.

# Examples

```rust
# use sqds_tools::{
#     LayoutMetrics,
#     impl_trait
#     };
# 
/* Example trait */
trait BitsSize {
    const CUSTOM_BITS: usize;
    }

/* Define the body and implement it */
impl_trait! {
    BitsSize,
    { const CUSTOM_BITS: usize = Self::SIZE * 8; },
    u8, u16, u32, u64
    }

/* Check whether it's correct */
assert_eq!(u8::CUSTOM_BITS, 8);
assert_eq!(u16::CUSTOM_BITS, 16);
assert_eq!(u32::CUSTOM_BITS, 32);
assert_eq!(u64::CUSTOM_BITS, 64);
```

```rust
# use sqds_tools::impl_trait;
# 
/* Example trait */
trait IntegerTest {
    fn is_int(&self) -> bool;
    }

/* Define the body and implement it */
impl_trait! {
    IntegerTest,
    {
    fn is_int(&self) -> bool {
        self.fract() == 0.0
        }
    },
    f32, f64
    }

/* Check whether it's correct */
assert!(f32::is_int(&0.0));
assert!(f64::is_int(&10.0));
assert!(f32::is_int(&-10.0));
assert_eq!(f64::is_int(&2.5), false);
assert_eq!(f32::is_int(&-2.5), false);
assert_eq!(f64::is_int(&f64::NAN), false);
```
*/
#[macro_export]
macro_rules! impl_trait {
    {$name:path, $block:tt, $($type:ty),+} => {
        $(
        impl $name for $type $block
        )+
        }
    }