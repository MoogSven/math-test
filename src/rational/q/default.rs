// Copyright © 2023 Marcel Luca Schmidt, Niklas Siemer
//
// This file is part of qFALL-math.
//
// qFALL-math is free software: you can redistribute it and/or modify it under
// the terms of the Mozilla Public License Version 2.0 as published by the
// Mozilla Foundation. See <https://mozilla.org/en-US/MPL/2.0/>.

//! Define default values for [`Q`].

use super::Q;
use flint_sys::{fmpq::fmpq, fmpz::fmpz};

impl Default for Q {
    /// Returns an instantiation of [`Q`] with value '0/1'.
    ///
    /// # Example
    /// ```
    /// use std::default::Default;
    /// use qfall_math::rational::Q;
    ///  
    /// let a: Q = Q::default();
    /// ```
    fn default() -> Self {
        // needs to stay a manual instantiation as try_from uses default inside
        Q {
            value: fmpq {
                num: fmpz(0),
                den: fmpz(1),
            },
        }
    }
}

impl Q {
    /// Returns an instantiation of [`Q`] with value `1`.
    ///
    /// # Example
    /// ```
    /// use qfall_math::rational::Q;
    ///  
    /// let a: Q = Q::ONE;
    /// ```
    pub const ONE: Q = Q {
        value: fmpq {
            num: fmpz(1),
            den: fmpz(1),
        },
    };

    /// Returns an instantiation of [`Q`] with value `0`.
    ///
    /// # Example
    /// ```
    /// use qfall_math::rational::Q;
    ///  
    /// let a: Q = Q::ZERO;
    /// ```
    pub const ZERO: Q = Q {
        value: fmpq {
            num: fmpz(0),
            den: fmpz(1),
        },
    };

    /// Returns an instantiation of [`Q`] with value `-1`.
    ///
    /// # Example
    /// ```
    /// use qfall_math::rational::Q;
    ///  
    /// let a: Q = Q::MINUS_ONE;
    /// ```
    pub const MINUS_ONE: Q = Q {
        value: fmpq {
            num: fmpz(-1),
            den: fmpz(1),
        },
    };
}

#[cfg(test)]
mod tests_init {
    use super::Q;

    /// Ensure that [`Default`] initializes [`Q`] with `0`.
    #[test]
    fn init_default() {
        assert_eq!(Q::ZERO, Q::default());
    }

    /// Ensure that `ZERO` initializes [`Q`] with `0`.
    #[test]
    fn init_zero() {
        assert_eq!(Q::try_from((&0, &1)).unwrap(), Q::ZERO);
    }

    /// Ensure that `ONE` initializes [`Q`] with `1`.
    #[test]
    fn init_one() {
        assert_eq!(Q::try_from((&1, &1)).unwrap(), Q::ONE);
    }

    /// Ensure that `MINUS_ONE` initializes [`Q`] with `-1`.
    #[test]
    fn init_minus_one() {
        assert_eq!(Q::try_from((&-1, &1)).unwrap(), Q::MINUS_ONE);
    }
}
