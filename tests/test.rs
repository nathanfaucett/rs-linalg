extern crate one;
extern crate zero;
extern crate linalg;


use one::One;
use zero::Zero;
use linalg::*;


#[test]
fn test_mul_matrix_matrix() {
    let a: Matrix<usize> = Matrix::identity(3, 3);
    let b: Matrix<usize> = Matrix::identity(3, 3);
    let c: Matrix<usize> = Matrix::identity(3, 3);
    let d: Matrix<usize> = &a * &b;
    assert_eq!(d, c);
}
#[test]
fn test_mul_matrix_scalar() {
    let a: Matrix<usize> = Matrix::identity(3, 3);
    let s: usize = 3;
    let mut c: Matrix<usize> = Matrix::identity(3, 3);
    c[0][0] = 3;
    c[1][1] = 3;
    c[2][2] = 3;
    let d: Matrix<usize> = &a * &s;
    assert_eq!(d, c);
}

#[test]
fn test_mul_vector_vector() {
    let a: Vector<usize> = vec_ones(3);
    let b: Vector<usize> = vec_ones(3);
    let c: Vector<usize> = vec_ones(3);
    let d: Vector<usize> = &a * &b;
    assert_eq!(d, c);
}
#[test]
fn test_mul_vector_matrix() {
    let a: Vector<usize> = vec_zeros(3);
    let b: Matrix<usize> = Matrix::identity(3, 3);
    let c: Vector<usize> = Vector::new(3);
    let d: Vector<usize> = &a * &b;
    assert_eq!(d, c);
}
#[test]
fn test_mul_vector_scalar() {
    let a: Vector<usize> = vec_ones(3);
    let s: usize = 3;
    let mut c: Vector<usize> = Vector::new(3);
    c[0] = 3;
    c[1] = 3;
    c[2] = 3;
    let d: Vector<usize> = &a * &s;
    assert_eq!(d, c);
}
#[test]
fn test_dot_vectors() {
    let a: Vector<usize> = vec_ones(3);
    let b: Vector<usize> = vec_ones(3);
    let c = a.dot(&b);
    assert_eq!(c, 3);
}
#[test]
fn test_add_vectors() {
    let a: Vector<usize> = vec_ones(3);
    let b: Vector<usize> = vec_ones(3);
    let c = &a + &b;
    assert_eq!(c[0], 2);
    assert_eq!(c[1], 2);
    assert_eq!(c[2], 2);
}
#[test]
fn test_add_vector_scale() {
    let a: Vector<usize> = vec_ones(3);
    let c = &a + &1;
    assert_eq!(c[0], 2);
    assert_eq!(c[1], 2);
    assert_eq!(c[2], 2);
}
#[test]
fn test_sub_vectors() {
    let a: Vector<usize> = vec_ones(3);
    let b: Vector<usize> = vec_ones(3);
    let c = &a - &b;
    assert_eq!(c[0], 0);
    assert_eq!(c[1], 0);
    assert_eq!(c[2], 0);
}
#[test]
fn test_sub_vector_scale() {
    let a: Vector<usize> = vec_ones(3);
    let c = &a - &1;
    assert_eq!(c[0], 0);
    assert_eq!(c[1], 0);
    assert_eq!(c[2], 0);
}


fn vec_zeros<T: Default + Zero>(count: usize) -> Vector<T> {
    let mut v = Vector::new(count);
    for i in 0..count {
        v[i] = T::zero();
    }
    v
}
fn vec_ones<T: Default + One>(count: usize) -> Vector<T> {
    let mut v = Vector::new(count);
    for i in 0..count {
        v[i] = T::one();
    }
    v
}
