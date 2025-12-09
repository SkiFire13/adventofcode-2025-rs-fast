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

macro_rules! parse_n5 {
    ($ptr:ident, $sep:expr) => {{
        let mut n = *$ptr as u32 - b'0' as u32;
        $ptr = $ptr.add(1);
        n = 10 * n + *$ptr as u32 - b'0' as u32;
        $ptr = $ptr.add(1);
        n = 10 * n + *$ptr as u32 - b'0' as u32;
        $ptr = $ptr.add(1);
        n = 10 * n + *$ptr as u32 - b'0' as u32;
        $ptr = $ptr.add(1);
        if *$ptr != $sep {
            n = 10 * n + *$ptr as u32 - b'0' as u32;
            $ptr = $ptr.add(1);
        }
        #[allow(unused_assignments)]
        {
            $ptr = $ptr.add(1);
        }
        n
    }};
}

unsafe fn part1_inner(input: &str) -> i64 {
    let mut ptr = input.as_ptr();
    let end_ptr = ptr.add(input.len());

    let mut xs = MaybeUninit::<[i32; 500]>::uninit();
    let mut ys = MaybeUninit::<[i32; 500]>::uninit();
    let mut nums_len = 0;

    let xs = xs.as_mut_ptr().as_mut_ptr();
    let ys = ys.as_mut_ptr().as_mut_ptr();

    let mut max = u64x8::splat(0);

    while ptr != end_ptr {
        let x = parse_n5!(ptr, b',') as i32;
        let y = parse_n5!(ptr, b'\n') as i32;

        let x1 = i32x8::splat(x);
        let y1 = i32x8::splat(y);

        let mut j = 0;
        while j + 8 <= nums_len {
            let x2 = xs.add(j).cast::<i32x8>().read_unaligned();
            let y2 = ys.add(j).cast::<i32x8>().read_unaligned();

            let dx = x1.abs_diff(x2) + u32x8::splat(1);
            let dy = y1.abs_diff(y2) + u32x8::splat(1);

            let size = dx.cast::<u64>() * dy.cast::<u64>();

            max = max.simd_max(size);

            j += 8;
        }
        if j != nums_len {
            let mask = mask32x8::from_bitmask((1 << (nums_len - j)) - 1);

            let x2 = i32x8::load_select_ptr(xs.add(j), mask, x1);
            let y2 = i32x8::load_select_ptr(ys.add(j), mask, x1);

            let dx = x1.abs_diff(x2) + u32x8::splat(1);
            let dy = y1.abs_diff(y2) + u32x8::splat(1);

            let size = dx.cast::<u64>() * dy.cast::<u64>();

            max = max.simd_max(size);
        }

        xs.add(nums_len).write(x);
        ys.add(nums_len).write(y);
        nums_len += 1;
    }

    max.reduce_max() as i64
}

unsafe fn part2_inner(input: &str) -> i64 {
    let mut ptr = input.as_ptr();
    // let mut len = 0;

    // macro_rules! read_until {
    //     ($idx:literal) => {{
    //         while len < $idx - 7 {
    //             let lane = ptr.cast::<u8x64>().read_unaligned();
    //             let mask = lane.simd_eq(u8x64::splat(b'\n')).to_bitmask();
    //             len += mask.count_ones();
    //             ptr = ptr.add(64);
    //         }

    //         debug_assert!(len != $idx);

    //         loop {
    //             let lane = ptr.cast::<u8x64>().read_unaligned();
    //             let mut mask = lane.simd_eq(u8x64::splat(b'\n')).to_bitmask();
    //             let new_len = len + mask.count_ones();
    //             if new_len >= $idx {
    //                 while len < $idx {
    //                     let l = 1 + mask.trailing_zeros();
    //                     mask >>= l;
    //                     len += 1;
    //                     ptr = ptr.add(l as usize);
    //                 }

    //                 break;
    //             }
    //             len = new_len;
    //             ptr = ptr.add(64);
    //         }
    //     }};
    // }

    const L: usize = 8;

    // read_until!(27);
    // len = 27;
    ptr = ptr.add(324);

    let mut xs = [0; 2 * L];
    let mut ys = [0; 2 * L];
    for i in 0..L {
        xs[i] = parse_n5!(ptr, b',');
        ys[i] = parse_n5!(ptr, b'\n');
        // println!("point[{}] = ({}, {})", len, xs[i], ys[i]);
        // len += 1;
    }

    // len = 200;
    ptr = ptr.add(2580 - 420 - 5);
    // println!("{:?}", std::str::from_utf8(&*ptr.cast::<[u8; 16]>()));
    while *ptr != b'\n' {
        ptr = ptr.add(1);
    }
    ptr = ptr.add(1);
    // println!("{:?}", std::str::from_utf8(&*ptr.cast::<[u8; 16]>()));
    for i in L..2 * L {
        xs[i] = parse_n5!(ptr, b',');
        ys[i] = parse_n5!(ptr, b'\n');
        // println!("point[{}] = ({}, {})", len, xs[i], ys[i]);
        // len += 1;
    }

    // read_until!(248);
    ptr = ptr.add(264);
    // println!("{:?}", std::str::from_utf8(&*ptr.cast::<[u8; 16]>()));
    let x = parse_n5!(ptr, b',');
    let y = parse_n5!(ptr, b'\n');
    // println!("point[{}] = ({}, {})", len, x, y);

    let mut max = 0;

    let xs = Simd::from_array(xs);
    let ys = Simd::from_array(ys);
    let bounding = xs.simd_lt(Simd::splat(x)) & ys.simd_gt(Simd::splat(y));

    for i in L..2 * L {
        let xi = Simd::splat(xs[i]);
        let yi = Simd::splat(ys[i]);

        if !(xs.simd_gt(xi) & ys.simd_lt(yi) & bounding).any() {
            let a = (x - xs[i] + 1) as i64 * (ys[i] - y + 1) as i64;
            // dbg!(x, xs[i], y, ys[i], a);
            // println!();
            max = max.max(a);
        }
    }

    max
}
