//! FFT on real inputs (RFFT)
//!
//! This implementation takes advantage of the symmetry properties of
//! the FFT to compute an `N`-point RFFT by internally invoking an
//! `N/2`-point CFFT, roughly doubling the computation speed compared
//! to used a full `N`-point CFFT.
//!
//! The produced output is the first half out the output returned by
//! the corresponding `N`-point CFFT, i.e. the real DC value and
//! `N/2 - 1` positive-frequency terms. The negative-frequency terms
//! are not computed, since they can be calculated from the
//! positive-frequency terms and are therefore redundant.

use core::convert::TryInto;

use crate::rfft::*;
use num_complex::Complex32;

/// Perform an in-place 2-point RFFT.
///
/// # Example
///
/// ```
/// use microfft::real::rfft_2;
///
/// let mut input = [0.; 2];
/// let result = rfft_2(&mut input);
/// ```
#[inline]
pub fn rfft_2(input: &mut [f32; 2]) -> &mut [Complex32; 1] {
    RFftN2::transform(input).try_into().unwrap()
}

/// Perform an in-place 4-point RFFT.
///
/// # Example
///
/// ```
/// use microfft::real::rfft_4;
///
/// let mut input = [0.; 4];
/// let result = rfft_4(&mut input);
/// ```
#[inline]
pub fn rfft_4(input: &mut [f32; 4]) -> &mut [Complex32; 2] {
    RFftN4::transform(input).try_into().unwrap()
}

/// Perform an in-place 8-point RFFT.
///
/// # Example
///
/// ```
/// use microfft::real::rfft_8;
///
/// let mut input = [0.; 8];
/// let result = rfft_8(&mut input);
/// ```
#[cfg(any(
    feature = "maxn-8",
    feature = "maxn-16",
    feature = "maxn-32",
    feature = "maxn-64",
    feature = "maxn-128",
    feature = "maxn-256",
    feature = "maxn-512",
    feature = "maxn-1024",
    feature = "maxn-2048",
    feature = "maxn-4096",
))]
#[inline]
pub fn rfft_8(input: &mut [f32; 8]) -> &mut [Complex32; 4] {
    RFftN8::transform(input).try_into().unwrap()
}

/// Perform an in-place 16-point RFFT.
///
/// # Example
///
/// ```
/// use microfft::real::rfft_16;
///
/// let mut input = [0.; 16];
/// let result = rfft_16(&mut input);
/// ```
#[cfg(any(
    feature = "maxn-16",
    feature = "maxn-32",
    feature = "maxn-64",
    feature = "maxn-128",
    feature = "maxn-256",
    feature = "maxn-512",
    feature = "maxn-1024",
    feature = "maxn-2048",
    feature = "maxn-4096",
))]
#[inline]
pub fn rfft_16(input: &mut [f32; 16]) -> &mut [Complex32; 8] {
    RFftN16::transform(input).try_into().unwrap()
}

/// Perform an in-place 32-point RFFT.
///
/// # Example
///
/// ```
/// use microfft::real::rfft_32;
///
/// let mut input = [0.; 32];
/// let result = rfft_32(&mut input);
/// ```
#[cfg(any(
    feature = "maxn-32",
    feature = "maxn-64",
    feature = "maxn-128",
    feature = "maxn-256",
    feature = "maxn-512",
    feature = "maxn-1024",
    feature = "maxn-2048",
    feature = "maxn-4096",
))]
#[inline]
pub fn rfft_32(input: &mut [f32; 32]) -> &mut [Complex32; 16] {
    RFftN32::transform(input).try_into().unwrap()
}

/// Perform an in-place 64-point RFFT.
///
/// # Example
///
/// ```
/// use microfft::real::rfft_64;
///
/// let mut input = [0.; 64];
/// let result = rfft_64(&mut input);
/// ```
#[cfg(any(
    feature = "maxn-64",
    feature = "maxn-128",
    feature = "maxn-256",
    feature = "maxn-512",
    feature = "maxn-1024",
    feature = "maxn-2048",
    feature = "maxn-4096",
))]
#[inline]
pub fn rfft_64(input: &mut [f32; 64]) -> &mut [Complex32; 32] {
    RFftN64::transform(input).try_into().unwrap()
}

/// Perform an in-place 128-point RFFT.
///
/// # Example
///
/// ```
/// use microfft::real::rfft_128;
///
/// let mut input = [0.; 128];
/// let result = rfft_128(&mut input);
/// ```
#[cfg(any(
    feature = "maxn-128",
    feature = "maxn-256",
    feature = "maxn-512",
    feature = "maxn-1024",
    feature = "maxn-2048",
    feature = "maxn-4096",
))]
#[inline]
pub fn rfft_128(input: &mut [f32; 128]) -> &mut [Complex32; 64] {
    RFftN128::transform(input).try_into().unwrap()
}

/// Perform an in-place 256-point RFFT.
///
/// # Example
///
/// ```
/// use microfft::real::rfft_256;
///
/// let mut input = [0.; 256];
/// let result = rfft_256(&mut input);
/// ```
#[cfg(any(
    feature = "maxn-256",
    feature = "maxn-512",
    feature = "maxn-1024",
    feature = "maxn-2048",
    feature = "maxn-4096",
))]
#[inline]
pub fn rfft_256(input: &mut [f32; 256]) -> &mut [Complex32; 128] {
    RFftN256::transform(input).try_into().unwrap()
}

/// Perform an in-place 512-point RFFT.
///
/// # Example
///
/// ```
/// use microfft::real::rfft_512;
///
/// let mut input = [0.; 512];
/// let result = rfft_512(&mut input);
/// ```
#[cfg(any(
    feature = "maxn-512",
    feature = "maxn-1024",
    feature = "maxn-2048",
    feature = "maxn-4096",
))]
#[inline]
pub fn rfft_512(input: &mut [f32; 512]) -> &mut [Complex32; 256] {
    RFftN512::transform(input).try_into().unwrap()
}

/// Perform an in-place 1024-point RFFT.
///
/// # Example
///
/// ```
/// use microfft::real::rfft_1024;
///
/// let mut input = [0.; 1024];
/// let result = rfft_1024(&mut input);
/// ```
#[cfg(any(feature = "maxn-1024", feature = "maxn-2048", feature = "maxn-4096"))]
#[inline]
pub fn rfft_1024(input: &mut [f32; 1024]) -> &mut [Complex32; 512] {
    RFftN1024::transform(input).try_into().unwrap()
}

/// Perform an in-place 2048-point RFFT.
///
/// # Example
///
/// ```
/// use microfft::real::rfft_2048;
///
/// let mut input = [0.; 2048];
/// let result = rfft_2048(&mut input);
/// ```
#[cfg(any(feature = "maxn-2048", feature = "maxn-4096"))]
#[inline]
pub fn rfft_2048(input: &mut [f32; 2048]) -> &mut [Complex32; 1024] {
    RFftN2048::transform(input).try_into().unwrap()
}

/// Perform an in-place 4096-point RFFT.
///
/// # Example
///
/// ```
/// use microfft::real::rfft_4096;
///
/// let mut input = [0.; 4096];
/// let result = rfft_4096(&mut input);
/// ```
#[cfg(any(feature = "maxn-4096"))]
#[inline]
pub fn rfft_4096(input: &mut [f32; 4096]) -> &mut [Complex32; 2048] {
    RFftN4096::transform(input).try_into().unwrap()
}
