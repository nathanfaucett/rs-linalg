extern crate linalg;


use linalg::*;


#[test]
fn test_matrix_mul() {
    let a = Matrix::<usize>::identity(2, 2);
    let b = Matrix::<usize>::identity(2, 2);
    let r = Matrix::<usize>::identity(2, 2);
    assert_eq!(&(&a * &b), &r);
}

#[test]
fn test_matrix_vec_mul() {
    let a = Matrix::<usize>::identity(2, 2);
    let mut b = Vector::<usize>::new(2);
    b[0] = 0;
    b[1] = 0;
    let mut r = Matrix::<usize>::identity(2, 2);
    r[0][0] = 0;
    r[0][1] = 0;
    r[1][0] = 0;
    r[1][1] = 0;
    assert_eq!(&(&a * &b), &r);
}
#[test]
fn test_vec_matrix_mul() {
    let mut a = Vector::<usize>::new(2);
    a[0] = 0;
    a[1] = 0;
    let b = Matrix::<usize>::identity(2, 2);
    let mut r = Vector::<usize>::new(2);
    r[0] = 0;
    r[1] = 0;
    assert_eq!(&(&a * &b), &r);
}
