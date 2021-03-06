use std::{
    f32::EPSILON,
    ops::{Add, AddAssign, Deref, DerefMut, Div, Mul, Neg, Sub},
};

use float_eq::float_eq;

#[derive(Clone)]
pub struct Point(pub f32, pub f32, pub f32);

impl Point {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self(x, y, z)
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        float_eq!(self.0, other.0, ulps <= 0)
            && float_eq!(self.1, other.1, ulps <= 0)
            && float_eq!(self.2, other.2, ulps <= 0)
    }
}

impl Add<&Vector> for &Point {
    type Output = Point;

    fn add(self, rhs: &Vector) -> Self::Output {
        let x = self.0 + rhs.0;
        let y = self.1 + rhs.1;
        let z = self.2 + rhs.2;

        Point(x, y, z)
    }
}

impl AddAssign<&Vector> for Point {
    fn add_assign(&mut self, rhs: &Vector) {
        self.0 = self.0 + rhs.0;
        self.1 = self.1 + rhs.1;
        self.2 = self.2 + rhs.2;
    }
}

impl Sub for &Point {
    type Output = Vector;

    fn sub(self, rhs: Self) -> Self::Output {
        let x = self.0 - rhs.0;
        let y = self.1 - rhs.1;
        let z = self.2 - rhs.2;

        Vector(x, y, z)
    }
}

impl Sub<&Vector> for &Point {
    type Output = Point;

    fn sub(self, rhs: &Vector) -> Self::Output {
        let x = self.0 - rhs.0;
        let y = self.1 - rhs.1;
        let z = self.2 - rhs.2;

        Point(x, y, z)
    }
}

#[derive(Clone)]
pub struct Vector(pub f32, pub f32, pub f32);

impl Vector {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self(x, y, z)
    }

    pub fn magnitude(&self) -> f32 {
        (self.0.powi(2) + self.1.powi(2) + self.2.powi(2)).sqrt()
    }

    pub fn normalize(&self) -> Self {
        let mag = self.magnitude();
        Self(self.0 / mag, self.1 / mag, self.2 / mag)
    }

    pub fn dot(&self, vec: &Vector) -> f32 {
        self.0 * vec.0 + self.1 * vec.1 + self.2 * vec.2
    }

    pub fn cross(&self, vec: &Vector) -> Self {
        let x = self.1 * vec.2 - self.2 * vec.1;
        let y = self.2 * vec.0 - self.0 * vec.2;
        let z = self.0 * vec.1 - self.1 * vec.0;
        Self(x, y, z)
    }
}

impl PartialEq for Vector {
    fn eq(&self, other: &Self) -> bool {
        float_eq!(self.0, other.0, ulps <= 0)
            && float_eq!(self.1, other.1, ulps <= 0)
            && float_eq!(self.2, other.2, ulps <= 0)
    }
}

impl Add<&Vector> for &Vector {
    type Output = Vector;

    fn add(self, rhs: &Vector) -> Self::Output {
        let x = self.0 + rhs.0;
        let y = self.1 + rhs.1;
        let z = self.2 + rhs.2;

        Vector(x, y, z)
    }
}

impl AddAssign for Vector {
    fn add_assign(&mut self, rhs: Self) {
        self.0 = self.0 + rhs.0;
        self.1 = self.1 + rhs.1;
        self.2 = self.2 + rhs.2;
    }
}

impl Sub for &Vector {
    type Output = Vector;

    fn sub(self, rhs: Self) -> Self::Output {
        let x = self.0 - rhs.0;
        let y = self.1 - rhs.1;
        let z = self.2 - rhs.2;

        Vector(x, y, z)
    }
}

impl Neg for Vector {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self(-self.0, -self.1, -self.2)
    }
}

impl Mul<f32> for &Vector {
    type Output = Vector;

    fn mul(self, rhs: f32) -> Self::Output {
        Vector(self.0 * rhs, self.1 * rhs, self.2 * rhs)
    }
}

impl Mul<&Vector> for f32 {
    type Output = Vector;

    fn mul(self, rhs: &Vector) -> Self::Output {
        Vector(rhs.0 * self, rhs.1 * self, rhs.2 * self)
    }
}

impl Mul<Vector> for f32 {
    type Output = Vector;

    fn mul(self, mut rhs: Vector) -> Self::Output {
        rhs.0 *= self;
        rhs.1 *= self;
        rhs.2 *= self;

        rhs
    }
}

impl Div<f32> for &Vector {
    type Output = Vector;

    fn div(self, rhs: f32) -> Self::Output {
        Vector(self.0 / rhs, self.1 / rhs, self.2 / rhs)
    }
}

pub struct Matrix<const N: usize>([[f32; N]; N]);

pub trait Matrixable<const N: usize> {
    fn new(arr: [[f32; N]; N]) -> Self;
}

impl<const N: usize> Matrixable<N> for Matrix<N> {
    fn new(arr: [[f32; N]; N]) -> Self {
        Self(arr)
    }
}

impl<const N: usize> Default for Matrix<N> {
    fn default() -> Self {
        Self([[0.; N]; N])
    }
}

impl<const N: usize> Deref for Matrix<N> {
    type Target = [[f32; N]; N];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<const N: usize> DerefMut for Matrix<N> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<const N: usize> Mul for Matrix<N> {
    type Output = Matrix<N>;

    fn mul(self, rhs: Self) -> Self::Output {
        let mut res = Matrix::default();

        for row in 0..N {
            for col in 0..N {
                for i in 0..N {
                    res[row][col] += self[row][i] * rhs[i][col];
                }
            }
        }
        res
    }
}

impl<const N: usize> Mul for &Matrix<N> {
    type Output = Matrix<N>;

    fn mul(self, rhs: Self) -> Self::Output {
        let mut res = Matrix::default();

        for row in 0..N {
            for col in 0..N {
                for i in 0..N {
                    res[row][col] += self[row][i] * rhs[i][col];
                }
            }
        }
        res
    }
}

impl<const N: usize> PartialEq for Matrix<N> {
    fn eq(&self, other: &Self) -> bool {
        for (a, b) in self.iter().flatten().zip(other.iter().flatten()) {
            if !float_eq!(a, b, abs <= EPSILON) {
                return false;
            }
        }
        true
    }
}

#[cfg(test)]
mod test {
    use std::f32::EPSILON;

    use float_eq::assert_float_eq;

    use crate::math::{Matrix, Matrixable, Point, Vector};

    #[test]
    fn create_a_point() {
        let ref_point = Point(1., 2., 3.);
        let my_point = Point::new(1., 2., 3.);

        assert_float_eq!(my_point.0, ref_point.0, abs <= EPSILON);
        assert_float_eq!(my_point.1, ref_point.1, abs <= EPSILON);
        assert_float_eq!(my_point.2, ref_point.2, abs <= EPSILON);
    }

    #[test]
    fn create_a_vector() {
        let ref_vec = Vector(1., 2., 3.);
        let my_vec = Vector::new(1., 2., 3.);

        assert_float_eq!(my_vec.0, ref_vec.0, abs <= EPSILON);
        assert_float_eq!(my_vec.1, ref_vec.1, abs <= EPSILON);
        assert_float_eq!(my_vec.2, ref_vec.2, abs <= EPSILON);
    }

    #[test]
    fn add_vector_to_point() {
        let origin = Point::new(3., -2., 5.);
        let trans = Vector::new(-2., 3., 1.);
        let dest = &origin + &trans;

        let ref_dest = Point::new(1., 1., 6.);

        assert_float_eq!(dest.0, ref_dest.0, abs <= EPSILON);
        assert_float_eq!(dest.1, ref_dest.1, abs <= EPSILON);
        assert_float_eq!(dest.2, ref_dest.2, abs <= EPSILON);
    }

    #[test]
    fn substract_point_to_point() {
        let dest = Point::new(3., 2., 1.);
        let origin = Point::new(5., 6., 7.);
        let my_vec = &dest - &origin;

        let ref_vec = Vector::new(-2., -4., -6.);

        assert_float_eq!(my_vec.0, ref_vec.0, abs <= EPSILON);
        assert_float_eq!(my_vec.1, ref_vec.1, abs <= EPSILON);
        assert_float_eq!(my_vec.2, ref_vec.2, abs <= EPSILON);
    }

    #[test]
    fn substract_vector_to_point() {
        let origin = Point::new(3., 2., 1.);
        let trans = Vector::new(5., 6., 7.);
        let dest = &origin - &trans;

        let ref_dest = Point::new(-2., -4., -6.);

        assert_float_eq!(dest.0, ref_dest.0, abs <= EPSILON);
        assert_float_eq!(dest.1, ref_dest.1, abs <= EPSILON);
        assert_float_eq!(dest.2, ref_dest.2, abs <= EPSILON);
    }

    #[test]
    fn substract_vector_to_vector() {
        let v1 = Vector::new(3., 2., 1.);
        let v2 = Vector::new(5., 6., 7.);
        let point = &v1 - &v2;

        let ref_point = Point::new(-2., -4., -6.);

        assert_float_eq!(point.0, ref_point.0, abs <= EPSILON);
        assert_float_eq!(point.1, ref_point.1, abs <= EPSILON);
        assert_float_eq!(point.2, ref_point.2, abs <= EPSILON);
    }

    #[test]
    fn negate_vector() {
        let v = -Vector::new(5., 6., 7.);

        let ref_vec = Vector::new(-5., -6., -7.);

        assert_float_eq!(v.0, ref_vec.0, abs <= EPSILON);
        assert_float_eq!(v.1, ref_vec.1, abs <= EPSILON);
        assert_float_eq!(v.2, ref_vec.2, abs <= EPSILON);
    }

    #[test]
    fn multiply_vector_by_scalar() {
        let scalar = 2.;
        let v1 = Vector::new(1., -2., 3.);
        let v2 = scalar * &v1;

        let ref_vec = Vector::new(2., -4., 6.);

        assert_float_eq!(v2.0, ref_vec.0, abs <= EPSILON);
        assert_float_eq!(v2.1, ref_vec.1, abs <= EPSILON);
        assert_float_eq!(v2.2, ref_vec.2, abs <= EPSILON);
    }

    #[test]
    fn divide_vector_by_scalar() {
        let scalar = 2.;
        let v1 = Vector::new(1., -2., 3.);
        let v2 = &v1 / scalar;

        let ref_vec = Vector::new(0.5, -1., 1.5);

        assert_float_eq!(v2.0, ref_vec.0, abs <= EPSILON);
        assert_float_eq!(v2.1, ref_vec.1, abs <= EPSILON);
        assert_float_eq!(v2.2, ref_vec.2, abs <= EPSILON);
    }

    #[test]
    fn computes_magnitude_of_vector_v1() {
        let v = Vector::new(0., 1., 0.);
        assert_float_eq!(v.magnitude(), 1., abs <= EPSILON);
    }

    #[test]
    fn computes_magnitude_of_vector_v2() {
        let v = Vector::new(0., 1., 0.);
        assert_float_eq!(v.magnitude(), 1., abs <= EPSILON);
    }

    #[test]
    fn computes_magnitude_of_vector_v3() {
        let v = Vector::new(1., 2., 3.);
        assert_float_eq!(v.magnitude(), 14_f32.sqrt(), abs <= EPSILON);
    }

    #[test]
    fn computes_magnitude_of_vector_v4() {
        let v = Vector::new(-1., -2., 3.);
        assert_float_eq!(v.magnitude(), 14_f32.sqrt(), abs <= EPSILON);
    }

    #[test]
    fn normalize_vector_v1() {
        let v = Vector::new(4., 0., 0.);
        let normalized_v = v.normalize();
        assert_float_eq!(normalized_v.0, 1., abs <= EPSILON);
        assert_float_eq!(normalized_v.1, 0., abs <= EPSILON);
        assert_float_eq!(normalized_v.2, 0., abs <= EPSILON);
    }

    #[test]
    fn normalize_vector_v2() {
        let v = Vector::new(1., 2., 3.);
        let normalized_v = v.normalize();
        assert_float_eq!(normalized_v.0, 1. / 14_f32.sqrt(), abs <= EPSILON);
        assert_float_eq!(normalized_v.1, 2. / 14_f32.sqrt(), abs <= EPSILON);
        assert_float_eq!(normalized_v.2, 3. / 14_f32.sqrt(), abs <= EPSILON);
    }

    #[test]
    fn normalized_vector_magnitude() {
        let v = Vector::new(1., 2., 3.);
        let normalized_v = v.normalize();
        let magnitude = normalized_v.magnitude();
        assert_float_eq!(magnitude, 1., abs <= EPSILON);
    }

    #[test]
    fn dot_product_of_2_vector() {
        let v1 = Vector::new(1., 2., 3.);
        let v2 = Vector::new(2., 3., 4.);

        let dot_res = v1.dot(&v2);

        assert_float_eq!(dot_res, 20., abs <= EPSILON);
    }

    #[test]
    fn cross_product_of_v1_and_v2() {
        let v1 = Vector::new(1., 2., 3.);
        let v2 = Vector::new(2., 3., 4.);

        let v3 = v1.cross(&v2);

        let ref_vec = Vector::new(-1., 2., -1.);

        assert_float_eq!(v3.0, ref_vec.0, abs <= EPSILON);
        assert_float_eq!(v3.1, ref_vec.1, abs <= EPSILON);
        assert_float_eq!(v3.2, ref_vec.2, abs <= EPSILON);
    }

    #[test]
    fn cross_product_of_v2_and_v1() {
        let v1 = Vector::new(1., 2., 3.);
        let v2 = Vector::new(2., 3., 4.);

        let v3 = v2.cross(&v1);

        let ref_vec = Vector::new(1., -2., 1.);

        assert_float_eq!(v3.0, ref_vec.0, abs <= EPSILON);
        assert_float_eq!(v3.1, ref_vec.1, abs <= EPSILON);
        assert_float_eq!(v3.2, ref_vec.2, abs <= EPSILON);
    }

    #[test]
    fn constructs_and_inspects_4x4_matrix() {
        let m = Matrix::new([
            [1., 2., 3., 4.],
            [5.5, 6.5, 7.5, 8.5],
            [9., 10., 11., 12.],
            [13.5, 14.5, 15.5, 16.5],
        ]);

        assert_float_eq!(m[0][0], 1., abs <= EPSILON);
        assert_float_eq!(m[0][3], 4., abs <= EPSILON);
        assert_float_eq!(m[1][0], 5.5, abs <= EPSILON);
        assert_float_eq!(m[1][2], 7.5, abs <= EPSILON);
        assert_float_eq!(m[2][2], 11., abs <= EPSILON);
        assert_float_eq!(m[3][0], 13.5, abs <= EPSILON);
        assert_float_eq!(m[3][2], 15.5, abs <= EPSILON);
    }

    #[test]
    fn constructs_and_inspects_3x3_matrix() {
        let m = Matrix::new([[-3., 5., 0.], [1., -2., -7.], [0., 0., 1.]]);

        assert_float_eq!(m[0][0], -3., abs <= EPSILON);
        assert_float_eq!(m[0][1], 5., abs <= EPSILON);
        assert_float_eq!(m[1][0], 1., abs <= EPSILON);
        assert_float_eq!(m[1][1], -2., abs <= EPSILON);
        assert_float_eq!(m[2][2], 1., abs <= EPSILON);
    }

    #[test]
    fn constructs_and_inspects_2x2_matrix() {
        let m = Matrix::new([[-3., 5.], [1., -2.]]);

        assert_float_eq!(m[0][0], -3., abs <= EPSILON);
        assert_float_eq!(m[0][1], 5., abs <= EPSILON);
        assert_float_eq!(m[1][0], 1., abs <= EPSILON);
        assert_float_eq!(m[1][1], -2., abs <= EPSILON);
    }

    #[test]
    fn identical_matrix_equals() {
        let m1 = Matrix::new([
            [1., 2., 3., 4.],
            [5., 6., 7., 8.],
            [9., 8., 7., 6.],
            [5., 4., 3., 2.],
        ]);

        let m2 = Matrix::new([
            [1., 2., 3., 4.],
            [5., 6., 7., 8.],
            [9., 8., 7., 6.],
            [5., 4., 3., 2.],
        ]);

        assert!(m1 == m2);
    }

    #[test]
    fn distinct_matrix_doesnt_equals() {
        let m1 = Matrix::new([
            [1., 2., 3., 4.],
            [5., 6., 7., 8.],
            [9., 8., 7., 6.],
            [5., 4., 3., 2.],
        ]);

        let m2 = Matrix::new([
            [1., 2., 3., 4.],
            [5., 6., 7., 8.],
            [8., 7., 6., 5.],
            [4., 3., 2., 1.],
        ]);

        assert!(m1 != m2);
    }

    #[test]
    fn multiply_matrices() {
        let m1 = Matrix::new([
            [1., 2., 3., 4.],
            [5., 6., 7., 8.],
            [9., 8., 7., 6.],
            [5., 4., 3., 2.],
        ]);

        let m2 = Matrix::new([
            [-2., 1., 2., 3.],
            [3., 2., 1., -1.],
            [4., 3., 6., 5.],
            [1., 2., 7., 8.],
        ]);

        let m3 = Matrix::new([
            [20., 22., 50., 48.],
            [44., 54., 114., 108.],
            [40., 58., 110., 102.],
            [16., 26., 46., 42.],
        ]);

        let m4 = m1 * m2;
        assert!(m4 == m3);
    }
}
