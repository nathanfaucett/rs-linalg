use core::fmt;
use core::ops::*;

use array::Array;
use zero::Zero;

use super::matrix::Matrix;


#[derive(Clone)]
pub struct Vector<T> {
    data: Array<T>,
}

unsafe impl<T: Send> Send for Vector<T> {}
unsafe impl<T: Sync> Sync for Vector<T> {}

impl<T: Default> Vector<T> {
    #[inline(always)]
    pub fn new(len: usize) -> Self {
        assert!(len != 0);
        Vector {
            data: Array::new(len),
        }
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

    #[inline(always)]
    pub fn get(&self, i: usize) -> &T {
        unsafe {
            self.data.get_unchecked(i)
        }
    }
    #[inline(always)]
    pub fn get_mut(&mut self, i: usize) -> &mut T {
        unsafe {
            self.data.get_unchecked_mut(i)
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

impl<T> Index<usize> for Vector<T> {
    type Output = T;

    #[inline(always)]
    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}
impl<T> IndexMut<usize> for Vector<T> {
    #[inline(always)]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index]
    }
}

impl<T: PartialEq> PartialEq for Vector<T> {
    #[inline(always)]
    fn eq(&self, other: &Self) -> bool {
        &**self == &**other
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
          &'a T: Mul<&'b T, Output = T>,
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

macro_rules! impl_bin_op {
    ($Trait: ident, $trait: ident, $name: ident, $matrix: ident, $scalar: ident, $op: tt) => (
        impl<'out, 'a, 'b, T> Vector<T>
            where T: 'a + 'b,
                  &'a T: $Trait<&'b T, Output = T>,
        {
            #[inline]
            pub fn $name(out: &'out mut Vector<T>, a: &'a Vector<T>, b: &'b Vector<T>) -> &'out mut Vector<T> {
                let a_len = a.len();
                let b_len = b.len();
                assert!(a_len == b_len, "a * b, a's length does not match b's length");

                for i in 0..a_len {
                    out[i] = &a[i] $op &b[i];
            	}
                out
            }
            #[inline]
            pub fn $matrix(out: &'out mut Vector<T>, v: &'a Vector<T>, m: &'b Matrix<T>) -> &'out mut Vector<T> {
                let len = v.len();
                let cols = m.cols();
                let rows = m.rows();
                assert!(len == cols, "v * A, v's length does not match A's columns");

                for i in 0..cols {
                    for j in 0..rows {
                        out[i] = &v[i] $op &m[i][j];
                    }
                }
                out
            }
            #[inline]
            pub fn $scalar(out: &'out mut Vector<T>, v: &'a Vector<T>, s: &'b T) -> &'out mut Vector<T> {
                let len = v.len();

                for i in 0..len {
                    out[i] = &v[i] $op s;
                }
                out
            }
        }

        impl<'a, 'b, T> $Trait<&'b Vector<T>> for  &'a Vector<T>
            where T: 'a + 'b,
                  &'a T: $Trait<&'b T, Output = T>,
        {
            type Output = Vector<T>;

            #[inline(always)]
            fn $trait(self, other: &'b Vector<T>) -> Self::Output {
                let mut out = Vector::zeroed(self.len());
                Vector::<T>::$name(&mut out, self, other);
                out
            }
        }

        impl<'a, 'b, T> $Trait<&'b Matrix<T>> for  &'a Vector<T>
            where T: 'a + 'b,
                  &'a T: $Trait<&'b T, Output = T>,
        {
            type Output = Vector<T>;

            #[inline(always)]
            fn $trait(self, other: &'b Matrix<T>) -> Self::Output {
                let mut out = Vector::zeroed(self.len());
                Vector::<T>::$matrix(&mut out, self, other);
                out
            }
        }

        impl<'a, 'b, T> $Trait<&'b T> for &'a Vector<T>
            where T: 'a + 'b,
                  &'a T: $Trait<&'b T, Output = T>,
        {
            type Output = Vector<T>;

            #[inline(always)]
            fn $trait(self, s: &'b T) -> Self::Output {
                let mut out = Vector::zeroed(self.len());
                Vector::<T>::$scalar(&mut out, self, s);
                out
            }
        }
    );
}

impl_bin_op!(Add, add, add, madd, sadd, +);
impl_bin_op!(Sub, sub, sub, msub, ssub, -);
impl_bin_op!(Mul, mul, mul, mmul, smul, *);
impl_bin_op!(Div, div, div, mdiv, sdiv, /);
