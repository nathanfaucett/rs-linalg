extern crate linalg;


use linalg::*;


#[test]
fn test_vector_ops() {
    let mut a = Vector::<usize>::new(2);
    let mut b = Vector::<usize>::new(2);
    a[0] = 1;
    a[1] = 1;
    b[0] = 1;
    b[1] = 1;

    let mut r = Vector::<usize>::new(2);

    r[0] = 2;
    r[1] = 2;
    assert_eq!(&(&a + &b), &r);

    r[0] = 0;
    r[1] = 0;
    assert_eq!(&(&a - &b), &r);

    r[0] = 1;
    r[1] = 1;
    assert_eq!(&(&a * &b), &r);

    r[0] = 1;
    r[1] = 1;
    assert_eq!(&(&a / &b), &r);

    r[0] = 0;
    r[1] = 0;
    assert_eq!(&(&a % &b), &r);
}
