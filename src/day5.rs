#![feature(portable_simd)]
#![feature(slice_ptr_get)]
#![feature(array_ptr_get)]

use std::{
    mem::MaybeUninit,
    simd::{prelude::*, *},
};

pub fn run(input: &str) -> impl std::fmt::Display {
    // part1(input)
    part2(input)
}

pub fn part1(input: &str) -> i64 {
    unsafe { part1_inner(input) }
}

pub fn part2(input: &str) -> i64 {
    unsafe { part2_inner(input) }
}

macro_rules! parse {
    ($ptr:expr, $len:expr) => {{
        let ptr = $ptr;
        let len = $len;

        let mask = mask8x16::from_bitmask(!0 << (16 - len));
        let zero = u8x16::splat(b'0');
        let lane = u8x16::load_select_ptr(ptr.sub(16 - len), mask, zero);

        let lane = lane - u8x16::splat(b'0');

        #[cfg(target_arch = "x86_64")]
        {
            let mul10 = u8x16::from_array([10, 1, 10, 1, 10, 1, 10, 1, 10, 1, 10, 1, 10, 1, 10, 1]);
            let mul100 = u8x16::from_array([
                100, 1, 100, 1, 100, 1, 100, 1, 100, 1, 100, 1, 100, 1, 100, 1,
            ]);
            let mul10000 = u16x8::from_array([10000, 1, 10000, 1, 10000, 1, 10000, 1]);

            use core::arch::x86_64::{_mm_madd_epi16, _mm_maddubs_epi16};
            let lane = u16x8::from(_mm_maddubs_epi16(lane.into(), mul10.into()));
            let lane = lane.cast::<u8>().resize::<16>(0);
            let lane = u16x8::from(_mm_maddubs_epi16(lane.into(), mul100.into()));
            let lane = u32x4::from(_mm_madd_epi16(lane.into(), mul10000.into()));
            100000000 * lane[0] as u64 + lane[1] as u64
        }
        #[cfg(not(target_arch = "x86_64"))]
        {
            let (hi, lo) = lane.deinterleave(lane);
            let hi = hi.extract::<0, 8>();
            let lo = lo.extract::<0, 8>();

            let lane = u8x8::splat(10) * hi + lo;

            let (hi, lo) = lane.deinterleave(lane);
            let hi = hi.extract::<0, 4>();
            let lo = lo.extract::<0, 4>();

            let lane = u16x4::splat(100) * hi.cast::<u16>() + lo.cast::<u16>();

            let (hi, lo) = lane.deinterleave(lane);
            let hi = hi.extract::<0, 2>();
            let lo = lo.extract::<0, 2>();

            let lane = u32x2::splat(10000) * hi.cast::<u32>() + lo.cast::<u32>();

            100000000 * lane[0] as u64 + lane[1] as u64
        }
    }};
}

unsafe fn part1_inner(input: &str) -> i64 {
    let bytes = input.as_bytes();
    let mut ptr = bytes.as_ptr();

    const LEN: usize = 49;
    const BUCKET_BITS: usize = 9;
    const LOW_BITS: usize = LEN - BUCKET_BITS;
    const LOW_MASK: u64 = (1 << LOW_BITS) - 1;
    const RANGE_MAX: u64 = 1 << LOW_BITS;

    let mut buckets_len = [0u32; 1 << BUCKET_BITS];
    let mut buckets = MaybeUninit::<[[[u64; 8]; 2]; 1 << BUCKET_BITS]>::uninit();

    while *ptr != b'\n' {
        let lane = ptr.cast::<u8x16>().read_unaligned();
        let len = lane
            .simd_eq(u8x16::splat(b'-'))
            .to_bitmask()
            .trailing_zeros() as usize;
        let s = parse!(ptr, len);
        ptr = ptr.add(len + 1);

        let lane = ptr.cast::<u8x16>().read_unaligned();
        let len = lane
            .simd_eq(u8x16::splat(b'\n'))
            .to_bitmask()
            .trailing_zeros() as usize;
        let e = parse!(ptr, len);
        ptr = ptr.add(len + 1);

        let mut stop = s >> LOW_BITS;
        let mut srem = s & LOW_MASK;
        let etop = e >> LOW_BITS;
        let erem = e & LOW_MASK;

        macro_rules! handle_range {
            ($top:expr, $srem:expr, $erem:expr) => {{
                let top = $top as usize;
                let bucket_len = &mut *buckets_len.get_unchecked_mut(top);
                let bucket = buckets.as_mut_ptr().as_mut_ptr().add(top);

                bucket
                    .as_mut_ptr()
                    .as_mut_ptr()
                    .add(*bucket_len as usize)
                    .write($srem);
                bucket
                    .as_mut_ptr()
                    .add(1)
                    .as_mut_ptr()
                    .add(*bucket_len as usize)
                    .write($erem);
                *bucket_len += 1;
            }};
        }

        while stop != etop {
            handle_range!(stop, srem, RANGE_MAX);

            stop += 1;
            srem = 0;
        }

        handle_range!(stop, srem, erem + 1);
    }

    let mut tot = 0;

    let end_ptr = ptr;
    let mut ptr = bytes.as_ptr().add(bytes.len()).sub(1);
    while ptr != end_ptr {
        let lane = ptr.sub(16).cast::<u8x16>().read_unaligned();
        let len = lane
            .simd_eq(u8x16::splat(b'\n'))
            .to_bitmask()
            .leading_zeros() as usize
            - (64 - 16);

        ptr = ptr.sub(len);
        let n = parse!(ptr, len);
        ptr = ptr.sub(1);

        let ntop = n >> LOW_BITS;
        let nrem = n & LOW_MASK;

        let bucket_len = &*buckets_len.get_unchecked(ntop as usize);
        let bucket = buckets.as_ptr().as_ptr().add(ntop as usize);

        for i in 0..8 {
            if i < *bucket_len as usize {
                let s = *bucket.as_ptr().cast::<u64>().add(i);
                let e = *bucket.as_ptr().add(1).cast::<u64>().add(i);
                if s <= nrem && nrem < e {
                    tot += 1;
                    break;
                }
            }
        }
    }

    tot
}

unsafe fn part2_inner(input: &str) -> i64 {
    let bytes = input.as_bytes();
    let mut ptr = bytes.as_ptr();

    const LEN: usize = 49;
    const BUCKET_BITS: usize = 9;
    const LOW_BITS: usize = LEN - BUCKET_BITS;
    const LOW_MASK: u64 = (1 << LOW_BITS) - 1;
    const RANGE_MAX: u64 = 1 << LOW_BITS;

    let mut buckets_len = [0u32; 1 << BUCKET_BITS];
    let mut buckets = MaybeUninit::<[[[u64; 8]; 2]; 1 << BUCKET_BITS]>::uninit();

    let mut tot = 0;

    while *ptr != b'\n' {
        let lane = ptr.cast::<u8x16>().read_unaligned();
        let len = lane
            .simd_eq(u8x16::splat(b'-'))
            .to_bitmask()
            .trailing_zeros() as usize;
        let s = parse!(ptr, len);
        ptr = ptr.add(len + 1);

        let lane = ptr.cast::<u8x16>().read_unaligned();
        let len = lane
            .simd_eq(u8x16::splat(b'\n'))
            .to_bitmask()
            .trailing_zeros() as usize;
        let e = parse!(ptr, len);
        ptr = ptr.add(len + 1);

        let mut stop = s >> LOW_BITS;
        let mut srem = s & LOW_MASK;
        let etop = e >> LOW_BITS;
        let erem = e & LOW_MASK;

        macro_rules! handle_range {
            ($top:expr, $srem:expr, $erem:expr) => {{
                let top = $top as usize;
                let mut srem = $srem;
                let mut erem = $erem;

                let bucket = buckets.as_mut_ptr().as_mut_ptr().add(top).as_mut_ptr();
                let bucket_len = &mut *buckets_len.get_unchecked_mut(top);
                let len = *bucket_len as usize;

                for i in 0..8 {
                    if i < len {
                        let s = bucket.cast::<u64>().add(i);
                        let e = bucket.add(1).cast::<u64>().add(i);

                        if srem <= *s {
                            if *e <= erem {
                                tot -= *e - *s;
                                *s = *e;
                            } else {
                                erem = erem.min(*s);
                            }
                        } else if *e <= erem {
                            srem = srem.max(*e);
                        } else {
                            srem = 0;
                            erem = 0;
                        }
                    }
                }

                let srem = srem.min(erem);
                tot += erem - srem;

                bucket.as_mut_ptr().add(len).write(srem);
                bucket.add(1).as_mut_ptr().add(len).write(erem);
                *bucket_len = (len + 1) as u32;
            }};
        }

        while stop != etop {
            handle_range!(stop, srem, RANGE_MAX);

            stop += 1;
            srem = 0;
        }

        handle_range!(stop, srem, erem + 1);
    }

    tot as i64
}
