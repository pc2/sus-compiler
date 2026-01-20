use crate::prelude::*;

use ibig::{IBig, UBig};
use sus_proc_macro::get_builtin_const;

use crate::typing::abstract_type::{AbstractRankedType, BOOL_SCALAR, INT_SCALAR};
use crate::{typing::concrete_type::ConcreteGlobalReference, value::Value};

pub fn evaluate_builtin_constant(
    cst_ref: &ConcreteGlobalReference<ConstantUUID>,
) -> Result<(Value, AbstractRankedType), String> {
    match cst_ref.id {
        get_builtin_const!("true") => Ok((Value::Bool(true), BOOL_SCALAR)),
        get_builtin_const!("false") => Ok((Value::Bool(false), BOOL_SCALAR)),
        get_builtin_const!("clog2") => {
            let [val] = cst_ref.template_args.cast_to_int_array();
            if val > &ibig::ibig!(0) {
                let val = UBig::try_from(val - 1).unwrap();
                Ok((
                    Value::Integer(IBig::from(val.bit_len())),
                    INT_SCALAR.clone(),
                ))
            } else {
                Err("V must be >= 1!".to_string())
            }
        }
        get_builtin_const!("pow2") => {
            let [exponent] = cst_ref.template_args.cast_to_int_array();
            let exponent = must_be_small_uint::<usize>(exponent, "E", usize::MAX)?;
            let mut result = ibig::ubig!(0);
            result.set_bit(exponent);
            Ok((Value::Integer(result.into()), INT_SCALAR.clone()))
        }
        get_builtin_const!("pow") => {
            let [base, exponent] = cst_ref.template_args.cast_to_int_array();
            let exponent = must_be_small_uint::<usize>(exponent, "E", usize::MAX)?;
            Ok((Value::Integer(base.pow(exponent)), INT_SCALAR.clone()))
        }
        get_builtin_const!("factorial") => {
            let [n] = cst_ref.template_args.cast_to_int_array();
            let n = must_be_small_uint::<u64>(n, "N", u64::MAX)?;

            Ok((Value::Integer(factorial(n).into()), INT_SCALAR.clone()))
        }
        get_builtin_const!("falling_factorial") => {
            let [n, k] = cst_ref.template_args.cast_to_int_array();
            let n = must_be_positive(n, "N")?;
            let k = must_be_small_uint::<u64>(k, "K", u64::MAX)?;

            if UBig::from(k) > n {
                return Err("K must be <= N.".to_string());
            }

            Ok((
                Value::Integer(falling_factorial(n, k).into()),
                INT_SCALAR.clone(),
            ))
        }
        get_builtin_const!("comb") => {
            let [n, k] = cst_ref.template_args.cast_to_int_array();
            let n = must_be_positive(n, "N")?;
            let k = must_be_small_uint::<u64>(k, "K", u64::MAX)?;

            if UBig::from(k) > n {
                return Err("K must be <= N.".to_string());
            }

            Ok((
                Value::Integer((falling_factorial(n, k) / factorial(k)).into()),
                INT_SCALAR.clone(),
            ))
        }
        get_builtin_const!("min") => {
            let [a, b] = cst_ref.template_args.cast_to_int_array();

            Ok((Value::Integer(a.min(b).clone()), INT_SCALAR.clone()))
        }
        get_builtin_const!("max") => {
            let [a, b] = cst_ref.template_args.cast_to_int_array();

            Ok((Value::Integer(a.max(b).clone()), INT_SCALAR.clone()))
        }
        get_builtin_const!("noinfer") => {
            let [v] = cst_ref.template_args.cast_to_int_array();
            Ok((Value::Integer(v.clone()), INT_SCALAR.clone()))
        }
        get_builtin_const!("assert") => {
            let [condition] = cst_ref.template_args.cast_to_array();

            if condition.unwrap_value().unwrap_bool() {
                Ok((Value::Bool(true), BOOL_SCALAR))
            } else {
                Err("Assertion failed".to_string())
            }
        }
        get_builtin_const!("sizeof") => {
            let [concrete_typ] = cst_ref.template_args.cast_to_array();

            Ok((
                Value::Integer(concrete_typ.unwrap_type().sizeof().into()),
                INT_SCALAR.clone(),
            ))
        }
        get_builtin_const!("BitsToUIntGen") => {
            let [num_bits, bits] = cst_ref.template_args.cast_to_unifyable_array();
            let bits = bits.unwrap_array();
            if &IBig::from(bits.len()) != num_bits.unwrap_integer() {
                return Err("NUM_BITS != length of BITS".to_string());
            }

            let resulting_ubig = cvt_bits_to_ubig::<false>(bits);

            Ok((Value::Integer(resulting_ubig.into()), INT_SCALAR.clone()))
        }
        get_builtin_const!("BitsToIntGen") => {
            let [num_bits, bits] = cst_ref.template_args.cast_to_unifyable_array();
            let bits = bits.unwrap_array();
            if &IBig::from(bits.len()) != num_bits.unwrap_integer() {
                return Err("NUM_BITS != length of BITS".to_string());
            }
            if bits.is_empty() {
                return Err("NUM_BITS must be >= 1 for signed bits".to_string());
            };

            let resulting_ibig = cvt_signed_bits_to_ibig(bits);

            Ok((Value::Integer(resulting_ibig), INT_SCALAR.clone()))
        }
        get_builtin_const!("UIntToBitsGen") => {
            let [num_bits, v] = cst_ref.template_args.cast_to_int_array();
            let num_bits = must_be_small_uint::<usize>(num_bits, "V", usize::MAX)?;
            let v = must_be_positive(v, "V")?;

            let v_bit_len = v.bit_len();
            if v_bit_len > num_bits {
                return Err(format!(
                    "NUM_BITS is too small to store the {v_bit_len} bits needed for {v} (unsigned)!"
                ));
            }

            match cvt_ubig_to_bits::<false, 0>(&v, num_bits) {
                Ok(bits) => Ok((Value::Array(bits), INT_SCALAR.clone())),
                Err(expected_num_bits) => Err(format!(
                    "NUM_BITS is too small to store the {expected_num_bits} bits needed for {v} (signed)!"
                )),
            }
        }
        get_builtin_const!("IntToBitsGen") => {
            let [num_bits, v] = cst_ref.template_args.cast_to_int_array();
            let num_bits = must_be_small_uint::<usize>(num_bits, "V", usize::MAX)?;

            match cvt_ibig_to_signed_bits(v.clone(), num_bits) {
                Ok(bits) => Ok((Value::Array(bits), INT_SCALAR.clone())),
                Err(expected_num_bits) => Err(format!(
                    "NUM_BITS is too small to store the {expected_num_bits} bits needed for {v} (signed)!"
                )),
            }
        }
        get_builtin_const!("RepeatGen") => {
            let [t, size, v] = cst_ref.template_args.cast_to_array();

            let t = t.unwrap_type().clone();
            let v = v.unwrap_value().unwrap();
            let size =
                must_be_small_uint::<usize>(size.unwrap_value().unwrap_integer(), "V", usize::MAX)?;

            let v_copies: Vec<Value> = (0..size).map(|_| v.clone()).collect();

            Ok((Value::Array(v_copies), t.to_abstract().rank_up()))
        }
        get_builtin_const!("ReverseGen") => {
            let [t, size, v] = cst_ref.template_args.cast_to_array();

            let t = t.unwrap_type().clone();
            let v = v.unwrap_value().unwrap_array();

            let size =
                must_be_small_uint::<usize>(size.unwrap_value().unwrap_integer(), "V", usize::MAX)?;

            if size != v.len() {
                return Err(format!(
                    "SIZE={size} is not the size of the input array={}",
                    v.len()
                ));
            }

            let v_copies: Vec<Value> = v.iter().rev().cloned().collect();

            Ok((Value::Array(v_copies), t.to_abstract().rank_up()))
        }
        get_builtin_const!("ConcatGen") => {
            let [t, size_a, size_b, v_a, v_b] = cst_ref.template_args.cast_to_array();

            let t = t.unwrap_type().clone();
            let v_a = v_a.unwrap_value().unwrap_array();
            let v_b = v_b.unwrap_value().unwrap_array();

            let size_a = must_be_small_uint::<usize>(
                size_a.unwrap_value().unwrap_integer(),
                "V_A",
                usize::MAX,
            )?;
            let size_b = must_be_small_uint::<usize>(
                size_b.unwrap_value().unwrap_integer(),
                "V_B",
                usize::MAX,
            )?;

            if size_a != v_a.len() {
                return Err(format!(
                    "SIZE_A={size_a} is not the size of the input array={}",
                    v_a.len()
                ));
            }

            if size_b != v_b.len() {
                return Err(format!(
                    "SIZE_A={size_b} is not the size of the input array={}",
                    v_b.len()
                ));
            }

            let v_copies: Vec<Value> = v_a.iter().chain(v_b.iter()).cloned().collect();

            Ok((Value::Array(v_copies), t.to_abstract().rank_up()))
        }
        get_builtin_const!("__crash_compiler") => {
            panic!("__crash_compiler Intentional ICE. This is for debugging the compiler and LSP.")
        }
        other => unreachable!("{other:?} is not a known builtin constant"),
    }
}

fn must_be_positive(v: &IBig, subject: &'static str) -> Result<UBig, String> {
    UBig::try_from(v).map_err(|_| format!("{subject} must be positive! Found {v}"))
}
fn must_be_small_uint<'a, UT: TryFrom<&'a IBig> + Ord + std::fmt::Display>(
    v: &'a IBig,
    subject: &'static str,
    max: UT,
) -> Result<UT, String> {
    match UT::try_from(v) {
        Err(_) => {
            if v < &IBig::from(0) {
                Err(format!("{subject} must be positive!"))
            } else {
                Err(format!("{subject} is too large! It may be max {max}"))
            }
        }
        Ok(v) => {
            if v <= max {
                Ok(v)
            } else {
                Err(format!("{subject} is too large! It may be max {max}"))
            }
        }
    }
}

/// When value doesn't fit in num_bits, returns Err(minimum_needed_bits)
fn cvt_ubig_to_bits<const INVERT: bool, const INCLUDE_SIGN_BIT: usize>(
    v: &UBig,
    num_bits: usize,
) -> Result<Vec<Value>, usize> {
    let num_bits_needed = v.bit_len() + INCLUDE_SIGN_BIT;
    if num_bits_needed > num_bits {
        Err(num_bits_needed)
    } else {
        Ok((0..num_bits)
            .map(|idx| Value::Bool(v.bit(idx) ^ INVERT))
            .collect())
    }
}
/// When value doesn't fit in num_bits, returns Err(minimum_needed_bits)
fn cvt_ibig_to_signed_bits(v: IBig, num_bits: usize) -> Result<Vec<Value>, usize> {
    if v >= IBig::from(0) {
        // Is positive
        let as_unsigned = UBig::try_from(v).unwrap();

        cvt_ubig_to_bits::<false, 1>(&as_unsigned, num_bits)
    } else {
        // Is negative
        let mut negative_as_unsigned = UBig::try_from(-v).unwrap();
        // -x = (!x + 1) = !(x - 1)
        negative_as_unsigned -= 1;
        cvt_ubig_to_bits::<true, 1>(&negative_as_unsigned, num_bits)
    }
}
fn cvt_bits_to_ubig<const INVERT: bool>(bits: &[Value]) -> UBig {
    let mut result = ibig::ubig!(0);

    for (idx, bit) in bits.iter().enumerate().rev() {
        let bit = bit.unwrap_bool() ^ INVERT;
        if bit {
            result.set_bit(idx);
        }
    }

    result
}
/// Requires `bits.len() >= 1`
fn cvt_signed_bits_to_ibig(bits: &[Value]) -> IBig {
    let is_negative = bits.last().unwrap().unwrap_bool();
    if is_negative {
        // Do manual 2s complement if negative, such that we don't work with an infinite number of leading 1 bits.
        // Of course, because we've inverted once, we need to re-invert again
        let as_ubig = cvt_bits_to_ubig::<true>(bits);
        -IBig::from(as_ubig + 1) // -bits = !bits + 1
    } else {
        cvt_bits_to_ubig::<false>(bits).into()
    }
}
/// n! / (n - k)!
fn falling_factorial(mut n: UBig, num_terms: u64) -> UBig {
    let mut result = ibig::ubig!(1);
    for _ in 0..num_terms {
        result *= &n;
        n -= 1;
    }
    result
}
fn factorial(n: u64) -> UBig {
    let mut total = UBig::from(n);
    for v in 2..n {
        total *= v;
    }
    total
}

#[cfg(test)]
mod tests {
    use super::*;
    use ibig::{ibig, ops::Abs, ubig};

    #[test]
    fn test_factorial() {
        assert_eq!(factorial(4), ibig::UBig::from(4u32 * 3 * 2));
        assert_eq!(factorial(5), ibig::UBig::from(5u32 * 4 * 3 * 2));
        assert_eq!(factorial(6), ibig::UBig::from(6u32 * 5 * 4 * 3 * 2));
        assert_eq!(factorial(7), ibig::UBig::from(7u32 * 6 * 5 * 4 * 3 * 2));
    }
    #[test]
    fn test_falling_factorial() {
        let a = 20;
        let b = 15;

        let a_factorial = factorial(a);
        let a_b_factorial = factorial(a - b);

        assert_eq!(
            falling_factorial(UBig::from(a), b),
            a_factorial / a_b_factorial
        )
    }

    #[test]
    fn test_cvt_ubig_to_bits_basic() {
        // 5 (0b101)
        let bits = cvt_ubig_to_bits::<false, 0>(&ubig!(5), 3).unwrap();
        assert_eq!(
            bits.iter().map(|b| b.unwrap_bool()).collect::<Vec<_>>(),
            vec![true, false, true]
        );
        // Not enough bits
        assert_eq!(cvt_ubig_to_bits::<false, 0>(&ubig!(5), 2), Err(3));
        // Large value (70 bits)
        let v = ubig!(1) << 69;
        let bits = cvt_ubig_to_bits::<false, 0>(&v, 70).unwrap();
        assert!(bits[69].unwrap_bool());
        assert!(bits[..69].iter().all(|b| !b.unwrap_bool()));
    }

    #[test]
    fn test_cvt_ibig_to_signed_bits_basic() {
        // Positive 5 (0b101)
        let v = ibig!(5);
        let bits = cvt_ibig_to_signed_bits(v, 4).unwrap();
        assert_eq!(
            bits.iter().map(|b| b.unwrap_bool()).collect::<Vec<_>>(),
            vec![true, false, true, false]
        );
        // Negative -5 (should be two's complement)
        let v = ibig!(-5);
        let bits = cvt_ibig_to_signed_bits(v, 4).unwrap();
        // -5 in 4 bits is 0b1011
        assert_eq!(
            bits.iter().map(|b| b.unwrap_bool()).collect::<Vec<_>>(),
            vec![true, true, false, true]
        );
        // Not enough bits
        assert!(cvt_ibig_to_signed_bits(ibig!(127), 8).is_ok());
        assert_eq!(cvt_ibig_to_signed_bits(ibig!(128), 8), Err(9)); // Needs 9 bits (8+1 sign)
        assert!(cvt_ibig_to_signed_bits(ibig!(-512), 10).is_ok());
        assert_eq!(cvt_ibig_to_signed_bits(ibig!(-513), 10), Err(11)); // Needs 11 bits (10+1 sign)
        // Large negative
        let v = -ibig!(1) << 70;
        let bits = cvt_ibig_to_signed_bits(v, 71).unwrap();
        assert!(bits[70].unwrap_bool());
        assert!(bits[..70].iter().all(|b| !b.unwrap_bool()));
    }

    #[test]
    fn test_cvt_bits_to_ubig_basic() {
        // 0b10101
        let bits = vec![
            Value::Bool(true),
            Value::Bool(false),
            Value::Bool(true),
            Value::Bool(false),
            Value::Bool(true),
        ];
        assert_eq!(cvt_bits_to_ubig::<false>(&bits), ubig!(21));
        // Invert
        assert_eq!(cvt_bits_to_ubig::<true>(&bits), ubig!(10)); // 0b01010
        // Large (>64 bits)
        let mut bits = vec![Value::Bool(false); 130];
        bits[129] = Value::Bool(true);
        assert_eq!(cvt_bits_to_ubig::<false>(&bits), ubig!(1) << 129);
    }

    #[test]
    fn test_cvt_signed_bits_to_ibig_basic() {
        // Positive: 0b0101 (4 bits, sign bit 0)
        let bits = vec![
            Value::Bool(true),
            Value::Bool(false),
            Value::Bool(true),
            Value::Bool(false),
        ];
        assert_eq!(cvt_signed_bits_to_ibig(&bits), ibig!(5));
        // Negative: 0b1011 (4 bits, sign bit 1)
        let bits = vec![
            Value::Bool(true),
            Value::Bool(true),
            Value::Bool(false),
            Value::Bool(true),
        ];
        assert_eq!(cvt_signed_bits_to_ibig(&bits), ibig!(-5));
        // Large negative
        let mut bits = vec![Value::Bool(false); 130];
        bits[129] = Value::Bool(true); // sign bit
        assert_eq!(cvt_signed_bits_to_ibig(&bits), -ibig!(1) << 129);
    }

    #[test]
    fn test_ubig_bits_roundtrip() {
        // Try a range of values, including large
        let vals = [
            ubig!(0),
            ubig!(1),
            ubig!(123456789),
            (ubig!(1) << 100) + 1234,
        ];
        for v in vals.iter() {
            let n = v.bit_len();
            let bits = cvt_ubig_to_bits::<false, 0>(v, n).unwrap();
            let v2 = cvt_bits_to_ubig::<false>(&bits);
            assert_eq!(v, &v2);
        }
    }

    #[test]
    fn test_ibig_signed_bits_roundtrip() {
        // Try a range of values, including large and negative
        let vals = [
            ibig!(0),
            ibig!(1),
            ibig!(-1),
            ibig!(123456789),
            ibig!(-123456789),
            (ibig!(1) << 100) + 1234,
            -((ibig!(1) << 100) + 1234u64),
        ];
        for v in vals.iter() {
            let n = UBig::try_from(v.abs()).unwrap().bit_len() + 1;
            let bits = cvt_ibig_to_signed_bits(v.clone(), n).unwrap();
            let v2 = cvt_signed_bits_to_ibig(&bits);
            assert_eq!(v, &v2);
        }
    }
}
