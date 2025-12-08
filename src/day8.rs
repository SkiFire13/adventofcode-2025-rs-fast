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
    // part2(input)
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
        if *$ptr != $sep {
            n = 10 * n + *$ptr as u32 - b'0' as u32;
            $ptr = $ptr.add(1);
            if *$ptr != $sep {
                n = 10 * n + *$ptr as u32 - b'0' as u32;
                $ptr = $ptr.add(1);
                if *$ptr != $sep {
                    n = 10 * n + *$ptr as u32 - b'0' as u32;
                    $ptr = $ptr.add(1);
                    if *$ptr != $sep {
                        n = 10 * n + *$ptr as u32 - b'0' as u32;
                        $ptr = $ptr.add(1);
                    }
                }
            }
        }
        $ptr = $ptr.add(1);
        n
    }};
}

unsafe fn part1_inner(input: &str) -> i64 {
    let mut ptr = input.as_ptr();
    let end_ptr = ptr.add(input.len());

    let mut counts = [0; 9];
    let mut xs = MaybeUninit::<[[i32; 200]; 7]>::uninit();
    let mut ys = MaybeUninit::<[[i32; 200]; 7]>::uninit();
    let mut zs = MaybeUninit::<[[i32; 200]; 7]>::uninit();

    let xs = xs.as_mut_ptr().as_mut_ptr();
    let ys = ys.as_mut_ptr().as_mut_ptr();
    let zs = zs.as_mut_ptr().as_mut_ptr();

    while ptr != end_ptr {
        let x = parse_n5!(ptr, b',') as i32;
        let y = parse_n5!(ptr, b',') as i32;
        let z = parse_n5!(ptr, b'\n') as i32;

        let bucket = x as usize / 15000;

        let len = *counts.get_unchecked(1 + bucket);
        *counts.get_unchecked_mut(1 + bucket) += 1;

        xs.add(bucket).as_mut_ptr().add(len).write(x);
        ys.add(bucket).as_mut_ptr().add(len).write(y);
        zs.add(bucket).as_mut_ptr().add(len).write(z);
    }

    let mut offsets = [0; 9];
    for i in 0..8 {
        offsets[i + 1] = offsets[i] + counts[i];
    }

    let mut heap = [u64::MAX >> 5; 1000];
    let mut top = heap[0] >> 32;
    let mut top_lane = u64x8::splat(top);

    for b in 1..8 {
        let leni = *counts.get_unchecked(b);
        let xsi = xs.add(b - 1).as_mut_ptr();
        let ysi = ys.add(b - 1).as_mut_ptr();
        let zsi = zs.add(b - 1).as_mut_ptr();

        for i in 0..leni {
            let oi = offsets[b] + i;
            let x1 = i32x8::splat(*xsi.add(i));
            let y1 = i32x8::splat(*ysi.add(i));
            let z1 = i32x8::splat(*zsi.add(i));

            let mut startj = i + 1;
            for bj in b..b + 2 {
                let lenj = *counts.get_unchecked(bj);
                let xsj = xs.wrapping_add(bj.wrapping_sub(1)).as_mut_ptr();
                let ysj = ys.wrapping_add(bj.wrapping_sub(1)).as_mut_ptr();
                let zsj = zs.wrapping_add(bj.wrapping_sub(1)).as_mut_ptr();

                macro_rules! handle_chunk {
                    ($j:expr, $x2:expr, $y2:expr, $z2:expr $(, $mask:expr)?) => {{
                        let dx = (x1 - $x2).cast::<i64>();
                        let dy = (y1 - $y2).cast::<i64>();
                        let dz = (z1 - $z2).cast::<i64>();
                        // let (dx, dy, dz) = ($x2 - x1, $y2 - y1, $z2 - z1);
                        let d = (dx * dx + dy * dy + dz * dz).cast::<u64>();

                        let mut mask = (top_lane.simd_gt(d) $(& $mask)?).to_bitmask();

                        if mask != 0 {
                            let d = *d.as_array();
                            'mask: loop {
                                let pos = mask.trailing_zeros() as usize;
                                mask ^= 1 << pos;

                                let j = $j + pos;
                                let oj = offsets[bj] + j;
                                let d = *d.get_unchecked(pos);

                                if top <= d as u64 {
                                    continue 'mask;
                                }

                                let key = ((d as u64) << 32) | ((oi as u64) << 16) | (oj as u64);

                                let mut heap_idx = 0;
                                'heap: {
                                    for _ in 0..8 {
                                        let mut child_idx = 2 * heap_idx + 1;
                                        child_idx += (*heap.get_unchecked(child_idx + 1)
                                            > *heap.get_unchecked(child_idx))
                                            as usize;

                                        if key >= *heap.get_unchecked(child_idx) {
                                            break 'heap;
                                        }

                                        *heap.get_unchecked_mut(heap_idx) = *heap.get_unchecked(child_idx);
                                        heap_idx = child_idx;
                                    }

                                    let mut child_idx = 2 * heap_idx + 1;
                                    if child_idx >= 1000 {
                                        break 'heap;
                                    }

                                    if child_idx < 999 {
                                        child_idx += (*heap.get_unchecked(child_idx + 1)
                                            > *heap.get_unchecked(child_idx))
                                            as usize;
                                    }

                                    if key >= *heap.get_unchecked(child_idx) {
                                        break 'heap;
                                    }

                                    *heap.get_unchecked_mut(heap_idx) = *heap.get_unchecked(child_idx);
                                    heap_idx = child_idx;
                                }
                                *heap.get_unchecked_mut(heap_idx) = key;
                                top = heap[0] >> 32;
                                top_lane =  u64x8::splat(top);

                                if mask == 0 {
                                    break 'mask;
                                }
                            }
                        }
                    }};
                }

                let mut j = startj;
                while j + 8 <= lenj {
                    let x2 = xsj.add(j).cast::<i32x8>().read_unaligned();
                    let y2 = ysj.add(j).cast::<i32x8>().read_unaligned();
                    let z2 = zsj.add(j).cast::<i32x8>().read_unaligned();
                    handle_chunk!(j, x2, y2, z2);
                    j += 8;
                }
                if j != lenj {
                    let rem = lenj - j;
                    let mask = mask32x8::from_bitmask((1 << rem) - 1);
                    let x2 = i32x8::load_select_ptr(xsj.add(j), mask, Simd::splat(0));
                    let y2 = i32x8::load_select_ptr(ysj.add(j), mask, Simd::splat(0));
                    let z2 = i32x8::load_select_ptr(zsj.add(j), mask, Simd::splat(0));
                    handle_chunk!(j, x2, y2, z2, mask.cast());
                }

                startj = 0;
            }
        }
    }

    let mut uf = [-1; 1024];

    for i in 0..1000 {
        let idxs = *heap.as_ptr().add(i).cast::<u32>();
        let i = idxs >> 16;
        let j = idxs & ((1 << 16) - 1);

        let mut ri = i;
        while *uf.get_unchecked(ri as usize) >= 0 {
            ri = *uf.get_unchecked(ri as usize) as u32;
        }

        let mut rj = j;
        while *uf.get_unchecked(rj as usize) >= 0 {
            rj = *uf.get_unchecked(rj as usize) as u32;
        }

        if ri == rj {
            continue;
        }

        if *uf.get_unchecked(ri as usize) > *uf.get_unchecked(rj as usize) {
            (ri, rj) = (rj, ri);
        }

        *uf.get_unchecked_mut(ri as usize) += *uf.get_unchecked(rj as usize);
        *uf.get_unchecked_mut(rj as usize) = ri as i32;
    }

    let mut b0 = 0;
    let mut b1 = 0;
    let mut b2 = 0;

    for i in 0..1000 {
        let b = -uf[i];
        if b > b2 {
            if b > b1 {
                b2 = b1;
                if b > b0 {
                    b1 = b0;
                    b0 = b;
                } else {
                    b1 = b;
                }
            } else {
                b2 = b;
            }
        }
    }

    (b0 * b1 * b2) as i64
}

unsafe fn part2_inner(input: &str) -> i64 {
    let mut ptr = input.as_ptr();
    let end_ptr = ptr.add(input.len());

    let mut counts = [0; 9];
    let mut xs = MaybeUninit::<[[i32; 200]; 7]>::uninit();
    let mut ys = MaybeUninit::<[[i32; 200]; 7]>::uninit();
    let mut zs = MaybeUninit::<[[i32; 200]; 7]>::uninit();

    let xs = xs.as_mut_ptr().as_mut_ptr();
    let ys = ys.as_mut_ptr().as_mut_ptr();
    let zs = zs.as_mut_ptr().as_mut_ptr();

    while ptr != end_ptr {
        let x = parse_n5!(ptr, b',') as i32;
        let y = parse_n5!(ptr, b',') as i32;
        let z = parse_n5!(ptr, b'\n') as i32;

        let bucket = x as usize / 15000;

        let len = *counts.get_unchecked(1 + bucket);
        *counts.get_unchecked_mut(1 + bucket) += 1;

        xs.add(bucket).as_mut_ptr().add(len).write(x);
        ys.add(bucket).as_mut_ptr().add(len).write(y);
        zs.add(bucket).as_mut_ptr().add(len).write(z);
    }

    let mut min_d = 0;
    let mut min_i = 0;
    let mut min_b = 0;

    for b in 1..8 {
        let leni = *counts.get_unchecked(b);
        let xsi = xs.add(b - 1).as_mut_ptr();
        let ysi = ys.add(b - 1).as_mut_ptr();
        let zsi = zs.add(b - 1).as_mut_ptr();

        for i in 0..leni {
            let x1 = i32x8::splat(*xsi.add(i));
            let y1 = i32x8::splat(*ysi.add(i));
            let z1 = i32x8::splat(*zsi.add(i));
            let mut min_id = u64x8::splat(u64::MAX);

            for bj in b - 1..b + 2 {
                let lenj = *counts.get_unchecked(bj);
                let xsj = xs.wrapping_add(bj.wrapping_sub(1)).as_mut_ptr();
                let ysj = ys.wrapping_add(bj.wrapping_sub(1)).as_mut_ptr();
                let zsj = zs.wrapping_add(bj.wrapping_sub(1)).as_mut_ptr();

                for j in 0..lenj / 8 {
                    let x2 = xsj.cast::<i32x8>().add(j).read_unaligned();
                    let y2 = ysj.cast::<i32x8>().add(j).read_unaligned();
                    let z2 = zsj.cast::<i32x8>().add(j).read_unaligned();

                    let dx = (x1 - x2).cast::<i64>();
                    let dy = (y1 - y2).cast::<i64>();
                    let dz = (z1 - z2).cast::<i64>();

                    let d = (dx * dx + dy * dy + dz * dz).cast::<u64>() - Simd::splat(1);
                    min_id = min_id.simd_min(d);
                }
                if lenj % 8 != 0 {
                    let mask = mask32x8::from_bitmask((1 << (lenj & 7)) - 1);
                    let x2 = i32x8::load_select_ptr(xsj.add(lenj & !7), mask, x1);
                    let y2 = i32x8::load_select_ptr(ysj.add(lenj & !7), mask, y1);
                    let z2 = i32x8::load_select_ptr(zsj.add(lenj & !7), mask, z1);

                    let dx = (x1 - x2).cast::<i64>();
                    let dy = (y1 - y2).cast::<i64>();
                    let dz = (z1 - z2).cast::<i64>();

                    let d = (dx * dx + dy * dy + dz * dz).cast::<u64>() - Simd::splat(1);
                    min_id = min_id.simd_min(d);
                }
            }

            let min_id = min_id.reduce_min();

            if min_id > min_d {
                min_d = min_id;
                min_i = i;
                min_b = b;
            }
        }
    }

    let xsi = xs.add(min_b - 1).as_mut_ptr();
    let ysi = ys.add(min_b - 1).as_mut_ptr();
    let zsi = zs.add(min_b - 1).as_mut_ptr();
    let x1 = i32x8::splat(*xsi.add(min_i));
    let y1 = i32x8::splat(*ysi.add(min_i));
    let z1 = i32x8::splat(*zsi.add(min_i));
    let di = u64x8::splat(min_d.wrapping_add(1));

    for bj in min_b - 1..min_b + 2 {
        let lenj = *counts.get_unchecked(bj);
        let xsj = xs.wrapping_add(bj.wrapping_sub(1)).as_mut_ptr();
        let ysj = ys.wrapping_add(bj.wrapping_sub(1)).as_mut_ptr();
        let zsj = zs.wrapping_add(bj.wrapping_sub(1)).as_mut_ptr();

        for j in 0..lenj / 8 {
            let x2 = xsj.cast::<i32x8>().add(j).read_unaligned();
            let y2 = ysj.cast::<i32x8>().add(j).read_unaligned();
            let z2 = zsj.cast::<i32x8>().add(j).read_unaligned();

            let dx = (x1 - x2).cast::<i64>();
            let dy = (y1 - y2).cast::<i64>();
            let dz = (z1 - z2).cast::<i64>();

            let d = (dx * dx + dy * dy + dz * dz).cast::<u64>();
            let mask = d.simd_eq(di);
            if mask.any() {
                let k = 8 * j + mask.first_set().unwrap_unchecked();
                return (*xsi.add(min_i) as i64) * (*xsj.add(k) as i64);
            }
        }
        if lenj % 8 != 0 {
            let j = lenj / 8;
            let mask = mask32x8::from_bitmask((1 << (lenj & 7)) - 1);
            let x2 = i32x8::load_select_ptr(xsj.add(lenj & !7), mask, x1);
            let y2 = i32x8::load_select_ptr(ysj.add(lenj & !7), mask, y1);
            let z2 = i32x8::load_select_ptr(zsj.add(lenj & !7), mask, z1);

            let dx = (x1 - x2).cast::<i64>();
            let dy = (y1 - y2).cast::<i64>();
            let dz = (z1 - z2).cast::<i64>();

            let d = (dx * dx + dy * dy + dz * dz).cast::<u64>();
            let mask = d.simd_eq(di);
            if mask.any() {
                let k = 8 * j + mask.first_set().unwrap_unchecked();
                dbg!(bj, k);
                return (*xsi.add(min_i) as i64) * (*xsj.add(k) as i64);
            }
        }
    }

    unreachable!()
}
