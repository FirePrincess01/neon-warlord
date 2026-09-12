//! Gradients of NeuralNetworkSimd

use itertools::izip;
use wide::f32x16;

use super::LANES;
use super::NR_LANES;
use super::NR_NEURONS;

use super::SVec;
use super::SMat;

pub struct GradientsSimd<const SIZE: usize> {
    pub dy_dw: [[[f32; NR_NEURONS]; NR_NEURONS]; SIZE],
    pub dy_db: [[f32; NR_NEURONS]; SIZE],

    pub dy_dw_y: [[f32; NR_NEURONS]; NR_NEURONS],
    pub dy_db_y: [f32; NR_NEURONS],
}

impl<const SIZE: usize> GradientsSimd<SIZE> {
    pub fn new() -> Self {
        let dy_dw = [[[0.0; NR_NEURONS]; NR_NEURONS]; SIZE];
        let dy_db = [[0.0; NR_NEURONS]; SIZE];

        let dy_dw_y = [[0.0; NR_NEURONS]; NR_NEURONS];
        let dy_db_y = [0.0; NR_NEURONS];

        Self {
            dy_dw,
            dy_db,
            dy_dw_y,
            dy_db_y,
        }
    }

    #[inline]
    pub fn multiply_constant(&self, val: f32) -> Self {
        let mut res = Self::new();
        let val_ = SVec::new([val; NR_NEURONS]);

        // dy_dw
        for (x, y) in std::iter::zip(&self.dy_dw, &mut res.dy_dw) {
            for (x, y) in std::iter::zip(x, y) {
                *y = (SVec::new(*x) * &val_).into();
            }
        }

        // dy_db
        for (x, y) in std::iter::zip(&self.dy_db, &mut res.dy_db) {
            *y = (SVec::new(*x) * &val_).into();
        }

        // dy_dw_y
        for (x, y) in std::iter::zip(&self.dy_dw_y, &mut res.dy_dw_y) {
            *y = (SVec::new(*x) * &val_).into();
        }

        // dy_db_y
        res.dy_db_y = (SVec::new(self.dy_db_y) * &val_).into();

        res
    }

    #[inline]
    pub fn add(&self, other: &Self) -> Self {
        let mut res = Self::new();

        // dy_dw
        for (a, b, y) in izip!(&self.dy_dw, &other.dy_dw, &mut res.dy_dw) {
            for (a, b, y) in izip!(a, b, y) {
                *y = (SVec::new(*a) + &SVec::new(*b)).into();
            }
        }

        // dy_db
        for (a, b, y) in izip!(&self.dy_db, &other.dy_db, &mut res.dy_db) {
            *y = (SVec::new(*a) + &SVec::new(*b)).into();
        }

        // dy_dw_y
        for (a, b, y) in izip!(&self.dy_dw_y, &other.dy_dw_y, &mut res.dy_dw_y) {
            *y = (SVec::new(*a) + &SVec::new(*b)).into();
        }

        // dy_db_y
        res.dy_db_y = (SVec::new(self.dy_db_y) + &SVec::new(other.dy_db_y)).into();

        res
    }

    #[inline]
    pub fn sub(&self, other: &Self) -> Self {
        let mut res = Self::new();

        // dy_dw
        for (a, b, y) in izip!(&self.dy_dw, &other.dy_dw, &mut res.dy_dw) {
            for (a, b, y) in izip!(a, b, y) {
                *y = (SVec::new(*a) - &SVec::new(*b)).into();
            }
        }

        // dy_db
        for (a, b, y) in izip!(&self.dy_db, &other.dy_db, &mut res.dy_db) {
            *y = (SVec::new(*a) - &SVec::new(*b)).into();
        }

        // dy_dw_y
        for (a, b, y) in izip!(&self.dy_dw_y, &other.dy_dw_y, &mut res.dy_dw_y) {
            *y = (SVec::new(*a) - &SVec::new(*b)).into();
        }

        // dy_db_y
        res.dy_db_y = (SVec::new(self.dy_db_y) - &SVec::new(other.dy_db_y)).into();

        res
    }
}

impl<const SIZE: usize> std::ops::Add for GradientsSimd<SIZE> {
    type Output = Self;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        GradientsSimd::add(&self, &rhs)
    }
}

impl<const SIZE: usize> std::ops::Sub for GradientsSimd<SIZE> {
    type Output = Self;

    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        GradientsSimd::sub(&self, &rhs)
    }
}

impl<const SIZE: usize> std::ops::Mul<f32> for &GradientsSimd<SIZE> {
    type Output = GradientsSimd<SIZE>;

    #[inline]
    fn mul(self, rhs: f32) -> Self::Output {
        GradientsSimd::multiply_constant(self, rhs)
    }
}

impl<const SIZE: usize> std::ops::AddAssign for GradientsSimd<SIZE> {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        *self = GradientsSimd::add(self, &rhs)
    }
}

impl<const SIZE: usize> std::ops::SubAssign for GradientsSimd<SIZE> {
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        *self = GradientsSimd::sub(self, &rhs)
    }
}
