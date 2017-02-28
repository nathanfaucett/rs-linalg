use core::fmt;
use core::ops::*;

use array::Array;
use zero::Zero;

use super::matrix::Matrix;


#[derive(Clone)]
pub struct Vector<T> {
    data: Array<T>,
}

impl<T: Default> Vector<T> {
    #[inline(always)]
    pub fn new(len: usize) -> Self {
        assert!(len != 0);
        Vector {
            data: Array::new(len),
        }
    }
}

impl<T: Default> Default for Vector<T> {
    #[inline(always)]
    fn default() -> Self {
        Vector::new(1)
    }
}

impl<T> Vector<T> {
    #[inline(always)]
    pub fn zeroed(len: usize) -> Self {
        assert!(len != 0);
        Vector {
            data: Array::zeroed(len),
        }
    }
}

impl<T> Deref for Vector<T> {
    type Target = [T];

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &*self.data
    }
}
impl<T> DerefMut for Vector<T> {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut *self.data
    }
}

impl<T: PartialEq> PartialEq for Vector<T> {
    #[inline(always)]
    fn eq(&self, other: &Self) -> bool {
        **self == **other
    }
}

impl<T: fmt::Debug> fmt::Debug for Vector<T> {
    #[inline(always)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(&**self, f)
    }
}

impl<'a, 'b, T> Vector<T>
    where T: 'a + 'b + Zero + AddAssign<T>,
          &'a T: Add<&'b T, Output = T> +
                 Mul<&'b T, Output = T>,
{
    #[inline]
    pub fn dot(&'a self, other: &'b Vector<T>) -> T {
        let len = self.len();
        let other_len = other.len();
        assert!(len == other_len, "a * b, a's length does not match b's length");
        let mut out = T::zero();

        for i in 0..len {
            out += &self[i] * &other[i];
    	}

        out
    }
}
impl<'out, 'a, 'b, T> Vector<T>
    where T: 'a + 'b + AddAssign<T>,
          &'a T: Add<&'b T, Output = T> +
                 Mul<&'b T, Output = T>,
{
    #[inline]
    pub fn mul(out: &'out mut Vector<T>, a: &'a Vector<T>, b: &'b Vector<T>) -> &'out mut Vector<T> {
        let a_len = a.len();
        let b_len = b.len();
        assert!(a_len == b_len, "a * b, a's length does not match b's length");

        for i in 0..a_len {
            out[i] += &a[i] * &b[i];
    	}
        out
    }
    #[inline]
    pub fn mmul(out: &'out mut Vector<T>, v: &'a Vector<T>, m: &'b Matrix<T>) -> &'out mut Vector<T> {
        let len = v.len();
        let cols = m.cols();
        let rows = m.rows();
        assert!(len == cols, "v * A, v's length does not match A's columns");

        for i in 0..cols {
            for j in 0..rows {
                out[i] += &v[i] * &m[i][j];
            }
        }
        out
    }
    #[inline]
    pub fn smul(out: &'out mut Vector<T>, v: &'a Vector<T>, s: &'b T) -> &'out mut Vector<T> {
        let len = v.len();

        for i in 0..len {
            out[i] += &v[i] * s;
        }
        out
    }
}

impl<'a, 'b, T> Mul<&'b Vector<T>> for  &'a Vector<T>
    where T: AddAssign<T>,
          &'a T: Add<&'b T, Output = T> +
                 Mul<&'b T, Output = T>,
{
    type Output = Vector<T>;

    #[inline(always)]
    fn mul(self, other: &'b Vector<T>) -> Self::Output {
        let mut out = Vector::zeroed(self.len());
        Vector::mul(&mut out, self, other);
        out
    }
}

impl<'a, 'b, T> Mul<&'b Matrix<T>> for  &'a Vector<T>
    where T: AddAssign<T>,
          &'a T: Add<&'b T, Output = T> +
                 Mul<&'b T, Output = T>,
{
    type Output = Vector<T>;

    #[inline(always)]
    fn mul(self, other: &'b Matrix<T>) -> Self::Output {
        let mut out = Vector::zeroed(self.len());
        Vector::mmul(&mut out, self, other);
        out
    }
}

impl<'a, 'b, T> Mul<&'b T> for &'a Vector<T>
    where T: AddAssign<T>,
          &'a T: Add<&'b T, Output = T> +
                 Mul<&'b T, Output = T>,
{
    type Output = Vector<T>;

    #[inline(always)]
    fn mul(self, s: &'b T) -> Self::Output {
        let mut out = Vector::zeroed(self.len());
        Vector::smul(&mut out, self, s);
        out
    }
}
