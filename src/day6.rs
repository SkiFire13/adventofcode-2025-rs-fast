#![feature(portable_simd)]
#![feature(slice_ptr_get)]
#![feature(array_ptr_get)]

use std::{
    mem::MaybeUninit,
    simd::{prelude::*, *},
    u64,
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

    let mut len = 0;
    let mut ptr = bytes.as_ptr().cast::<u8x64>();

    loop {
        let mask = ptr.read().simd_eq(u8x64::splat(b'\n')).to_bitmask();
        if mask != 0 {
            len += mask.trailing_zeros() as usize;
            break;
        }
        len += 64;
        ptr = ptr.add(1);
    }

    let ptr1 = bytes.as_ptr();
    let ptr2 = ptr1.add(len + 1);
    let ptr3 = ptr2.add(len + 1);
    let ptr4 = ptr3.add(len + 1);
    let ptr_o = ptr4.add(len + 1);
    let mut offset = 0;

    let mut tot = 0;
    let mut tot_acc = u32x4::splat(0);

    while offset != len + 1 {
        let v1 = ptr1.add(offset).cast::<u32>().read_unaligned();
        let v2 = ptr2.add(offset).cast::<u32>().read_unaligned();
        let v3 = ptr3.add(offset).cast::<u32>().read_unaligned();
        let v4 = ptr4.add(offset).cast::<u32>().read_unaligned();

        let lane = u32x4::from_array([v1, v2, v3, v4]);
        let lane = std::mem::transmute::<_, u8x16>(lane);

        let len_mask = std::mem::transmute::<_, u32x4>(simd_swizzle!(
            lane,
            [0, 4, 8, 12, 1, 5, 9, 13, 2, 6, 10, 14, 3, 7, 11, 15]
        ))
        .simd_le(u32x4::splat(u32::from_ne_bytes([b' '; 4])))
        .to_bitmask();
        let len = (len_mask | 0b10000).trailing_zeros() as usize;

        let mask = (u32::MAX as u64) >> (8 * (4 - len));
        let mask = u32x4::splat(mask as u32);
        let mask = mask8x16::from_int_unchecked(std::mem::transmute(mask));
        let lane = mask.select(lane, u8x16::splat(b' '));

        let lane = std::mem::transmute::<_, u32x4>(lane);

        let shift = (lane & u32x4::splat(0b10000 << 24)) >> u32x4::splat(25);
        let lane = lane << (u32x4::splat(8) - shift);
        let shift = (lane & u32x4::splat(0b10000 << 24)) >> u32x4::splat(25);
        let lane = lane << (u32x4::splat(8) - shift);
        let shift = (lane & u32x4::splat(0b10000 << 24)) >> u32x4::splat(25);
        let lane = lane << (u32x4::splat(8) - shift);

        let lane = std::mem::transmute::<_, u8x16>(lane);
        let lane = lane.saturating_sub(u8x16::splat(b'0'));

        let mul10 = u8x16::from_array([10, 1, 10, 1, 10, 1, 10, 1, 10, 1, 10, 1, 10, 1, 10, 1]);
        let mul100 = u16x8::from_array([100, 1, 100, 1, 100, 1, 100, 1]);

        #[cfg(target_arch = "x86_64")]
        let lane = {
            use core::arch::x86_64::{_mm_madd_epi16, _mm_maddubs_epi16};
            let lane = u16x8::from(_mm_maddubs_epi16(lane.into(), mul10.into()));
            let lane = u32x4::from(_mm_madd_epi16(lane.into(), mul100.into()));
            lane
        };

        #[cfg(not(target_arch = "x86_64"))]
        let lane = {
            let mul = lane.cast::<u16>() * mul10.cast::<u16>();
            let (lo, hi) = mul.deinterleave(mul);
            let lane = lo.extract::<0, 8>() + hi.extract::<0, 8>();

            let mul = lane * mul100;
            let (lo, hi) = mul.deinterleave(mul);
            let lane = lo.extract::<0, 4>() + hi.extract::<0, 4>();
            lane.cast::<u32>()
        };

        // println!("{:?} {} {:04b}", lane, len, len_mask);
        // let tot_prev = tot;

        if *ptr_o.add(offset) == b'+' {
            tot_acc += lane;
        } else {
            tot += (lane.extract::<0, 2>() * lane.extract::<2, 2>())
                .cast::<u64>()
                .reduce_product();
        }

        // println!("{:?} {} {}", lane, *ptr_o as char, tot - tot_prev);

        offset += len + 1;
    }

    tot as i64 + tot_acc.reduce_sum() as i64
}

unsafe fn part2_inner(input: &str) -> i64 {
    let bytes = input.as_bytes();

    let mut len = 0;
    let mut ptr = bytes.as_ptr().cast::<u8x64>();

    loop {
        let mask = ptr.read().simd_eq(u8x64::splat(b'\n')).to_bitmask();
        if mask != 0 {
            len += mask.trailing_zeros() as usize;
            break;
        }
        len += 64;
        ptr = ptr.add(1);
    }

    let ptr1 = bytes.as_ptr();
    let ptr2 = ptr1.add(len + 1);
    let ptr3 = ptr2.add(len + 1);
    let ptr4 = ptr3.add(len + 1);
    let ptr_o = ptr4.add(len + 1);
    let mut offset = 0;

    let mut tot_sum = u32x4::splat(0);
    let mut tot_prod = u64x4::splat(0);

    while offset != len + 1 {
        let remaining = (len + 1) - offset;
        let mask = Mask::from_bitmask(u64::MAX >> 64usize.saturating_sub(remaining));
        let line_o = u8x32::load_select_ptr(ptr_o.add(offset), mask, Simd::splat(b'+'));
        let mut mask_ops = line_o
            .simd_ge(Simd::splat(u8::min(b'+', b'*')))
            .to_bitmask();

        mask_ops >>= 1;

        let mut line1 = u32x4::splat(0);
        let mut line2 = u32x4::splat(0);
        let mut line3 = u32x4::splat(0);
        let mut line4 = u32x4::splat(0);
        let mut lens = u32x4::splat(0);
        let mut ops = u32x4::splat(0);

        for i in 0..4 {
            let len = mask_ops.trailing_zeros();

            line1[i] = ptr1.add(offset).cast::<u32>().read_unaligned();
            line2[i] = ptr2.add(offset).cast::<u32>().read_unaligned();
            line3[i] = ptr3.add(offset).cast::<u32>().read_unaligned();
            line4[i] = ptr4.add(offset).cast::<u32>().read_unaligned();
            lens[i] = len;
            ops[i] = *ptr_o.add(offset) as u32;

            mask_ops >>= len + 1;
            offset += (len + 1) as usize;
        }

        let line1 = std::mem::transmute::<_, u8x16>(line1);
        let line2 = std::mem::transmute::<_, u8x16>(line2);
        let line3 = std::mem::transmute::<_, u8x16>(line3);
        let line4 = std::mem::transmute::<_, u8x16>(line4);

        let empty8 = Simd::splat(b' ');
        let zero8 = Simd::splat(b'0');
        let empty32 = Simd::splat(b' ' as u32);
        let zero32 = Simd::splat(b'0' as u32);

        let mut acc = line1.saturating_sub(zero8);
        acc = line2
            .simd_eq(empty8)
            .select(acc, Simd::splat(10) * acc + line2 - zero8);

        let mut acc = acc.cast::<u32>();
        let line3 = line3.cast::<u32>();
        let line4 = line4.cast::<u32>();
        acc = line3
            .simd_eq(empty32)
            .select(acc, Simd::splat(10) * acc + line3 - zero32);
        acc = line4
            .simd_eq(empty32)
            .select(acc, Simd::splat(10) * acc + line4 - zero32);

        let mask_valid = u32x4::splat(u32::MAX) >> (u32x4::splat(8) * (u32x4::splat(4) - lens));
        let mask_valid = std::mem::transmute::<_, i8x16>(mask_valid);
        let mask_valid = mask8x16::from_int_unchecked(mask_valid);

        let acc_sum = mask_valid.cast().select(acc, u32x16::splat(0));
        let acc_sum = acc_sum + acc_sum.shift_elements_left::<2>(0);
        let acc_sum = acc_sum + acc_sum.shift_elements_left::<1>(0);
        let acc_sum = simd_swizzle!(acc_sum, [0, 4, 8, 12]);
        tot_sum += ops
            .simd_eq(Simd::splat(b'+' as u32))
            .cast()
            .select(acc_sum, u32x4::splat(0));

        let acc_prod = mask_valid.cast().select(acc, u32x16::splat(1));
        let acc_prod = acc_prod * acc_prod.shift_elements_left::<2>(0);
        let acc_prod1 = simd_swizzle!(acc_prod, [0, 15, 4, 15, 8, 15, 12, 15]);
        let acc_prod2 = simd_swizzle!(acc_prod, [1, 15, 5, 15, 9, 15, 13, 15]);
        let acc_prod =
            std::mem::transmute::<_, u64x4>(acc_prod1) * std::mem::transmute::<_, u64x4>(acc_prod2);
        tot_prod += ops
            .simd_eq(Simd::splat(b'*' as u32))
            .cast()
            .select(acc_prod, u64x4::splat(0));
    }

    tot_sum.reduce_sum() as i64 + tot_prod.reduce_sum() as i64
}
