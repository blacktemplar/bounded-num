use bounded_num::{BoundedNum, SafeGet};
use typenum::{P1, P6};

fn main() {
    let array = [100, 101, 102, 103, 104, 105];
    let index: BoundedNum<usize, P1, P6> = BoundedNum::try_new(5).unwrap();

    let value = array.get_safe_mut(index);
}
