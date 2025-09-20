use typenum::{Const, IsLess, NonZero, PInt, ToUInt, True, U, Unsigned};

use crate::{BoundedNum, Representable};

pub trait SafeGet<T, I> {
    fn get_safe(&self, index: I) -> &T;
    fn get_safe_mut(&mut self, index: I) -> &mut T;
}

impl<T, Min, Max, const N: usize> SafeGet<T, BoundedNum<usize, Min, Max>> for [T; N]
where
    Min: Representable<usize>,
    Max: Representable<usize> + IsLess<PInt<U<N>>, Output = True>,
    U<N>: Unsigned + NonZero,
    Const<N>: ToUInt,
{
    fn get_safe(&self, index: BoundedNum<usize, Min, Max>) -> &T {
        let index = index.inner();
        // SAFETY: This is safe because index is guaranteed to be less or equal than `Max` and `Max` is guaranteed to be less than `N`.
        unsafe { self.get_unchecked(index) }
    }

    fn get_safe_mut(&mut self, index: BoundedNum<usize, Min, Max>) -> &mut T {
        let index = index.inner();
        unsafe { self.get_unchecked_mut(index) }
    }
}

#[cfg(test)]
mod tests {
    use typenum::{P1, P5};

    use super::*;

    #[test]
    fn get_safe() {
        let array = [100, 101, 102, 103, 104, 105];
        let index: BoundedNum<usize, P1, P5> = BoundedNum::try_new(5).unwrap();

        let value = array.get_safe(index);

        assert_eq!(*value, 105);
    }

    #[test]
    fn get_safe_mut() {
        let mut array = [100, 101, 102, 103, 104, 105];
        let index: BoundedNum<usize, P1, P5> = BoundedNum::try_new(5).unwrap();

        let value = array.get_safe_mut(index);

        assert_eq!(*value, 105);
        *value = 205;
        assert_eq!(array[5], 205);
        assert_eq!(array.get_safe(index), &205);
    }
}
