//! FFT on complex inputs (CFFT)

use crate::{cfft::*, Complex32};

/// Perform an in-place 2-point CFFT.
///
/// # Example
///
/// ```
/// use microfft::{Complex32, complex::cfft_2};
///
/// let mut input = [Complex32::default(); 2];
/// let result = cfft_2(&mut input);
/// ```
#[inline]
#[must_use]
pub fn cfft_2(input: &mut [Complex32; 2]) -> &mut [Complex32; 2] {
    CFftN2::transform(input);
    input
}

/// Perform an in-place 4-point CFFT.
///
/// # Example
///
/// ```
/// use microfft::{Complex32, complex::cfft_4};
///
/// let mut input = [Complex32::default(); 4];
/// let result = cfft_4(&mut input);
/// ```
#[cfg(feature = "size-4")]
#[inline]
#[must_use]
pub fn cfft_4(input: &mut [Complex32; 4]) -> &mut [Complex32; 4] {
    CFftN4::transform(input);
    input
}

/// Perform an in-place 8-point CFFT.
///
/// # Example
///
/// ```
/// use microfft::{Complex32, complex::cfft_8};
///
/// let mut input = [Complex32::default(); 8];
/// let result = cfft_8(&mut input);
/// ```
#[cfg(feature = "size-8")]
#[inline]
#[must_use]
pub fn cfft_8(input: &mut [Complex32; 8]) -> &mut [Complex32; 8] {
    CFftN8::transform(input);
    input
}

/// Perform an in-place 16-point CFFT.
///
/// # Example
///
/// ```
/// use microfft::{Complex32, complex::cfft_16};
///
/// let mut input = [Complex32::default(); 16];
/// let result = cfft_16(&mut input);
/// ```
#[cfg(feature = "size-16")]
#[inline]
#[must_use]
pub fn cfft_16(input: &mut [Complex32; 16]) -> &mut [Complex32; 16] {
    CFftN16::transform(input);
    input
}

/// Perform an in-place 32-point CFFT.
///
/// # Example
///
/// ```
/// use microfft::{Complex32, complex::cfft_32};
///
/// let mut input = [Complex32::default(); 32];
/// let result = cfft_32(&mut input);
/// ```
#[cfg(feature = "size-32")]
#[inline]
#[must_use]
pub fn cfft_32(input: &mut [Complex32; 32]) -> &mut [Complex32; 32] {
    CFftN32::transform(input);
    input
}

/// Perform an in-place 64-point CFFT.
///
/// # Example
///
/// ```
/// use microfft::{Complex32, complex::cfft_64};
///
/// let mut input = [Complex32::default(); 64];
/// let result = cfft_64(&mut input);
/// ```
#[cfg(feature = "size-64")]
#[inline]
#[must_use]
pub fn cfft_64(input: &mut [Complex32; 64]) -> &mut [Complex32; 64] {
    CFftN64::transform(input);
    input
}

/// Perform an in-place 128-point CFFT.
///
/// # Example
///
/// ```
/// use microfft::{Complex32, complex::cfft_128};
///
/// let mut input = [Complex32::default(); 128];
/// let result = cfft_128(&mut input);
/// ```
#[cfg(feature = "size-128")]
#[inline]
#[must_use]
pub fn cfft_128(input: &mut [Complex32; 128]) -> &mut [Complex32; 128] {
    CFftN128::transform(input);
    input
}

/// Perform an in-place 256-point CFFT.
///
/// # Example
///
/// ```
/// use microfft::{Complex32, complex::cfft_256};
///
/// let mut input = [Complex32::default(); 256];
/// let result = cfft_256(&mut input);
/// ```
#[cfg(feature = "size-256")]
#[inline]
#[must_use]
pub fn cfft_256(input: &mut [Complex32; 256]) -> &mut [Complex32; 256] {
    CFftN256::transform(input);
    input
}

/// Perform an in-place 512-point CFFT.
///
/// # Example
///
/// ```
/// use microfft::{Complex32, complex::cfft_512};
///
/// let mut input = [Complex32::default(); 512];
/// let result = cfft_512(&mut input);
/// ```
#[cfg(feature = "size-512")]
#[inline]
#[must_use]
pub fn cfft_512(input: &mut [Complex32; 512]) -> &mut [Complex32; 512] {
    CFftN512::transform(input);
    input
}

/// Perform an in-place 1024-point CFFT.
///
/// # Example
///
/// ```
/// use microfft::{Complex32, complex::cfft_1024};
///
/// let mut input = [Complex32::default(); 1024];
/// let result = cfft_1024(&mut input);
/// ```
#[cfg(feature = "size-1024")]
#[inline]
#[must_use]
pub fn cfft_1024(input: &mut [Complex32; 1024]) -> &mut [Complex32; 1024] {
    CFftN1024::transform(input);
    input
}

/// Perform an in-place 2048-point CFFT.
///
/// # Example
///
/// ```
/// use microfft::{Complex32, complex::cfft_2048};
///
/// let mut input = [Complex32::default(); 2048];
/// let result = cfft_2048(&mut input);
/// ```
#[cfg(feature = "size-2048")]
#[inline]
#[must_use]
pub fn cfft_2048(input: &mut [Complex32; 2048]) -> &mut [Complex32; 2048] {
    CFftN2048::transform(input);
    input
}

/// Perform an in-place 4096-point CFFT.
///
/// # Example
///
/// ```
/// use microfft::{Complex32, complex::cfft_4096};
///
/// let mut input = [Complex32::default(); 4096];
/// let result = cfft_4096(&mut input);
/// ```
#[cfg(feature = "size-4096")]
#[inline]
#[must_use]
pub fn cfft_4096(input: &mut [Complex32; 4096]) -> &mut [Complex32; 4096] {
    CFftN4096::transform(input);
    input
}

/// Perform an in-place 8192-point CFFT.
///
/// # Example
///
/// ```
/// use microfft::{Complex32, complex::cfft_8192};
///
/// let mut input = [Complex32::default(); 8192];
/// let result = cfft_8192(&mut input);
/// ```
#[cfg(feature = "size-8192")]
#[inline]
#[must_use]
pub fn cfft_8192(input: &mut [Complex32; 8192]) -> &mut [Complex32; 8192] {
    CFftN8192::transform(input);
    input
}

/// Perform an in-place 16384-point CFFT.
///
/// # Example
///
/// ```
/// use microfft::{Complex32, complex::cfft_16384};
///
/// let mut input = [Complex32::default(); 16384];
/// let result = cfft_16384(&mut input);
/// ```
#[cfg(feature = "size-16384")]
#[inline]
#[must_use]
pub fn cfft_16384(input: &mut [Complex32; 16384]) -> &mut [Complex32; 16384] {
    CFftN16384::transform(input);
    input
}
