use typenum::{IsGreaterOrEqual, IsLessOrEqual, True};

use crate::{BoundedNum, OutOfBoundsError, Representable};

#[derive(Debug, PartialEq)]
pub enum TryIntoError<E> {
    InnerConversionError(E),
    TooSmall,
    TooLarge,
}

impl<E> From<OutOfBoundsError> for TryIntoError<E> {
    fn from(value: OutOfBoundsError) -> Self {
        match value {
            OutOfBoundsError::TooSmall => Self::TooSmall,
            OutOfBoundsError::TooLarge => Self::TooLarge,
        }
    }
}

impl<T1, Min1, Max1> BoundedNum<T1, Min1, Max1>
where
    T1: PartialOrd,
    Min1: Representable<T1>,
    Max1: Representable<T1>,
{
    pub fn into<T2, Min2, Max2>(self) -> BoundedNum<T2, Min2, Max2>
    where
        T1: Into<T2>,
        T2: PartialOrd,
        Min2: Representable<T2>,
        Max2: Representable<T2>,
        Min2: IsLessOrEqual<Min1, Output = True>,
        Max2: IsGreaterOrEqual<Max2, Output = True>,
    {
        let value: T2 = self.inner().into();
        // SAFETY: The bounds on Min2 and Max2 guarantee that any value between Min1 and Max1 is also between Min2 and Max2
        unsafe { BoundedNum::unchecked_new(value) }
    }

    pub fn try_into<T2, Min2, Max2>(
        self,
    ) -> Result<BoundedNum<T2, Min2, Max2>, TryIntoError<<T1 as TryInto<T2>>::Error>>
    where
        T1: TryInto<T2>,
        T2: PartialOrd,
        Min2: Representable<T2>,
        Max2: Representable<T2>,
    {
        let value: T2 = self
            .inner()
            .try_into()
            .map_err(TryIntoError::InnerConversionError)?;

        Ok(BoundedNum::try_new(value)?)
    }
}

#[cfg(test)]
mod tests {
    use typenum::{N128, P5, P120, P127, P128, P255, Z0};

    use super::*;

    #[test]
    fn u8_valid_into() {
        let value: BoundedNum<u8, Z0, P128> = BoundedNum::try_new(95).unwrap();

        let new_value: BoundedNum<u8, Z0, P255> = value.into();

        assert_eq!(value.inner(), new_value.inner());
    }

    #[test]
    fn u8_ok_try_into() {
        let value: BoundedNum<u8, Z0, P128> = BoundedNum::try_new(95).unwrap();

        let new_value: Result<BoundedNum<u8, Z0, P255>, _> = value.try_into();

        assert!(new_value.is_ok());
        assert_eq!(value.inner(), new_value.unwrap().inner());
    }

    #[test]
    fn u8_try_into_i8_unrepresentable_value() {
        let value: BoundedNum<u8, Z0, P128> = BoundedNum::try_new(128).unwrap();

        let new_value: Result<BoundedNum<i8, N128, P127>, _> = value.try_into();

        assert!(matches!(
            new_value,
            Err(TryIntoError::InnerConversionError(_))
        ));
    }

    #[test]
    fn u8_try_into_i8_too_large() {
        let value: BoundedNum<u8, Z0, P128> = BoundedNum::try_new(121).unwrap();

        let new_value: Result<BoundedNum<i8, N128, P120>, _> = value.try_into();

        assert_eq!(new_value, Err(TryIntoError::TooLarge));
    }

    #[test]
    fn u8_try_into_i8_too_small() {
        let value: BoundedNum<u8, Z0, P128> = BoundedNum::try_new(3).unwrap();

        let new_value: Result<BoundedNum<i8, P5, P120>, _> = value.try_into();

        assert_eq!(new_value, Err(TryIntoError::TooSmall));
    }
}
