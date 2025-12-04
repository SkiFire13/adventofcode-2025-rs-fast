#![feature(portable_simd)]

use std::simd::{cmp::SimdPartialEq, u8x64};

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

    while ptr != end {
        // fn solve(line: &[u8], n: usize) -> usize {
        //     let mut stack = Vec::with_capacity(n);

        //     for (i, &b) in line.iter().enumerate() {
        //         let remaining = line.len() - i - 1;
        //         while let Some(&last) = stack.last()
        //             && last < b
        //             && stack.len() + remaining >= n
        //         {
        //             stack.pop();
        //         }
        //         if stack.len() < n {
        //             stack.push(b);
        //         }
        //     }

        //     stack.into_iter().fold(0, |acc, d| 10 * acc + d as usize)
        // }
        // let prev_tot = tot;

        'inner: {
            let b1 = ptr.cast::<u8x64>().read_unaligned();
            let b2 = ptr.add(36).cast::<u8x64>().read_unaligned();

            let mut mask1 = u64::MAX;
            let mut mask2 = (u64::MAX << 28) ^ 1 << 63;

            let last = *ptr.add(99);
            let mut acc = 0;

            let mut i = 9;
            while i != 0 {
                let m1 = b1.simd_eq(u8x64::splat(i + b'0')).to_bitmask() & mask1;
                let m2 = b2.simd_eq(u8x64::splat(i + b'0')).to_bitmask() & mask2;

                let c1 = m1.count_ones();
                let c2 = m2.count_ones();

                // println!("{i}");
                // println!("{mask1:064b}");
                // println!("{mask2:064b}");
                // println!("{m1:064b}");
                // println!("{m2:064b}");
                // println!("{m1:064b}");
                // println!("{m2:064b}");

                if c1 + c2 >= 2 {
                    acc = acc.max(i as u64);
                    let l = (last as u64 - b'0' as u64).max(i as u64);
                    tot += 10 * acc + l;
                    break 'inner;
                }

                if c1 + c2 == 1 {
                    if acc != 0 {
                        tot += 10 * acc + i as u64;
                        break 'inner;
                    }

                    if last >= i + b'0' {
                        tot += 10 * i as u64 + (last as u64 - b'0' as u64);
                        break 'inner;
                    }

                    acc = i as u64;

                    if c1 == 1 {
                        mask1 = u64::MAX << m1.trailing_zeros();
                        mask2 = u64::MAX << 28;
                    } else {
                        mask1 = 0;
                        mask2 = u64::MAX << m2.trailing_zeros();
                    }
                }

                i -= 1;
            }

            while i != 0 {
                let m2 = b2.simd_eq(u8x64::splat(i + b'0')).to_bitmask() & mask2;

                let c2 = m2.count_ones();

                // println!("{i}");
                // println!("{mask1:064b}");
                // println!("{mask2:064b}");
                // println!("{m1:064b}");
                // println!("{m2:064b}");
                // println!("{m1:064b}");
                // println!("{m2:064b}");

                if c2 >= 2 {
                    acc = acc.max(i as u64);
                    let l = (last as u64 - b'0' as u64).max(i as u64);
                    tot += 10 * acc + l;
                    break 'inner;
                }

                if c2 == 1 {
                    if acc != 0 {
                        tot += 10 * acc + i as u64;
                        break 'inner;
                    }

                    if last >= i + b'0' {
                        tot += 10 * i as u64 + (last as u64 - b'0' as u64);
                        break 'inner;
                    }

                    acc = i as u64;
                    mask2 = u64::MAX << m2.trailing_zeros();
                }

                i -= 1;
            }

            tot += 10 * acc;
        }

        // let got = tot - prev_tot;
        // let expected = solve(&ptr.cast::<[u8; 100]>().read().map(|b| b - b'0'), 2) as u64;
        // assert_eq!(
        //     expected,
        //     got,
        //     "{:?}",
        //     std::str::from_utf8(&ptr.cast::<[u8; 100]>().read())
        // );

        ptr = ptr.add(101);
    }

    tot as i64
}

unsafe fn part2_inner(input: &str) -> i64 {
    // 0
    let bytes = input.as_bytes();
    let mut ptr = bytes.as_ptr();
    let end = ptr.add(bytes.len());

    let mut tot = 0;

    // TODO: optimization: use length 65 and avoid .min(remaining)
    const POWERS_OR_TEN: [u64; 12] = {
        let mut out = [1; 12];
        let mut i = 1;
        while i != 12 {
            out[i] = 10 * out[i - 1];
            i += 1;
        }
        out
    };
    const FACTOR_ONES: [u64; 12] = {
        let mut out = POWERS_OR_TEN;
        let mut i = 1;
        while i != 12 {
            out[i] += out[i - 1];
            i += 1;
        }
        out
    };

    while ptr != end {
        let b1 = ptr.cast::<u8x64>().read_unaligned();
        let b2 = ptr.add(36).cast::<u8x64>().read_unaligned();

        #[inline(always)]
        fn mask_danger_tail(len: usize) -> u64 {
            ((1 << len) - 1) << (64 - len)
        }

        let mut remaining = 12;
        let mut danger_mask = mask_danger_tail(remaining - 1);

        let mut mask1 = u64::MAX >> 28;
        let mut mask2 = u64::MAX;

        let mut tail_len = 0;
        let mut tail_acc = 0;

        let mut front_acc = 0;

        // println!();

        let mut i = 9;
        let mut b2e = b2.simd_eq(u8x64::splat(i + b'0')).to_bitmask();
        'outer: loop {
            // println!(
            //     "{}",
            //     std::str::from_utf8(&ptr.cast::<[u8; 100]>().read()).unwrap()
            // );
            // println!("{:064b}{:036b}", mask1.reverse_bits(), mask2.reverse_bits());
            // println!("{:064b}{:036b}", 0, danger_mask.reverse_bits());

            // println!(
            //     "i={i}, front_acc={front_acc}, front_len={front_len}, tail_acc={tail_acc}, tail_len={tail_len}, remaining={remaining}"
            // );
            // assert_eq!(remaining - 1, danger_mask.count_ones() as usize);

            let m1 = b1.simd_eq(u8x64::splat(i + b'0')).to_bitmask() & mask1;
            let c1 = m1.count_ones();

            if c1 != 0 {
                let extra_len = (c1 as usize).min(remaining);
                // println!("extra_len1={extra_len}");

                let power_of_ten = *POWERS_OR_TEN.get_unchecked(extra_len);
                let factor_ones = *FACTOR_ONES.get_unchecked(extra_len - 1);

                front_acc = power_of_ten * front_acc + i as u64 * factor_ones;
                remaining -= extra_len;
                danger_mask &= danger_mask << extra_len;

                if remaining == 0 {
                    break;
                }

                mask1 &= !(u64::MAX >> m1.leading_zeros());
            }

            let tail_mask = !(u64::MAX >> tail_len);

            let mut m2 = b2e & mask2 & !tail_mask & !danger_mask;
            let mut c2 = m2.count_ones();

            if c2 != 0 {
                loop {
                    let extra_len = (c2 as usize).min(remaining);
                    // println!("extra_len2={extra_len}");
                    let power_of_ten = *POWERS_OR_TEN.get_unchecked(extra_len);
                    let factor_ones = *FACTOR_ONES.get_unchecked(extra_len - 1);

                    front_acc = power_of_ten * front_acc + i as u64 * factor_ones;
                    remaining -= extra_len;
                    danger_mask &= danger_mask << extra_len;

                    if remaining == 0 {
                        break 'outer;
                    }

                    // mask1 = 0;
                    mask2 = !(u64::MAX >> m2.leading_zeros());

                    m2 = b2e & mask2 & !tail_mask & !danger_mask;
                    c2 = m2.count_ones();

                    if c2 == 0 {
                        break;
                    }
                }

                break;
            }

            let m3 = b2e & danger_mask;
            if m3 != 0 {
                let extra_len = 64 - m3.trailing_zeros() as usize - tail_len;
                // println!("extra_len3={extra_len}");

                let end = ptr.add(100 - tail_len);
                let mut ptr = end.sub(extra_len).add(1);
                let mut extra_tail_acc = i as u64;
                while ptr != end {
                    extra_tail_acc = 10 * extra_tail_acc + (*ptr as u64 - b'0' as u64);
                    ptr = ptr.add(1);
                }

                let power_of_ten = *POWERS_OR_TEN.get_unchecked(tail_len);

                tail_acc += power_of_ten * extra_tail_acc;
                tail_len += extra_len;
                remaining -= extra_len;

                if remaining == 0 {
                    break;
                }

                danger_mask &= danger_mask >> extra_len;
            }

            i -= 1;
            b2e = b2.simd_eq(u8x64::splat(i + b'0')).to_bitmask();
        }

        if remaining != 0 {
            'outer: loop {
                // println!(
                //     "{}",
                //     std::str::from_utf8(&ptr.cast::<[u8; 100]>().read()).unwrap()
                // );
                // println!("{:064b}{:036b}", mask1.reverse_bits(), mask2.reverse_bits());
                // println!("{:064b}{:036b}", 0, danger_mask.reverse_bits());

                // println!(
                //     "i={i}, front_acc={front_acc}, front_len={front_len}, tail_acc={tail_acc}, tail_len={tail_len}, remaining={remaining}"
                // );
                // assert_eq!(remaining - 1, danger_mask.count_ones() as usize);

                let m3 = b2e & danger_mask;
                if m3 != 0 {
                    let extra_len = 64 - m3.trailing_zeros() as usize - tail_len;
                    // println!("extra_len3={extra_len}");

                    let end = ptr.add(100 - tail_len);
                    let mut ptr = end.sub(extra_len).add(1);
                    let mut extra_tail_acc = i as u64;
                    while ptr != end {
                        extra_tail_acc = 10 * extra_tail_acc + (*ptr as u64 - b'0' as u64);
                        ptr = ptr.add(1);
                    }

                    let power_of_ten = *POWERS_OR_TEN.get_unchecked(tail_len);

                    tail_acc += power_of_ten * extra_tail_acc;
                    tail_len += extra_len;
                    remaining -= extra_len;

                    if remaining == 0 {
                        break;
                    }

                    danger_mask &= danger_mask >> extra_len;
                }

                i -= 1;
                b2e = b2.simd_eq(u8x64::splat(i + b'0')).to_bitmask();

                let tail_mask = !(u64::MAX >> tail_len);
                while let m2 = b2e & mask2 & !tail_mask & !danger_mask
                    && let c2 = m2.count_ones()
                    && c2 != 0
                {
                    let extra_len = (c2 as usize).min(remaining);
                    // println!("extra_len2={extra_len}");
                    let power_of_ten = *POWERS_OR_TEN.get_unchecked(extra_len);
                    let factor_ones = *FACTOR_ONES.get_unchecked(extra_len - 1);

                    front_acc = power_of_ten * front_acc + i as u64 * factor_ones;
                    remaining -= extra_len;
                    danger_mask &= danger_mask << extra_len;

                    if remaining == 0 {
                        break 'outer;
                    }

                    mask2 = !(u64::MAX >> m2.leading_zeros());

                    // TODO: Exit and go to loop for only m2?
                    // continue;
                }
            }
        }

        // println!(
        //     "i={i}, front_acc={front_acc}, front_len={front_len}, tail_acc={tail_acc}, tail_len={tail_len}, remaining={remaining}"
        // );

        let line_tot = tail_acc + *POWERS_OR_TEN.get_unchecked(tail_len) * front_acc;
        tot += line_tot;

        // fn solve(line: &[u8], n: usize) -> usize {
        //     let mut stack = Vec::with_capacity(n);

        //     for (i, &b) in line.iter().enumerate() {
        //         let remaining = line.len() - i - 1;
        //         while let Some(&last) = stack.last()
        //             && last < b
        //             && stack.len() + remaining >= n
        //         {
        //             stack.pop();
        //         }
        //         if stack.len() < n {
        //             stack.push(b);
        //         }
        //     }

        //     stack.into_iter().fold(0, |acc, d| 10 * acc + d as usize)
        // }
        // let expected = solve(&ptr.cast::<[u8; 100]>().read().map(|b| b - b'0'), 12) as u64;
        // assert_eq!(
        //     expected,
        //     line_tot,
        //     "{:?}",
        //     std::str::from_utf8(&ptr.cast::<[u8; 100]>().read())
        // );

        ptr = ptr.add(101);
    }

    tot as i64
}
