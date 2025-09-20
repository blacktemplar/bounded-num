use bounded_num::BoundedNum;
use typenum::{P128, Z0};

fn main() {
    let left: BoundedNum<u8, Z0, P128> = BoundedNum::try_new(95).unwrap();
    let right: BoundedNum<u8, Z0, P128> = BoundedNum::try_new(55).unwrap();

    let result = left + right;
}
