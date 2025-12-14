#![feature(portable_simd)]
#![feature(slice_ptr_get)]
#![feature(array_ptr_get)]

use std::{
    mem::MaybeUninit,
    simd::{prelude::*, *},
    u64,
};

pub fn run(input: &str) -> impl std::fmt::Display {
    part1(input)
}

pub fn part1(input: &str) -> i64 {
    unsafe { part1_inner(input) }
}

unsafe fn part1_inner(input: &str) -> i64 {
    let mut ptr = input.as_ptr().add(96);

    let mut count = 0;

    for _ in 0..1000 {
        let w = 10 * (*ptr.add(0) as u32 - b'0' as u32) + (*ptr.add(1) as u32 - b'0' as u32);
        let h = 10 * (*ptr.add(3) as u32 - b'0' as u32) + (*ptr.add(4) as u32 - b'0' as u32);
        let a = w * h;

        let block = ptr.add(7).cast::<u8x16>().read_unaligned() - u8x16::splat(b'0');
        const COEFFS: u8x16 =
            u8x16::from_array([90, 9, 0, 90, 9, 0, 90, 9, 0, 90, 9, 0, 90, 9, 0, 90]);

        let sum;
        #[cfg(target_arch = "x86_64")]
        {
            sum = u16x8::from(core::arch::x86_64::_mm_maddubs_epi16(
                block.into(),
                COEFFS.into(),
            ))
            .reduce_sum() as u32;
        }

        #[cfg(not(target_arch = "x86_64"))]
        {
            sum = (block.cast::<u16>() * COEFFS.cast::<u16>()).reduce_sum() as u32;
        }

        count += (sum <= a) as i64;

        ptr = ptr.add(25);
    }

    count
}
