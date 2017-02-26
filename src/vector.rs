use core::fmt;
use core::ops::*;

use array::Array;


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
    #[inline(always)]
    pub fn uninitialized(len: usize) -> Self {
        assert!(len != 0);
        Vector {
            data: Array::uninitialized(len),
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
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(&**self, f)
    }
}


macro_rules! impl_vector_bin_ops {
    (
        $BinTrait: ident, $bin_fn: ident, $bin_op: tt,
        $AssignTrait: ident, $assign_fn: ident, $assign_op: tt
    ) => (
        impl<'a, 'b, T> $BinTrait<&'b Vector<T>> for &'a Vector<T>
            where &'a T: $BinTrait<&'b T, Output = T>
        {
            type Output = Vector<T>;

            fn $bin_fn(self, other: &'b Vector<T>) -> Self::Output {
                let len = self.len();

                assert!(len == other.len(), "a * b, a's length does not match b's length");

                let mut out = Vector::uninitialized(len);

                for i in 0..len {
                    out[i] = &self[i] $bin_op &other[i];
                }

                out
            }
        }

        impl<'a, T> $AssignTrait<&'a Vector<T>> for Vector<T>
            where T: $AssignTrait<&'a T>,
        {
            fn $assign_fn(&mut self, other: &'a Vector<T>) {
                let len = self.len();

                assert!(len == other.len(), "a * b, a's length does not match b's length");

                for i in 0..len {
                    self[i] $assign_op &other[i];
                }
            }
        }
    )
}

impl_vector_bin_ops!(Add, add, +, AddAssign, add_assign, +=);
impl_vector_bin_ops!(Sub, sub, -, SubAssign, sub_assign, -=);
impl_vector_bin_ops!(Mul, mul, *, MulAssign, mul_assign, *=);
impl_vector_bin_ops!(Div, div, /, DivAssign, div_assign, /=);
impl_vector_bin_ops!(Rem, rem, %, RemAssign, rem_assign, %=);