#![feature(portable_simd)]

use std::simd::{
    ToBytes,
    cmp::{SimdOrd, SimdPartialEq, SimdPartialOrd},
    i16x8,
    num::{SimdInt, SimdUint},
    u8x32, u8x64, u16x8, u32x8,
};

pub fn run(input: &str) -> impl std::fmt::Display {
    // part1(input)
    part2(input)
}

pub fn part1(input: &str) -> i32 {
    unsafe { part1_v4_inner(input) }
}

pub fn part2(input: &str) -> i32 {
    unsafe { part2_inner(input) }
}

#[allow(unused)]
unsafe fn part1_inner(input: &str) -> i32 {
    let bytes = input.as_bytes();
    let std::ops::Range { mut start, end } = bytes.as_ptr_range();

    let mut cnt = 0;
    let mut off = 50;

    while start < end {
        let dir = *start;
        start = start.add(1);

        let swar = start.cast::<u32>().read_unaligned();
        let swarnl = swar ^ u32::from_ne_bytes([b'\n'; 4]);
        let masknl = (swarnl.wrapping_sub(0x01010101)) & (!swarnl & 0x80808080);
        let len8 = masknl.trailing_zeros() + 1;

        start = start.add(len8 as usize / 8);

        const MASKS: [u64; 4 * 3] = [
            0, 0x000000FF, 0x0000FFFF, 0x00FFFF00, // AND
            0, 24, 16, 8, // SHIFT
            0, 0x00303030, 0x00003030, 0x00003030, // OR
        ];

        let mask_and = MASKS
            .as_ptr()
            .cast::<u8>()
            .add(len8 as usize - 8)
            .cast::<u32>()
            .read();
        let mask_shift = MASKS
            .as_ptr()
            .cast::<u8>()
            .add(len8 as usize - 8 + 8 * 4)
            .cast::<u32>()
            .read();
        let mask_or = MASKS
            .as_ptr()
            .cast::<u8>()
            .add(len8 as usize - 8 + 8 * 8)
            .cast::<u32>()
            .read();

        let swar = ((swar & mask_and) << mask_shift) | mask_or;
        let swar = swar - u32::from_ne_bytes([b'0'; 4]);
        let bytes = u32::to_ne_bytes(swar >> 16);
        let n = 10 * bytes[0] as i32 + bytes[1] as i32;

        let n = if dir == b'R' { n } else { 100 - n };
        off += n;
        if off >= 100 {
            off -= 100;
            cnt += (off == 0) as i32;
        }
    }

    while start != end {
        let dir = *start;
        start = start.add(1);

        let d1 = (*start - b'0') as i32;
        start = start.add(1);

        let n = if *start != b'\n' {
            let d2 = (*start - b'0') as i32;
            start = start.add(1);

            if *start != b'\n' {
                let d3 = (*start - b'0') as i32;
                start = start.add(1);

                10 * d2 + d3
            } else {
                10 * d1 + d2
            }
        } else {
            d1
        };

        start = start.add(1);

        let n = if dir == b'R' { n } else { 100 - n };
        off += n;
        if off >= 100 {
            off -= 100;
            cnt += (off == 0) as i32;
        }
    }

    cnt
}

#[allow(unused)]
unsafe fn part1_v2_inner(input: &str) -> i32 {
    let bytes = input.as_bytes();
    let std::ops::Range { mut start, end } = bytes.as_ptr_range();

    let mut cnt = 0;
    let mut off = 50;

    while start != end {
        let dir = *start;
        start = start.add(1);

        let d1 = (*start - b'0') as i32;
        start = start.add(1);

        let n = if *start != b'\n' {
            let d2 = (*start - b'0') as i32;
            start = start.add(1);

            if *start != b'\n' {
                let d3 = (*start - b'0') as i32;
                start = start.add(1);

                10 * d2 + d3
            } else {
                10 * d1 + d2
            }
        } else {
            d1
        };

        start = start.add(1);

        let n = if dir == b'R' { n } else { 100 - n };
        off += n;
        if off >= 100 {
            off -= 100;
            cnt += (off == 0) as i32;
        }
    }

    cnt
}

#[allow(unused)]
unsafe fn part1_v3_inner(input: &str) -> i32 {
    let bytes = input.as_bytes();
    let std::ops::Range { mut start, end } = bytes.as_ptr_range();

    let mut cnt = 0;
    let mut off = 50;

    fn simd_cast<T: ToBytes, U: ToBytes<Bytes = T::Bytes>>(t: T) -> U {
        U::from_ne_bytes(t.to_ne_bytes())
    }

    while start.wrapping_add(64) < end {
        let mut mask = u8x64::from_array(start.cast::<[u8; 64]>().read())
            .simd_eq(u8x64::splat(b'\n'))
            .to_bitmask();

        let mut chunks = [0u32; 8];
        for i in 0..8 {
            let len = mask.trailing_zeros() as usize - 1;
            chunks[i] = start.add(1 + len).sub(4).cast::<u32>().read();
            mask >>= 1 + len + 1;
            start = start.add(1 + len + 1);
        }

        let lane: u8x32 = simd_cast(u32x8::from_array(chunks));

        let lane_lr = lane & lane.simd_gt(u8x32::splat(b'A')).to_int().cast();
        let lane_lr = u32x8::from_ne_bytes(lane_lr.to_ne_bytes());
        let lr = (lane_lr | (lane_lr >> u32x8::splat(8)) | (lane_lr >> u32x8::splat(16)))
            & u32x8::splat(255);

        let lane_digits = lane - u8x32::splat(b'0');
        let lane_digits = lane_digits & lane_digits.simd_le(u8x32::splat(9)).to_int().cast();

        let d1: u8x32 = simd_cast(simd_cast::<_, u32x8>(lane_digits) >> u32x8::splat(24));
        let d2: u8x32 = simd_cast(simd_cast::<_, u32x8>(lane_digits) >> u32x8::splat(16));

        let n = simd_cast::<_, u32x8>(d1 + u8x32::splat(10) * d2) & u32x8::splat(255);

        let invn = u32x8::splat(100) - n;
        let n = lr.simd_eq(u32x8::splat(b'R' as u32)).select(n, invn);

        let n = n + n.shift_elements_right::<1>(0);
        let n = n.simd_min(n - u32x8::splat(100));
        let n = n + n.shift_elements_right::<2>(0);
        let n = n.simd_min(n - u32x8::splat(100));
        let n = n + n.shift_elements_right::<4>(0);
        let n = n.simd_min(n - u32x8::splat(100));
        let n = n + u32x8::splat(off as u32);
        let n = n.simd_min(n - u32x8::splat(100));

        cnt += n.simd_eq(u32x8::splat(0)).to_bitmask().count_ones() as i32;
        off = n[7] as i32;
    }

    while start != end {
        let dir = *start;
        start = start.add(1);

        let d1 = (*start - b'0') as i32;
        start = start.add(1);

        let n = if *start != b'\n' {
            let d2 = (*start - b'0') as i32;
            start = start.add(1);

            if *start != b'\n' {
                let d3 = (*start - b'0') as i32;
                start = start.add(1);

                10 * d2 + d3
            } else {
                10 * d1 + d2
            }
        } else {
            d1
        };

        start = start.add(1);

        let n = if dir == b'R' { n } else { 100 - n };
        off += n;
        if off >= 100 {
            off -= 100;
            cnt += (off == 0) as i32;
        }
    }

    cnt as i32
}

unsafe fn part1_v4_inner(input: &str) -> i32 {
    let bytes = input.as_bytes();
    let std::ops::Range { mut start, end } = bytes.as_ptr_range();

    let mut cnt = 0;
    let mut off = 50;

    while start.wrapping_add(64) < end {
        let mut mask = u8x64::from_array(start.cast::<[u8; 64]>().read())
            .simd_eq(u8x64::splat(b'\n'))
            .to_bitmask();

        let mut lrs = [0u16; 8];
        let mut digits = [0u16; 8];
        for i in 0..8 {
            let len = mask.trailing_zeros() as usize - 1;
            lrs[i] = start.read() as u16;
            digits[i] = start.add(1 + len).sub(2).cast::<u16>().read();
            mask >>= 1 + len + 1;
            start = start.add(1 + len + 1);
        }

        let lr = u16x8::from_array(lrs);
        let digits = u16x8::from_array(digits) - u16x8::splat(b'0' as u16 * 256 + b'0' as u16);

        let d1 = digits >> u16x8::splat(8);
        let d2 = digits & u16x8::splat(255);
        let d2 = d2 & d2.simd_lt(u16x8::splat(10)).to_int().cast();
        let n = d1 + u16x8::splat(10) * d2;

        let invn = u16x8::splat(100) - n;
        let n = lr.simd_eq(u16x8::splat(b'R' as u16)).select(n, invn);

        let n = n + n.shift_elements_right::<1>(0);
        let n = n + n.shift_elements_right::<2>(0);
        let n = n + n.shift_elements_right::<4>(0);
        let n = n + u16x8::splat(off as u16);
        let n = n % u16x8::splat(100);

        cnt += n.simd_eq(u16x8::splat(0)).to_bitmask().count_ones() as i32;
        off = n[7] as i32;
    }

    while start != end {
        let dir = *start;
        start = start.add(1);

        let d1 = (*start - b'0') as i32;
        start = start.add(1);

        let n = if *start != b'\n' {
            let d2 = (*start - b'0') as i32;
            start = start.add(1);

            if *start != b'\n' {
                let d3 = (*start - b'0') as i32;
                start = start.add(1);

                10 * d2 + d3
            } else {
                10 * d1 + d2
            }
        } else {
            d1
        };

        start = start.add(1);

        let n = if dir == b'R' { n } else { 100 - n };
        off += n;
        if off >= 100 {
            off -= 100;
            cnt += (off == 0) as i32;
        }
    }

    cnt as i32
}

unsafe fn part2_inner(input: &str) -> i32 {
    let bytes = input.as_bytes();
    let std::ops::Range { mut start, end } = bytes.as_ptr_range();

    let mut cnt = 0;
    let mut off = 50;

    while start.wrapping_add(64) < end {
        let mut mask = u8x64::from_array(start.cast::<[u8; 64]>().read())
            .simd_eq(u8x64::splat(b'\n'))
            .to_bitmask();

        let mut lrs = [0u32; 8];
        let mut digits = [0u16; 8];
        for i in 0..8 {
            let len = mask.trailing_zeros() as usize - 1;
            lrs[i] = start.cast::<u32>().read();
            digits[i] = start.add(1 + len).sub(2).cast::<u16>().read();
            mask >>= 1 + len + 1;
            start = start.add(1 + len + 1);
        }

        let lrs = u32x8::from_array(lrs);
        let digits = u16x8::from_array(digits) - u16x8::splat(b'0' as u16 * 256 + b'0' as u16);

        let d1 = digits >> u16x8::splat(8);
        let d2 = digits & u16x8::splat(255);
        let d2 = d2 & d2.simd_lt(u16x8::splat(10)).to_int().cast();
        let n = d1 + u16x8::splat(10) * d2;

        let d3_base = lrs - u32x8::splat(u32::from_ne_bytes([b'0'; 4]));
        let d3: std::simd::Simd<u32, 8> = (d3_base >> u32x8::splat(8))
            & u32x8::splat(255)
            & d3_base.simd_lt(u32x8::splat(10 << 24)).to_int().cast();

        let lr = (lrs & u32x8::splat(255))
            .cast()
            .simd_eq(u16x8::splat(b'R' as u16));
        let invn = u16x8::splat(100) - n;
        let m = lr.select(n, invn);

        let m = m + m.shift_elements_right::<1>(0);
        let m = m + m.shift_elements_right::<2>(0);
        let m = m + m.shift_elements_right::<4>(0);
        let m = m + u16x8::splat(off as u16);
        let m = m % u16x8::splat(100);

        let prev = m.shift_elements_right::<1>(off as u16);

        let crossed = m.simd_gt(prev) ^ lr;
        let left_zero = prev.simd_eq(u16x8::splat(0)) & !lr;
        let right_zero = m.simd_eq(u16x8::splat(0)) & lr;
        let crossed_adjusted = crossed & !left_zero & !right_zero;

        cnt += m.simd_eq(u16x8::splat(0)).to_bitmask().count_ones() as i32;
        cnt += d3.reduce_sum() as i32;
        cnt += (crossed_adjusted.to_int() & i16x8::splat(1)).reduce_sum() as i32;
        off = m[7] as i32;
    }

    while start != end {
        let dir = *start;
        start = start.add(1);

        let d1 = (*start - b'0') as i32;
        start = start.add(1);

        let n = if *start != b'\n' {
            let d2 = (*start - b'0') as i32;
            start = start.add(1);

            if *start != b'\n' {
                let d3 = (*start - b'0') as i32;
                start = start.add(1);

                cnt += d1;
                10 * d2 + d3
            } else {
                10 * d1 + d2
            }
        } else {
            d1
        };

        start = start.add(1);

        if n != 0 {
            if dir == b'R' {
                off += n;
                if off >= 100 {
                    cnt += 1;
                    off -= 100;
                }
            } else {
                cnt -= (off == 0) as i32;
                off -= n;
                if off < 0 {
                    cnt += 1;
                    off += 100;
                }
                cnt += (off == 0) as i32;
            }
        }
    }

    cnt
}
