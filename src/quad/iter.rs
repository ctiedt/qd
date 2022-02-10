// Copyright (c) 2021 Thomas Otterson
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use crate::quad::Quad;
use core::iter::{Product, Sum};

impl Sum for Quad {
    /// Sums all of the values in an iterator of `Quad`s.
    ///
    /// # Examples
    /// ```
    /// # use qd::{qd, Quad};
    /// use core::iter::Sum;
    ///
    /// let expected = qd!(15);
    /// let actual: Quad = alloc::vec![qd!(1), qd!(2), qd!(3), qd!(4), qd!(5)].into_iter().sum();
    /// assert!(expected == actual);
    /// ```
    fn sum<I>(iter: I) -> Quad
    where
        I: Iterator<Item = Quad>,
    {
        iter.fold(Quad::ZERO, |a, b| a + b)
    }
}

impl<'a> Sum<&'a Quad> for Quad {
    /// Sums all of the referenced values in an iterator of `Quad`s.
    ///
    /// # Examples
    /// ```
    /// # use qd::{qd, Quad};
    /// use core::iter::Sum;
    ///
    /// let expected = qd!(15);
    /// let actual: Quad = alloc::vec![qd!(1), qd!(2), qd!(3), qd!(4), qd!(5)].iter().sum();
    /// assert!(expected == actual);
    /// ```
    fn sum<I>(iter: I) -> Quad
    where
        I: Iterator<Item = &'a Quad>,
    {
        iter.fold(Quad::ZERO, |a, b| a + *b)
    }
}

impl Product for Quad {
    /// Multiplies all of the values in an iterator of `Quad`s.
    ///
    /// # Examples
    /// ```
    /// # use qd::{qd, Quad};
    /// use core::iter::Product;
    ///
    /// let expected = qd!(120);
    /// let actual: Quad = alloc::vec![qd!(1), qd!(2), qd!(3), qd!(4), qd!(5)].into_iter().product();
    /// assert!(expected == actual);
    /// ```
    fn product<I>(iter: I) -> Quad
    where
        I: Iterator<Item = Quad>,
    {
        iter.fold(Quad::ONE, |a, b| a * b)
    }
}

impl<'a> Product<&'a Quad> for Quad {
    /// Multiples all of the referenced values in an iterator of `Quad`s.
    ///
    /// # Examples
    /// ```
    /// # use qd::{qd, Quad};
    /// use core::iter::Product;
    ///
    /// let expected = qd!(120);
    /// let actual: Quad = alloc::vec![qd!(1), qd!(2), qd!(3), qd!(4), qd!(5)].iter().product();
    /// assert!(expected == actual);
    /// ```
    fn product<I>(iter: I) -> Quad
    where
        I: Iterator<Item = &'a Quad>,
    {
        iter.fold(Quad::ONE, |a, b| a * *b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // sum tests
    test_all_near!(
        sum_nums_pi_234:
            qd!("3.4033920413889426750011969985527947912136001826563646393895233083321"),
            alloc::vec![Quad::FRAC_PI_2, Quad::FRAC_PI_3, Quad::FRAC_PI_4]
                .into_iter()
                .sum::<Quad>();
        sum_refs_pi_234:
            qd!("3.4033920413889426750011969985527947912136001826563646393895233083321"),
            alloc::vec![Quad::FRAC_PI_2, Quad::FRAC_PI_3, Quad::FRAC_PI_4]
                .iter()
                .sum::<Quad>();
    );
    test_all_exact!(
        sum_nums_15:
            qd!(15),
            alloc::vec![qd!(1), qd!(2), qd!(3), qd!(4), qd!(5)].into_iter().sum::<Quad>();
        sum_refs_15:
            qd!(15),
            alloc::vec![qd!(1), qd!(2), qd!(3), qd!(4), qd!(5)].iter().sum::<Quad>();
        sum_empty:
            Quad::ZERO,
            alloc::vec![].iter().sum::<Quad>();
        sum_inf:
            Quad::INFINITY,
            alloc::vec![qd!(1), Quad::INFINITY, qd!(3)].iter().sum::<Quad>();
        sum_neg_inf:
            Quad::NEG_INFINITY,
            alloc::vec![qd!(1), Quad::NEG_INFINITY, qd!(3)].iter().sum::<Quad>();
        sum_both_inf:
            Quad::NAN,
            alloc::vec![Quad::INFINITY, Quad::NEG_INFINITY].iter().sum::<Quad>();
        sum_nan:
            Quad::NAN,
            alloc::vec![qd!(1), qd!(2), Quad::NAN].iter().sum::<Quad>();
    );

    // product tests
    test_all_near!(
        product_nums_pi_234:
            qd!("1.2919281950124925073115131277958914667593870235785461539226890876574"),
            alloc::vec![Quad::FRAC_PI_2, Quad::FRAC_PI_3, Quad::FRAC_PI_4]
                .into_iter()
                .product::<Quad>();
        product_refs_pi_234:
            qd!("1.2919281950124925073115131277958914667593870235785461539226890876574"),
            alloc::vec![Quad::FRAC_PI_2, Quad::FRAC_PI_3, Quad::FRAC_PI_4]
                .iter()
                .product::<Quad>();
    );
    test_all_exact!(
        product_nums_15:
            qd!(120),
            alloc::vec![qd!(1), qd!(2), qd!(3), qd!(4), qd!(5)].into_iter().product::<Quad>();
        product_refs_15:
            qd!(120),
            alloc::vec![qd!(1), qd!(2), qd!(3), qd!(4), qd!(5)].iter().product::<Quad>();
        product_empty:
            Quad::ONE,
            alloc::vec![].iter().product::<Quad>();
        product_inf:
            Quad::INFINITY,
            alloc::vec![qd!(1), Quad::INFINITY, qd!(3)].iter().product::<Quad>();
        product_neg_inf:
            Quad::NEG_INFINITY,
            alloc::vec![qd!(1), Quad::NEG_INFINITY, qd!(3)].iter().product::<Quad>();
        product_both_inf:
            Quad::NEG_INFINITY,
            alloc::vec![Quad::INFINITY, Quad::NEG_INFINITY].iter().product::<Quad>();
        product_nan:
            Quad::NAN,
            alloc::vec![qd!(1), qd!(2), Quad::NAN].iter().product::<Quad>();
    );
}
