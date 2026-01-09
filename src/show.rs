use core::{
    convert::Infallible,
    fmt::{
        Display,
        Formatter,
        Result as FmtResult,
        Write
        }
    };



/**
Empty, zero-sized printable struct.

The only use of this type is to be a plug,
for when there's a need for an argument with the [`Display`] trait.

In nearly all other cases, one should prefer the [`Infallible`], e.g.: when annotating a type.

# Examples

```rust
# use sqds_tools::EmptyDisplay;
#
/* Example variables */
let nothing = EmptyDisplay;

/* Check whether it's correct */
assert_eq!(nothing.to_string(), "");
```

### Intended use case: 

```rust
# use sqds_tools::{
#    EmptyDisplay as Empty,
#    ShowOption
#    };
#
/* Example variables */
let value = Some("Text...");

/* Check whether it's correct */
assert_eq!(
    value.show_or_affix(Empty, "> ", Empty)
        .to_string(),
    "> Text..."
    );
```

[`Display`]: https://doc.rust-lang.org/std/fmt/trait.Display.html
[`Infallible`]: https://doc.rust-lang.org/std/convert/enum.Infallible.html
*/
pub struct EmptyDisplay;

impl Display for EmptyDisplay {
    fn fmt(&self, _: &mut Formatter<'_>) -> FmtResult {
        Ok(())
        }
    }



/** Helper function for writing an Option to buffer. */
fn write_option<T, W>(w: &mut W, option: Option<T>) -> FmtResult
where T: Display, W: Write {
    match option {
        Some(value) =>
            write!(w, "{value}"),
        _ => Ok(())
        }
    }

/** Helper container representing a specific configuration of affixes. */
#[doc(hidden)]
enum Delimiter<T, U> {
    Prefix(T),
    Suffix(U),
    Enclosed {
        prefix: T,
        suffix: U
        }
    }

impl<T, U> Delimiter<T, U> {
    /** Container constructor. */
    fn new(prefix: Option<T>, suffix: Option<U>) -> Option<Self> {
        match (prefix, suffix) {
            (Some(prefix), Some(suffix)) =>
                Some(Self::Enclosed { prefix, suffix }),
            (Some(prefix), None) =>
                Some(Self::Prefix(prefix)),
            (None, Some(suffix)) =>
                Some(Self::Suffix(suffix)),
            (None, None) => None
            }
        }

    /** Helper function for extracting the prefix field. */
    const fn get_prefix(&self) -> Option<&T> {
        match self {
            Self::Prefix(prefix) | Self::Enclosed { prefix, .. } =>
                Some(prefix),
            _ => None
            }
        }

    /** Helper function for extracting the suffix field. */
    const fn get_suffix(&self) -> Option<&U> {
        match self {
            Self::Suffix(suffix) | Self::Enclosed { suffix, .. } =>
                Some(suffix),
            _ => None
            }
        }
    }

/** Helper container representing a value, together with its possible affix configuration. */
#[doc(hidden)]
struct Formatted<T, U, V> {
    value: T,
    delimiter: Option<Delimiter<U, V>>
    }

impl<T, U, V> Formatted<T, U, V> {
    /** Container constructor. */
    fn new(value: T, prefix: Option<U>, suffix: Option<V>) -> Self {
        Self {
            value,
            delimiter: Delimiter::new(prefix, suffix)
            }
        }
    }

impl<T, U, V> Display for Formatted<T, U, V>
where T: Display, U: Display, V: Display {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        let Formatted { value, .. } = self;
        let delimiter = self.delimiter.as_ref();

        /* Display logic for prefix */
        write_option(f, delimiter.and_then(Delimiter::get_prefix))?;
        
        /* Display logic for inner value */
        write!(f, "{value}")?;

        /* Display logic for prefix */
        write_option(f, delimiter.and_then(Delimiter::get_suffix))?;
        
        Ok(())
        }
    }



/**
Display wrapper struct for [`Option`].

A struct to help with [`Display`] implementations.

This struct only exists to output a user-facing representation of [`Option`] as text,
though this should not be taken as a golden standard for representing the type.

This struct is outputted by functions of the [`ShowOption`] trait.

[`Option`]: https://doc.rust-lang.org/std/option/enum.Option.html
[`Display`]: https://doc.rust-lang.org/std/fmt/trait.Display.html
[`ShowOption`]: ./trait.ShowOption.html
*/
#[must_use = "this is a `Display` wrapper, which should be used"]
pub struct DisplayOption<'a, T, U, V, W> {
    inner: Result<Formatted<&'a T, V, W>, U>,
    }

impl<'a, T, U, V, W> Display for DisplayOption<'a, T, U, V, W>
where T: Display, U: Display, V: Display, W: Display {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        /* Match inner result */
        match self.inner.as_ref() {
            /* Display logic for Some variant */
            Ok(some_display) =>
                write!(f, "{some_display}"),
            /* Display logic for None variant */
            Err(none_display) =>
                write!(f, "{none_display}")
            }
        }
    }

/**
Value printing trait for [`Option`] types.

This trait provides a set of functions output [`DisplayOption`] wrapper struct,
which implements the [`Display`] trait.
It's main purpose,
is to take a short-lived reference to the value,
and then print a user-facing representation of the [`Option`].

Some functions allow passing a few additional values,
which can be used to improve formatting.
Meanwhile other functions have predefined values,
as shorthands for more popular combinations.

For this trait to work,
it needs **all** of the values to implement the [`Display`] trait.

# Disclaimer

Rust's core guidelines give a reason,
why [`Option`] type doesn't implement [`Display`] trait,
and why it can't be derived.

Therefore, one should take usage of this trait with a pinch of salt,
as it might not suit everyone's vision and their needs.

# Examples

```rust
# use sqds_tools::ShowOption;
#
/* Example variables */
let tmp = Some("It is Wednesday my dudes!");

/* Check whether it's correct */
assert_eq!(
    tmp.show_or_none()
        .to_string(),
    "It is Wednesday my dudes!"
    );
```

```rust
# use sqds_tools::ShowOption;
#
/* Example variables */
let tmp: Option<bool> = None;

/* Check whether it's correct */
assert_eq!(
    tmp.show_or("Null")
        .to_string(),
    "Null"
    );
```

```rust
# use sqds_tools::ShowOption;
#
/* Example variables */
let tmp = Some(420);

/* Check whether it's correct */
assert_eq!(
    tmp.show_or_affix("Nothing", "Can we ", " today?")
        .to_string(),
    "Can we 420 today?"
    );
```

[`Option`]: https://doc.rust-lang.org/std/option/enum.Option.html
[`Display`]: https://doc.rust-lang.org/std/fmt/trait.Display.html
[`DisplayOption`]: ./struct.DisplayOption.html
*/
pub trait ShowOption<T>
where T: Display {
    /**
    Show `Option`s *affixed* `Some` variant, or the other provided value,
    by individually passing `Option`s for each affix.

    Wrapper returned by this function might need additional type annotation.
    It's advised to use the [`Infallible`] type whenever possible.

    [`Infallible`]: https://doc.rust-lang.org/std/convert/enum.Infallible.html
    */
    #[must_use = "this function returns a `Display` wrapper, which should be used"]
    fn show_or_manual<U, V, W>(&self, other: U, prefix: Option<V>, suffix: Option<W>) -> DisplayOption<'_, T, U, V, W>
    where U: Display, V: Display, W: Display;
    /** Show `Option`s *affixed* `Some` variant, or the other provided value. */
    #[must_use = "this function returns a `Display` wrapper, which should be used"]
    fn show_or_affix<U, V, W>(&self, other: U, prefix: V, suffix: W) -> DisplayOption<'_, T, U, V, W>
    where U: Display, V: Display, W: Display;
    /** Show `Option`s `Some` variant, or the other provided value. */
    #[must_use = "this function returns a `Display` wrapper, which should be used"]
    fn show_or<U>(&self, other: U) -> DisplayOption<'_, T, U, Infallible, Infallible>
    where U: Display;
    /** Show `Option`s `Some` variant, or a `"None"` text. */
    #[must_use = "this function returns a `Display` wrapper, which should be used"]
    fn show_or_none(&self) -> DisplayOption<'_, T, &str, Infallible, Infallible>;
    }

impl<T> ShowOption<T> for Option<T>
where T: Display {
    fn show_or_manual<U, V, W>(&self, other: U, prefix: Option<V>, suffix: Option<W>) -> DisplayOption<'_, T, U, V, W>
    where U: Display, V: Display, W: Display {
        /* Construct the inner value of the struct for the basic case */
        let inner = self.as_ref()
            .map(|val| Formatted::new(val, prefix, suffix))
            .ok_or(other);

        /* Output */
        DisplayOption { inner }
        }
    fn show_or_affix<U, V, W>(&self, other: U, prefix: V, suffix: W) -> DisplayOption<'_, T, U, V, W>
    where U: Display, V: Display, W: Display {
        self.show_or_manual(other, Some(prefix), Some(suffix))
        }
    fn show_or<U>(&self, other: U) -> DisplayOption<'_, T, U, Infallible, Infallible>
    where U: Display {
        self.show_or_manual(other, None, None)
        }
    fn show_or_none(&self) -> DisplayOption<'_, T, &str, Infallible, Infallible> {
        self.show_or("None")
        }
    }



/**
Display wrapper struct for [`Result`].

A struct to help with [`Display`] implementations.

This struct only exists to output a user-facing representation of [`Result`] as text,  
though this should not be taken as a golden standard for representing the type.

This struct is outputted by functions of the [`ShowResult`] trait.

[`Result`]: https://doc.rust-lang.org/std/result/enum.Result.html
[`Display`]: https://doc.rust-lang.org/std/fmt/trait.Display.html
[`ShowResult`]: ./trait.ShowResult.html
*/
#[must_use = "this is a `Display` wrapper, which should be used"]
pub struct DisplayResult<'a, T, U, V, W, X, Y> {
    inner: Result<Formatted<&'a T, V, W>, Formatted<&'a U, X, Y>>,
    }

impl<T, U, V, W, X, Y> Display for DisplayResult<'_, T, U, V, W, X, Y>
where T: Display, U: Display, V: Display, W: Display, X: Display, Y: Display {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        /* Match inner result */
        match self.inner.as_ref() {
            /* Display logic for Ok variant */
            Ok(ok_display) =>
                write!(f, "{ok_display}"),
            /* Display logic for Err variant */
            Err(err_display) =>
                write!(f, "{err_display}")
            }
        }
    }

/**
Value printing trait for [`Result`] types.

This trait provides a set of functions output [`DisplayResult`] wrapper struct,
which implements the [`Display`] trait.
It's main purpose,
is to take a short-lived reference to the value,
and then print a user-facing representation of the [`Result`].

Some functions allow passing a few additional values,
which can be used to improve formatting.
Meanwhile other functions have predefined values,
as shorthands for more popular combinations.

For this trait to work,
it needs **all** of the values to implement the [`Display`] trait.

# Disclaimer

Rust's core guidelines give a reason,
why [`Result`] type doesn't implement [`Display`] trait,
and why it can't be derived.

Therefore, one should take usage of this trait with a pinch of salt,
as it might not suit everyone's vision and their needs.

# Examples

```rust
# use sqds_tools::ShowResult;
#
/* Example variables */
let tmp: Result<&str, bool> = Ok("Nah, I'd win");

/* Check whether it's correct */
assert_eq!(
    tmp.show_either()
        .to_string(),
    "Nah, I'd win"
    );
```

```rust
# use sqds_tools::ShowResult;
#
/* Example variables */
let tmp: Result<&str, usize> = Err(69);

/* Check whether it's correct */
assert_eq!(
    tmp.show_either()
        .to_string(),
    "69"
    );
```

```rust
# use sqds_tools::ShowResult;
#
/* Example variables */
let tmp: Result<u16, bool> = Ok(515);

/* Check whether it's correct */
assert_eq!(
    tmp.show_affix_ok("She had ", " on her shoulder")
        .to_string(),
    "She had 515 on her shoulder"
    );
```

[`Result`]: https://doc.rust-lang.org/std/result/enum.Result.html
[`Display`]: https://doc.rust-lang.org/std/fmt/trait.Display.html
[`DisplayResult`]: ./struct.DisplayResult.html
*/
pub trait ShowResult<T, E>
where T: Display, E: Display {
    /**
    Show `Result`s *affixed* `Ok` variant, or its *affixed* `Err` variant,
    by individually passing `Option`s for each affix.

    Wrapper returned by this function might need additional type annotation.
    It's advised to use the [`Infallible`] type whenever possible.

    [`Infallible`]: https://doc.rust-lang.org/std/convert/enum.Infallible.html
    */
    #[must_use = "this function returns a `Display` wrapper, which should be used"]
    fn show_manual<U, V, W, X>(&self,  prefix: Option<U>, suffix: Option<V>, err_prefix: Option<W>, err_suffix: Option<X>) -> DisplayResult<'_, T, E, U, V, W, X>
    where U: Display, V: Display, W: Display, X: Display;
    /** Show `Result`s *affixed* `Ok` variant, or its `Err` variant. */
    #[must_use = "this function returns a `Display` wrapper, which should be used"]
    fn show_affix_ok<U, V>(&self, prefix: U, suffix: V) -> DisplayResult<'_, T, E, U, V, Infallible, Infallible>
    where U: Display, V: Display;
    /** Show `Result`s `Ok` variant, or its *affixed* `Err` variant, */
    #[must_use = "this function returns a `Display` wrapper, which should be used"]
    fn show_affix_err<U, V>(&self, err_prefix: U, err_suffix: V) -> DisplayResult<'_, T, E, Infallible, Infallible, U, V>
    where U: Display, V: Display;
    /** Show `Result`s *affixed* `Ok` variant, or its *affixed* `Err` variant. */
    #[must_use = "this function returns a `Display` wrapper, which should be used"]
    fn show_affix<U, V, W, X>(&self,  prefix: U, suffix: V, err_prefix: W, err_suffix: X) -> DisplayResult<'_, T, E, U, V, W, X>
    where U: Display, V: Display, W: Display, X: Display;
    /** Show `Result`s `Ok` variant, or its `Err` variant. */
    #[must_use = "this function returns a `Display` wrapper, which should be used"]
    fn show_either(&self) -> DisplayResult<'_, T, E, Infallible, Infallible, Infallible, Infallible>;
    }

impl<T, E> ShowResult<T, E> for Result<T, E>
where T: Display, E: Display {
    fn show_manual<U, V, W, X>(&self,  prefix: Option<U>, suffix: Option<V>, err_prefix: Option<W>, err_suffix: Option<X>) -> DisplayResult<'_, T, E, U, V, W, X>
        where U: Display, V: Display, W: Display, X: Display {
        /* Construct the inner value of the struct for the basic case */
        let inner = self.as_ref()
            .map(|ok| Formatted::new(ok, prefix, suffix))
            .map_err(|err| Formatted::new(err, err_prefix, err_suffix));

        /* Output */
        DisplayResult { inner }
        }
    fn show_affix_ok<U, V>(&self, prefix: U, suffix: V) -> DisplayResult<'_, T, E, U, V, Infallible, Infallible>
    where U: Display, V: Display {
        self.show_manual(Some(prefix), Some(suffix), None, None)
        }
    fn show_affix_err<U, V>(&self, err_prefix: U, err_suffix: V) -> DisplayResult<'_, T, E, Infallible, Infallible, U, V>
    where U: Display, V: Display {
        self.show_manual(None, None, Some(err_prefix), Some(err_suffix))
        }
    fn show_affix<U, V, W, X>(&self,  prefix: U, suffix: V, err_prefix: W, err_suffix: X) -> DisplayResult<'_, T, E, U, V, W, X>
    where U: Display, V: Display, W: Display, X: Display {
        self.show_manual(Some(prefix), Some(suffix), Some(err_prefix), Some(err_suffix))
        }
    fn show_either(&self) -> DisplayResult<'_, T, E, Infallible, Infallible, Infallible, Infallible> {
        self.show_manual(None, None, None, None)
        }
    }



/**
Display wrapper struct for [`Slice`].

A struct to help with [`Display`] implementations.

This struct only exists to output a user-facing representation of [`Slice`] as text,  
though this should not be taken as a golden standard for representing the type.

This struct is outputted by functions of the [`ShowSlice`] trait.

[`Slice`]: https://doc.rust-lang.org/std/primitive.slice.html
[`Display`]: https://doc.rust-lang.org/std/fmt/trait.Display.html
[`ShowSlice`]: ./trait.ShowSlice.html
*/
#[must_use = "this is a `Display` wrapper, which should be used"]
pub struct DisplaySlice<'a, T, U, V, W> {
    inner: Formatted<&'a [T], U, W>,
    separator: Option<V>
    }

impl<T, U, V, W> Display for DisplaySlice<'_, T, U, V, W>
where T: Display, U: Display, V: Display, W: Display {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        let (mut iter, delimiter, separator) = (
            self.inner.value.into_iter(),
            self.inner.delimiter.as_ref(),
            self.separator.as_ref()
            );

        /* Display logic for prefix */
        write_option(f, delimiter.and_then(Delimiter::get_prefix))?;

        /* Display logic for first item */
        write_option(f, iter.next())?;        
        
        /* Display rest of the iterator */
        for item in iter {
            /* Display logic for separator */
            write_option(f, separator)?;

            /* Display logic for single item */
            write!(f, "{item}")?;
            }

        /* Display logic for prefix */
        write_option(f, delimiter.and_then(Delimiter::get_suffix))?;

        Ok(())
        }
    }

/**
Value printing trait for [`Slice`] types.

This trait provides a set of functions output [`DisplaySlice`] wrapper struct,
which implements the [`Display`] trait.
It's main purpose,
is to take a short-lived reference to the value,
and then print a user-facing representation of the [`Slice`].

Some functions allow passing a few additional values,
which can be used to improve formatting.
Meanwhile other functions have predefined values,
as shorthands for more popular combinations.

For this trait to work,
it needs **all** of the values to implement the [`Display`] trait.

# Disclaimer

Rust's core guidelines give a reason,
why [`Slice`] type doesn't implement [`Display`] trait,
and why it can't be derived.

Therefore, one should take usage of this trait with a pinch of salt,
as it might not suit everyone's vision and their needs.

# Examples

```rust
# use sqds_tools::ShowSlice;
#
/* Example variables */
let tmp = vec![1, 2, 3];

/* Check whether it's correct */
assert_eq!(
    tmp.show_slice()
        .to_string(),
    "[1, 2, 3]"
    );
```

```rust
# use sqds_tools::ShowSlice;
#
/* Example variables */
let tmp = ["foo", "bar"];

/* Check whether it's correct */
assert_eq!(
    tmp.show_concat()
        .to_string(),
    "foobar"
    );
```

```rust
# use sqds_tools::ShowSlice;
#
/* Example variables */
let tmp = "K9evt2";

/* Check whether it's correct */
assert_eq!(
    tmp.as_bytes().show_join(';')
        .to_string(),
    "75;57;101;118;116;50"
    );
```

[`Slice`]: https://doc.rust-lang.org/std/primitive.slice.html
[`Display`]: https://doc.rust-lang.org/std/fmt/trait.Display.html
[`DisplaySlice`]: ./struct.DisplaySlice.html
*/
pub trait ShowSlice<T>
where T: Display {
    /**
    Show the `Slice`'s elements *enclosed*, and *separated* by the provided values,
    by individually passing `Option`s for each one.

    Wrapper returned by this function might need additional type annotation.
    It's advised to use the [`Infallible`] type whenever possible.

    [`Infallible`]: https://doc.rust-lang.org/std/convert/enum.Infallible.html
    */
    #[must_use = "this function returns a `Display` wrapper, which should be used"]
    fn show_manual<U, V, W>(&self, prefix: Option<U>, separator: Option<V>, suffix: Option<W>) -> DisplaySlice<'_, T, U, V, W>
    where U: Display, V: Display, W: Display;
    /** Show the `Slice`'s elements *enclosed*, and *separated* by the provided values. */
    #[must_use = "this function returns a `Display` wrapper, which should be used"]
    fn show_enclosed<U, V, W>(&self, prefix: U, separator: V, suffix: W) -> DisplaySlice<'_, T, U, V, W>
    where U: Display, V: Display, W: Display;
    /** Show the `Slice`'s elements *separated* by the provided value. */
    #[must_use = "this function returns a `Display` wrapper, which should be used"]
    fn show_join<U>(&self, separator: U) -> DisplaySlice<'_, T, Infallible, U, Infallible>
    where U: Display;
    /** Show the `Slice`'s elements directly by attaching them without any affixes, or separatorst. */
    #[must_use = "this function returns a `Display` wrapper, which should be used"]
    fn show_concat(&self) -> DisplaySlice<'_, T, Infallible, Infallible, Infallible>;
    /** Show the `Slice`'s elements *enclosed* by `'['`, `']'` signs, and *separated* by `", "` text. */
    #[must_use = "this function returns a `Display` wrapper, which should be used"]
    fn show_slice(&self) -> DisplaySlice<'_, T, char, &str, char>;
    }

impl<T> ShowSlice<T> for [T]
where T: Display {
    fn show_manual<U, V, W>(&self, prefix: Option<U>, separator: Option<V>, suffix: Option<W>) -> DisplaySlice<'_, T, U, V, W>
    where U: Display, V: Display, W: Display {
        /* Construct the inner value of the struct for the basic case */
        DisplaySlice {
            inner: Formatted::new(self, prefix, suffix),
            separator
            }
        }
    fn show_enclosed<U, V, W>(&self, prefix: U, separator: V, suffix: W) -> DisplaySlice<'_, T, U, V, W>
    where U: Display, V: Display, W: Display {
        self.show_manual(Some(prefix), Some(separator), Some(suffix))
        }
    fn show_join<U>(&self, separator: U) -> DisplaySlice<'_, T, Infallible, U, Infallible>
    where U: Display {
        self.show_manual(None, Some(separator), None)
        }
    fn show_concat(&self) -> DisplaySlice<'_, T, Infallible, Infallible, Infallible> {
        self.show_manual(None, None, None)
        }
    fn show_slice(&self) -> DisplaySlice<'_, T, char, &str, char> {
        self.show_enclosed('[', ", ", ']')
        }
    }


/** Top level test module. */
#[cfg(test)]
mod test {

mod option {
    use {
        std::string::ToString,
        crate::show::{
            EmptyDisplay as Empty,
            ShowOption
            }
        };

    #[test]
    fn basic() {
        let num = Some(420).show_or_none()
            .to_string();
        let dec = Some(21.37).show_or_none()
            .to_string();
        let bool = Some(true).show_or_none()
            .to_string();
        let string = Some("test".to_string()).show_or_none()
            .to_string();
        let none = Option::<bool>::None.show_or_none()
            .to_string();

        assert_eq!(num, "420");
        assert_eq!(dec, "21.37");
        assert_eq!(bool, "true");
        assert_eq!(string, "test");
        assert_eq!(none, "None");
        }

    #[test]
    fn no_value() {
        let num = Option::<bool>::None.show_or(69)
            .to_string();
        let bool = Option::<bool>::None.show_or(false)
            .to_string();
        let str = Option::<bool>::None.show_or("tmp")
            .to_string();

        assert_eq!(num, "69");
        assert_eq!(bool, "false");
        assert_eq!(str, "tmp");
        }

    #[test]
    fn affixed() {
        let value = Some(true);

        let pref = value.show_or_affix(Empty, "This is prefixed... ", Empty)
            .to_string();
        let suff = value.show_or_affix(Empty, Empty, " ...this is suffixed")
            .to_string();
        let affix = value.show_or_affix(Empty, "From start... ", " ...to finish")
            .to_string();
        let none = Option::<bool>::None.show_or_affix("Nil", "This wont show", "This one too")
            .to_string();

        assert_eq!(pref, "This is prefixed... true");
        assert_eq!(suff, "true ...this is suffixed");
        assert_eq!(affix, "From start... true ...to finish");
        assert_eq!(none, "Nil");
        }

    #[test]
    fn not_owning() {
        let mut value = Some("Half".to_string());
        let mut suff = "life".to_string();

        assert_eq!(
            value.show_or_affix(Empty, Empty, &suff).to_string(),
            "Halflife"
            );
    
        if let Some(val) = value.as_mut() {
            val.push('-');
            suff.push('3');
            }

        assert_eq!(
            value.show_or_affix(Empty, Empty, &suff).to_string(),
            "Half-life3"
            );
        }
    }

mod result {
    use {
        std::string::ToString,
        crate::show::{
            EmptyDisplay as Empty,
            ShowResult
            }
        };

    #[test]
    fn basic() {
        let num = Result::<u16, bool>::Ok(1984).show_either()
            .to_string();
        let dec = Result::<f64, bool>::Ok(451.15).show_either()
            .to_string();
        let bool = Result::<bool, bool>::Ok(true).show_either()
            .to_string();
        let str = Result::<&str, bool>::Ok("Done!").show_either()
            .to_string();

        assert_eq!(num, "1984");
        assert_eq!(dec, "451.15");
        assert_eq!(bool, "true");
        assert_eq!(str, "Done!");
        }

    #[test]
    fn err_value() {
        let num = Result::<bool, u16>::Err(1987).show_either()
            .to_string();
        let dec = Result::<bool, f64>::Err(273.16).show_either()
            .to_string();
        let bool = Result::<bool, bool>::Err(false).show_either()
            .to_string();
        let str = Result::<bool, &str>::Err("Dang!").show_either()
            .to_string();

        assert_eq!(num, "1987");
        assert_eq!(dec, "273.16");
        assert_eq!(bool, "false");
        assert_eq!(str, "Dang!");
        }

    #[test]
    fn affixed() {
        let value_ok: Result<bool, bool> = Ok(true);
        let value_err: Result<bool, bool> = Err(false);

        let pref_ok = value_ok.show_affix_ok("This is prefixed... ", Empty)
            .to_string();
        let suff_ok = value_ok.show_affix_ok(Empty, " ...this is suffixed")
        .to_string();
        let affix_ok = value_ok.show_affix_ok("From start... ", " ...to finish")
            .to_string();
        let pref_err = value_err.show_affix_err("This is prefixed... ", Empty)
            .to_string();
        let suff_err = value_err.show_affix_err(Empty, " ...this is suffixed")
            .to_string();
        let affix_err = value_err.show_affix_err("From start... ", " ...to finish")
            .to_string();

        assert_eq!(pref_ok, "This is prefixed... true");
        assert_eq!(suff_ok, "true ...this is suffixed");
        assert_eq!(affix_ok, "From start... true ...to finish");
        assert_eq!(pref_err, "This is prefixed... false");
        assert_eq!(suff_err, "false ...this is suffixed");
        assert_eq!(affix_err, "From start... false ...to finish");
        }

    #[test]
    fn not_owning() {
        let mut value_ok: Result<_, bool> = Ok("Excellent".to_string());
        let mut value_err: Result<bool, _> = Err("Preposterous".to_string());
        let mut pref = "Quoted".to_string();

        assert_eq!(
            value_ok.show_affix_ok(&pref, Empty).to_string(),
            "QuotedExcellent"
            );
        assert_eq!(
            value_err.show_affix_err(&pref, Empty).to_string(),
            "QuotedPreposterous"
            );
    
        if let (Ok(val_ok), Err(val_err)) = (value_ok.as_mut(), value_err.as_mut()) {
            val_ok.push('!');
            val_err.push('?');
            pref.push(':');
            }

        assert_eq!(
            value_ok.show_affix_ok(&pref, Empty).to_string(),
            "Quoted:Excellent!"
            );
        assert_eq!(
            value_err.show_affix_err(&pref, Empty).to_string(),
            "Quoted:Preposterous?"
            );
        }
    }

mod slice {
    use {
        std::{
            vec,
            string::ToString,
            vec::Vec
            },
        crate::show::{
            EmptyDisplay as Empty,
            ShowOption,
            ShowSlice
            }
        };

    #[test]
    fn basic() {
        let array = [0; 4].show_slice()
            .to_string();
        let vec = vec![true, false, true].show_slice()
            .to_string();
        let slice = "abc".as_bytes().show_slice()
            .to_string();

        assert_eq!(array, "[0, 0, 0, 0]");
        assert_eq!(vec, "[true, false, true]");
        assert_eq!(slice, "[97, 98, 99]");
        }

    #[test]
    fn other_methods() {
        let value = [0, 1, 2, 3];
        
        let concat = value.show_concat()
            .to_string();
        let join = value.show_join(',')
            .to_string();
        let enclosed = value.show_enclosed('{', ';', '}')
            .to_string();

        assert_eq!(concat, "0123");
        assert_eq!(join, "0,1,2,3");
        assert_eq!(enclosed, "{0;1;2;3}");
        }

    #[test]
    fn affixed() {
        let value = [0, 1, 2, 3];
        
        let pref = value.show_enclosed("These are the elemetns... ", ',', Empty)
            .to_string();
        let suff = value.show_enclosed(Empty, ',', " ...are the elements")
            .to_string();
        let affix = value.show_enclosed("The... ", ',', " ...elements")
            .to_string();
        let none = value.show_enclosed(Empty, ',', Empty)
            .to_string();

        assert_eq!(pref, "These are the elemetns... 0,1,2,3");
        assert_eq!(suff, "0,1,2,3 ...are the elements");
        assert_eq!(affix, "The... 0,1,2,3 ...elements");
        assert_eq!(none, "0,1,2,3");
        }

    #[test]
    fn empty() {
        let array = [0; 0].show_slice()
            .to_string();
        let vec = vec![true; 0].show_slice()
            .to_string();
        let slice = "".as_bytes().show_slice()
            .to_string();

        assert_eq!(array, "[]");
        assert_eq!(vec, "[]");
        assert_eq!(slice, "[]");
        }

    #[test]
    fn single() {
        let array = [1; 1].show_slice()
            .to_string();
        let vec = vec![true; 1].show_slice()
            .to_string();
        let slice = "A".as_bytes().show_slice()
            .to_string();

        assert_eq!(array, "[1]");
        assert_eq!(vec, "[true]");
        assert_eq!(slice, "[65]");
        }

    #[test]
    fn not_owning() {
        let mut value = vec![0, 1, 2, 3];

        assert_eq!(
            value.show_slice().to_string(),
            "[0, 1, 2, 3]"
            );

        value.push(4);

        assert_eq!(
            value.show_slice().to_string(),
            "[0, 1, 2, 3, 4]"
            );
        }

    #[test]
    fn combined() {
        let value = vec![
            Some(1),
            None,
            Some(2),
            None,
            Some(3)
            ];

        assert_eq!(
            value.iter()
                .map(|e| e.show_or("NaN"))
                .collect::<Vec<_>>()
                .show_slice()
                .to_string(),
            "[1, NaN, 2, NaN, 3]"
            )
        }
    }

}