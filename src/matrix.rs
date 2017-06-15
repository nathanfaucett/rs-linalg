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

unsafe impl<T: Send> Send for Matrix<T> {}
unsafe impl<T: Sync> Sync for Matrix<T> {}

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

impl<T> Matrix<T>
    where T: Clone,
{
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

impl<T> Matrix<T>
    where T: Zero + Clone + AddAssign<T>,
          for<'a> T: Mul<&'a T, Output = T>,
          for<'a> &'a T: Neg<Output = T>,
          for<'a, 'b> &'a T: Mul<&'b T, Output = T> +
                             Sub<&'b T, Output = T>,
{
    #[inline(always)]
    pub fn determinant(&self) -> T {
        if self.rows == self.cols {
            determinant::<T>(&self.data, self.rows)
        } else {
            panic!("can not find the determinant of a {}x{} matrix", self.rows, self.cols)
        }
    }
}

impl<T> Matrix<T>
    where T: One + Zero + Clone + AddAssign<T>,
          for<'a> T: Mul<&'a T, Output = T>,
          for<'a> &'a T: PartialEq + Neg<Output = T>,
          for<'a, 'b> &'a T: Div<&'b T, Output = T> +
                             Mul<&'b T, Output = T> +
                             Sub<&'b T, Output = T>,
{
    #[inline(always)]
    pub fn inverse(&self) -> Matrix<T> {
        if self.rows == self.cols {
            let mut out = Matrix::<T>::zeroed(self.rows(), self.cols());
            inverse::<T>(&mut out.data, &self.data, self.rows);
            out
        } else {
            panic!("can not find the determinant of a {}x{} matrix", self.rows, self.cols)
        }
    }
}

#[inline]
fn get_cofactor<T>(out: &mut Vector<Vector<T>>, m: &Vector<Vector<T>>, p: usize, q: usize, size: usize)
    where T: Clone,
{
    let mut i = 0;
    let mut j = 0;

    for row in 0..size {
        for col in 0..size {
            if row != p && col != q {
                out[i][j] = m[row][col].clone();
                j += 1;

                if j == size - 1 {
                    j = 0;
                    i += 1;
                }
            }
        }
    }
}

#[inline]
fn determinant<T>(m: &Vector<Vector<T>>, size: usize) -> T
    where T: Zero + Clone + AddAssign<T>,
          for<'a> T: Mul<&'a T, Output = T>,
          for<'a> &'a T: Neg<Output = T>,
          for<'a, 'b> &'a T: Mul<&'b T, Output = T> +
                             Sub<&'b T, Output = T>,
{
    if size == 1 {
        m[0][0].clone()
    } else {
        let mut tmp = Vector::of_vectors(size, size);
        let mut d = T::zero();
        let mut sign = 1;

        for f in 0..size {
            get_cofactor::<T>(&mut tmp, m, 0, f, size);

            if sign == -1 {
                d += &-&m[0][f] * &determinant::<T>(&tmp, size - 1);
            } else {
                d += &m[0][f] * &determinant::<T>(&tmp, size - 1);
            }

            sign = -sign;
        }

        d
    }
}

#[inline]
fn adjoint<T>(out: &mut Vector<Vector<T>>, m: &Vector<Vector<T>>, size: usize)
    where T: One + Zero + Clone + AddAssign<T>,
          for<'a> T: Mul<&'a T, Output = T>,
          for<'a> &'a T: Neg<Output = T>,
          for<'a, 'b> &'a T: Mul<&'b T, Output = T> +
                             Sub<&'b T, Output = T>,
{
    if size == 1 {
        out[0][0] = T::one();
    } else {
        let mut sign;
        let mut tmp = Vector::of_vectors(size, size);

        for i in 0..size {
            for j in 0..size {
                get_cofactor::<T>(&mut tmp, m, i, j, size);
                sign = if (i + j) % 2 == 0 {1} else {-1};

                if sign == -1 {
                    out[j][i] = -&determinant::<T>(&tmp, size - 1);
                } else {
                    out[j][i] = determinant::<T>(&tmp, size - 1);
                }
            }
        }
    }
}

#[inline]
fn inverse<T>(out: &mut Vector<Vector<T>>, m: &Vector<Vector<T>>, size: usize)
    where T: One + Zero + Clone + AddAssign<T>,
          for<'a> T: Mul<&'a T, Output = T>,
          for<'a> &'a T: PartialEq + Neg<Output = T>,
          for<'a, 'b> &'a T: Div<&'b T, Output = T> +
                             Mul<&'b T, Output = T> +
                             Sub<&'b T, Output = T>,
{
    let d = determinant::<T>(m, size);

    if &d == &T::zero() {
        panic!("can not find the inverse matrix")
    } else {
        let mut adj = Vector::of_vectors(size, size);
        adjoint::<T>(&mut adj, m, size);

        for i in 0..size {
            for j in 0..size {
                out[i][j] = &adj[i][j] / &d;
            }
        }
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

impl<'out, T> Neg for &'out Matrix<T>
    where T: One + Zero + Clone + AddAssign<T>,
          for<'a> T: Mul<&'a T, Output = T>,
          for<'a> &'a T: PartialEq + Neg<Output = T>,
          for<'a, 'b> &'a T: Div<&'b T, Output = T> +
                             Mul<&'b T, Output = T> +
                             Sub<&'b T, Output = T>,
{
    type Output = Matrix<T>;

    #[inline]
    fn neg(self) -> Self::Output {
        self.inverse()
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


#[cfg(test)]
mod test {
    use super::*;


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
    fn test_det_matrix_matrix() {
        let a: Matrix<isize> = Matrix::identity(2, 2);
        assert_eq!(a.determinant(), 1);

        let b: Matrix<isize> = Matrix::identity(3, 3);
        assert_eq!(b.determinant(), 1);
    }
    #[test]
    fn test_inverse_matrix() {
        let a: Matrix<isize> = Matrix::identity(2, 2);
        let b = a.inverse();

        assert_eq!(b[0][0], 1);
        assert_eq!(b[1][1], 1);

        let a: Matrix<f32> = Matrix::identity(2, 2);
        let b = (&a * &2f32).inverse();

        assert_eq!(b[0][0], 0.5);
        assert_eq!(b[1][1], 0.5);
    }
}
