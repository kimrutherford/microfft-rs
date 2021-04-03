#![no_std]
#![no_main]
#![allow(dead_code)]

use cortex_m::{iprint, singleton};
use cortex_m_rt::entry;
use hal::{prelude::*, time::MonoTimer};
use microfft::Complex32;
use panic_semihosting as _;

#[entry]
fn main() -> ! {
    let core = cortex_m::Peripherals::take().unwrap();
    let device = hal::stm32::Peripherals::take().unwrap();

    let mut flash = device.FLASH.constrain();
    let rcc = device.RCC.constrain();
    let clocks = rcc
        .cfgr
        .use_hse(8.mhz())
        .sysclk(72.mhz())
        .hclk(72.mhz())
        .pclk1(36.mhz())
        .pclk2(72.mhz())
        .freeze(&mut flash.acr);

    let timer = MonoTimer::new(core.DWT, clocks);
    let cycles = bench::run(timer);

    let mut itm = core.ITM;
    iprint!(&mut itm.stim[0], "{}", cycles);

    panic!("bench done");
}

#[cfg(feature = "microfft-c")]
mod bench {
    use super::{n, singleton, timeit, Complex32, MonoTimer};

    pub fn run(timer: MonoTimer) -> u32 {
        let x = singleton!(: [Complex32; n::N] = [Complex32::new(0., 0.); n::N]).unwrap();
        for (i, c) in x.iter_mut().enumerate() {
            *c = Complex32::new(i as f32, 0.);
        }

        timeit(timer, || n::CFFT(x))
    }
}

#[cfg(feature = "microfft-r")]
mod bench {
    use super::{n, singleton, timeit, MonoTimer};

    pub fn run(timer: MonoTimer) -> u32 {
        let x = singleton!(: [f32; n::N] = [0.; n::N]).unwrap();
        for (i, f) in x.iter_mut().enumerate() {
            *f = i as f32;
        }

        timeit(timer, || n::RFFT(x))
    }
}

fn timeit<F, R>(timer: MonoTimer, f: F) -> u32
where
    F: FnOnce() -> R,
{
    let instant = timer.now();
    let _ = f();
    instant.elapsed()
}

type FnCFft<const N: usize> = fn(&mut [Complex32; N]) -> &mut [Complex32; N];
type FnRFft<const N: usize, const M: usize> = fn(&mut [f32; N]) -> &mut [Complex32; M];

#[cfg(feature = "n-4")]
mod n {
    pub const N: usize = 4;
    pub const CFFT: super::FnCFft<N> = microfft::complex::cfft_4;
    pub const RFFT: super::FnRFft<N, { N / 2 }> = microfft::real::rfft_4;
}

#[cfg(feature = "n-8")]
mod n {
    pub const N: usize = 8;
    pub const CFFT: super::FnCFft<N> = microfft::complex::cfft_8;
    pub const RFFT: super::FnRFft<N, { N / 2 }> = microfft::real::rfft_8;
}

#[cfg(feature = "n-16")]
mod n {
    pub const N: usize = 16;
    pub const CFFT: super::FnCFft<N> = microfft::complex::cfft_16;
    pub const RFFT: super::FnRFft<N, { N / 2 }> = microfft::real::rfft_16;
}

#[cfg(feature = "n-32")]
mod n {
    pub const N: usize = 32;
    pub const CFFT: super::FnCFft<N> = microfft::complex::cfft_32;
    pub const RFFT: super::FnRFft<N, { N / 2 }> = microfft::real::rfft_32;
}

#[cfg(feature = "n-64")]
mod n {
    pub const N: usize = 64;
    pub const CFFT: super::FnCFft<N> = microfft::complex::cfft_64;
    pub const RFFT: super::FnRFft<N, { N / 2 }> = microfft::real::rfft_64;
}

#[cfg(feature = "n-128")]
mod n {
    pub const N: usize = 128;
    pub const CFFT: super::FnCFft<N> = microfft::complex::cfft_128;
    pub const RFFT: super::FnRFft<N, { N / 2 }> = microfft::real::rfft_128;
}

#[cfg(feature = "n-256")]
mod n {
    pub const N: usize = 256;
    pub const CFFT: super::FnCFft<N> = microfft::complex::cfft_256;
    pub const RFFT: super::FnRFft<N, { N / 2 }> = microfft::real::rfft_256;
}

#[cfg(feature = "n-512")]
mod n {
    pub const N: usize = 512;
    pub const CFFT: super::FnCFft<N> = microfft::complex::cfft_512;
    pub const RFFT: super::FnRFft<N, { N / 2 }> = microfft::real::rfft_512;
}

#[cfg(feature = "n-1024")]
mod n {
    pub const N: usize = 1024;
    pub const CFFT: super::FnCFft<N> = microfft::complex::cfft_1024;
    pub const RFFT: super::FnRFft<N, { N / 2 }> = microfft::real::rfft_1024;
}

#[cfg(feature = "n-2048")]
mod n {
    pub const N: usize = 2048;
    pub const CFFT: super::FnCFft<N> = microfft::complex::cfft_2048;
    pub const RFFT: super::FnRFft<N, { N / 2 }> = microfft::real::rfft_2048;
}

#[cfg(feature = "n-4096")]
mod n {
    pub const N: usize = 4096;
    pub const CFFT: super::FnCFft<N> = microfft::complex::cfft_4096;
    pub const RFFT: super::FnRFft<N, { N / 2 }> = microfft::real::rfft_4096;
}
