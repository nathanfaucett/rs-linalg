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

impl<T: fmt::Debug> fmt::Debug for Matrix<T> {
    #[inline(always)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(&**self, f)
    }
}


impl<'out, 'a, 'b, T> Matrix<T>
    where T: 'a + 'b + AddAssign<T>,
          &'a T: Add<&'b T, Output = T> +
                 Mul<&'b T, Output = T>,
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
    where T: AddAssign<T>,
          &'a T: Add<&'b T, Output = T> +
                 Mul<&'b T, Output = T>,
{
    type Output = Matrix<T>;

    #[inline(always)]
    fn mul(self, other: &'b Matrix<T>) -> Self::Output {
        let mut out = Matrix::zeroed(self.rows(), other.cols());
        Matrix::mul(&mut out, self, other);
        out
    }
}

impl<'a, 'b, T> Mul<&'b T> for  &'a Matrix<T>
    where T: AddAssign<T>,
          &'a T: Add<&'b T, Output = T> +
                 Mul<&'b T, Output = T>,
{
    type Output = Matrix<T>;

    #[inline(always)]
    fn mul(self, s: &'b T) -> Self::Output {
        let mut out = Matrix::zeroed(self.rows(), self.cols());
        Matrix::smul(&mut out, self, s);
        out
    }
}
