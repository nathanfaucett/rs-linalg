use core::fmt;
use core::ops::*;

use one::One;

use super::vector::Vector;


#[derive(Clone, PartialEq)]
pub struct Matrix<T> {
    rows: usize,
    cols: usize,
    data: Vector<Vector<T>>,
}

unsafe impl<T: Send> Send for Matrix<T> {}
unsafe impl<T: Sync> Sync for Matrix<T> {}

impl<T: Default> Matrix<T> {
    #[inline]
    pub fn new(rows: usize, cols: usize) -> Self {
        let mut data = Vector::zeroed(cols);

        for col in data.iter_mut() {
            *col = Vector::new(rows);
        }

        Matrix {
            rows: rows,
            cols: cols,
            data: data,
        }
    }
}

impl<T: One> Matrix<T> {
    #[inline]
    pub fn identity(rows: usize, cols: usize) -> Self {
        let mut data = Vector::zeroed(cols);

        let mut i = 0;
        for col in data.iter_mut() {
            let mut new_col = Vector::zeroed(rows);
            new_col[i] = T::one();
            *col = new_col;
            i += 1;
        }

        Matrix {
            rows: rows,
            cols: cols,
            data: data,
        }
    }
}

impl<T> Matrix<T> {

    #[inline]
    pub fn zeroed(rows: usize, cols: usize) -> Self {
        let mut data = Vector::zeroed(cols);

        for col in data.iter_mut() {
            *col = Vector::zeroed(rows);
        }

        Matrix {
            rows: rows,
            cols: cols,
            data: data,
        }
    }

    #[inline(always)]
    pub fn rows(&self) -> usize { self.rows }
    #[inline(always)]
    pub fn cols(&self) -> usize { self.cols }

    #[inline(always)]
    pub fn get(&self, i: usize, j: usize) -> &T {
        self.data.get(i).get(j)
    }
    #[inline(always)]
    pub fn get_mut(&mut self, i: usize, j: usize) -> &mut T {
        self.data.get_mut(i).get_mut(j)
    }
}

impl<T: Clone> Matrix<T> {

    #[inline]
    pub fn transpose(&self) -> Self {
        let mut matrix = Matrix::<T>::zeroed(self.cols, self.rows);

        for i in 0..self.rows {
            for j in 0..self.cols {
                matrix[j][i] = self[i][j].clone();
            }
        }

        matrix
    }
}

impl<T> Deref for Matrix<T> {
    type Target = [Vector<T>];

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &*self.data
    }
}
impl<T> DerefMut for Matrix<T> {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut *self.data
    }
}

impl<T> Index<usize> for Matrix<T> {
    type Output = Vector<T>;

    #[inline(always)]
    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}
impl<T> IndexMut<usize> for Matrix<T> {
    #[inline(always)]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index]
    }
}

impl<T: fmt::Debug> fmt::Debug for Matrix<T> {
    #[inline(always)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(&**self, f)
    }
}


impl<'out, 'a, 'b, T> Matrix<T>
    where T: 'a + 'b + AddAssign<T>,
          &'a T: Mul<&'b T, Output = T>,
{
    #[inline]
    pub fn mul(out: &'out mut Matrix<T>, a: &'a Matrix<T>, b: &'b Matrix<T>) -> &'out mut Matrix<T> {
        let a_cols = a.cols();
        let a_rows = a.rows();
        let b_cols = b.cols();
        let b_rows = b.rows();
        assert!(a_cols == b_rows, "A * B, A's columns does not match B's rows");

        for i in 0..a_rows {
    		for j in 0..b_cols {
    			for k in 0..b_cols {
    				out[i][j] += &a[i][k] * &b[k][j];
    			}
    		}
    	}
        out
    }
    #[inline]
    pub fn smul(out: &'out mut Matrix<T>, m: &'a Matrix<T>, s: &'b T) -> &'out mut Matrix<T> {
        let rows = m.rows();
        let cols = m.cols();

        for i in 0..rows {
    		for j in 0..cols {
    			out[i][j] += &m[i][j] * s;
    		}
    	}
        out
    }
}

impl<'a, 'b, T> Mul<&'b Matrix<T>> for  &'a Matrix<T>
    where T: 'a + 'b + AddAssign<T>,
          &'a T: Mul<&'b T, Output = T>,
{
    type Output = Matrix<T>;

    #[inline(always)]
    fn mul(self, other: &'b Matrix<T>) -> Self::Output {
        let mut out = Matrix::zeroed(self.rows(), other.cols());
        Matrix::<T>::mul(&mut out, self, other);
        out
    }
}

impl<'a, 'b, T> Mul<&'b T> for  &'a Matrix<T>
    where T: AddAssign<T>,
          &'a T: Mul<&'b T, Output = T>,
{
    type Output = Matrix<T>;

    #[inline(always)]
    fn mul(self, s: &'b T) -> Self::Output {
        let mut out = Matrix::zeroed(self.rows(), self.cols());
        Matrix::<T>::smul(&mut out, self, s);
        out
    }
}

macro_rules! impl_bin_op {
    ($Trait: ident, $trait: ident, $name: ident, $scalar: ident, $op: tt) => (
        impl<'out, 'a, 'b, T> Matrix<T>
            where T: 'a + 'b,
                  &'a T: $Trait<&'b T, Output = T>,
        {
            #[inline]
            pub fn $name(out: &'out mut Matrix<T>, a: &'a Matrix<T>, b: &'b Matrix<T>) -> &'out mut Matrix<T> {
                let a_cols = a.cols();
                let a_rows = a.rows();
                let b_cols = b.cols();
                let b_rows = b.rows();
                assert!(a_cols == b_rows, "A * B, A's columns does not match B's rows");

                for i in 0..a_rows {
            		for j in 0..b_cols {
            			for k in 0..b_cols {
            				out[i][j] = &a[i][k] $op &b[k][j];
            			}
            		}
            	}
                out
            }
            #[inline]
            pub fn $scalar(out: &'out mut Matrix<T>, m: &'a Matrix<T>, s: &'b T) -> &'out mut Matrix<T> {
                let rows = m.rows();
                let cols = m.cols();

                for i in 0..rows {
            		for j in 0..cols {
            			out[i][j] = &m[i][j] $op s;
            		}
            	}
                out
            }
        }

        impl<'a, 'b, T> $Trait<&'b Matrix<T>> for  &'a Matrix<T>
            where T: 'a + 'b,
                  &'a T: $Trait<&'b T, Output = T>,
        {
            type Output = Matrix<T>;

            #[inline(always)]
            fn $trait(self, other: &'b Matrix<T>) -> Self::Output {
                let mut out = Matrix::zeroed(self.rows(), other.cols());
                Matrix::<T>::$name(&mut out, self, other);
                out
            }
        }

        impl<'a, 'b, T> $Trait<&'b T> for  &'a Matrix<T>
            where T: 'a + 'b,
                  &'a T: $Trait<&'b T, Output = T>,
        {
            type Output = Matrix<T>;

            #[inline(always)]
            fn $trait(self, s: &'b T) -> Self::Output {
                let mut out = Matrix::zeroed(self.rows(), self.cols());
                Matrix::<T>::$scalar(&mut out, self, s);
                out
            }
        }
    );
}

impl_bin_op!(Add, add, add, sadd, +);
impl_bin_op!(Sub, sub, sub, ssub, -);
