use std::ops::{Shl, Sub};

#[cfg(feature = "128bit")]
use typenum::U128;
use typenum::{
    B1, IsGreaterOrEqual, IsLessOrEqual, NInt, NonZero, PInt, True, U1, U8, U16, U32, U64,
    Unsigned, Z0,
};

trait TypeBounds {
    type MIN;
    type MAX;
}

pub trait Representable<T> {
    const VALUE: T;
}

pub trait ToInt<T> {
    const INT: T;
}

macro_rules! impl_to_int_zero {
    ($($t:ty)*) => {
        $(
            impl ToInt<$t> for Z0
            {
                const INT: $t = 0;
            }
        )*
    }
}

impl_to_int_zero!(u8 u16 u32 u64 usize i8 i16 i32 i64 isize);
#[cfg(feature = "128bit")]
impl_to_int_zero!(u128 i128);

impl<T, U> ToInt<T> for PInt<U>
where
    U: Unsigned + NonZero,
    U: typenum::ToInt<T>,
{
    const INT: T = U::INT;
}

impl<T, U> ToInt<T> for NInt<U>
where
    U: Unsigned + NonZero,
    NInt<U>: typenum::ToInt<T>,
{
    const INT: T = <NInt<U> as typenum::ToInt<T>>::INT;
}

impl<C, T> Representable<T> for C
where
    T: TypeBounds,
    C: IsGreaterOrEqual<T::MIN, Output = True> + IsLessOrEqual<T::MAX, Output = True> + ToInt<T>,
{
    const VALUE: T = C::INT;
}

macro_rules! impl_bounds {
    ($t:ty, $min:ty, $max:ty) => {
        impl TypeBounds for $t {
            type MIN = $min;
            type MAX = $max;
        }
    };
}

macro_rules! impl_bounds_unsigned {
    ($t:ty, $bytes: ty) => {
        impl_bounds!(
            $t,
            Z0,
            PInt<<<U1 as Shl<$bytes>>::Output as Sub<B1>>::Output>
        );
    };
}

impl_bounds_unsigned!(u8, U8);
impl_bounds_unsigned!(u16, U16);
impl_bounds_unsigned!(u32, U32);
impl_bounds_unsigned!(u64, U64);
#[cfg(target_pointer_width = "16")]
impl_bounds_unsigned!(usize, U16);
#[cfg(target_pointer_width = "32")]
impl_bounds_unsigned!(usize, U32);
#[cfg(target_pointer_width = "64")]
impl_bounds_unsigned!(usize, U64);
#[cfg(feature = "128bit")]
impl_bounds_unsigned!(u128, U128);

macro_rules! impl_bounds_signed {
    ($t:ty, $bytes: ty) => {
        impl_bounds!(
            $t,
            NInt<<U1 as Shl<<$bytes as Sub<B1>>::Output>>::Output>,
            PInt<<<U1 as Shl<<$bytes as Sub<B1>>::Output>>::Output as Sub<B1>>::Output>
        );
    };
}

impl_bounds_signed!(i8, U8);
impl_bounds_signed!(i16, U16);
impl_bounds_signed!(i32, U32);
impl_bounds_signed!(i64, U64);
#[cfg(target_pointer_width = "16")]
impl_bounds_signed!(isize, U16);
#[cfg(target_pointer_width = "32")]
impl_bounds_signed!(isize, U32);
#[cfg(target_pointer_width = "64")]
impl_bounds_signed!(isize, U64);
#[cfg(feature = "128bit")]
impl_bounds_signed!(i128, U128);

#[cfg(test)]
mod tests {
    #[cfg(feature = "128bit")]
    use typenum::U120;
    use typenum::{
        Integer, N100, N10000, N1000000000, N1000000000000000000, P100, P150, P10000, P1000000000,
        P1000000000000000000, U10000000000000000000,
    };

    use super::*;

    #[test]
    fn u8() {
        assert_eq!(<<u8 as TypeBounds>::MAX as ToInt<u8>>::INT, u8::MAX);
        assert_eq!(<<u8 as TypeBounds>::MIN as ToInt<u8>>::INT, 0);
        assert_eq!(<P150 as Representable<u8>>::VALUE, 150);
    }

    #[test]
    fn u16() {
        assert_eq!(<<u16 as TypeBounds>::MAX as ToInt<u16>>::INT, u16::MAX);
        assert_eq!(<<u16 as TypeBounds>::MIN as ToInt<u16>>::INT, 0);
        assert_eq!(<P10000 as Representable<u16>>::VALUE, 10000);
    }

    #[test]
    fn u32() {
        assert_eq!(<<u32 as TypeBounds>::MAX as ToInt<u32>>::INT, u32::MAX);
        assert_eq!(<<u32 as TypeBounds>::MIN as ToInt<u32>>::INT, 0);
        assert_eq!(<P1000000000 as Representable<u32>>::VALUE, 1_000_000_000);
    }

    #[test]
    fn u64() {
        assert_eq!(<<u64 as TypeBounds>::MAX as ToInt<u64>>::INT, u64::MAX);
        assert_eq!(<<u64 as TypeBounds>::MIN as ToInt<u64>>::INT, 0);
        assert_eq!(
            <PInt<U10000000000000000000> as Representable<u64>>::VALUE,
            10_000_000_000_000_000_000
        );
    }

    #[test]
    fn usize() {
        assert_eq!(
            <<usize as TypeBounds>::MAX as ToInt<usize>>::INT,
            usize::MAX
        );
        assert_eq!(<<usize as TypeBounds>::MIN as ToInt<usize>>::INT, 0);
        assert_eq!(<P10000 as Representable<usize>>::VALUE, 10_000);
    }

    #[cfg(feature = "128bit")]
    #[test]
    fn u128() {
        assert_eq!(<<u128 as TypeBounds>::MAX as ToInt<u128>>::INT, u128::MAX);
        assert_eq!(<<u128 as TypeBounds>::MIN as ToInt<u128>>::INT, 0);
        assert_eq!(
            <PInt<<U1 as Shl<U120>>::Output> as Representable<u128>>::VALUE,
            1 << 120
        );
    }

    #[test]
    fn i8() {
        assert_eq!(<<i8 as TypeBounds>::MAX as Integer>::I8, i8::MAX);
        assert_eq!(<<i8 as TypeBounds>::MIN as Integer>::I8, i8::MIN);
        assert_eq!(<N100 as Representable<i8>>::VALUE, -100);
        assert_eq!(<P100 as Representable<i8>>::VALUE, 100);
    }

    #[test]
    fn i16() {
        assert_eq!(<<i16 as TypeBounds>::MAX as Integer>::I16, i16::MAX);
        assert_eq!(<<i16 as TypeBounds>::MIN as Integer>::I16, i16::MIN);
        assert_eq!(<N10000 as Representable<i16>>::VALUE, -10_000);
        assert_eq!(<P10000 as Representable<i16>>::VALUE, 10_000);
    }

    #[test]
    fn i32() {
        assert_eq!(<<i32 as TypeBounds>::MAX as Integer>::I32, i32::MAX);
        assert_eq!(<<i32 as TypeBounds>::MIN as Integer>::I32, i32::MIN);
        assert_eq!(<N1000000000 as Representable<i32>>::VALUE, -1_000_000_000);
        assert_eq!(<P1000000000 as Representable<i32>>::VALUE, 1_000_000_000);
    }

    #[test]
    fn i64() {
        assert_eq!(<<i64 as TypeBounds>::MAX as Integer>::I64, i64::MAX);
        assert_eq!(<<i64 as TypeBounds>::MIN as Integer>::I64, i64::MIN);
        assert_eq!(
            <N1000000000000000000 as Representable<i64>>::VALUE,
            -1_000_000_000_000_000_000
        );
        assert_eq!(
            <P1000000000000000000 as Representable<i64>>::VALUE,
            1_000_000_000_000_000_000
        );
    }

    #[test]
    fn isize() {
        assert_eq!(<<isize as TypeBounds>::MAX as Integer>::ISIZE, isize::MAX);
        assert_eq!(<<isize as TypeBounds>::MIN as Integer>::ISIZE, isize::MIN);
        assert_eq!(<N10000 as Representable<isize>>::VALUE, -10_000);
        assert_eq!(<P10000 as Representable<isize>>::VALUE, 10_000);
    }

    #[cfg(feature = "128bit")]
    #[test]
    fn i128() {
        assert_eq!(<<i128 as TypeBounds>::MAX as Integer>::I128, i128::MAX);
        assert_eq!(<<i128 as TypeBounds>::MIN as Integer>::I128, i128::MIN);
        assert_eq!(
            <NInt<<U1 as Shl<U120>>::Output> as Representable<i128>>::VALUE,
            -(1 << 120)
        );
        assert_eq!(
            <PInt<<U1 as Shl<U120>>::Output> as Representable<i128>>::VALUE,
            1 << 120
        );
    }
}
