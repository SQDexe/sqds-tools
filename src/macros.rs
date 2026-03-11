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
A shorthand for extracting values from a fallible pattern matching expression.  

This macro, similarly to [`matches!`], expands into a `match`,  
which returns `Some` if the pattern fits, and `None` otherwise.  

It fits best with custom, broad `enum`` types,  
but works just as well with `structs`, `slices`, `Options`, `Results`, etc.  

# Examples

```rust
# use sqds_tools::try_match;
#
/* Example type */
enum Test {
    Variant,
    Other(bool),
    YetAnother(u8)
    }

/* Example variable */
let some_var = Test::Other(false);

/* Check whether it's correct */
assert_eq!(Some(false), try_match!(some_var, Test::Other(val) => val));
assert_eq!(None, try_match!(some_var, Test::YetAnother(val) if val < 5 => val));
```

When paired with a try operator `?`,  
this macro can be used as a convenient early return mechanism.

```rust
# use sqds_tools::try_match;
#
/* Example type */
enum Test<'a> {
    Variant(&'a [u8]),
    Other(bool),
    YetAnother
    }

/* Example function */
fn sum_enum_values(value: Test<'_>) -> Option<u8> {
    let [first, last] = try_match!(value, Test::Variant([first, .., last]) => [first, last])?;

    Some(first * last)
    }

/* Check whether it's correct */
assert_eq!(Some(64), sum_enum_values(Test::Variant(&[4, 16])));
assert_eq!(None, sum_enum_values(Test::YetAnother));
```

[`matches!`]: https://doc.rust-lang.org/core/macro.matches.html
*/
#[macro_export]
macro_rules! try_match {
    ($value:expr, $pattern:pat $(if $guard:expr)? => $output:expr) => {
        match $value {
            $pattern $(if $guard)? => ::core::option::Option::Some($output),
            _ => ::core::option::Option::None,
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