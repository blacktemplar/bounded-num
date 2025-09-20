use bounded_num::BoundedNum;
use typenum::{P1, P128, P255, Z0};

fn main() {
    let value: BoundedNum<u8, Z0, P128> = BoundedNum::try_new(95).unwrap();

    let new_value: BoundedNum<u8, P1, P255> = value.into();
}
