# Changelog

All notable changes to this project will be documented in this file.

## 0.5.0 (unreleased)

- **Breaking:** Introduced a `std` feature that is enabled by default. It
  transitively enables the `std` feature on the `num-complex` dependency,
  enabling more methods on the `Complex32` values returned by the FFT
  functions. `no_std` users will need to disable this feature.


## 0.4.0 (2021-04-03)

### Changed

- **Breaking:** The MSRV has been increased to 1.51.0.
- **Breaking:** All API functions for computing FFTs now take references to
  arrays instead of references to slices. This has the benefit of moving the
  length checking to compile time, which makes those functions panic-proof.
- **Breaking:** When computing RFFTs, the real-valued coefficient at the
  Nyquist frequency is now packed into the imaginary part of the DC bin
  (bin 0). Previously this value was simply dropped from the RFFT output.
- Added `#[must_use]` annotations to all FFT API functions.

### Fixed

- Thanks to Cargo's new resolver it is now possible to build microfft as a
  stand-alone library.


## 0.3.1 (2020-11-09)

### Fixed

- Fixed a bug during the RFFT recombination calculation that caused a wrong
  output value in bin `N / 4`.


## 0.3.0 (2020-03-08)

### Changed

- Store only the largest sine table, instead of one for each FFT size. By
  default this is the table for the 4096-point FFT (the largest supported one),
  but smaller ones can be selected via the new `maxn-*` crate features.


## 0.2.0 (2020-03-08)

### Changed

- Bitrev tables are not used anymore by default, instead the bit-reversed
  indices are computed directly at runtime. This significantly reduces the
  memory usage of microfft. On architectures that provide a dedicated
  bit-reversal instruction (like `RBIT` on ARMv7), speed is also increased.
  The `bitrev-tables` feature can be enabled to still opt into using bitrev
  tables.


## 0.1.2 (2020-03-07)

### Changed

- Store pre-computed sine values instead of full twiddles, reducing the size
  of the twiddle tables to one fourth the prior size.

## 0.1.1 (2020-03-05)

### Added

- Support for FFT sizes 2048 and 4096.


## 0.1.0 (2020-03-04)

### Added

- Support for complex and real FFTs up to size 1024.
