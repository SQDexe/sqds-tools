/**
Synthetic sugar over memory layout functions.

This trait provides no further functionality other than functions which already exist inside the language.

# Examples

```rust
# use sqds_tools::LayoutMetrics;
# 
/* Example variables */
let int_size = i32::SIZE;
let array_size = <[u8; 16]>::SIZE;
let float_size = 6.9f64.size();
let string_size = String::new().size();

/* Check whether it's correct */
assert_eq!(int_size, 4);
assert_eq!(array_size, 16);
assert_eq!(float_size, 8);
assert_eq!(string_size, 24);
```

```rust
# use sqds_tools::LayoutMetrics;
# 
/* Example variables */
let int_align = i32::ALIGN;
let string_align = String::ALIGN;
let bool_align = true.align();
let array_align = [10u16; 8].align();

/* Check whether it's correct */
assert_eq!(int_align, 4);
assert_eq!(string_align, 8);
assert_eq!(bool_align, 1);
assert_eq!(array_align, 2);
```
*/
pub trait LayoutMetrics {
    /**
    The size of the type in bytes provided by
    [`size_of`](https://doc.rust-lang.org/std/mem/fn.size_of.html)
    */
    const SIZE: usize;
    /**
    The align of the type in bytes provided by
    [`align_of`](https://doc.rust-lang.org/std/mem/fn.align_of.html)
    */
    const ALIGN: usize;
    /**
    The size of the value in bytes provided by
    [`size_of_val`](https://doc.rust-lang.org/std/mem/fn.size_of_val.html)
    */
    fn size(&self) -> usize;
    /**
    The align of the value in bytes provided by
    [`align_of_val`](https://doc.rust-lang.org/std/mem/fn.align_of_val.html)
    */
    fn align(&self) -> usize;
    }

impl<T> LayoutMetrics for T {
    const SIZE: usize = size_of::<Self>();
    const ALIGN: usize = align_of::<Self>();
    fn size(&self) -> usize {
        size_of_val(self)
        }
    fn align(&self) -> usize {
        align_of_val(self)
        }
    }



#[cfg(test)]
mod test {
    use {
        std::vec::Vec,
        core::{
            mem::size_of,
            num::NonZeroI16
            },
        crate::layout::LayoutMetrics
        };

    #[allow(dead_code)]
    #[derive(Default)]
    struct Tmp0 {
        field0: &'static str,
        field1: bool,
        field2: u32,
        field3: Option<f64>
        }

    #[allow(dead_code)]
    enum Tmp1 {
        Type0(u8),
        Type1(u16),
        Type2(u32)
        }

    #[test]
    fn size_num() {
        assert_eq!(i8::SIZE, 1);
        assert_eq!(u16::SIZE, 2);
        assert_eq!(i32::SIZE, 4);
        assert_eq!(u64::SIZE, 8);
        assert_eq!(i128::SIZE, 16);
        assert_eq!(f32::SIZE, 4);
        assert_eq!(f64::SIZE, 8);
        }

    #[test]
    fn size_other() {
        assert_eq!(bool::SIZE, 1);
        assert_eq!(<[u16; 4]>::SIZE, 8);
        assert_eq!(<(bool, bool, bool)>::SIZE, 3);
        assert_eq!(<&str>::SIZE, 16);
        assert_eq!(Vec::<u8>::SIZE, 24);
        assert_eq!(Tmp0::SIZE, 40);
        assert_eq!(Tmp1::SIZE, 8);
        }

    #[test]
    fn size_equal() {
        assert_eq!(i32::SIZE, size_of::<i32>());
        assert_eq!(<&str>::SIZE, size_of::<&str>());
        assert_eq!(<[u8; 4]>::SIZE, size_of::<[u8; 4]>());
        assert_eq!(<(bool, i16)>::SIZE, size_of::<(bool, u16)>());
        assert_eq!(Option::<NonZeroI16>::SIZE, size_of::<Option<NonZeroI16>>());
        assert_eq!(Tmp0::SIZE, size_of::<Tmp0>());
        assert_eq!(Tmp1::SIZE, size_of::<Tmp1>());
        }

    #[test]
    fn size_value() {
        let int = 0i32;
        let unsigned = 0u8;
        let float = 0f64;
        let bool = true;
        let str = "test";
        let array = [0i8; 4];
        let tuple = (true, false);
        let option = Some(NonZeroI16::new(1).expect("Unreachable"));
        let enumerator = Tmp1::Type0(1);
        let structure = Tmp0::default();

        assert_eq!(int.size(), 4);
        assert_eq!(unsigned.size(), 1);
        assert_eq!(float.size(), 8);
        assert_eq!(bool.size(), 1);
        assert_eq!(str.size(), 16);
        assert_eq!(array.size(), 4);
        assert_eq!(tuple.size(), 2);
        assert_eq!(option.size(), 2);
        assert_eq!(enumerator.size(), 8);
        assert_eq!(structure.size(), 40);
        }

    #[test]
    fn size_value_equals() {
        let int = 1u128;
        let float = 10f32;
        let enumerator = Tmp1::Type0(1);
        let structure = Tmp0::default();

        assert_eq!(int.size(), size_of_val(&int));
        assert_eq!(float.size(), size_of_val(&float));
        assert_eq!(enumerator.size(), size_of_val(&enumerator));
        assert_eq!(structure.size(), size_of_val(&structure));
        }

    #[test]
    fn align_num() {
        assert_eq!(u8::ALIGN, 1);
        assert_eq!(i16::ALIGN, 2);
        assert_eq!(u32::ALIGN, 4);
        assert_eq!(i64::ALIGN, 8);
        assert_eq!(u128::ALIGN, 16);
        assert_eq!(f32::ALIGN, 4);
        assert_eq!(f64::ALIGN, 8);
        }

    #[test]
    fn align_other() {
        assert_eq!(bool::ALIGN, 1);
        assert_eq!(<[u16; 4]>::ALIGN, 2);
        assert_eq!(<(bool, bool, bool)>::ALIGN, 1);
        assert_eq!(<&str>::ALIGN, 8);
        assert_eq!(Vec::<u8>::ALIGN, 8);
        assert_eq!(Tmp0::ALIGN, 8);
        assert_eq!(Tmp1::ALIGN, 4);
        }

    #[test]
    fn align_equal() {
        assert_eq!(i32::ALIGN, align_of::<i32>());
        assert_eq!(<&str>::ALIGN, align_of::<&str>());
        assert_eq!(<[u8; 4]>::ALIGN, align_of::<[u8; 4]>());
        assert_eq!(<(bool, i16)>::ALIGN, align_of::<(bool, u16)>());
        assert_eq!(Option::<NonZeroI16>::ALIGN, align_of::<Option<NonZeroI16>>());
        assert_eq!(Tmp0::ALIGN, align_of::<Tmp0>());
        assert_eq!(Tmp1::ALIGN, align_of::<Tmp1>());
        }

    #[test]
    fn align_value() {
        let int = 0i32;
        let unsigned = 0u8;
        let float = 0f64;
        let bool = true;
        let str = "test";
        let array = [0i8; 4];
        let tuple = (true, false);
        let option = Some(NonZeroI16::new(1).expect("Unreachable"));
        let enumerator = Tmp1::Type0(1);
        let structure = Tmp0::default();

        assert_eq!(int.align(), 4);
        assert_eq!(unsigned.align(), 1);
        assert_eq!(float.align(), 8);
        assert_eq!(bool.align(), 1);
        assert_eq!(str.align(), 8);
        assert_eq!(array.align(), 1);
        assert_eq!(tuple.align(), 1);
        assert_eq!(option.align(), 2);
        assert_eq!(enumerator.align(), 4);
        assert_eq!(structure.align(), 8);
        }

    #[test]
    fn align_value_equals() {
        let int = 1u128;
        let float = 10f32;
        let enumerator = Tmp1::Type0(1);
        let structure = Tmp0::default();

        assert_eq!(int.align(), align_of_val(&int));
        assert_eq!(float.align(), align_of_val(&float));
        assert_eq!(enumerator.align(), align_of_val(&enumerator));
        assert_eq!(structure.align(), align_of_val(&structure));
        }

    #[test]
    fn align_equals_size() {
        assert_eq!(i16::ALIGN, i16::SIZE);
        assert_eq!(Tmp0::ALIGN, usize::SIZE);
        assert_eq!(Tmp1::ALIGN, u32::SIZE);
        }
    }