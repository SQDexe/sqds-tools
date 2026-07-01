/*! A shorthand for converting different types into corresponding `ExitCode` values. */

#[cfg(feature = "std")]
use {
    core::{
        convert::Infallible,
        error::Error
        },
    std::process::ExitCode,
    crate::{
        impl_trait,
        select
        }
    };



/**
Type converting trait for retrieving [`ExitCode`] from values.

It's similar to the [`report`] method, except it doesn't cause any
side effects, like writing to the output.

Additionally, it provides implementation on a wider range of
types that could be interpreted as errors:

| Type | `SUCCESS` | `FAILURE` |
| ---- | --------- | --------- |
| `iN`, `uN` | `0` | `..` |
| `bool` | `true` | `false` |
| `Option<T>` | `Some(T)` | `None` |
| `Result<T, E>` | `Ok(T)` | `Err(E)` |
| `dyn Error` | - | always |
| `()` | always | - |

## Note on containers 

This trait usually resolves the exit status on container variant types,
rather than values they hold. For this reason, when one wants to decide
which value to use, based on the contents of types like `Option`, or `Result`,
the choice still comes down to using `match`, `map_or`, or a combination of `map`, and `unwrap_or`.

# Examples

```rust
# use {
#    std::process::ExitCode,
#    sqds_tools::ResolveExitCode
#    };
#
/* Example variables */
let error: Result<&str, usize> = Err(420);

/* Check whether it's correct */
assert_eq!(error.resolve(), ExitCode::FAILURE);
```

[`ExitCode`]: https://doc.rust-lang.org/std/process/struct.ExitCode.html
[`report`]: https://doc.rust-lang.org/std/process/trait.Termination.html#tymethod.report
*/
#[cfg(feature = "std")]
pub trait ResolveExitCode {
    /** Collapse the value into a corresponding exit code. */
    fn resolve(self) -> ExitCode;
    }

#[cfg(feature = "std")]
impl ResolveExitCode for ExitCode {
    fn resolve(self) -> ExitCode {
        match self {
            ExitCode::SUCCESS => ExitCode::SUCCESS,
            _ => ExitCode::FAILURE
            }
        }
    }

#[cfg(feature = "std")]
impl ResolveExitCode for () {
    fn resolve(self) -> ExitCode {
        ExitCode::SUCCESS
        }
    }

#[cfg(feature = "std")]
impl ResolveExitCode for Infallible {
    fn resolve(self) -> ExitCode {
        match self {}
        }
    }

#[cfg(feature = "std")]
impl_trait! {
    #[cfg(feature = "std")]
    ResolveExitCode,
    {
    fn resolve(self) -> ExitCode {
        ExitCode::FAILURE
        }
    },
    &dyn Error, &mut dyn Error
    }

#[cfg(feature = "std")]
impl<T> ResolveExitCode for Option<T> {
    fn resolve(self) -> ExitCode {
        match self {
            Some(_) => ExitCode::SUCCESS,
            None => ExitCode::FAILURE
            }
        }
    }

#[cfg(feature = "std")]
impl<T, E> ResolveExitCode for Result<T, E> {
    fn resolve(self) -> ExitCode {
        match self {
            Ok(_) => ExitCode::SUCCESS,
            Err(_) => ExitCode::FAILURE
            }
        }
    }

#[cfg(feature = "std")]
impl ResolveExitCode for bool {
    fn resolve(self) -> ExitCode {
        select!(
            self,
            ExitCode::SUCCESS,
            ExitCode::FAILURE
            )
        }
    }

#[cfg(feature = "std")]
impl_trait! {
    #[cfg(feature = "std")]
    ResolveExitCode,
    {
    fn resolve(self) -> ExitCode {
        match self {
            0 => ExitCode::SUCCESS,
            _ => ExitCode::FAILURE
            }
        }
    },
    u8, u16, u32, u64, u128, usize,
    i8, i16, i32, i64, i128, isize
    }



#[cfg(feature = "std")]
#[cfg(test)]
mod test {
    use {
        std::process::ExitCode,
        core::{
            fmt::{
                Display,
                Formatter,
                Result as FmtResult
                },
            error::Error
            },
        crate::ResolveExitCode
        };

    

    #[test]
    fn integers() {
        assert_eq!(0i32.resolve(), ExitCode::SUCCESS);
        assert_eq!(1i32.resolve(), ExitCode::FAILURE);
        assert_eq!(0u8.resolve(), ExitCode::SUCCESS);
        assert!((1 ..= u8::MAX).all(|n| n.resolve() == ExitCode::FAILURE));
        }

    #[test]
    fn option() {
        assert_eq!(Some(1).resolve(), ExitCode::SUCCESS);
        assert_eq!(Some(0).resolve(), ExitCode::SUCCESS);
        assert_eq!(Some("test").resolve(), ExitCode::SUCCESS);
        assert_eq!(None::<usize>.resolve(), ExitCode::FAILURE);
        assert_eq!(None::<&str>.resolve(), ExitCode::FAILURE);
        }

    #[test]
    fn result() {
        assert_eq!(Ok::<_, usize>(1).resolve(), ExitCode::SUCCESS);
        assert_eq!(Ok::<_, usize>(0).resolve(), ExitCode::SUCCESS);
        assert_eq!(Ok::<_, bool>("test").resolve(), ExitCode::SUCCESS);
        assert_eq!(Err::<usize, _>(true).resolve(), ExitCode::FAILURE);
        assert_eq!(Err::<bool, _>(6.9).resolve(), ExitCode::FAILURE);
        assert_eq!(Err::<&str, _>(Some("test")).resolve(), ExitCode::FAILURE);
        }

    #[test]
    fn error() {
        /* Define a test case error type */
        #[derive(Debug)]
        struct SomeError;
        impl Display for SomeError {
            fn fmt(&self, _: &mut Formatter<'_>) -> FmtResult { Ok(()) }
            }
        impl Error for SomeError {}

        let error = Box::new(SomeError) as Box<dyn Error>;

        assert_eq!(error.resolve(), ExitCode::FAILURE);
        }

    #[test]
    fn other() {
        assert_eq!(true.resolve(), ExitCode::SUCCESS);
        assert_eq!(false.resolve(), ExitCode::FAILURE);
        assert_eq!(().resolve(), ExitCode::SUCCESS);
        }
    }