#![cfg_attr(not(test), no_std)]
#![forbid(unsafe_code)]
#![deny(missing_docs)]
//! Approximate Shannon Entropy
//!
//! Based on [shannon-entropy](https://github.com/insanitybit/shannon-entropy)
//!
//! Usable on no_std due to use of approximate natural log from [micromath](https://github.com/tarcieri/micromath)

use micromath::F32Ext;

/// Supported word size
type Word = u8;
/// Supported rational/floating point type
type Rational = f32;

/// Shannon entropy of a slice
///
/// # Arguments
/// * `word_slice` - Slice of bytes
///
/// # Result
/// * Returns - Rational shannon entropy fraction, inclusive range from 0 to bits-per-word
///
/// ```
/// # use crate::approx_shannon_entropy::*;
/// let zeros = [0u8; 35usize];
/// let found = shannon_entropy(&zeros);
/// # let expect = 0f32;
/// # assert_eq!(found, expect);
/// ```
#[allow(clippy::let_and_return)]
pub fn shannon_entropy(word_slice: &[Word]) -> Rational {
    // Create buckets for a histogram, u8 is 256 slots
    // Anything other than `Word = 8 bits` would require a different approach
    debug_assert!((1usize) + (Word::max_value() as usize) <= usize::max_value());
    debug_assert_eq!(Word::min_value(), 0);
    let mut word_map = [0usize; (1usize) + (Word::max_value() as usize)];
    for word in word_slice {
        word_map[(*word) as usize] += 1;
    }

    // Attempting to only use natural log inside loop and scale to log2 later for speed.
    // Not verified if changes timing on any arch
    let rat_len: Rational = word_slice.len() as Rational;
    let log_div: Rational = F32Ext::ln(2.0 as Rational);
    let en_sum = word_map
        .iter()
        .fold(0.0 as Rational, |acc, &freq| match freq {
            0 => acc,
            freq => {
                let rat_freq: Rational = freq as Rational;
                acc + (rat_freq * F32Ext::ln(rat_freq / rat_len))
            }
        });
    // Value `en_sum` is always zero or negative and needs to be inverted
    // Assume abs() of Rational is a bitwise operation on some arch, so may be faster
    let entropy = en_sum.abs() / (rat_len * log_div);
    entropy
}

/// Shannon metric entropy of a slice
///
/// Shannon metric entropy is the Shannon entropy value divided by the input slice length
///
/// # Arguments
/// * `word_slice` - Slice of bytes
///
/// # Result
/// * Returns - Shannon metric entropy fraction (normalized by length), inclusive range from 0 to 1
///
/// ```
/// # use crate::approx_shannon_entropy::*;
/// let zeros = [1u8; 35usize];
/// let found = shannon_entropy_metric(&zeros);
/// # let expect = 0f32;
/// # assert_eq!(found, expect);
/// ```
pub fn shannon_entropy_metric(word_slice: &[Word]) -> Rational {
    shannon_entropy(word_slice) / (word_slice.len() as Rational)
}

#[cfg(test)]
mod tests {
    use super::*;

    // Arbitrary test error constants that floating point and micromath should be within
    /// Absolute difference max allowed
    const SIGMA: Rational = 10e-1;
    /// Fractional error margin allowed (n*100 == %)
    const FRACT: Rational = 0.14;

    #[test]
    fn shannon_expected_value() {
        // Allowed error bound due to floating point arithmetic and micromath
        let test_vectors = [
            [0, 0, 0, 0, 0, 0, 0, 0],
            [1, 1, 1, 1, 1, 1, 1, 1],
            [0, 0, 1, 1, 0, 1, 0, 1],
            [0, 0, 1, 1, 2, 2, 3, 3],
            [0, 0, 0, 1, 1, 2, 3, 4],
            [1, 2, 3, 4, 5, 6, 7, 8],
        ];
        // Helpful to calculate the expected entropy value https://planetcalc.com/2476/
        let test_expect = [0.0, 0.0, 1.0, 2.0, 2.15563906, 3.0];
        for i in 0..test_vectors.len() {
            let e = test_expect[i];
            let l = e - SIGMA;
            let r = e + SIGMA;
            let s_en = shannon_entropy(&test_vectors[i]);
            let e_wid = f32::max(e, s_en) * FRACT;

            dbg!(i, l, r, e_wid, e, s_en);
            assert!(F32Ext::abs(f32::max(e, s_en) - f32::min(e, s_en)) <= e_wid);
            assert!(l <= s_en);
            assert!(r >= s_en);
            assert!(s_en >= 0.0);
            assert!(s_en <= 8.0);
        }
    }

    #[test]
    fn shannon_all_unique_bytes() {
        let pattern = {
            let mut bytes = [0u8; 256usize];
            for i in 0..bytes.len() {
                bytes[i] = i as u8;
            }
            bytes
        };
        let e = 8.0;
        let l = e - SIGMA;
        let r = e + SIGMA;
        let s_en = shannon_entropy(&pattern);
        let e_wid = f32::max(e, s_en) * FRACT;

        dbg!(l, r, e_wid, e, s_en);
        assert!(F32Ext::abs(f32::max(e, s_en) - f32::min(e, s_en)) <= e_wid);
        assert!(l <= s_en);
        assert!(r >= s_en);
        assert!(s_en >= 0.0);
        assert!(s_en <= 8.0);
    }

    #[test]
    fn shannon_odd_slice_length() {
        // Same test pattern as https://github.com/ambron60/shannon-entropy-calculator/blob/master/README.md
        let pattern = [0, 0, 0, 0, 1, 1, 2, 2, 3, 4, 5];
        let e = 2.04037339;
        let l = e - SIGMA;
        let r = e + SIGMA;
        let s_en = shannon_entropy(&pattern);
        let e_wid = f32::max(e, s_en) * FRACT;

        dbg!(l, r, e_wid, e, s_en);
        assert!(F32Ext::abs(f32::max(e, s_en) - f32::min(e, s_en)) <= e_wid);
        assert!(l <= s_en);
        assert!(r >= s_en);
        assert!(s_en >= 0.0);
        assert!(s_en <= 8.0);
    }

    #[test]
    fn shannon_metric_odd_slice_length() {
        let pattern = [0, 0, 0, 0, 1, 1, 2, 2, 3, 4, 5];
        let e = 0.18548849;
        let l = e - (SIGMA / pattern.len() as Rational);
        let r = e + (SIGMA / pattern.len() as Rational);
        let m_en = shannon_entropy_metric(&pattern);
        let e_wid = f32::max(e, m_en) * FRACT;

        dbg!(l, r, e_wid, e, m_en);
        assert!(F32Ext::abs(f32::max(e, m_en) - f32::min(e, m_en)) <= e_wid);
        assert!(l <= m_en);
        assert!(r >= m_en);
        assert!(m_en >= 0.0);
        assert!(m_en <= 1.0);
    }
}
