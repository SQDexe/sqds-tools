use core::mem::size_of;


/// Handful shorthand for [`size_of`].
/// 
/// # Examples
/// 
/// ```rust
/// # use sqds_tools::Size;
/// # 
/// /* Example variables */
/// let int_size = u64::BYTES;
/// let string_size = String::BYTES;
/// let array_size = <[u8; 16]>::BYTES;
/// 
/// /* Check whether it's correct */
/// assert_eq!(int_size, 8);
/// assert_eq!(string_size, 24);
/// assert_eq!(array_size, 16);
/// ```
/// 
/// [`size_of`]: https://doc.rust-lang.org/std/mem/fn.size_of.html
pub trait Size {
    /// The size of the type in bytes
    const BYTES: usize;
    }

impl<T> Size for T {
    const BYTES: usize = size_of::<T>();
    }


#[cfg(test)]
mod test {
    use {
        std::vec::Vec,
        core::{
            mem::size_of,
            num::NonZeroI16
            },
        crate::Size
        };

    struct Tmp {
        _field0: &'static str,
        _field1: bool,
        _field2: u32,
        _field3: Option<f64>
        }

    #[test]
    fn size_int() {
        assert_eq!(i8::BYTES, 1);
        assert_eq!(i16::BYTES, 2);
        assert_eq!(i32::BYTES, 4);
        assert_eq!(i64::BYTES, 8);
        assert_eq!(i128::BYTES, 16);
        }

    #[test]
    fn size_uint() {
        assert_eq!(u8::BYTES, 1);
        assert_eq!(u16::BYTES, 2);
        assert_eq!(u32::BYTES, 4);
        assert_eq!(u64::BYTES, 8);
        assert_eq!(u128::BYTES, 16);
        }

    #[test]
    fn size_float() {
        assert_eq!(f32::BYTES, 4);
        assert_eq!(f64::BYTES, 8);
        }

    #[test]
    fn size_other() {
        assert_eq!(bool::BYTES, 1);
        assert_eq!(<[u16; 4]>::BYTES, 8);
        assert_eq!(<(bool, bool, bool)>::BYTES, 3);
        assert_eq!(<&str>::BYTES, 16);
        assert_eq!(Vec::<u8>::BYTES, 24);
        assert_eq!(Tmp::BYTES, 40);
        }

    #[test]
    fn size_equal() {
        assert_eq!(i32::BYTES, size_of::<i32>());
        assert_eq!(<&str>::BYTES, size_of::<&str>());
        assert_eq!(<[u8; 4]>::BYTES, size_of::<[u8; 4]>());
        assert_eq!(<(bool, i16)>::BYTES, size_of::<(bool, u16)>());
        assert_eq!(Option::<NonZeroI16>::BYTES, size_of::<Option<NonZeroI16>>());
        assert_eq!(Tmp::BYTES, size_of::<Tmp>())
        }
    }