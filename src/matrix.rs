use core::fmt;
use core::ops::*;

use one::One;
use zero::Zero;

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
        let mut data = Vector::zeroed(rows);

        for row in data.iter_mut() {
            *row = Vector::new(cols);
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
        let mut data = Vector::zeroed(rows);

        let mut i = 0;
        for row in data.iter_mut() {
            let mut new_row = Vector::zeroed(cols);
            new_row[i] = T::one();
            *row = new_row;
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
        let mut data = Vector::zeroed(rows);

        for row in data.iter_mut() {
            *row = Vector::zeroed(cols);
        }

        Matrix {
            rows: rows,
            cols: cols,
            data: data,
        }
    }
    #[inline]
    pub fn uninitialized(rows: usize, cols: usize) -> Self {
        let mut data = Vector::zeroed(rows);

        for row in data.iter_mut() {
            *row = Vector::uninitialized(cols);
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
    pub fn row(&self, index: usize) -> &Vector<T> {
        &self[index]
    }
    #[inline(always)]
    pub fn col_value(&self, index: usize, offset:usize) -> &T {
        &self[index][offset]
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

impl<T: fmt::Debug> fmt::Debug for Matrix<T> {
    #[inline(always)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(&**self, f)
    }
}


impl<'a, 'b, T> Mul<&'b Matrix<T>> for &'a Matrix<T>
    where T: Zero + AddAssign<T>,
          &'a T: Add<&'b T, Output = T> +
                 Mul<&'b T, Output = T>
{
    type Output = Matrix<T>;

    fn mul(self, other: &'b Matrix<T>) -> Matrix<T> {
        assert!(self.cols() == other.rows(), "A * B, A's columns do not match B's rows");
        let new_rows = self.rows();
        let new_cols = other.cols();
        let mut out = Matrix::uninitialized(new_rows, new_cols);

        for i in 0..new_rows {
            for j in 0..new_cols {
                let mut out_value = T::zero();
                let row = self.row(i);

                let mut offset = 0;
                for value in row.iter() {
                    out_value += value * other.col_value(j, offset);
                    offset += 1;
                }

                out[i][j] = out_value;
            }
        }

        out
    }
}
