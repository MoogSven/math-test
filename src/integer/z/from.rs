// Copyright © 2023 Sven Moog, Marcel Luca Schmidt
//
// This file is part of qFALL-math.
//
// qFALL-math is free software: you can redistribute it and/or modify it under
// the terms of the Mozilla Public License Version 2.0 as published by the
// Mozilla Foundation. See <https://mozilla.org/en-US/MPL/2.0/>.

//! Implementations to create a [`Z`] value from other types.
//! For each reasonable type, an explicit function with the format
//! `from_<type_name>` and the [`From`] trait should be implemented.
//!
//! The explicit functions contain the documentation.

use super::Z;
use crate::{
    error::MathError,
    integer_mod_q::{Modulus, Zq},
    macros::from::{from_trait, from_type},
};
use flint_sys::fmpz::{
    fmpz, fmpz_get_si, fmpz_init_set_si, fmpz_init_set_ui, fmpz_set, fmpz_set_str,
};
use std::{ffi::CString, str::FromStr};

impl Z {
    /// Create a new Integer that can grow arbitrary large.
    ///
    /// Parameters:
    /// - `value`: the initial value the integer should have
    ///
    /// Returns the new integer.
    ///
    /// # Example
    /// ```
    /// use qfall_math::integer::Z;
    ///
    /// let a: Z = Z::from_i64(42);
    /// ```
    pub fn from_i64(value: i64) -> Self {
        let mut ret_value = fmpz(0);
        unsafe { fmpz_init_set_si(&mut ret_value, value) }
        Z { value: ret_value }
    }

    /// Create a new Integer that can grow arbitrary large.
    ///
    /// Parameters:
    /// - `value`: the initial value the integer should have
    ///
    /// Returns the new integer.
    ///
    /// # Example
    /// ```
    /// use qfall_math::integer::Z;
    ///
    /// let a: Z = Z::from_u64(42);
    /// ```
    pub fn from_u64(value: u64) -> Self {
        let mut ret_value = fmpz(0);
        unsafe { fmpz_init_set_ui(&mut ret_value, value) }
        Z { value: ret_value }
    }

    // Generate from_<type> functions for singed and unsigned source types.
    from_type!(i32, i64, Z, Z::from_i64);
    from_type!(i16, i64, Z, Z::from_i64);
    from_type!(i8, i64, Z, Z::from_i64);

    from_type!(u32, u64, Z, Z::from_u64);
    from_type!(u16, u64, Z, Z::from_u64);
    from_type!(u8, u64, Z, Z::from_u64);

    /// Create a new Integer that can grow arbitrary large.
    ///
    /// Parameters:
    /// - `value`: the initial value the integer should have
    ///
    /// Returns the new integer.
    ///
    /// # Example
    /// ```
    /// use qfall_math::integer::Z;
    /// use qfall_math::integer_mod_q::Modulus;
    /// use std::str::FromStr;
    ///
    /// let m = Modulus::from_str("42").unwrap();
    ///
    /// let a: Z = Z::from_modulus(m);
    /// ```
    pub fn from_modulus(value: Modulus) -> Self {
        let mut out = Z::default();
        unsafe { fmpz_set(&mut out.value, &value.get_fmpz_mod_ctx_struct().n[0]) };
        out
    }

    #[allow(dead_code)]
    /// Create a new Integer that can grow arbitrary large.
    ///
    /// Parameters:
    /// - `value`: the initial value the integer should have
    ///
    /// Returns the new integer.
    ///
    /// # Example
    /// ```compile_fail
    /// use qfall_math::integer::Z;
    /// use flint_sys::fmpz::fmpz;
    ///
    /// let value = fmpz(0);
    ///
    /// let a: Z = Z::from_fmpz(&value);
    /// ```
    pub(crate) fn from_fmpz(value: &fmpz) -> Self {
        let mut out = Z::default();
        unsafe {
            fmpz_set(&mut out.value, value);
        }
        out
    }

    /// Create a new Integer that can grow arbitrary large.
    ///
    /// Parameters:
    /// - `value`: the initial value the integer should have
    ///
    /// Returns the new integer.
    ///
    /// ```
    /// use qfall_math::integer::Z;
    /// use qfall_math::integer_mod_q::Zq;
    /// use std::str::FromStr;
    ///
    /// let m = Zq::from_str("13 mod 17").unwrap();
    ///
    /// let a: Z = Z::from_zq(m);
    /// ```
    pub fn from_zq(value: Zq) -> Self {
        value.value
    }

    /// Create a [`Z`] integer from a [`String`]. This function takes a base in which the number is represented between `2` and `62`
    ///
    /// Parameters:
    /// - `s`: the integer value as a string
    /// - `base`: the base in which the integer is represented
    ///
    /// Returns a [`Z`] or an error, if the provided string was not formatted
    /// correctly or the base is out bounds.
    ///
    /// # Example:
    /// ```
    /// use qfall_math::integer::Z;
    ///  
    /// let a: Z = Z::from_str_b("100", 2).unwrap();
    /// assert_eq!(Z::from(4), a);
    /// ```
    ///
    /// # Errors and Failures
    /// - Returns a [`MathError`] of type [`OutOfBounds`](MathError::OutOfBounds) if the
    /// base is not between `2` and `62`.
    /// - Returns a [`MathError`] of type
    /// [`InvalidStringToCStringInput`](MathError::InvalidStringToCStringInput)
    /// if the provided string contains a Nul byte.
    /// - Returns a [`MathError`] of type
    /// [`InvalidStringToZInput`](MathError::InvalidStringToZInput)
    /// if the provided string was not formatted correctly.
    pub fn from_str_b(s: &str, base: i32) -> Result<Self, MathError> {
        if !(2..=62).contains(&base) {
            return Err(MathError::OutOfBounds(
                "between 2 and 62".to_owned(),
                base.to_string(),
            ));
        }

        if s.contains(char::is_whitespace) {
            return Err(MathError::InvalidStringToZInput(s.to_owned()));
        }

        // since |value| = |0| < 62 bits, we do not need to free the allocated space manually
        let mut value: fmpz = fmpz(0);

        let c_string = CString::new(s)?;

        // -1 is returned if the string is an invalid input.
        // Given the documentation `c_string.as_ptr()` is freed once c_string is deallocated
        // 'The pointer will be valid for as long as `self` is'
        // For reading more look at the documentation of `.as_ptr()`.
        match unsafe { fmpz_set_str(&mut value, c_string.as_ptr(), base) } {
            0 => Ok(Z { value }),
            _ => Err(MathError::InvalidStringToZInput(s.to_owned())),
        }
    }
}

// Generate [`From`] trait for the different types.
from_trait!(i64, Z, Z::from_i64);
from_trait!(i32, Z, Z::from_i32);
from_trait!(i16, Z, Z::from_i16);
from_trait!(i8, Z, Z::from_i8);

from_trait!(u64, Z, Z::from_u64);
from_trait!(u32, Z, Z::from_u32);
from_trait!(u16, Z, Z::from_u16);
from_trait!(u8, Z, Z::from_u8);

from_trait!(Modulus, Z, Z::from_modulus);
from_trait!(Zq, Z, Z::from_zq);

impl FromStr for Z {
    type Err = MathError;

    /// Create a [`Z`] integer from a [`String`]
    /// The format of that string looks like this `(-)12` for the number 12 or -12
    ///
    /// Parameters:
    /// - `s`: the integer value
    ///
    /// Returns a [`Z`] or an error, if the provided string was not formatted
    /// correctly.
    ///
    /// # Example:
    /// ```
    /// use std::str::FromStr;
    /// use qfall_math::integer::Z;
    ///  
    /// let a: Z = "100".parse().unwrap();
    /// let b: Z = Z::from_str("100").unwrap();
    /// ```
    ///
    /// # Errors and Failures
    /// - Returns a [`MathError`] of type
    /// [`InvalidStringToCStringInput`](MathError::InvalidStringToCStringInput)
    /// if the provided string contains a Nul byte.
    /// - Returns a [`MathError`] of type
    /// [`InvalidStringToZInput`](MathError::InvalidStringToZInput)
    /// if the provided string was not formatted correctly.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Z::from_str_b(s, 10)
    }
}

impl TryFrom<&Z> for i64 {
    type Error = MathError;

    /// Converts a [`Z`] into an [`i64`]. If the value is either too large
    /// or too small an error is returned.
    ///
    /// Parameters:
    /// - `value`: the value that will be converted into an [`i64`]
    ///
    /// Returns the value as an [`i64`] or an error, if it does not fit
    /// into an [`i64`]
    ///
    /// # Example
    /// ```
    /// use qfall_math::integer::Z;
    ///
    /// let max = Z::from(i64::MAX);
    /// assert_eq!(i64::MAX, i64::try_from(&max).unwrap());
    ///
    /// let max = Z::from(u64::MAX);
    /// assert!(i64::try_from(&max).is_err());
    /// ```
    ///
    /// # Errors and Failures
    /// - Returns a [`MathError`] of type [`ConversionError`](MathError::ConversionError)
    /// if the value does not fit into an [`i64`]
    fn try_from(value: &Z) -> Result<Self, Self::Error> {
        // fmpz_get_si returns the i64::MAX or respectively i64::MIN
        // if the value is too large/small to fit into an [`i64`].
        // Hence we are required to manually check if the value is actually correct
        let value_i64 = unsafe { fmpz_get_si(&value.value) };
        if &Z::from(value_i64) == value {
            Ok(value_i64)
        } else {
            Err(MathError::ConversionError(format!(
                "The provided value has to fit into an i64 and it doesn't as the 
                provided value is {}.",
                value
            )))
        }
    }
}

#[cfg(test)]
mod tests_from_int {
    use super::Z;

    /// Ensure that initialization with large numbers works.
    /// Numbers larger than 2^62 bits are represented differently in FLINT.
    #[test]
    fn from_i64_max_positive() {
        Z::from_i64(i64::MAX);
    }

    /// Ensure that initialization with large negative numbers works.
    /// Numbers smaller than -2^62 bits are represented differently in FLINT.
    #[test]
    fn from_i64_max_negative() {
        Z::from_i64(i64::MIN);
    }

    /// Ensure that the [`From`] trait is available for i64 values
    #[test]
    fn from_i64_trait() {
        let _ = Z::from(-10i64);
    }

    /// Ensure that the `from_<type_name>` functions are available for
    /// singed and unsigned integers of 8, 16, 32, and 64 bit length.
    /// Tested with their maximum value.
    #[test]
    fn from_functions_max() {
        // signed
        let _ = Z::from_i8(i8::MAX);
        let _ = Z::from_i16(i16::MAX);
        let _ = Z::from_i32(i32::MAX);
        let _ = Z::from_i64(i64::MAX);

        // unsigned
        let _ = Z::from_u8(u8::MAX);
        let _ = Z::from_u16(u16::MAX);
        let _ = Z::from_u32(u32::MAX);
        let _ = Z::from_u64(u64::MAX);
    }

    /// Ensure that the [`From`] trait is available for singed and unsigned integers
    /// of 8, 16, 32, and 64 bit length. Tested with their maximum value.
    #[test]
    fn from_trait_max() {
        // signed
        let _ = Z::from(i8::MAX);
        let _ = Z::from(i16::MAX);
        let _ = Z::from(i32::MAX);
        let _ = Z::from(i64::MAX);

        // unsigned
        let _ = Z::from(u8::MAX);
        let _ = Z::from(u16::MAX);
        let _ = Z::from(u32::MAX);
        let _ = Z::from(u64::MAX);
    }

    /// Ensure that the [`From`] trait is available for singed and unsigned integers
    /// of 8, 16, 32, and 64 bit length. Tested with their minimum value.
    #[test]
    fn from_trait_min() {
        // signed
        let _ = Z::from(i8::MIN);
        let _ = Z::from(i16::MIN);
        let _ = Z::from(i32::MIN);
        let _ = Z::from(i64::MIN);

        // unsigned
        let _ = Z::from(u8::MIN);
        let _ = Z::from(u16::MIN);
        let _ = Z::from(u32::MIN);
        let _ = Z::from(u64::MIN);
    }
}

#[cfg(test)]
mod tests_from_str {

    use crate::integer::Z;
    use std::str::FromStr;

    /// Ensure that initialization with large numbers works.
    #[test]
    fn max_int_positive() {
        assert!(Z::from_str(&(i64::MAX).to_string()).is_ok());
    }

    /// Ensure that initialization with large numbers (larger than i64) works.
    #[test]
    fn big_positive() {
        assert!(Z::from_str(&"1".repeat(65)).is_ok());
    }

    /// Ensure that initialization with large negative numbers works.
    #[test]
    fn max_int_negative() {
        assert!(Z::from_str(&(i64::MIN).to_string()).is_ok());
    }

    /// Ensure that initialization with large negative numbers (larger than i64) works.
    #[test]
    fn big_negative() {
        let mut s = "-".to_string();
        s.push_str(&"1".repeat(65));

        assert!(Z::from_str(&s).is_ok());
    }

    /// Ensure that wrong initialization yields an Error.
    #[test]
    fn error_wrong_letters() {
        assert!(Z::from_str("hbrkt35itu3gg").is_err());
    }

    /// Ensure that wrong initialization yields an Error.
    #[test]
    fn error_wrong_order() {
        assert!(Z::from_str("3-2").is_err());
    }

    /// Ensure that wrong initialization yields an Error.
    #[test]
    fn error_rational() {
        assert!(Z::from_str("876/543").is_err());
    }

    /// Ensure that wrong initialization yields an Error.
    #[test]
    fn whitespace_mid() {
        assert!(Z::from_str("876 543").is_err());
    }

    /// Ensure that wrong initialization yields an Error.
    #[test]
    fn whitespace_start() {
        assert!(Z::from_str(" 876543").is_err());
    }

    /// Ensure that wrong initialization yields an Error.
    #[test]
    fn whitespace_end() {
        assert!(Z::from_str("876543 ").is_err());
    }

    /// Ensure that wrong initialization yields an Error.
    #[test]
    fn whitespace_minus() {
        assert!(Z::from_str("- 876543").is_err());
    }
}

#[cfg(test)]
mod test_from_str_b {
    use crate::integer::Z;

    /// ensure that an error is returned, if an invalid base is provided
    #[test]
    fn out_of_bounds() {
        let value = "100010";

        assert!(Z::from_str_b(value, -1).is_err());
        assert!(Z::from_str_b(value, 0).is_err());
        assert!(Z::from_str_b(value, 1).is_err());
        assert!(Z::from_str_b(value, 63).is_err());
    }

    /// ensure that from_str works with a binary-string
    #[test]
    fn from_str_binary() {
        assert_eq!(Z::from(20), Z::from_str_b("10100", 2).unwrap());
        assert_eq!(Z::from(-20), Z::from_str_b("-10100", 2).unwrap());
    }

    /// ensure that from_str works with a hex-string
    #[test]
    fn from_str_hex() {
        assert_eq!(Z::from(160), Z::from_str_b("a0", 16).unwrap());
        assert_eq!(Z::from(-170), Z::from_str_b("-aa", 16).unwrap());
    }
}

#[cfg(test)]
mod tests_from_modulus {
    use super::Z;
    use crate::integer_mod_q::Modulus;
    use std::str::FromStr;

    /// Ensure that `from_modulus` is available for small and large numbers
    #[test]
    fn large_and_small_numbers() {
        let mod_1 = Modulus::from_str(&"1".repeat(65)).unwrap();
        let mod_2 = Modulus::from_str("10").unwrap();

        let _ = Z::from_modulus(mod_1);
        let _ = Z::from_modulus(mod_2);
    }

    /// Ensure that the [`From`] trait is available for large
    /// [`Modulus`] instances
    #[test]
    fn from_trait() {
        let mod_1 = Modulus::from_str(&"1".repeat(65)).unwrap();
        let mod_2 = Modulus::from_str("10").unwrap();

        let _ = Z::from(mod_1);
        let _ = Z::from(mod_2);
    }
}

#[cfg(test)]
mod test_from_fmpz {
    use super::Z;

    /// Ensure that `from_fmpz` is available for small and large numbers
    #[test]
    fn large_small_numbers() {
        let mod_1 = Z::from(u64::MAX);
        let mod_2 = Z::ZERO;

        let _ = Z::from_fmpz(&mod_1.value);
        let _ = Z::from_fmpz(&mod_2.value);
    }
}

#[cfg(test)]
mod test_from_zq {
    use super::Z;
    use crate::integer_mod_q::Zq;

    /// Ensure that the `from_zq` function is available and works correctly for
    /// small and large [`Zq`] entries.
    #[test]
    fn large_small_numbers() {
        let zq_1 = Zq::try_from((i64::MAX, u64::MAX)).unwrap();
        let zq_2 = Zq::try_from((17, u64::MAX)).unwrap();

        assert_eq!(Z::from(i64::MAX), Z::from_zq(zq_1));
        assert_eq!(Z::from(17), Z::from_zq(zq_2));
    }

    /// Ensure that the [`From`] trait is available for small and large
    /// [`Zq`] instances.
    #[test]
    fn from_trait() {
        let zq_1 = Zq::try_from((i64::MAX, u64::MAX)).unwrap();
        let zq_2 = Zq::try_from((17, u64::MAX)).unwrap();

        assert_eq!(Z::from(i64::MAX), Z::from(zq_1));
        assert_eq!(Z::from(17), Z::from(zq_2));
    }
}

#[cfg(test)]
mod test_try_from_into_i64 {
    use crate::integer::Z;

    //// ensure that an error is returned, if the value of the [`Z`]
    /// does not fit into an [`i64`]
    #[test]
    fn overflow() {
        assert!(i64::try_from(&Z::from(u64::MAX)).is_err());
        assert!(i64::try_from(&(-1 * Z::from(u64::MAX))).is_err());
    }

    /// ensure that a correct value is returned for values in bounds.
    #[test]
    fn correct() {
        let min = Z::from(i64::MIN);
        let max = Z::from(i64::MAX);
        let z_42 = Z::from(42);

        assert_eq!(i64::MIN, i64::try_from(&min).unwrap());
        assert_eq!(i64::MAX, i64::try_from(&max).unwrap());
        assert_eq!(0, i64::try_from(&Z::ZERO).unwrap());
        assert_eq!(42, i64::try_from(&z_42).unwrap());
    }
}
