// Copyright (c) 2021 Thomas J. Otterson
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use crate::common::display as d;
use crate::quad::Quad;
use alloc::{
    fmt::{Debug, Display, Formatter, LowerExp, Result, UpperExp},
    string::String,
    vec::Vec,
};
use core::char;

const TEN: Quad = Quad(10.0, 0.0, 0.0, 0.0);
const MAX_ACCURACY: usize = 62;

impl Display for Quad {
    /// Formats a `Quad` for display.
    ///
    /// All formatting options that are shown in [`alloc::fmt`] are supported
    /// *except* for ones that are typically meant only for integers
    /// (hexadecimal, binary, octal, and pointer formats). Because of this,
    /// the "alternate" (`#`) flag is only recognized along with `?`,
    /// pretty-printing the `Debug` output.
    ///
    /// By default, `Quad`s are printed with 62 digits but drop trailing zeros.
    ///
    /// This function also provides the formatting for [`to_string`], which
    /// renders the `Quad` as if formatted with an empty format specifier
    /// (`"{}"`).
    ///
    /// # Examples
    /// ```
    /// # use qd::{qd, Quad};
    /// assert!(alloc::format!("{}", qd!(1.5)) == "1.5");
    ///
    /// assert!(alloc::format!("{}", Quad::PI) ==
    ///     "3.1415926535897932384626433832795028841971693993751058209749446");
    /// assert!(alloc::format!("{}", Quad::E) ==
    ///     "2.7182818284590452353602874713526624977572470936999595749669676");
    ///
    /// // to_string renders as if formatted with "{}"
    /// assert!(Quad::PI.to_string() ==
    ///     "3.1415926535897932384626433832795028841971693993751058209749446");
    ///
    /// // debug
    /// assert!(alloc::format!("{:?}", Quad::PI) ==
    ///     "Quad(3.141592653589793e0, 1.2246467991473532e-16, -2.9947698097183397e-33, 1.1124542208633655e-49)");
    /// assert!(alloc::format!("{:#?}", Quad::PI) ==
    /// "Quad(
    ///     3.141592653589793e0,
    ///     1.2246467991473532e-16,
    ///     -2.9947698097183397e-33,
    ///     1.1124542208633655e-49
    /// )");
    ///
    /// // precision and exponents
    /// let value = qd!(0.016_777_216);
    /// assert!(alloc::format!("{:.0}", value) == "0");
    /// assert!(alloc::format!("{:.5}", value) == "0.01678");
    /// assert!(alloc::format!("{:.12}", value) == "0.016777216000");
    /// assert!(alloc::format!("{:.3e}", value) == "1.678e-2");
    /// assert!(alloc::format!("{:.*e}", 3, value) == "1.678e-2");
    /// assert!(alloc::format!("{0:.1$E}", value, 4) == "1.6777E-2");
    /// assert!(alloc::format!("{:.prec$E}", value, prec = 10) == "1.6777216000E-2");
    ///
    /// // width, alignment, and fill
    /// let value = qd!(123_456);
    /// assert_eq!(alloc::format!("{:10}", value), "    123456"); // right-align is the default
    /// assert!(alloc::format!("{:>10}", value) == "    123456");
    /// assert!(alloc::format!("{:<10}", value) == "123456    ");
    /// assert!(alloc::format!("{:^10}", value) == "  123456  ");
    /// assert!(alloc::format!("{:0>10}", value) == "0000123456");
    /// assert!(alloc::format!("{:*<10}", value) == "123456****");
    /// assert!(alloc::format!("{:'^10}", value) == "''123456''");
    ///
    /// // plus sign and sign-aware zero fill
    /// let value = qd!(123_456);
    /// assert!(alloc::format!("{:+}", value) == "+123456");
    /// assert!(alloc::format!("{:0>10}", -value) == "000-123456");
    /// assert!(alloc::format!("{:010}", -value) == "-000123456");
    /// assert!(alloc::format!("{:+012e}", value) == "+001.23456e5");
    /// ```
    ///
    /// [`alloc::fmt`]: https://doc.rust-lang.org/std/fmt/index.html
    /// [`to_string`]: #tymethod.to_string
    fn fmt(&self, f: &mut Formatter) -> Result {
        let mut result = alloc::vec![];
        let signed = push_sign(&mut result, self, f);

        if self.is_nan() {
            d::push_nan(&mut result);
        } else if self.is_infinite() {
            d::push_inf(&mut result);
        } else if self.is_zero() {
            d::push_zero(&mut result, f);
        } else {
            push_digits_fixed(&mut result, self, f);
        }
        d::align_and_fill(&mut result, signed, f);

        write!(f, "{}", result.into_iter().collect::<String>())
    }
}

impl LowerExp for Quad {
    /// Formats a `Quad` for display when the "`e`" formatting option is
    /// specified.
    ///
    /// See [`Display::fmt`](#method.fmt-1) for more information.
    fn fmt(&self, f: &mut Formatter) -> Result {
        let mut result = alloc::vec![];
        let signed = push_sign(&mut result, self, f);

        if self.is_nan() {
            d::push_nan(&mut result);
        } else if self.is_infinite() {
            d::push_inf(&mut result);
        } else if self.is_zero() {
            d::push_zero(&mut result, f);
        } else {
            push_digits_exp(&mut result, self, f);
        }

        if self.is_finite() {
            let exp = if self.is_zero() {
                0
            } else {
                libm::floor(libm::log10(libm::fabs(self.0))) as i32
            };
            d::push_exp(&mut result, 'e', exp)
        }

        d::align_and_fill(&mut result, signed, f);

        write!(f, "{}", result.into_iter().collect::<String>())
    }
}

impl UpperExp for Quad {
    /// Formats a `Double` for display when the "`E`" formatting option is
    /// specified.
    ///
    /// See [`Display::fmt`](#method.fmt-1) for more information.
    fn fmt(&self, f: &mut Formatter) -> Result {
        let mut result = alloc::vec![];
        let signed = push_sign(&mut result, self, f);

        if self.is_nan() {
            d::push_nan(&mut result);
        } else if self.is_infinite() {
            d::push_inf(&mut result);
        } else if self.is_zero() {
            d::push_zero(&mut result, f);
        } else {
            push_digits_exp(&mut result, self, f);
        }

        if self.is_finite() {
            let exp = if self.is_zero() {
                0
            } else {
                libm::floor(libm::log10(libm::fabs(self.0))) as i32
            };
            d::push_exp(&mut result, 'E', exp)
        }

        d::align_and_fill(&mut result, signed, f);

        write!(f, "{}", result.into_iter().collect::<String>())
    }
}

impl Debug for Quad {
    /// Formats a `Double` for display when the "`?`" formatting option is
    /// specified.
    ///
    /// See [`Display::fmt`](#method.fmt-1) for more information.
    fn fmt(&self, f: &mut Formatter) -> Result {
        let alt = f.alternate();
        let mut str = String::from("Quad(");
        if alt {
            str.push_str("\n    ");
        }
        str.push_str(alloc::format!("{:e},", self.0).as_str());
        if alt {
            str.push_str("\n    ");
        } else {
            str.push(' ');
        }
        str.push_str(alloc::format!("{:e},", self.1).as_str());
        if alt {
            str.push_str("\n    ");
        } else {
            str.push(' ');
        }
        str.push_str(alloc::format!("{:e},", self.2).as_str());
        if alt {
            str.push_str("\n    ");
        } else {
            str.push(' ');
        }
        str.push_str(alloc::format!("{:e}", self.3).as_str());
        if alt {
            str.push('\n');
        }
        str.push(')');
        write!(f, "{}", str)
    }
}

fn push_sign(chars: &mut Vec<char>, value: &Quad, f: &Formatter) -> bool {
    if value.is_sign_negative() {
        chars.push('-');
        true
    } else if f.sign_plus() {
        chars.push('+');
        true
    } else {
        false
    }
}

fn push_digits_fixed(chars: &mut Vec<char>, value: &Quad, f: &mut Formatter) {
    let value = value.abs();
    let exp = libm::floor(libm::log10(value.0)) as i32;
    let prec = f.precision();

    let mut digits = extract_digits(&value, exp);
    d::adjust_zeros(&mut digits, exp);
    d::adjust_prec(&mut digits, exp, prec);

    chars.append(&mut d::place_decimal(digits, exp));
}

fn push_digits_exp(chars: &mut Vec<char>, value: &Quad, f: &mut Formatter) {
    let value = value.abs();
    let exp = libm::floor(libm::log10(value.0)) as i32;
    let prec = f.precision();

    let mut digits = extract_digits(&value, exp);
    d::adjust_zeros(&mut digits, 0);
    d::adjust_prec(&mut digits, 0, prec);

    chars.append(&mut d::place_decimal(digits, 0));
}

// Extracts the decimal digits of `value` into an array of unsigned integers.
//
// This function assumes that `value` is positive. Zero and non-finite values
// are handled before we get to this function, and the sign is already pushed to
// the output vector. With that assumption, this function will return a vector
// of numbers from 0-9 - digits.
fn extract_digits(value: &Quad, exp: i32) -> Vec<u8> {
    // Normalize the number to have an exponent of 0 (i.e., one digit before the
    // decimal point). We don't actually otherwise need the exponent in this
    // function, as all we're doing is parsing digits from the mantissa. This
    // normalization makes the math involved much faster. It also ensures that
    // really large numbers don't overflow on multiplication by ten.
    let divisor = TEN.powi(exp);

    let mut value = value / divisor;
    let mut digits = alloc::vec![];

    for _ in 0..(MAX_ACCURACY + 1) {
        let digit = libm::trunc(value.0);

        value -= Quad(digit, 0.0, 0.0, 0.0);
        value *= TEN;

        digits.push(digit as u8);
    }

    // We will not record digits after the 323rd (308 for the largest negative
    // exponent, plus 15 digits for the `f64` accurate width) decimal place;
    // since the `f64`s that make up `Double`s can't do that, it's just garbage
    // after that point.
    //
    // The 324 is the 323 digits plus the one needed because the first digit of the
    // number is at position exp + 1 in a negative exponent number.
    //
    // If this isn't an issue, we still truncate by one because we produced an extra
    // digit for rounding.
    d::round_and_trunc(&mut digits, (324 + exp).min(MAX_ACCURACY as i32) as usize);

    digits
}

#[cfg(test)]
mod tests {
    use super::*;

    // debug tests
    test_all_eq!(
        debug_zero:
            "Quad(0e0, 0e0, 0e0, 0e0)",
            alloc::format!("{:?}", Quad::ZERO);
        debug_pi:
            concat!(
                "Quad(",
                    "3.141592653589793e0, ",
                    "1.2246467991473532e-16, ",
                    "-2.9947698097183397e-33, ",
                    "1.1124542208633655e-49",
                ")"
            ),
            alloc::format!("{:?}", Quad::PI);
        debug_alt_zero:
            "Quad(\n    0e0,\n    0e0,\n    0e0,\n    0e0\n)",
            alloc::format!("{:#?}", Quad::ZERO);
        debug_alt_pi:
            concat!(
                "Quad(\n",
                "    3.141592653589793e0,\n",
                "    1.2246467991473532e-16,\n",
                "    -2.9947698097183397e-33,\n",
                "    1.1124542208633655e-49\n",
                ")"
            ),
            alloc::format!("{:#?}", Quad::PI);
    );

    // special number tests
    test_all_eq!(
        nan:
            "NaN",
            alloc::format!("{}", Quad::NAN);
        neg_nan:
            "-NaN",
            alloc::format!("{}", -Quad::NAN);
        plus_nan:
            "+NaN",
            alloc::format!("{:+}", Quad::NAN);
        plus_neg_nan:
            "-NaN",
            alloc::format!("{:+}", -Quad::NAN);
        inf:
            "inf",
            alloc::format!("{}", Quad::INFINITY);
        neg_inf:
            "-inf",
            alloc::format!("{}", Quad::NEG_INFINITY);
        plus_inf:
            "+inf",
            alloc::format!("{:+}", Quad::INFINITY);
        plus_neg_inf:
            "-inf",
            alloc::format!("{:+}", Quad::NEG_INFINITY);
        zero:
            "0",
            alloc::format!("{}", Quad::ZERO);
        neg_zero:
            "-0",
            alloc::format!("{}", Quad::NEG_ZERO);
        plus_zero:
            "+0",
            alloc::format!("{:+}", Quad::ZERO);
        plus_neg_zero:
            "-0",
            alloc::format!("{:+}", Quad::NEG_ZERO);
    );

    // Basic fixed number tests
    test_all_eq!(
        pi:
            "3.1415926535897932384626433832795028841971693993751058209749446",
            alloc::format!("{}", Quad::PI);
        neg_pi:
            "-3.1415926535897932384626433832795028841971693993751058209749446",
            alloc::format!("{}", -Quad::PI);
        plus_pi:
            "+3.1415926535897932384626433832795028841971693993751058209749446",
            alloc::format!("{:+}", Quad::PI);
        plus_neg_pi:
            "-3.1415926535897932384626433832795028841971693993751058209749446",
            alloc::format!("{:+}", -Quad::PI);

        ln2:
            "0.69314718055994530941723212145817656807550013436025525412068001",
            alloc::format!("{}", Quad::LN_2);
        neg_ln2:
            "-0.69314718055994530941723212145817656807550013436025525412068001",
            alloc::format!("{}", -Quad::LN_2);
        plus_ln2:
            "+0.69314718055994530941723212145817656807550013436025525412068001",
            alloc::format!("{:+}", Quad::LN_2);
        plus_neg_ln2:
            "-0.69314718055994530941723212145817656807550013436025525412068001",
            alloc::format!("{:+}", -Quad::LN_2);

        ln2_100:
            "0.0069314718055994530941723212145817656807550013436025525412068001",
            alloc::format!("{}", Quad::LN_2 / qd!(100));
        neg_ln2_100:
            "-0.0069314718055994530941723212145817656807550013436025525412068001",
            alloc::format!("{}", -Quad::LN_2 / qd!(100));
        plus_ln2_100:
            "+0.0069314718055994530941723212145817656807550013436025525412068001",
            alloc::format!("{:+}", Quad::LN_2 / qd!(100));
        plus_neg_ln2_100:
            "-0.0069314718055994530941723212145817656807550013436025525412068001",
            alloc::format!("{:+}", -Quad::LN_2 / qd!(100));

        large:
            alloc::format!("123456789012345678901234567890123456789{}", "0".repeat(270)),
            alloc::format!("{}", qd!("1.23456789012345678901234567890123456789e308"));
        neg_large:
            alloc::format!("-123456789012345678901234567890123456789{}", "0".repeat(270)),
            alloc::format!("{}", qd!("-1.23456789012345678901234567890123456789e308"));
        plus_large:
            alloc::format!("+123456789012345678901234567890123456789{}", "0".repeat(270)),
            alloc::format!("{:+}", qd!("1.23456789012345678901234567890123456789e308"));

        small:
            alloc::format!("0.{}1234567890123457", "0".repeat(307)),
            alloc::format!("{}", qd!("1.234567890123456789e-308"));
        neg_small:
            alloc::format!("-0.{}1234567890123457", "0".repeat(307)),
            alloc::format!("{}", qd!("-1.234567890123456789e-308"));
        plus_small:
            alloc::format!("+0.{}1234567890123457", "0".repeat(307)),
            alloc::format!("{:+}", qd!("1.234567890123456789e-308"));
    );

    // precision tests
    test_all_eq!(
        prec_zero:
            "0.0000",
            alloc::format!("{:.4}", Quad::ZERO);
        prec_neg_zero:
            "-0.0",
            alloc::format!("{:.1}", Quad::NEG_ZERO);
        prec_plus_zero:
            "+0.000000000000",
            alloc::format!("{:+.12}", Quad::ZERO);
        prec_plus_neg_zero:
            "-0.000000",
            alloc::format!("{:+.6}", Quad::NEG_ZERO);

        pi_prec_10:
            "3.1415926536",
            alloc::format!("{:.10}", Quad::PI);
        pi_prec_14:
            "3.14159265358979",
            alloc::format!("{:.14}", Quad::PI);
        pi_prec_40:
            "3.1415926535897932384626433832795028841972",
            alloc::format!("{:.40}", Quad::PI);
        pi_prec_70:
            "3.1415926535897932384626433832795028841971693993751058209749446000000000",
            alloc::format!("{:.70}", Quad::PI);
        pi_prec_0:
            "3",
            alloc::format!("{:.0}", Quad::PI);
        neg_pi_prec_70:
            "-3.1415926535897932384626433832795028841971693993751058209749446000000000",
            alloc::format!("{:.70}", -Quad::PI);
        neg_pi_prec_0:
            "-3",
            alloc::format!("{:.0}", -Quad::PI);
        plus_pi_prec_70:
            "+3.1415926535897932384626433832795028841971693993751058209749446000000000",
            alloc::format!("{:+.70}", Quad::PI);
        plus_pi_prec_0:
            "+3",
            alloc::format!("{:+.0}", Quad::PI);


        ln2_prec_10:
            "0.6931471806",
            alloc::format!("{:.10}", Quad::LN_2);
        ln2_prec_14:
            "0.69314718055995",
            alloc::format!("{:.14}", Quad::LN_2);
        ln2_prec_40:
            "0.6931471805599453094172321214581765680755",
            alloc::format!("{:.40}", Quad::LN_2);
        ln2_prec_70:
            "0.6931471805599453094172321214581765680755001343602552541206800100000000",
            alloc::format!("{:.70}", Quad::LN_2);
        ln2_prec_0:
            "1",
            alloc::format!("{:.0}", Quad::LN_2);
        neg_ln2_prec_70:
            "-0.6931471805599453094172321214581765680755001343602552541206800100000000",
            alloc::format!("{:.70}", -Quad::LN_2);
        neg_ln2_prec_0:
            "-1",
            alloc::format!("{:.0}", -Quad::LN_2);
        plus_ln2_prec_70:
            "+0.6931471805599453094172321214581765680755001343602552541206800100000000",
            alloc::format!("{:+.70}", Quad::LN_2);
        plus_ln2_prec_0:
            "+1",
            alloc::format!("{:+.0}", Quad::LN_2);

        ln2_100_prec_10:
            "0.0069314718",
            alloc::format!("{:.10}", Quad::LN_2 / qd!(100));
        ln2_100_prec_14:
            "0.00693147180560",
            alloc::format!("{:.14}", Quad::LN_2 / qd!(100));
        ln2_100_prec_40:
            "0.0069314718055994530941723212145817656808",
            alloc::format!("{:.40}", Quad::LN_2 / qd!(100));
        ln2_100_prec_70:
            "0.0069314718055994530941723212145817656807550013436025525412068001000000",
            alloc::format!("{:.70}", Quad::LN_2 / qd!(100));
        ln2_100_prec_0:
            "0",
            alloc::format!("{:.0}", Quad::LN_2 / qd!(100));
        neg_ln2_100_prec_70:
            "-0.0069314718055994530941723212145817656807550013436025525412068001000000",
            alloc::format!("{:.70}", -Quad::LN_2 / qd!(100));
        neg_ln2_100_prec_0:
            "-0",
            alloc::format!("{:.0}", -Quad::LN_2 / qd!(100));
        plus_ln2_100_prec_70:
            "+0.0069314718055994530941723212145817656807550013436025525412068001000000",
            alloc::format!("{:+.70}", Quad::LN_2 / qd!(100));
        plus_ln2_100_prec_0:
            "+0",
            alloc::format!("{:+.0}", Quad::LN_2 / qd!(100));


        large_prec_0:
            alloc::format!("123456789012345678901234567890123456789{}", "0".repeat(270)),
            alloc::format!("{:.0}", qd!("1.23456789012345678901234567890123456789e308"));
        large_prec_8:
            alloc::format!("123456789012345678901234567890123456789{}.00000000", "0".repeat(270)),
            alloc::format!("{:.8}", qd!("1.23456789012345678901234567890123456789e308"));


        small_prec_0:
            "0",
            alloc::format!("{:.0}", qd!("1.234567890123456789e-308"));
        small_prec_8:
            "0.00000000",
            alloc::format!("{:.8}", qd!("1.234567890123456789e-308"));
        small_prec_316:
            alloc::format!("0.{}123456789", "0".repeat(307)),
            alloc::format!("{:.316}", qd!("1.234567890123456789e-308"));
    );

    // width tests (default right align)
    test_all_eq!(
        zero_width:
            "         0",
            alloc::format!("{:10}", Quad::ZERO);
        neg_zero_width:
            "        -0",
            alloc::format!("{:10}", Quad::NEG_ZERO);
        inf_width:
            "       inf",
            alloc::format!("{:10}", Quad::INFINITY);
        neg_inf_width:
            "      -inf",
            alloc::format!("{:10}", Quad::NEG_INFINITY);
        nan_width:
            "       NaN",
            alloc::format!("{:10}", Quad::NAN);
        one_width:
            "         1",
            alloc::format!("{:10}", Quad::ONE);
        neg_one_width:
            "        -1",
            alloc::format!("{:10}", Quad::NEG_ONE);
        plus_one_width:
            "        +1",
            alloc::format!("{:+10}", Quad::ONE);
        pi_width:
            "3.1415926535897932384626433832795028841971693993751058209749446",
            alloc::format!("{:10}", Quad::PI);
        pi_width_prec:
            "   3.14159",
            alloc::format!("{:10.5}", Quad::PI);
    );

    // right-align tests
    test_all_eq!(
        zero_right:
            "         0",
            alloc::format!("{:>10}", Quad::ZERO);
        neg_zero_right:
            "        -0",
            alloc::format!("{:>10}", Quad::NEG_ZERO);
        inf_right:
            "       inf",
            alloc::format!("{:>10}", Quad::INFINITY);
        neg_inf_right:
            "      -inf",
            alloc::format!("{:>10}", Quad::NEG_INFINITY);
        nan_right:
            "       NaN",
            alloc::format!("{:>10}", Quad::NAN);
        one_right:
            "         1",
            alloc::format!("{:>10}", Quad::ONE);
        neg_one_right:
            "        -1",
            alloc::format!("{:>10}", Quad::NEG_ONE);
        plus_one_right:
            "        +1",
            alloc::format!("{:>+10}", Quad::ONE);
        pi_right:
            "3.1415926535897932384626433832795028841971693993751058209749446",
            alloc::format!("{:>10}", Quad::PI);
        pi_right_prec:
            "   3.14159",
            alloc::format!("{:>10.5}", Quad::PI);
    );

    // left-align tests
    test_all_eq!(
        zero_left:
            "0         ",
            alloc::format!("{:<10}", Quad::ZERO);
        neg_zero_left:
            "-0        ",
            alloc::format!("{:<10}", Quad::NEG_ZERO);
        inf_left:
            "inf       ",
            alloc::format!("{:<10}", Quad::INFINITY);
        neg_inf_left:
            "-inf      ",
            alloc::format!("{:<10}", Quad::NEG_INFINITY);
        nan_left:
            "NaN       ",
            alloc::format!("{:<10}", Quad::NAN);
        one_left:
            "1         ",
            alloc::format!("{:<10}", Quad::ONE);
        neg_one_left:
            "-1        ",
            alloc::format!("{:<10}", Quad::NEG_ONE);
        plus_one_left:
            "+1        ",
            alloc::format!("{:<+10}", Quad::ONE);
        pi_left:
            "3.1415926535897932384626433832795028841971693993751058209749446",
            alloc::format!("{:<10}", Quad::PI);
        pi_left_prec:
            "3.14159   ",
            alloc::format!("{:<10.5}", Quad::PI);
    );

    // center-align tests
    test_all_eq!(
        zero_center:
            "    0     ",
            alloc::format!("{:^10}", Quad::ZERO);
        neg_zero_center:
            "    -0    ",
            alloc::format!("{:^10}", Quad::NEG_ZERO);
        inf_center:
            "   inf    ",
            alloc::format!("{:^10}", Quad::INFINITY);
        neg_inf_center:
            "   -inf   ",
            alloc::format!("{:^10}", Quad::NEG_INFINITY);
        nan_center:
            "   NaN    ",
            alloc::format!("{:^10}", Quad::NAN);
        one_center:
            "    1     ",
            alloc::format!("{:^10}", Quad::ONE);
        neg_one_center:
            "    -1    ",
            alloc::format!("{:^10}", Quad::NEG_ONE);
        plus_one_center:
            "    +1    ",
            alloc::format!("{:^+10}", Quad::ONE);
        pi_center:
            "3.1415926535897932384626433832795028841971693993751058209749446",
            alloc::format!("{:^10}", Quad::PI);
        pi_center_prec:
            " 3.14159  ",
            alloc::format!("{:^10.5}", Quad::PI);
    );

    // fill tests
    test_all_eq!(
        zero_fill:
            "_________0",
            alloc::format!("{:_>10}", Quad::ZERO);
        neg_zero_fill:
            "-0________",
            alloc::format!("{:_<10}", Quad::NEG_ZERO);
        inf_fill:
            "___inf____",
            alloc::format!("{:_^10}", Quad::INFINITY);
        neg_inf_fill:
            "______-inf",
            alloc::format!("{:_>10}", Quad::NEG_INFINITY);
        nan_fill:
            "NaN_______",
            alloc::format!("{:_<10}", Quad::NAN);
        one_fill:
            "____1_____",
            alloc::format!("{:_^10}", Quad::ONE);
        neg_one_fill:
            "________-1",
            alloc::format!("{:_>10}", Quad::NEG_ONE);
        plus_one_fill:
            "+1________",
            alloc::format!("{:_<+10}", Quad::ONE);
        pi_fill:
            "3.1415926535897932384626433832795028841971693993751058209749446",
            alloc::format!("{:_^10}", Quad::PI);
        pi_fill_prec:
            "___3.14159",
            alloc::format!("{:_>10.5}", Quad::PI);
    );

    // sign-aware zero-poading tests
    test_all_eq!(
        zero_zero_pad:
            "0000000000",
            alloc::format!("{:010}", Quad::ZERO);
        neg_zero_zero_pad:
            "-000000000",
            alloc::format!("{:010}", Quad::NEG_ZERO);
        inf_zero_pad:
            "0000000inf",
            alloc::format!("{:010}", Quad::INFINITY);
        neg_inf_zero_pad:
            "-000000inf",
            alloc::format!("{:010}", Quad::NEG_INFINITY);
        nan_zero_pad:
            "0000000NaN",
            alloc::format!("{:010}", Quad::NAN);
        one_zero_pad:
            "0000000001",
            alloc::format!("{:010}", Quad::ONE);
        neg_one_zero_pad:
            "-000000001",
            alloc::format!("{:010}", Quad::NEG_ONE);
        plus_one_zero_pad:
            "+000000001",
            alloc::format!("{:+010}", Quad::ONE);
        pi_zero_pad:
            "3.1415926535897932384626433832795028841971693993751058209749446",
            alloc::format!("{:010}", Quad::PI);
        pi_width_zero_pad_prec:
            "0003.14159",
            alloc::format!("{:010.5}", Quad::PI);
    );

    // special number exp tests
    test_all_eq!(
        nan_lexp:
            "NaN",
            alloc::format!("{:e}", Quad::NAN);
        neg_nan_lexp:
            "-NaN",
            alloc::format!("{:e}", -Quad::NAN);
        plus_nan_lexp:
            "+NaN",
            alloc::format!("{:+e}", Quad::NAN);
        plus_neg_nan_lexp:
            "-NaN",
            alloc::format!("{:+e}", -Quad::NAN);
        inf_lexp:
            "inf",
            alloc::format!("{:e}", Quad::INFINITY);
        neg_inf_lexp:
            "-inf",
            alloc::format!("{:e}", Quad::NEG_INFINITY);
        plus_inf_lexp:
            "+inf",
            alloc::format!("{:+e}", Quad::INFINITY);
        plus_neg_inf_lexp:
            "-inf",
            alloc::format!("{:+e}", Quad::NEG_INFINITY);
        zero_lexp:
            "0e0",
            alloc::format!("{:e}", Quad::ZERO);
        neg_zero_lexp:
            "-0e0",
            alloc::format!("{:e}", Quad::NEG_ZERO);
        plus_zero_lexp:
            "+0e0",
            alloc::format!("{:+e}", Quad::ZERO);
        plus_neg_zero_lexp:
            "-0e0",
            alloc::format!("{:+e}", Quad::NEG_ZERO);
    );

    // special number Exp tests
    test_all_eq!(
        nan_uexp:
            "NaN",
            alloc::format!("{:E}", Quad::NAN);
        neg_nan_uexp:
            "-NaN",
            alloc::format!("{:E}", -Quad::NAN);
        plus_nan_uexp:
            "+NaN",
            alloc::format!("{:+E}", Quad::NAN);
        plus_neg_nan_uexp:
            "-NaN",
            alloc::format!("{:+E}", -Quad::NAN);
        inf_uexp:
            "inf",
            alloc::format!("{:E}", Quad::INFINITY);
        neg_inf_uexp:
            "-inf",
            alloc::format!("{:E}", Quad::NEG_INFINITY);
        plus_inf_uexp:
            "+inf",
            alloc::format!("{:+E}", Quad::INFINITY);
        plus_neg_inf_uexp:
            "-inf",
            alloc::format!("{:+E}", Quad::NEG_INFINITY);
        zero_uexp:
            "0E0",
            alloc::format!("{:E}", Quad::ZERO);
        neg_zero_uexp:
            "-0E0",
            alloc::format!("{:E}", Quad::NEG_ZERO);
        plus_zero_uexp:
            "+0E0",
            alloc::format!("{:+E}", Quad::ZERO);
        plus_neg_zero_uexp:
            "-0E0",
            alloc::format!("{:+E}", Quad::NEG_ZERO);
    );

    // Basic exp tests
    test_all_eq!(
        pi_lexp:
            "3.1415926535897932384626433832795028841971693993751058209749446e0",
            alloc::format!("{:e}", Quad::PI);
        neg_pi_lexp:
            "-3.1415926535897932384626433832795028841971693993751058209749446e0",
            alloc::format!("{:e}", -Quad::PI);
        plus_pi_lexp:
            "+3.1415926535897932384626433832795028841971693993751058209749446e0",
            alloc::format!("{:+e}", Quad::PI);
        plus_neg_pi_lexp:
            "-3.1415926535897932384626433832795028841971693993751058209749446e0",
            alloc::format!("{:+e}", -Quad::PI);

        ln2_lexp:
            "6.9314718055994530941723212145817656807550013436025525412068001e-1",
            alloc::format!("{:e}", Quad::LN_2);
        neg_ln2_lexp:
            "-6.9314718055994530941723212145817656807550013436025525412068001e-1",
            alloc::format!("{:e}", -Quad::LN_2);
        plus_ln2_lexp:
            "+6.9314718055994530941723212145817656807550013436025525412068001e-1",
            alloc::format!("{:+e}", Quad::LN_2);
        plus_neg_ln2_lexp:
            "-6.9314718055994530941723212145817656807550013436025525412068001e-1",
            alloc::format!("{:+e}", -Quad::LN_2);

        ln2_100_lexp:
            "6.9314718055994530941723212145817656807550013436025525412068001e-3",
            alloc::format!("{:e}", Quad::LN_2 / qd!(100));
        neg_ln2_100_lexp:
            "-6.9314718055994530941723212145817656807550013436025525412068001e-3",
            alloc::format!("{:e}", -Quad::LN_2 / qd!(100));
        plus_ln2_100_lexp:
            "+6.9314718055994530941723212145817656807550013436025525412068001e-3",
            alloc::format!("{:+e}", Quad::LN_2 / qd!(100));
        plus_neg_ln2_100_lexp:
            "-6.9314718055994530941723212145817656807550013436025525412068001e-3",
            alloc::format!("{:+e}", -Quad::LN_2 / qd!(100));

        large_lexp:
            "1.23456789012345678901234567890123456789e308",
            alloc::format!("{:e}", qd!("1.23456789012345678901234567890123456789e308"));
        neg_large_lexp:
            "-1.23456789012345678901234567890123456789e308",
            alloc::format!("{:e}", qd!("-1.23456789012345678901234567890123456789e308"));
        plus_large_lexp:
            "+1.23456789012345678901234567890123456789e308",
            alloc::format!("{:+e}", qd!("1.23456789012345678901234567890123456789e308"));

        small_lexp:
            "1.234567890123457e-308",
            alloc::format!("{:e}", qd!("1.234567890123456789e-308"));
        neg_small_lexp:
            "-1.234567890123457e-308",
            alloc::format!("{:e}", qd!("-1.234567890123456789e-308"));
        plus_small_lexp:
            "+1.234567890123457e-308",
            alloc::format!("{:+e}", qd!("1.234567890123456789e-308"));
    );

    // Basic EXP tests
    test_all_eq!(
        pi_uexp:
            "3.1415926535897932384626433832795028841971693993751058209749446E0",
            alloc::format!("{:E}", Quad::PI);
        neg_pi_uexp:
            "-3.1415926535897932384626433832795028841971693993751058209749446E0",
            alloc::format!("{:E}", -Quad::PI);
        plus_pi_uexp:
            "+3.1415926535897932384626433832795028841971693993751058209749446E0",
            alloc::format!("{:+E}", Quad::PI);
        plus_neg_pi_uexp:
            "-3.1415926535897932384626433832795028841971693993751058209749446E0",
            alloc::format!("{:+E}", -Quad::PI);

        ln2_uexp:
            "6.9314718055994530941723212145817656807550013436025525412068001E-1",
            alloc::format!("{:E}", Quad::LN_2);
        neg_ln2_uexp:
            "-6.9314718055994530941723212145817656807550013436025525412068001E-1",
            alloc::format!("{:E}", -Quad::LN_2);
        plus_ln2_uexp:
            "+6.9314718055994530941723212145817656807550013436025525412068001E-1",
            alloc::format!("{:+E}", Quad::LN_2);
        plus_neg_ln2_uexp:
            "-6.9314718055994530941723212145817656807550013436025525412068001E-1",
            alloc::format!("{:+E}", -Quad::LN_2);

        ln2_100_uexp:
            "6.9314718055994530941723212145817656807550013436025525412068001E-3",
            alloc::format!("{:E}", Quad::LN_2 / qd!(100));
        neg_ln2_100_uexp:
            "-6.9314718055994530941723212145817656807550013436025525412068001E-3",
            alloc::format!("{:E}", -Quad::LN_2 / qd!(100));
        plus_ln2_100_uexp:
            "+6.9314718055994530941723212145817656807550013436025525412068001E-3",
            alloc::format!("{:+E}", Quad::LN_2 / qd!(100));
        plus_neg_ln2_100_uexp:
            "-6.9314718055994530941723212145817656807550013436025525412068001E-3",
            alloc::format!("{:+E}", -Quad::LN_2 / qd!(100));

        large_uexp:
            "1.23456789012345678901234567890123456789E308",
            alloc::format!("{:E}", qd!("1.23456789012345678901234567890123456789e308"));
        neg_large_uexp:
            "-1.23456789012345678901234567890123456789E308",
            alloc::format!("{:E}", qd!("-1.23456789012345678901234567890123456789e308"));
        plus_large_uexp:
            "+1.23456789012345678901234567890123456789E308",
            alloc::format!("{:+E}", qd!("1.23456789012345678901234567890123456789e308"));

        small_uexp:
            "1.234567890123457E-308",
            alloc::format!("{:E}", qd!("1.234567890123456789e-308"));
        neg_small_uexp:
            "-1.234567890123457E-308",
            alloc::format!("{:E}", qd!("-1.234567890123456789e-308"));
        plus_small_uexp:
            "+1.234567890123457E-308",
            alloc::format!("{:+E}", qd!("1.234567890123456789e-308"));
    );

    // NOTE
    //
    // The code is so similar between UpperExp and LowerExp that after those tests
    // there's not much need to continue to test them both. Further tests are
    // being done solely with LowerExp.

    // precision exp tests
    test_all_eq!(
        prec_zero_exp:
            "0.0000e0",
            alloc::format!("{:.4e}", Quad::ZERO);
        prec_neg_zero_exp:
            "-0.0e0",
            alloc::format!("{:.1e}", Quad::NEG_ZERO);
        prec_plus_zero_exp:
            "+0.000000000000e0",
            alloc::format!("{:+.12e}", Quad::ZERO);
        prec_plus_neg_zero_exp:
            "-0.000000e0",
            alloc::format!("{:+.6e}", Quad::NEG_ZERO);

        pi_prec_10_exp:
            "3.1415926536e0",
            alloc::format!("{:.10e}", Quad::PI);
        pi_prec_14_exp:
            "3.14159265358979e0",
            alloc::format!("{:.14e}", Quad::PI);
        pi_prec_40_exp:
            "3.1415926535897932384626433832795028841972e0",
            alloc::format!("{:.40e}", Quad::PI);
        pi_prec_70_exp:
            "3.1415926535897932384626433832795028841971693993751058209749446000000000e0",
            alloc::format!("{:.70e}", Quad::PI);
        pi_prec_0_exp:
            "3e0",
            alloc::format!("{:.0e}", Quad::PI);
        neg_pi_prec_70_exp:
            "-3.1415926535897932384626433832795028841971693993751058209749446000000000e0",
            alloc::format!("{:.70e}", -Quad::PI);
        neg_pi_prec_0_exp:
            "-3e0",
            alloc::format!("{:.0e}", -Quad::PI);
        plus_pi_prec_70_exp:
            "+3.1415926535897932384626433832795028841971693993751058209749446000000000e0",
            alloc::format!("{:+.70e}", Quad::PI);
        plus_pi_prec_0_exp:
            "+3e0",
            alloc::format!("{:+.0e}", Quad::PI);


        ln2_prec_10_exp:
            "6.9314718056e-1",
            alloc::format!("{:.10e}", Quad::LN_2);
        ln2_prec_14_exp:
            "6.93147180559945e-1",
            alloc::format!("{:.14e}", Quad::LN_2);
        ln2_prec_40_exp:
            "6.9314718055994530941723212145817656807550e-1",
            alloc::format!("{:.40e}", Quad::LN_2);
        ln2_prec_70_exp:
            "6.9314718055994530941723212145817656807550013436025525412068001000000000e-1",
            alloc::format!("{:.70e}", Quad::LN_2);
        ln2_prec_0_exp:
            "7e-1",
            alloc::format!("{:.0e}", Quad::LN_2);
        neg_ln2_prec_70_exp:
            "-6.9314718055994530941723212145817656807550013436025525412068001000000000e-1",
            alloc::format!("{:.70e}", -Quad::LN_2);
        neg_ln2_prec_0_exp:
            "-7e-1",
            alloc::format!("{:.0e}", -Quad::LN_2);
        plus_ln2_prec_70_exp:
            "+6.9314718055994530941723212145817656807550013436025525412068001000000000e-1",
            alloc::format!("{:+.70e}", Quad::LN_2);
        plus_ln2_prec_0_exp:
            "+7e-1",
            alloc::format!("{:+.0e}", Quad::LN_2);

        ln2_100_prec_10_exp:
            "6.9314718056e-3",
            alloc::format!("{:.10e}", Quad::LN_2 / qd!(100));
        ln2_100_prec_14_exp:
            "6.93147180559945e-3",
            alloc::format!("{:.14e}", Quad::LN_2 / qd!(100));
        ln2_100_prec_40_exp:
            "6.9314718055994530941723212145817656807550e-3",
            alloc::format!("{:.40e}", Quad::LN_2 / qd!(100));
        ln2_100_prec_70_exp:
            "6.9314718055994530941723212145817656807550013436025525412068001000000000e-3",
            alloc::format!("{:.70e}", Quad::LN_2 / qd!(100));
        ln2_100_prec_0_exp:
            "7e-3",
            alloc::format!("{:.0e}", Quad::LN_2 / qd!(100));
        neg_ln2_100_prec_70_exp:
            "-6.9314718055994530941723212145817656807550013436025525412068001000000000e-3",
            alloc::format!("{:.70e}", -Quad::LN_2 / qd!(100));
        neg_ln2_100_prec_0_exp:
            "-7e-3",
            alloc::format!("{:.0e}", -Quad::LN_2 / qd!(100));
        plus_ln2_100_prec_40_exp:
            "+6.9314718055994530941723212145817656807550013436025525412068001000000000e-3",
            alloc::format!("{:+.70e}", Quad::LN_2 / qd!(100));
        plus_ln2_100_prec_0_exp:
            "+7e-3",
            alloc::format!("{:+.0e}", Quad::LN_2 / qd!(100));


        large_prec_0_exp:
            "1e308",
            alloc::format!("{:.0e}", qd!("1.23456789012345678901234567890123456789e308"));
        large_prec_8_exp:
            "1.23456789e308",
            alloc::format!("{:.8e}", qd!("1.23456789012345678901234567890123456789e308"));


        small_prec_0_exp:
            "1e-308",
            alloc::format!("{:.0e}", qd!("1.234567890123456789e-308"));
        small_prec_8_exp:
            "1.23456789e-308",
            alloc::format!("{:.8e}", qd!("1.234567890123456789e-308"));
    );

    // width tests (default right align)
    test_all_eq!(
        zero_width_exp:
            "       0e0",
            alloc::format!("{:10e}", Quad::ZERO);
        neg_zero_width_exp:
            "      -0e0",
            alloc::format!("{:10e}", Quad::NEG_ZERO);
        inf_width_exp:
            "       inf",
            alloc::format!("{:10e}", Quad::INFINITY);
        neg_inf_width_exp:
            "      -inf",
            alloc::format!("{:10e}", Quad::NEG_INFINITY);
        nan_width_exp:
            "       NaN",
            alloc::format!("{:10e}", Quad::NAN);
        one_width_exp:
            "       1e0",
            alloc::format!("{:10e}", Quad::ONE);
        neg_one_width_exp:
            "      -1e0",
            alloc::format!("{:10e}", Quad::NEG_ONE);
        plus_one_width_exp:
            "      +1e0",
            alloc::format!("{:+10e}", Quad::ONE);
        pi_width_exp:
            "3.1415926535897932384626433832795028841971693993751058209749446e0",
            alloc::format!("{:10e}", Quad::PI);
        pi_width_prec_exp:
            " 3.14159e0",
            alloc::format!("{:10.5e}", Quad::PI);
    );

    // right-align tests
    test_all_eq!(
        zero_right_exp:
            "       0e0",
            alloc::format!("{:>10e}", Quad::ZERO);
        neg_zero_right_exp:
            "      -0e0",
            alloc::format!("{:>10e}", Quad::NEG_ZERO);
        inf_right_exp:
            "       inf",
            alloc::format!("{:>10e}", Quad::INFINITY);
        neg_inf_right_exp:
            "      -inf",
            alloc::format!("{:>10e}", Quad::NEG_INFINITY);
        nan_right_exp:
            "       NaN",
            alloc::format!("{:>10e}", Quad::NAN);
        one_right_exp:
            "       1e0",
            alloc::format!("{:>10e}", Quad::ONE);
        neg_one_right_exp:
            "      -1e0",
            alloc::format!("{:>10e}", Quad::NEG_ONE);
        plus_one_right_exp:
            "      +1e0",
            alloc::format!("{:>+10e}", Quad::ONE);
        pi_right_exp:
            "3.1415926535897932384626433832795028841971693993751058209749446e0",
            alloc::format!("{:>10e}", Quad::PI);
        pi_right_prec_exp:
            " 3.14159e0",
            alloc::format!("{:>10.5e}", Quad::PI);
    );

    // left-align tests
    test_all_eq!(
        zero_left_exp:
            "0e0       ",
            alloc::format!("{:<10e}", Quad::ZERO);
        neg_zero_left_exp:
            "-0e0      ",
            alloc::format!("{:<10e}", Quad::NEG_ZERO);
        inf_left_exp:
            "inf       ",
            alloc::format!("{:<10e}", Quad::INFINITY);
        neg_inf_left_exp:
            "-inf      ",
            alloc::format!("{:<10e}", Quad::NEG_INFINITY);
        nan_left_exp:
            "NaN       ",
            alloc::format!("{:<10e}", Quad::NAN);
        one_left_exp:
            "1e0       ",
            alloc::format!("{:<10e}", Quad::ONE);
        neg_one_left_exp:
            "-1e0      ",
            alloc::format!("{:<10e}", Quad::NEG_ONE);
        plus_one_left_exp:
            "+1e0      ",
            alloc::format!("{:<+10e}", Quad::ONE);
        pi_left_exp:
            "3.1415926535897932384626433832795028841971693993751058209749446e0",
            alloc::format!("{:<10e}", Quad::PI);
        pi_left_prec_exp:
            "3.14159e0 ",
            alloc::format!("{:<10.5e}", Quad::PI);
    );

    // center-align tests
    test_all_eq!(
        zero_center_exp:
            "   0e0    ",
            alloc::format!("{:^10e}", Quad::ZERO);
        neg_zero_center_exp:
            "   -0e0   ",
            alloc::format!("{:^10e}", Quad::NEG_ZERO);
        inf_center_exp:
            "   inf    ",
            alloc::format!("{:^10e}", Quad::INFINITY);
        neg_inf_center_exp:
            "   -inf   ",
            alloc::format!("{:^10e}", Quad::NEG_INFINITY);
        nan_center_exp:
            "   NaN    ",
            alloc::format!("{:^10e}", Quad::NAN);
        one_center_exp:
            "   1e0    ",
            alloc::format!("{:^10e}", Quad::ONE);
        neg_one_center_exp:
            "   -1e0   ",
            alloc::format!("{:^10e}", Quad::NEG_ONE);
        plus_one_center_exp:
            "   +1e0   ",
            alloc::format!("{:^+10e}", Quad::ONE);
        pi_center_exp:
            "3.1415926535897932384626433832795028841971693993751058209749446e0",
            alloc::format!("{:^10e}", Quad::PI);
        pi_center_prec_exp:
            "3.14159e0 ",
            alloc::format!("{:^10.5e}", Quad::PI);
    );

    // fill tests
    test_all_eq!(
        zero_fill_exp:
            "_______0e0",
            alloc::format!("{:_>10e}", Quad::ZERO);
        neg_zero_fill_exp:
            "-0e0______",
            alloc::format!("{:_<10e}", Quad::NEG_ZERO);
        inf_fill_exp:
            "___inf____",
            alloc::format!("{:_^10e}", Quad::INFINITY);
        neg_inf_fill_exp:
            "______-inf",
            alloc::format!("{:_>10e}", Quad::NEG_INFINITY);
        nan_fill_exp:
            "NaN_______",
            alloc::format!("{:_<10e}", Quad::NAN);
        one_fill_exp:
            "___1e0____",
            alloc::format!("{:_^10e}", Quad::ONE);
        neg_one_fill_exp:
            "______-1e0",
            alloc::format!("{:_>10e}", Quad::NEG_ONE);
        plus_one_fill_exp:
            "+1e0______",
            alloc::format!("{:_<+10e}", Quad::ONE);
        pi_fill_exp:
            "3.1415926535897932384626433832795028841971693993751058209749446e0",
            alloc::format!("{:_^10e}", Quad::PI);
        pi_fill_prec_exp:
            "_3.14159e0",
            alloc::format!("{:_>10.5e}", Quad::PI);
    );

    // sign-aware zero-poading tests
    test_all_eq!(
        zero_zero_pad_exp:
            "00000000e0",
            alloc::format!("{:010e}", Quad::ZERO);
        neg_zero_zero_pad_exp:
            "-0000000e0",
            alloc::format!("{:010e}", Quad::NEG_ZERO);
        inf_zero_pad_exp:
            "0000000inf",
            alloc::format!("{:010e}", Quad::INFINITY);
        neg_inf_zero_pad_exp:
            "-000000inf",
            alloc::format!("{:010e}", Quad::NEG_INFINITY);
        nan_zero_pad_exp:
            "0000000NaN",
            alloc::format!("{:010e}", Quad::NAN);
        one_zero_pad_exp:
            "00000001e0",
            alloc::format!("{:010e}", Quad::ONE);
        neg_one_zero_pad_exp:
            "-0000001e0",
            alloc::format!("{:010e}", Quad::NEG_ONE);
        plus_one_zero_pad_exp:
            "+0000001e0",
            alloc::format!("{:+010e}", Quad::ONE);
        pi_zero_pad_exp:
            "3.1415926535897932384626433832795028841971693993751058209749446e0",
            alloc::format!("{:010e}", Quad::PI);
        pi_width_zero_pad_prec_exp:
            "03.14159e0",
            alloc::format!("{:010.5e}", Quad::PI);
    );
}
