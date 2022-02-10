// Copyright (c) 2021 Thomas Otterson
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

// Copyright (c) 2021 Thomas Otterson
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use crate::double::Double;
use core::iter::{Product, Sum};

impl Sum for Double {
    /// Sums all of the values in an iterator of `Double`s.
    ///
    /// # Examples
    /// ```
    /// # use qd::{dd, Double};
    /// use core::iter::Sum;
    ///
    /// let expected = dd!(15);
    /// let actual: Double = alloc::vec![dd!(1), dd!(2), dd!(3), dd!(4), dd!(5)].into_iter().sum();
    /// assert!(expected == actual);
    /// ```
    fn sum<I>(iter: I) -> Double
    where
        I: Iterator<Item = Double>,
    {
        iter.fold(Double::ZERO, |a, b| a + b)
    }
}

impl<'a> Sum<&'a Double> for Double {
    /// Sums all of the referenced values in an iterator of `Double`s.
    ///
    /// # Examples
    /// ```
    /// # use qd::{dd, Double};
    /// use core::iter::Sum;
    ///
    /// let expected = dd!(15);
    /// let actual: Double = alloc::vec![dd!(1), dd!(2), dd!(3), dd!(4), dd!(5)].iter().sum();
    /// assert!(expected == actual);
    /// ```
    fn sum<I>(iter: I) -> Double
    where
        I: Iterator<Item = &'a Double>,
    {
        iter.fold(Double::ZERO, |a, b| a + *b)
    }
}

impl Product for Double {
    /// Multiplies all of the values in an iterator of `Double`s.
    ///
    /// # Examples
    /// ```
    /// # use qd::{dd, Double};
    /// use core::iter::Product;
    ///
    /// let expected = dd!(120);
    /// let actual: Double = alloc::vec![dd!(1), dd!(2), dd!(3), dd!(4), dd!(5)].into_iter().product();
    /// assert!(expected == actual);
    /// ```
    fn product<I>(iter: I) -> Double
    where
        I: Iterator<Item = Double>,
    {
        iter.fold(Double::ONE, |a, b| a * b)
    }
}

impl<'a> Product<&'a Double> for Double {
    /// Multiples all of the referenced values in an iterator of `Double`s.
    ///
    /// # Examples
    /// ```
    /// # use qd::{dd, Double};
    /// use core::iter::Product;
    ///
    /// let expected = dd!(120);
    /// let actual: Double = alloc::vec![dd!(1), dd!(2), dd!(3), dd!(4), dd!(5)].iter().product();
    /// assert!(expected == actual);
    /// ```
    fn product<I>(iter: I) -> Double
    where
        I: Iterator<Item = &'a Double>,
    {
        iter.fold(Double::ONE, |a, b| a * *b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // sum tests
    test_all_near!(
        sum_nums_pi_234:
            dd!("3.403392041388942675001196998552795"),
            alloc::vec![Double::FRAC_PI_2, Double::FRAC_PI_3, Double::FRAC_PI_4]
                .into_iter()
                .sum::<Double>();
        sum_refs_pi_234:
            dd!("3.403392041388942675001196998552795"),
            alloc::vec![Double::FRAC_PI_2, Double::FRAC_PI_3, Double::FRAC_PI_4]
                .iter()
                .sum::<Double>();
    );
    test_all_exact!(
        sum_nums_15:
            dd!(15),
            alloc::vec![dd!(1), dd!(2), dd!(3), dd!(4), dd!(5)].into_iter().sum::<Double>();
        sum_refs_15:
            dd!(15),
            alloc::vec![dd!(1), dd!(2), dd!(3), dd!(4), dd!(5)].iter().sum::<Double>();
        sum_empty:
            Double::ZERO,
            alloc::vec![].iter().sum::<Double>();
        sum_inf:
            Double::INFINITY,
            alloc::vec![dd!(1), Double::INFINITY, dd!(3)].iter().sum::<Double>();
        sum_neg_inf:
            Double::NEG_INFINITY,
            alloc::vec![dd!(1), Double::NEG_INFINITY, dd!(3)].iter().sum::<Double>();
        sum_both_inf:
            Double::NAN,
            alloc::vec![Double::INFINITY, Double::NEG_INFINITY].iter().sum::<Double>();
        sum_nan:
            Double::NAN,
            alloc::vec![dd!(1), dd!(2), Double::NAN].iter().sum::<Double>();
    );

    // product tests
    test_all_near!(
        product_nums_pi_234:
            dd!("1.2919281950124925073115131277958906"),
            alloc::vec![Double::FRAC_PI_2, Double::FRAC_PI_3, Double::FRAC_PI_4]
                .into_iter()
                .product::<Double>();
        product_refs_pi_234:
            dd!("1.2919281950124925073115131277958906"),
            alloc::vec![Double::FRAC_PI_2, Double::FRAC_PI_3, Double::FRAC_PI_4]
                .iter()
                .product::<Double>();
    );
    test_all_exact!(
        product_nums_15:
            dd!(120),
            alloc::vec![dd!(1), dd!(2), dd!(3), dd!(4), dd!(5)].into_iter().product::<Double>();
        product_refs_15:
            dd!(120),
            alloc::vec![dd!(1), dd!(2), dd!(3), dd!(4), dd!(5)].iter().product::<Double>();
        product_empty:
            Double::ONE,
            alloc::vec![].iter().product::<Double>();
        product_inf:
            Double::INFINITY,
            alloc::vec![dd!(1), Double::INFINITY, dd!(3)].iter().product::<Double>();
        product_neg_inf:
            Double::NEG_INFINITY,
            alloc::vec![dd!(1), Double::NEG_INFINITY, dd!(3)].iter().product::<Double>();
        product_both_inf:
            Double::NEG_INFINITY,
            alloc::vec![Double::INFINITY, Double::NEG_INFINITY].iter().product::<Double>();
        product_nan:
            Double::NAN,
            alloc::vec![dd!(1), dd!(2), Double::NAN].iter().product::<Double>();
    );
}
