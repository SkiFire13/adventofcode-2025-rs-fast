#![feature(portable_simd)]

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

unsafe fn part1_inner(input: &str) -> i64 {
    let bytes = input.as_bytes();
    let mut ptr = bytes.as_ptr();
    let end = ptr.add(bytes.len());

    let mut tot = 0;

    let len = 128
        + u8x32::from_array(ptr.add(128).cast::<[u8; 32]>().read())
            .simd_eq(Simd::splat(b'\n'))
            .to_bitmask()
            .trailing_zeros() as usize;
    let mask1 = (u64::MAX << 2) >> 1;
    let mask2 = mask1;
    let mask3: u64 = u64::MAX << (64 - (len - 124 + 1));

    // Yeah this doesn't totally make sense because I'm also computing stuff for the first out-of-bound position but whatever.
    // lane1 =  -1 ..=  62.  (   0 ..= 61 )
    // lane2 =  61 ..= 124   (  62 ..= 123 )
    // lane3 = 123 ..= len+1 ( 124 ..= len with masking! )

    let bit = 0b10;
    let mask_bit = u8x64::splat(bit);
    let outside_lane = mask_bit * u8x64::splat(3);
    let zero = u8x64::splat(0);
    let threshold = u8x64::splat(5 * bit);

    let mut lane11 = outside_lane;
    let mut lane12 = outside_lane;
    let mut lane13 = outside_lane;

    let mut lane21c =
        (ptr.cast::<u8x64>().read_unaligned() & mask_bit).shift_elements_right::<1>(bit);
    let lane21r = (ptr.cast::<u8x64>().read_unaligned() & mask_bit).shift_elements_right::<2>(bit);
    let lane21l = ptr.cast::<u8x64>().read_unaligned() & mask_bit;
    let mut lane21 = lane21c + lane21r + lane21l;

    let mut lane22c = ptr.add(61).cast::<u8x64>().read_unaligned() & mask_bit;
    let lane22r = ptr.add(61 - 1).cast::<u8x64>().read_unaligned() & mask_bit;
    let lane22l = ptr.add(61 + 1).cast::<u8x64>().read_unaligned() & mask_bit;
    let mut lane22 = lane22c + lane22r + lane22l;

    let mut lane23c = ptr.add(len + 1 - 64).cast::<u8x64>().read_unaligned() & mask_bit;
    let lane23r = ptr.add(len + 1 - 64 - 1).cast::<u8x64>().read_unaligned() & mask_bit;
    let lane23l = ptr.add(len + 1 - 64 + 1).cast::<u8x64>().read_unaligned() & mask_bit;
    let mut lane23 = lane23c + lane23r + lane23l;

    ptr = ptr.add(len + 1);

    macro_rules! count {
        ($lane31:ident, $lane32:ident, $lane33:ident) => {{
            let (lane31, lane32, lane33) = ($lane31, $lane32, $lane33);

            let sum1 = lane11 + lane21 + lane31;
            let sum2 = lane12 + lane22 + lane32;
            let sum3 = lane13 + lane23 + lane33;

            // println!("{:?}", sum1 >> 1);
            // return 0;

            tot += ((lane21c.simd_eq(zero) & sum1.simd_ge(threshold)).to_bitmask() & mask1)
                .count_ones();
            tot += ((lane22c.simd_eq(zero) & sum2.simd_ge(threshold)).to_bitmask() & mask2)
                .count_ones();
            tot += ((lane23c.simd_eq(zero) & sum3.simd_ge(threshold)).to_bitmask() & mask3)
                .count_ones();
        }};
    }

    loop {
        let lane31c = ptr.sub(1).cast::<u8x64>().read_unaligned() & mask_bit;
        let lane31r = ptr.sub(2).cast::<u8x64>().read_unaligned() & mask_bit;
        let lane31l = ptr.cast::<u8x64>().read_unaligned() & mask_bit;
        let lane31 = lane31c + lane31r + lane31l;

        let lane32c = ptr.add(61).cast::<u8x64>().read_unaligned() & mask_bit;
        let lane32r = ptr.add(61 - 1).cast::<u8x64>().read_unaligned() & mask_bit;
        let lane32l = ptr.add(61 + 1).cast::<u8x64>().read_unaligned() & mask_bit;
        let lane32 = lane32c + lane32r + lane32l;

        let lane33c = ptr.add(len + 1 - 64).cast::<u8x64>().read_unaligned() & mask_bit;
        let lane33r = ptr.add(len + 1 - 64 - 1).cast::<u8x64>().read_unaligned() & mask_bit;
        let lane33l = ptr.add(len + 1 - 64 + 1).cast::<u8x64>().read_unaligned() & mask_bit;
        let lane33 = lane33c + lane33r + lane33l;

        count!(lane31, lane32, lane33);

        (lane11, lane12, lane13) = (lane21, lane22, lane23);
        (lane21, lane22, lane23) = (lane31, lane32, lane33);
        (lane21c, lane22c, lane23c) = (lane31c, lane32c, lane33c);

        ptr = ptr.add(len + 1);
        if ptr == end {
            break;
        }
    }

    let [lane31, lane32, lane33] = [outside_lane; 3];
    count!(lane31, lane32, lane33);

    tot as i64
}

unsafe fn part2_inner(input: &str) -> i64 {
    let outside_byte = 0;
    // let outside_lane = u8x64::splat(outside_byte);

    const MAX_SIZE: usize = 140;
    const PREFIX: usize = 1 + (MAX_SIZE + 1) + (MAX_SIZE + 1);
    const SUFFIX: usize = (MAX_SIZE + 1) + 64;

    let mut data = MaybeUninit::<[u8; PREFIX + (MAX_SIZE + 1) * MAX_SIZE + SUFFIX]>::uninit();
    data.as_mut_ptr()
        .cast::<u8>()
        .write_bytes(outside_byte, PREFIX);
    for i in 0..input.len() {
        let b = *input.as_ptr().add(i) == b'@';
        data.as_mut_ptr()
            .cast::<u8>()
            .add(PREFIX + i)
            .write(if b { 255 } else { 0 });
    }
    data.as_mut_ptr()
        .cast::<u8>()
        .add(PREFIX + input.len())
        .write_bytes(outside_byte, SUFFIX);

    let start_ptr = data.as_mut_ptr().cast::<u8>().add(PREFIX);
    let end_ptr = start_ptr.add(input.len());

    let len = 128
        + u8x32::from_array(input.as_ptr().add(128).cast::<[u8; 32]>().read())
            .simd_eq(Simd::splat(b'\n'))
            .to_bitmask()
            .trailing_zeros() as usize;

    let mut removed = u8x64::splat(0);
    let mut update_start_ptr = start_ptr;
    let mut update_end_ptr = end_ptr;

    macro_rules! run_iter {
        ($ptr:expr) => {{
            let ptr = $ptr;

            let lane1l = ptr.sub(len + 1).sub(1).cast::<i8x64>().read_unaligned();
            let lane1c = ptr.sub(len + 1).cast::<i8x64>().read_unaligned();
            let lane1r = ptr.sub(len + 1).add(1).cast::<i8x64>().read_unaligned();

            let lane2l = ptr.sub(1).cast::<i8x64>().read_unaligned();
            let lane2c = ptr.cast::<mask8x64>().read_unaligned();
            let lane2r = ptr.add(1).cast::<i8x64>().read_unaligned();

            let lane3l = ptr.add(len + 1).sub(1).cast::<i8x64>().read_unaligned();
            let lane3c = ptr.add(len + 1).cast::<i8x64>().read_unaligned();
            let lane3r = ptr.add(len + 1).add(1).cast::<i8x64>().read_unaligned();

            let sum = lane1l + lane1c + lane1r + lane2l + lane2r + lane3l + lane3c + lane3r;
            let filtered = lane2c & sum.simd_le(i8x64::splat(-4));
            ptr.cast::<mask8x64>().write_unaligned(filtered);
            let changed = lane2c & !filtered;
            removed += changed.select(u8x64::splat(1), u8x64::splat(0));
            changed.any()
        }};
    }

    while update_end_ptr.offset_from(update_start_ptr) >= 10 * 64 {
        let mut ptr = update_start_ptr;
        // let removed_prev = removed;

        let (mut minptr, mut maxptr) = (end_ptr, start_ptr);

        while ptr < update_end_ptr {
            if run_iter!(ptr) {
                minptr = minptr.min(ptr);
                maxptr = maxptr.max(ptr);

                if run_iter!(ptr.sub(len + 1)) {
                    minptr = minptr.min(ptr.sub(len + 1));

                    if run_iter!(ptr) {
                        // if run_iter!(ptr.sub(len + 1)) {
                        //     // _ = run_iter!(ptr);
                        // }
                    }
                }
            }

            ptr = ptr.add(64);
        }

        update_start_ptr = start_ptr.max(minptr.wrapping_sub(len + 1 + 1));
        update_end_ptr = end_ptr.min(maxptr.wrapping_add(len + 1 + 1).wrapping_add(64));

        // println!(
        //     "{} {} {}",
        //     (removed - removed_prev).cast::<u64>().reduce_sum(),
        //     update_start_ptr.offset_from(start_ptr) / 64,
        //     update_end_ptr.offset_from(start_ptr) / 64
        // );
    }

    loop {
        let mut ptr = update_start_ptr;
        let removed_prev = removed;

        let (mut minptr, mut maxptr) = (end_ptr, start_ptr);

        while ptr < update_end_ptr {
            if run_iter!(ptr) {
                minptr = minptr.min(ptr);
                maxptr = maxptr.max(ptr);

                // if run_iter!(ptr.sub(len + 1)) {
                //     minptr = minptr.min(ptr.sub(len + 1));

                //     if run_iter!(ptr) {
                //         if run_iter!(ptr.sub(len + 1)) {
                //             _ = run_iter!(ptr);
                //         }
                //     }
                // }
            }

            ptr = ptr.add(64);
        }

        update_start_ptr = start_ptr.max(minptr.wrapping_sub(len + 1 + 1));
        update_end_ptr = end_ptr.min(maxptr.wrapping_add(len + 1 + 1).wrapping_add(64));

        // println!(
        //     "{} {} {}",
        //     (removed - removed_prev).cast::<u64>().reduce_sum(),
        //     update_start_ptr.offset_from(start_ptr) / 64,
        //     update_end_ptr.offset_from(start_ptr) / 64
        // );

        if removed_prev == removed {
            break;
        }
    }

    removed.cast::<u8>().cast::<u64>().reduce_sum() as i64
}
