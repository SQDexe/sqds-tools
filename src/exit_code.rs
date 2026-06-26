/*!  */

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



#[cfg(feature = "std")]
pub trait ResolveExitCode {
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
    ResolveExitCode,
    {
    fn resolve(self) -> ExitCode {
        ExitCode::FAILURE
        }
    },
    Box<dyn Error>, &dyn Error
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