use std::fmt::Debug;
use std::marker::PhantomData;

use crate::Representable;

pub struct BoundedNum<T, Min, Max>(T, PhantomData<(Min, Max)>)
where
    T: PartialOrd,
    Min: Representable<T>,
    Max: Representable<T>;

impl<T: Debug, Min, Max> Debug for BoundedNum<T, Min, Max>
where
    T: PartialOrd,
    Min: Representable<T>,
    Max: Representable<T>,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("BoundedNum")
            .field(&self.0)
            .field(&self.1)
            .finish()
    }
}

impl<T: PartialEq, Min, Max> PartialEq for BoundedNum<T, Min, Max>
where
    T: PartialOrd,
    Min: Representable<T>,
    Max: Representable<T>,
{
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl<T: Clone, Min, Max> Clone for BoundedNum<T, Min, Max>
where
    T: PartialOrd,
    Min: Representable<T>,
    Max: Representable<T>,
{
    fn clone(&self) -> Self {
        Self(self.0.clone(), PhantomData)
    }
}

impl<T: Copy, Min, Max> Copy for BoundedNum<T, Min, Max>
where
    T: PartialOrd,
    Min: Representable<T>,
    Max: Representable<T>,
{
}

#[derive(Debug, PartialEq)]
pub enum OutOfBoundsError {
    TooSmall,
    TooLarge,
}

impl<T, Min, Max> BoundedNum<T, Min, Max>
where
    T: PartialOrd,
    Min: Representable<T>,
    Max: Representable<T>,
{
    /// Tries to create a new bound
    pub fn try_new(t: T) -> Result<Self, OutOfBoundsError> {
        if t < Min::VALUE {
            Err(OutOfBoundsError::TooSmall)
        } else if t > Max::VALUE {
            Err(OutOfBoundsError::TooLarge)
        } else {
            Ok(Self(t, PhantomData))
        }
    }

    /// Creates a new bounded num without doing any bound checks
    ///
    /// # Safety
    /// `Self::Min::VALUE <= t <= Self::Max::VALUE` must be satisfied, otherwise this can lead to undefined behaviour.
    pub unsafe fn unchecked_new(t: T) -> Self {
        Self(t, PhantomData)
    }

    pub fn inner(self) -> T {
        self.0
    }
}

#[cfg(test)]
mod tests {
    use typenum::{P99, P100, P101, P155, Z0};

    use crate::{BoundedNum, OutOfBoundsError};

    #[test]
    fn try_new_between_min_and_max() {
        let t: u8 = 100;

        let bounded: Result<BoundedNum<u8, P99, P101>, _> = BoundedNum::try_new(t);

        assert!(bounded.is_ok());
        assert_eq!(bounded.unwrap().inner(), t);
    }

    #[test]
    fn try_new_equals_min() {
        let t: u8 = 100;

        let bounded: Result<BoundedNum<u8, P100, P101>, _> = BoundedNum::try_new(t);

        assert!(bounded.is_ok());
        assert_eq!(bounded.unwrap().inner(), t);
    }

    #[test]
    fn try_new_equals_max() {
        let t: u8 = 100;

        let bounded: Result<BoundedNum<u8, P99, P100>, _> = BoundedNum::try_new(t);

        assert!(bounded.is_ok());
        assert_eq!(bounded.unwrap().inner(), t);
    }

    #[test]
    fn try_new_equals_min_and_max() {
        let t: u8 = 100;

        let bounded: Result<BoundedNum<u8, P100, P100>, _> = BoundedNum::try_new(t);

        assert!(bounded.is_ok());
        assert_eq!(bounded.unwrap().inner(), t);
    }

    #[test]
    fn try_new_less_than_min() {
        let t: u8 = 100;

        let bounded: Result<BoundedNum<u8, P101, P155>, _> = BoundedNum::try_new(t);

        assert!(!bounded.is_ok());
        assert_eq!(bounded.unwrap_err(), OutOfBoundsError::TooSmall);
    }

    #[test]
    fn try_new_more_than_max() {
        let t: u8 = 100;

        let bounded: Result<BoundedNum<u8, Z0, P99>, _> = BoundedNum::try_new(t);

        assert!(!bounded.is_ok());
        assert_eq!(bounded.unwrap_err(), OutOfBoundsError::TooLarge);
    }
}
