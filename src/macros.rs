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

/* Check whether it's correct */
assert_eq!(motd, "It's the weekend!");
```
*/
#[macro_export]
macro_rules! select {
    ($boolean:expr, $truthy:expr, $falsy:expr) => {
        match $boolean {
            true => $truthy,
            false => $falsy
            }
        };
    }



/**
A shorthand for extracting values from a fallible pattern matching expression.  

This macro, similarly to [`matches!`], expands into a `match`,  
which returns `Some` if the pattern fits, and `None` otherwise.  

It fits best with custom, broad `enum` types,  
but works just as well with `structs`, `slices`, `Options`, `Results`, etc.  

# Examples

```rust
# use sqds_tools::get_match;
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
assert_eq!(Some(false), get_match!(some_var, Test::Other(val) => val));
assert_eq!(None, get_match!(some_var, Test::YetAnother(val) if val < 5 => val));
```

When paired with a try operator `?`,  
this macro can be used as a convenient early return mechanism.

```rust
# use sqds_tools::get_match;
#
/* Example type */
enum Test<'a> {
    Variant(&'a [u8]),
    Other(bool),
    YetAnother
    }

/* Example function */
fn mul_enum_values(value: Test<'_>) -> Option<u8> {
    let (first, last) = get_match!(value, Test::Variant(&[first, .., last]) => (first, last))?;

    first.checked_mul(last)
    }

/* Check whether it's correct */
assert_eq!(Some(64), mul_enum_values(Test::Variant(&[4, 16])));
assert_eq!(None, mul_enum_values(Test::YetAnother));
```

[`matches!`]: https://doc.rust-lang.org/core/macro.matches.html
*/
#[macro_export]
macro_rules! get_match {
    ($value:expr, $pattern:pat $(if $guard:expr)? => $output:expr) => {
        match $value {
            $pattern $(if $guard)? => ::core::option::Option::Some($output),
            _ => ::core::option::Option::None,
            }
        };
    }



/**
A shorthand for extracting values from a infallible pattern matching expression.  

This macro is syntactically similar to [`matches!`],
yet expands into a simple destructuring assignment,  
and a return of the extracted value.  

It fits best with custom, broad `enum` types,  
but works just as well with `structs`, `slices`, `Options`, `Results`, etc.  

# Examples

```rust
# use sqds_tools::unpack_match;
#
/* Example type */
enum Role {
    Player ( String ),
    Op { name: String, permissions: u8 }
    }

/* Example variable */
let role = Role::Op {
    name: String::from("John Doe"),
    permissions: 0b1010
    };

/* Check whether it's correct */
assert_eq!("John Doe", unpack_match!(role, Role::Player(name) | Role::Op { name, .. } => name));
```

In most cases, it will usually be preferred,  
to extract such logic into a separate function.

```rust
# use sqds_tools::unpack_match;
#
/* Example type */
enum Character {
    Villager { health: u32 },
    Warrior { health: u32, damage: f32 },
    Mage { health: u32, mana: u8 }
    }

/* Example method */
impl Character {
    fn get_health(&self) -> u32 {
        unpack_match!(self,
            Self::Villager { health } |
            Self::Warrior { health, .. } |
            Self::Mage { health, .. } =>
            *health
            )
        }
    }

/* Example variable */
let my_character = Character::Mage { health: 100, mana: 50 };

/* Check whether it's correct */
assert_eq!(100, my_character.get_health());
```

[`matches!`]: https://doc.rust-lang.org/core/macro.matches.html
*/
#[macro_export]
macro_rules! unpack_match {
    ($value:expr, $pattern:pat => $output:expr) => {
        { let ( $pattern ) = $value; $output }
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

[`panic!`]: https://doc.rust-lang.org/core/macro.panic.html
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