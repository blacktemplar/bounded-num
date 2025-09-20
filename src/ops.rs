use std::ops::Add;

use crate::{BoundedNum, Representable};

impl<T1, T2, MIN1, MAX1, MIN2, MAX2> Add<BoundedNum<T2, MIN2, MAX2>> for BoundedNum<T1, MIN1, MAX1>
where
    T1: PartialOrd + Add<T2>,
    T2: PartialOrd,
    MIN1: Representable<T1> + Add<MIN2>,
    MAX1: Representable<T1> + Add<MAX2>,
    MIN2: Representable<T2>,
    MAX2: Representable<T2>,
    <T1 as Add<T2>>::Output: PartialOrd,
    <MIN1 as Add<MIN2>>::Output: Representable<<T1 as Add<T2>>::Output>,
    <MAX1 as Add<MAX2>>::Output: Representable<<T1 as Add<T2>>::Output>,
{
    type Output = BoundedNum<
        <T1 as Add<T2>>::Output,
        <MIN1 as Add<MIN2>>::Output,
        <MAX1 as Add<MAX2>>::Output,
    >;

    fn add(self, rhs: BoundedNum<T2, MIN2, MAX2>) -> Self::Output {
        let result = self.inner() + rhs.inner();
        // SAFETY: We constructed the bounds of the result in a way that the result will always be inside the bounds
        unsafe { BoundedNum::unchecked_new(result) }
    }
}

#[cfg(test)]
mod tests {
    mod add {
        use typenum::{P5, P17, P22, P55, P100, P155};

        use crate::BoundedNum;

        #[test]
        fn u8_valid_add() {
            let left: BoundedNum<u8, P17, P100> = BoundedNum::try_new(95).unwrap();
            let right: BoundedNum<u8, P5, P55> = BoundedNum::try_new(55).unwrap();

            let result: BoundedNum<u8, P22, P155> = left + right;

            assert_eq!(result, BoundedNum::try_new(150).unwrap());
        }
    }
}
