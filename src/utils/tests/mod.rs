
use super::*;

#[test]
fn compare_floats() {
    assert!(equal_floats(0.000001, 0.0000006));
}


#[test]
fn compare_floats_2() {
    assert_eq!(equal_floats(0.00001, 0.000026), false);
}