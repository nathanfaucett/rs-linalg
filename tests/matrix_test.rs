extern crate linalg;


use linalg::*;


#[test]
fn test_matrix_mul() {
    let a = Matrix::<usize>::identity(2, 2);
    let b = Matrix::<usize>::identity(2, 2);
    let r = Matrix::<usize>::identity(2, 2);
    assert_eq!(&(&a * &b), &r);
}
