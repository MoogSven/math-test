// Copyright © 2023 Marvin Beckmann
//
// This file is part of qFALL-math.
//
// qFALL-math is free software: you can redistribute it and/or modify it under
// the terms of the Mozilla Public License Version 2.0 as published by the
// Mozilla Foundation. See <https://mozilla.org/en-US/MPL/2.0/>.

//! This module contains implementations of functions
//! important for serialization such as the [`Serialize`] and [`Deserialize`] trait.
//!
//! The explicit functions contain the documentation.

use super::MatQ;
use crate::macros::serialize::{deserialize, serialize};
use core::fmt;
use serde::{
    de::{Error, MapAccess, Unexpected, Visitor},
    ser::SerializeStruct,
    Deserialize, Serialize,
};
use std::str::FromStr;

serialize!("matrix", MatQ);
deserialize!("matrix", Matrix, MatQ);

#[cfg(test)]
mod test_serialize {
    use crate::rational::MatQ;
    use std::str::FromStr;

    /// tests whether the serialization of a positive [`MatQ`] works.
    #[test]
    fn serialize_output_positive() {
        let mat_poly_str = "[[17/3, 42/17],[1/3, 17/13]]";
        let mat_poly_z = MatQ::from_str(mat_poly_str).unwrap();
        let cmp_string = format!("{{\"matrix\":\"{}\"}}", mat_poly_str);

        assert_eq!(cmp_string, serde_json::to_string(&mat_poly_z).unwrap())
    }

    /// tests whether the serialization of a negative [`MatQ`] works.
    #[test]
    fn serialize_output_negative() {
        let mat_poly_str = "[[17/3, -42/17],[1/3, -17/13]]";
        let mat_poly_z = MatQ::from_str(mat_poly_str).unwrap();
        let cmp_string = format!("{{\"matrix\":\"{}\"}}", mat_poly_str);

        assert_eq!(cmp_string, serde_json::to_string(&mat_poly_z).unwrap())
    }

    /// tests whether the serialization of a positive large [`MatQ`] works.
    #[test]
    fn serialize_output_positive_large() {
        let mat_poly_str = format!("[[3, 17, {}/2, 1, 2, 13/{}, 5]]", u64::MAX, u64::MAX);
        let mat_poly_z = MatQ::from_str(&mat_poly_str).unwrap();
        let cmp_string = format!("{{\"matrix\":\"{}\"}}", mat_poly_str);

        assert_eq!(cmp_string, serde_json::to_string(&mat_poly_z).unwrap())
    }

    /// tests whether the serialization of a negative large [`MatQ`] works.
    #[test]
    fn serialize_output_negative_large() {
        let mat_poly_str = format!("[[3, -17, -{}/2, 1, 2, -13/{}, 5]]", u64::MAX, u64::MAX);
        let mat_poly_z = MatQ::from_str(&mat_poly_str).unwrap();
        let cmp_string = format!("{{\"matrix\":\"{}\"}}", mat_poly_str);

        assert_eq!(cmp_string, serde_json::to_string(&mat_poly_z).unwrap())
    }
}

#[cfg(test)]
mod test_deserialize {
    use crate::rational::MatQ;
    use std::str::FromStr;

    /// tests whether the deserialization of a positive [`MatQ`] works.
    #[test]
    fn deserialize_positive() {
        let mat_poly_str = "[[17, 42/17],[1, 17/3]]";
        let cmp_string = format!("{{\"matrix\":\"{}\"}}", mat_poly_str);

        let mat_poly_z = MatQ::from_str(mat_poly_str).unwrap();
        assert_eq!(mat_poly_z, serde_json::from_str(&cmp_string).unwrap())
    }

    /// tests whether the deserialization of a negative [`MatQ`] works.
    #[test]
    fn deserialize_negative() {
        let mat_poly_str = "[[-17, -42/17, 1],[-13, -5, -42/3]]";
        let cmp_string = format!("{{\"matrix\":\"{}\"}}", mat_poly_str);

        let mat_poly_z = MatQ::from_str(mat_poly_str).unwrap();
        assert_eq!(mat_poly_z, serde_json::from_str(&cmp_string).unwrap())
    }

    /// tests whether the deserialization of a positive large [`MatQ`] works.
    #[test]
    fn deserialize_positive_large() {
        let mat_poly_str = format!("[[3, 17, {}, 1, 2, 13/{}, 5]]", u64::MAX - 1, u64::MAX);
        let cmp_string = format!("{{\"matrix\":\"{}\"}}", mat_poly_str);

        let mat_poly_z = MatQ::from_str(&mat_poly_str).unwrap();
        assert_eq!(mat_poly_z, serde_json::from_str(&cmp_string).unwrap())
    }

    /// tests whether the deserialization of a negative large [`MatQ`] works.
    #[test]
    fn deserialize_negative_large() {
        let mat_poly_str = format!("[[3, 17, -{}, 1, 2, -13/{}, -5]]", u64::MAX - 1, u64::MAX);
        let cmp_string = format!("{{\"matrix\":\"{}\"}}", mat_poly_str);

        let mat_poly_z = MatQ::from_str(&mat_poly_str).unwrap();
        assert_eq!(mat_poly_z, serde_json::from_str(&cmp_string).unwrap())
    }

    /// tests whether no fields 'matrix' provided yield an error
    #[test]
    fn no_field_matrix() {
        let a: Result<MatQ, serde_json::Error> =
            serde_json::from_str("{{\"tree\":\"{[[2, 17, 42]] mod 57}\"}}");
        assert!(a.is_err());

        let b: Result<MatQ, serde_json::Error> = serde_json::from_str("{{}}");
        assert!(b.is_err());
    }

    /// tests whether too many fields yield an error
    #[test]
    fn too_many_fields() {
        let a: Result<MatQ, serde_json::Error> = serde_json::from_str(
            "{{\"tree\":\"{[[[2  17 42]]}\", \"matrix\":\"{[[2, 17, 42/13]]}\"}}",
        );
        assert!(a.is_err());

        let b: Result<MatQ, serde_json::Error> = serde_json::from_str(
            "{{\"matrix\":\"{[[1, 1]]}\", \"matrix\":\"{[[2, 17, -42/13]]}\"}}",
        );
        assert!(b.is_err());
    }
}
