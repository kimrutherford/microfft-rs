# Benchmarks

This code is used to run benchmarks on an embedded ARM Cortex-M4 system,
specifically the [STM32F3DISCOVERY][1] board.

It measures the number of CPU cycles required to compute both complex and real
FFTs of all sizes supported by microfft.

As a point of comparison, the same benchmarks were originally also performed
against the [Fourier crate][2] which, at that time, was the only other Rust FFT
library with `no_std` support. Unfortunately, the current version of Fourier
refuses to compile without the `std` feature, so it had to be removed from the
benchmark code. The old benchmark results are still included in the table
below.

## Running

To run the benchmarks, make sure the `thumbv7em-none-eabihf` rustc target
and OpenOCD are installed and the board is connected. Then just execute
the `run.py` script.

`run.py` starts OpenOCD in the background. It then builds for every FFT-size
combination a benchmark binary, flashes it onto the board, and runs it.
The results are printed to stdout.

## Results

The following table lists the `microfft` benchmark results from 2021-04-03
together with the `fourier` benchmark results from 2020-03-08.

Measurements are in CPU cycles, so lower is better.

| FFT size | microfft (CFFT) | microfft (RFFT) | Fourier (CFFT) |
| -------: | --------------: | --------------: | -------------: |
|    **4** |              69 |              12 |            564 |
|    **8** |             199 |             131 |          1,462 |
|   **16** |             718 |             369 |          2,202 |
|   **32** |           2,320 |           1,200 |          4,173 |
|   **64** |           6,249 |           3,340 |         10,943 |
|  **128** |          15,734 |           8,588 |         20,904 |
|  **256** |          37,775 |          20,234 |         42,724 |
|  **512** |          89,758 |          47,708 |         97,380 |
| **1024** |         207,317 |         108,527 |          s/o\* |
| **2048** |         467,198 |         242,356 |          s/o\* |
| **4096** |       1,028,211 |         530,090 |          s/o\* |

\* FFT cannot be computed due to stack overflow.

[1]: https://www.st.com/en/evaluation-tools/stm32f3discovery.html
[2]: https://crates.io/crates/fourier
