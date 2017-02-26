#![feature(test)]


extern crate test;

extern crate linalg;
extern crate mat4;


use test::Bencher;

use linalg::*;


#[bench]
fn bench_matrix_mul(b: &mut Bencher) {
    let m0 = Matrix::<usize>::identity(4, 4);
    let m1 = Matrix::<usize>::identity(4, 4);

    b.iter(move || {
        &m0 * &m1;
    });
}

#[bench]
fn bench_mat4_mul(b: &mut Bencher) {
    let m0 = mat4::new_identity::<usize>();
    let m1 = mat4::new_identity::<usize>();
    let mut out = mat4::new_identity::<usize>();

    b.iter(move || {
        mat4::mul(&mut out, &m0, &m1);
    });
}
