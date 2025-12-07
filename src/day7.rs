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

    let mut ptr = bytes.as_ptr().add(142 * 2);
    let mut mask = 1 << 31; // offset = 70 - 31;

    let mut splits = 0;

    for _ in 0..31 {
        let block = ptr.add(70 - 31).cast::<u8x64>().read_unaligned();
        let arrows = block.simd_eq(u8x64::splat(b'^')).to_bitmask();

        let hits = mask & arrows;
        mask = (mask & !arrows) | (hits >> 1) | (hits << 1);
        ptr = ptr.add(142 * 2);

        splits += hits.count_ones();

        // println!("{}{:064b}", " ".repeat(70 - 31), mask.reverse_bits());
    }

    let mut mask1 = mask << 32; // offset = 7 ??
    let mut mask2 = mask >> 31; // offset = 70

    // println!(
    //     "{}{:064b}\n{}{:064b}",
    //     " ".repeat(7),
    //     mask1.reverse_bits(),
    //     " ".repeat(7 + 63),
    //     mask2.reverse_bits()
    // );

    // println!(
    //     "{}{:064b}{:063b}",
    //     " ".repeat(7),
    //     mask1.reverse_bits(),
    //     (mask2 & !1).reverse_bits()
    // );

    for _ in 31..63 {
        let block1 = ptr.add(7).cast::<u8x64>().read_unaligned();
        let arrows1 = block1.simd_eq(u8x64::splat(b'^')).to_bitmask();

        let block2 = ptr.add(7 + 63).cast::<u8x64>().read_unaligned();
        let arrows2 = block2.simd_eq(u8x64::splat(b'^')).to_bitmask();

        let hits1 = mask1 & arrows1;
        mask1 = (mask1 & !arrows1) | (hits1 >> 1) | (hits1 << 1);

        let hits2 = mask2 & arrows2;
        mask2 = (mask2 & !arrows2) | (hits2 >> 1) | (hits2 << 1);

        let b12 = mask1 >> 63;
        let b21 = mask2;

        mask1 |= b21 << 63;
        mask2 |= b12;

        splits += hits1.count_ones() + (hits2 >> 1).count_ones();

        ptr = ptr.add(142 * 2);

        // println!(
        //     "{}{:064b}{:063b}",
        //     " ".repeat(7),
        //     mask1.reverse_bits(),
        //     (mask2 & !1).reverse_bits()
        // );
    }

    let mask = mask1 as u128 | ((mask2 as u128) << 63);

    let mut mask1 = (mask << 7) as u64; // offset 0
    let mut mask2 = (mask >> (63 - 7)) as u64; // offset 63
    let mut mask3 = (mask >> (63 + 63 - 7)) as u64; // offset 63 + 63

    // println!("{}{:0128b}", " ".repeat(7), mask.reverse_bits());

    // println!(
    //     "{:064b}\n{}{:064b}\n{}{:64b}",
    //     mask1.reverse_bits(),
    //     " ".repeat(63),
    //     mask2.reverse_bits(),
    //     " ".repeat(63 + 63),
    //     mask3.reverse_bits(),
    // );

    // println!(
    //     "{:064b}{:063b}{:63b}",
    //     mask1.reverse_bits(),
    //     (mask2 & !1).reverse_bits(),
    //     (mask3 & !1).reverse_bits(),
    // );

    for _ in 63..70 {
        let block1 = ptr.add(0).cast::<u8x64>().read_unaligned();
        let arrows1 = block1.simd_eq(u8x64::splat(b'^')).to_bitmask();

        let block2 = ptr.add(63).cast::<u8x64>().read_unaligned();
        let arrows2 = block2.simd_eq(u8x64::splat(b'^')).to_bitmask();

        let block3 = ptr.add(63 + 63).cast::<u8x16>().read_unaligned();
        let arrows3 = block3.simd_eq(u8x16::splat(b'^')).to_bitmask();

        let hits1 = mask1 & arrows1;
        mask1 = (mask1 & !arrows1) | (hits1 >> 1) | (hits1 << 1);

        let hits2 = mask2 & arrows2;
        mask2 = (mask2 & !arrows2) | (hits2 >> 1) | (hits2 << 1);

        let hits3 = mask3 & arrows3;
        mask3 = (mask3 & !arrows3) | (hits3 >> 1) | (hits3 << 1);

        let b12 = mask1 >> 63;
        let b21 = mask2;
        let b23 = mask2 >> 63;
        let b32 = mask3;

        mask1 |= b21 << 63;
        mask2 |= b12;
        mask2 |= b32 << 63;
        mask3 |= b23;

        splits += hits1.count_ones() + (hits2 >> 1).count_ones() + (hits3 >> 1).count_ones();

        ptr = ptr.add(142 * 2);

        // println!(
        //     "{:064b}{:063b}{:063b}",
        //     mask1.reverse_bits(),
        //     (mask2 & !1).reverse_bits(),
        //     (mask3 & !1).reverse_bits(),
        // );
    }

    splits as i64
}

unsafe fn part2_inner(input: &str) -> i64 {
    let bytes = input.as_bytes();

    let mut ptr = bytes.as_ptr().add(142 * 2);
    let mut lines = u8x64::splat(0); // offset = 70 - 31;
    lines[31] = 1;

    for _ in 0..10 {
        let block = ptr.add(70 - 31).cast::<u8x64>().read_unaligned();
        let arrows = block.simd_eq(u8x64::splat(b'^'));

        let hits = arrows.select(lines, Simd::splat(0));
        lines = arrows.select(Simd::splat(0), lines)
            + hits.shift_elements_right::<1>(0)
            + hits.shift_elements_left::<1>(0);
        ptr = ptr.add(142 * 2);

        // println!(
        //     "[{}{}{}]",
        //     "0, ".repeat(70 - 31),
        //     print(lines.as_array()),
        //     ", 0".repeat(141 - 64 - (70 - 31))
        // );
    }

    let mut lines = lines.cast::<u16>();

    for _ in 10..20 {
        let block = ptr.add(70 - 31).cast::<u8x64>().read_unaligned();
        let arrows = block.simd_eq(u8x64::splat(b'^')).cast();

        let hits = arrows.select(lines, Simd::splat(0));
        lines = arrows.select(Simd::splat(0), lines)
            + hits.shift_elements_right::<1>(0)
            + hits.shift_elements_left::<1>(0);
        ptr = ptr.add(142 * 2);

        // println!(
        //     "[{}{}{}]",
        //     "0, ".repeat(70 - 31),
        //     print(lines.as_array()),
        //     ", 0".repeat(141 - 64 - (70 - 31))
        // );
    }

    let mut lines = lines.cast::<u32>();

    for _ in 20..31 {
        let block = ptr.add(70 - 31).cast::<u8x64>().read_unaligned();
        let arrows = block.simd_eq(u8x64::splat(b'^')).cast();

        let hits = arrows.select(lines, Simd::splat(0));
        lines = arrows.select(Simd::splat(0), lines)
            + hits.shift_elements_right::<1>(0)
            + hits.shift_elements_left::<1>(0);
        ptr = ptr.add(142 * 2);

        // println!(
        //     "[{}{}{}]",
        //     "0, ".repeat(70 - 31),
        //     print(lines.as_array()),
        //     ", 0".repeat(141 - 64 - (70 - 31))
        // );
    }

    let mut lines1 = lines.shift_elements_right::<32>(0); // offset = 7 ??
    let mut lines2 = lines.shift_elements_left::<31>(0); // offset = 70

    // println!(
    //     "{}{:064b}\n{}{:064b}",
    //     " ".repeat(7),
    //     mask1.reverse_bits(),
    //     " ".repeat(7 + 63),
    //     mask2.reverse_bits()
    // );

    // println!(
    //     "{}{:064b}{:063b}",
    //     " ".repeat(7),
    //     mask1.reverse_bits(),
    //     (mask2 & !1).reverse_bits()
    // );

    for _ in 31..40 {
        // assert_eq!(lines1[63], lines2[0]);

        let block1 = ptr.add(7).cast::<u8x64>().read_unaligned();
        let arrows1 = block1.simd_eq(u8x64::splat(b'^')).cast();

        let block2 = ptr.add(7 + 63).cast::<u8x64>().read_unaligned();
        let arrows2 = block2.simd_eq(u8x64::splat(b'^')).cast();

        let hits1 = arrows1.select(lines1, Simd::splat(0));
        lines1 = arrows1.select(Simd::splat(0), lines1)
            + hits1.shift_elements_right::<1>(0)
            + hits1.shift_elements_left::<1>(0);

        let hits2 = arrows2.select(lines2, Simd::splat(0));
        lines2 = arrows2.select(Simd::splat(0), lines2)
            + hits2.shift_elements_right::<1>(0)
            + hits2.shift_elements_left::<1>(0);

        // let b12 = hits1[62];
        let b21 = hits2[1];

        lines1[63] += b21;
        lines2[0] = lines1[63];

        // assert_eq!(lines1[63], lines2[0]);

        ptr = ptr.add(142 * 2);

        // println!(
        //     "[{}{}, {}{}]",
        //     "0, ".repeat(7),
        //     print(lines1.as_array()),
        //     print(&lines2.as_array()[1..]),
        //     ", 0".repeat(141 - 64 - 63 - 7)
        // );
    }

    let mut lines1 = lines1.cast::<u64>();
    let mut lines2 = lines2.cast::<u64>();

    for _ in 40..63 {
        // assert_eq!(lines1[63], lines2[0]);

        let block1 = ptr.add(7).cast::<u8x64>().read_unaligned();
        let arrows1 = block1.simd_eq(u8x64::splat(b'^')).cast();

        let block2 = ptr.add(7 + 63).cast::<u8x64>().read_unaligned();
        let arrows2 = block2.simd_eq(u8x64::splat(b'^')).cast();

        let hits1 = arrows1.select(lines1, Simd::splat(0));
        lines1 = arrows1.select(Simd::splat(0), lines1)
            + hits1.shift_elements_right::<1>(0)
            + hits1.shift_elements_left::<1>(0);

        let hits2 = arrows2.select(lines2, Simd::splat(0));
        lines2 = arrows2.select(Simd::splat(0), lines2)
            + hits2.shift_elements_right::<1>(0)
            + hits2.shift_elements_left::<1>(0);

        // let b12 = hits1[62];
        let b21 = hits2[1];

        lines1[63] += b21;
        lines2[0] = lines1[63];

        // assert_eq!(lines1[63], lines2[0]);

        ptr = ptr.add(142 * 2);

        // println!(
        //     "[{}{}, {}{}]",
        //     "0, ".repeat(7),
        //     print(lines1.as_array()),
        //     print(&lines2.as_array()[1..]),
        //     ", 0".repeat(141 - 64 - 63 - 7)
        // );
    }

    // println!("{:?}", lines1);
    // println!("{:?}", lines2);

    let lines4 = lines1.shift_elements_right::<7>(0); // offset 0
    let lines5 = lines1.shift_elements_left::<{ 63 - 7 }>(0) | lines2.shift_elements_right::<7>(0); // offset 63
    let lines6 = lines2
        .shift_elements_left::<{ 63 - 7 }>(0)
        .extract::<0, 16>(); // offset 63 + 63

    let mut lines1 = lines4;
    let mut lines2 = lines5;
    let mut lines3 = lines6;

    // println!(
    //     "X[{}, {}, {}]",
    //     print(lines1.as_array()),
    //     print(&lines2.as_array()[1..]),
    //     print(&lines3.as_array()[1..][..141 - 64 - 63]),
    // );

    // println!("{:?}", lines1);
    // println!("{:?}", lines2);
    // println!("{:?}", lines3);

    // println!("{}{:0128b}", " ".repeat(7), mask.reverse_bits());

    // println!(
    //     "{:064b}\n{}{:064b}\n{}{:64b}",
    //     mask1.reverse_bits(),
    //     " ".repeat(63),
    //     mask2.reverse_bits(),
    //     " ".repeat(63 + 63),
    //     mask3.reverse_bits(),
    // );

    // println!(
    //     "{:064b}{:063b}{:63b}",
    //     mask1.reverse_bits(),
    //     (mask2 & !1).reverse_bits(),
    //     (mask3 & !1).reverse_bits(),
    // );

    for _ in 63..70 {
        // assert_eq!(lines1[63], lines2[0]);
        // assert_eq!(lines2[63], lines3[0]);

        // println!("{:?}", std::str::from_utf8(&*ptr.cast::<[u8; 141]>()));

        let block1 = ptr.add(0).cast::<u8x64>().read_unaligned();
        let arrows1 = block1.simd_eq(u8x64::splat(b'^')).cast();

        let block2 = ptr.add(63).cast::<u8x64>().read_unaligned();
        let arrows2 = block2.simd_eq(u8x64::splat(b'^')).cast();

        let block3 = ptr.add(63 + 63).cast::<u8x16>().read_unaligned();
        let arrows3 = block3.simd_eq(u8x16::splat(b'^')).cast();

        let hits1 = arrows1.select(lines1, Simd::splat(0));
        lines1 = arrows1.select(Simd::splat(0), lines1)
            + hits1.shift_elements_right::<1>(0)
            + hits1.shift_elements_left::<1>(0);

        let hits2 = arrows2.select(lines2, Simd::splat(0));
        lines2 = arrows2.select(Simd::splat(0), lines2)
            + hits2.shift_elements_right::<1>(0)
            + hits2.shift_elements_left::<1>(0);

        let hits3 = arrows3.select(lines3, Simd::splat(0));
        lines3 = arrows3.select(Simd::splat(0), lines3)
            + hits3.shift_elements_right::<1>(0)
            + hits3.shift_elements_left::<1>(0);

        // let b12 = hits1[62];
        let b21 = hits2[1];
        // let b23 = hits2[62];
        let b32 = hits3[1];

        lines1[63] += b21;
        lines2[0] = lines1[63];
        lines2[63] += b32;
        lines3[0] = lines2[63];

        // assert_eq!(
        //     lines1[63], lines2[0],
        //     "\n{:?}\n{:?}\n{:?}\n{:?}\n",
        //     lines1, lines2, hits1, hits2
        // );
        // assert_eq!(lines2[63], lines3[0]);

        ptr = ptr.add(142 * 2);

        // println!(
        //     "[{}, {}, {}]",
        //     print(lines1.as_array()),
        //     print(&lines2.as_array()[1..]),
        //     print(&lines3.as_array()[1..][..141 - 64 - 63]),
        // );
    }

    ((lines1 + lines2).reduce_sum() + lines3.reduce_sum() - lines2[0] - lines3[0]) as i64
}
