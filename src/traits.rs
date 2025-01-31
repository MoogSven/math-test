// Copyright © 2023 Marvin Beckmann, Marcel Luca Schmidt
//
// This file is part of qFALL-math.
//
// qFALL-math is free software: you can redistribute it and/or modify it under
// the terms of the Mozilla Public License Version 2.0 as published by the
// Mozilla Foundation. See <https://mozilla.org/en-US/MPL/2.0/>.

//! This module contains basic traits for this library. These include
//! specific traits for matrices and polynomials.

use crate::error::MathError;
use std::fmt::Display;

/// Is implemented by polynomials to evaluate it for a certain input.
pub trait Evaluate<T, U> {
    /// Evaluates the object, e.g. polynomial or a matrix of polynomials,
    /// for a given input value.
    ///
    /// Parameters:
    /// - `value`: The value with which to evaluate the object.
    ///
    /// Returns the evaluation of the object.
    fn evaluate(&self, value: T) -> U;
}

/// Is implemented by polynomials get a coefficient.
pub trait GetCoefficient<T> {
    /// Returns a coefficient of the given object, e.g. a polynomial,
    /// for a given index.
    ///
    /// Parameters:
    /// - `index`: The index of the coefficient
    ///
    /// Returns the coefficient of the polynomial.
    fn get_coeff(&self, index: impl TryInto<i64> + Display + Copy) -> Result<T, MathError>;
}

/// Is implemented by polynomials to set individual coefficients.
pub trait SetCoefficient<T> {
    /// Sets coefficient of the object, e.g. polynomial,
    /// for a given input value and a index.
    ///
    /// Parameters:
    /// - `index` : The coefficient to be set.
    /// - `value`: The value the coefficient is set to.
    fn set_coeff(
        &mut self,
        index: impl TryInto<i64> + Display + Copy,
        value: T,
    ) -> Result<(), MathError>;
}

/// Is implemented by matrices to get the number of rows of the matrix.
pub trait GetNumRows {
    /// Returns the number of rows of a matrix.
    fn get_num_rows(&self) -> i64;
}

/// Is implemented by matrices to get the number of columns of the matrix.
pub trait GetNumColumns {
    /// Returns the number of columns of a matrix.
    fn get_num_columns(&self) -> i64;
}

/// Is implemented by matrices to get entries.
pub trait GetEntry<T> {
    /// Returns the value of a specific matrix entry.
    ///
    /// Parameters:
    /// - `row`: specifies the row in which the entry is located.
    /// - `column`: specifies the column in which the entry is located.
    ///
    /// Returns the value of the matrix at the position of the given
    /// row and column or an error, if the number of rows or columns is
    /// greater than the matrix or negative.
    fn get_entry(
        &self,
        row: impl TryInto<i64> + Display + Copy,
        column: impl TryInto<i64> + Display + Copy,
    ) -> Result<T, MathError>;
}

/// Is implemented by matrices to set entries.
pub trait SetEntry<T> {
    /// Sets the value of a specific matrix entry according to a given value.
    ///
    /// Returns an error, if the number of rows or columns is
    /// greater than the matrix or negative.
    ///
    /// Parameters:
    /// - `row`: specifies the row in which the entry is located.
    /// - `column`: specifies the column in which the entry is located.
    /// - `value`: specifies the value to which the entry is set.
    fn set_entry(
        &mut self,
        row: impl TryInto<i64> + Display + Copy,
        column: impl TryInto<i64> + Display + Copy,
        value: T,
    ) -> Result<(), MathError>;
}

/// Is implemented by matrices to compute the tensor product.
pub trait Tensor {
    /// Computes the tensor product of `self` with `other`
    ///
    /// Parameters:
    /// - `other`: the value with which the tensor product is computed.
    ///
    /// Returns the tensor product
    fn tensor(&self, other: &Self) -> Self;
}

/// Is implemented by matrices to concatenate two matrices.
pub trait Concatenate {
    type Output;

    /// Concatenates `self` with `other` vertically.
    ///
    /// Parameters:
    /// - `other`: the other matrix to concatenate with `self`
    ///
    /// # Errors and Failures
    /// Returns a `MathError` of type
    /// [`MismatchingMatrixDimension`](MathError::MismatchingMatrixDimension)
    /// if the matrices can not be concatenated due to mismatching dimensions
    fn concat_vertical(self, other: Self) -> Result<Self::Output, MathError>;

    /// Concatenates `self` with `other` horizontally.
    ///
    /// Parameters:
    /// - `other`: the other matrix to concatenate with `self`
    ///
    /// # Errors and Failures
    /// Returns a `MathError` of type
    /// [`MismatchingMatrixDimension`](MathError::MismatchingMatrixDimension)
    /// if the matrices can not be concatenated due to mismatching dimensions
    fn concat_horizontal(self, other: Self) -> Result<Self::Output, MathError>;
}

/// Is implemented for [`Z`](crate::integer::Z).
pub trait Distance<T = Self> {
    type Output;

    /// Computes the absolute distance between two values.
    ///
    /// Parameters:
    /// - `other`: specifies the value whose distance is calculated to `self`
    ///
    /// Returns the absolute difference, i.e. distance between the two given values
    /// as a new instance.
    fn distance(&self, other: T) -> Self::Output;
}

/// Is implemented for [`Z`](crate::integer::Z).
pub trait Lcm<T = Self> {
    type Output;

    /// Outputs the least common multiple (lcm) of the two given values
    /// with `lcm(a, 0) = 0`.
    ///
    /// Paramters:
    /// - `other`: specifies one of the values of which the `lcm` is computed
    ///
    /// Returns the least common multiple of `self` and `other` as a new value.
    fn lcm(&self, other: T) -> Self::Output;
}

/// Is implemented by [`Zq`](crate::integer_mod_q::Zq) powered by [`Z`](crate::integer::Z).
pub trait Pow<T> {
    type Output;

    /// Raises the value of `self` to the power of an `exp`.
    ///
    /// Parameters:
    /// - `exp`: specifies the exponent to which the value is raised
    ///
    /// Returns the value of `self` powered by `exp` as a new `Output` instance.
    fn pow(&self, exp: T) -> Result<Self::Output, MathError>;
}

/// Is implemented by [`Z`](crate::integer::Z) instances to calculate the `gcd`
pub trait Gcd<T = Self> {
    type Output;

    /// Outputs the greatest common divisor (gcd) of the two given values
    /// with `gcd(a,0) = |a|`.
    ///
    /// Paramters:
    /// - `other`: specifies one of the values of which the gcd is computed
    ///
    /// Returns the greatest common divisor of `self` and `other`.
    fn gcd(&self, other: T) -> Self::Output;
}

/// Is implemented by [`Z`](crate::integer::Z) instances to calculate the
/// extended `gcd`
pub trait Xgcd<T = Self> {
    type Output;

    /// Outputs the extended greatest common divisor (xgcd) of the two given values,
    /// i.e. a triple `(gcd(a,b), x, y)`, where `a*x + b*y = gcd(a,b)*`.
    ///
    /// Paramters:
    /// - `other`: specifies one of the values of which the gcd is computed
    ///
    /// Returns a triple `(gcd(a,b), x, y)` containing the greatest common divisor,
    /// `x`, and `y` s.t. `gcd(a,b) = a*x + b*y`.
    fn xgcd(&self, other: T) -> Self::Output;
}
