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

#[inline(always)]
unsafe fn parse_n3_s(ptr: *const u8) -> u32 {
    let n = ptr.cast::<u32>().read_unaligned() - u32::from_ne_bytes(*b"aaa:");

    #[cfg(target_arch = "x86_64")]
    {
        core::arch::x86_64::_pext_u32(n, 0b00000000000111110001111100011111)
    }

    #[cfg(not(target_arch = "x86_64"))]
    {
        let a = (n >> 0) & 0b11111;
        let b = (n >> 3) & 0b1111100000;
        let c = (n >> 6) & 0b111110000000000;
        a | b | c
    }
}

#[inline(always)]
unsafe fn parse_n3(ptr: *const u8) -> u32 {
    let n = ptr.cast::<u32>().read_unaligned() - u32::from_ne_bytes(*b" aaa");

    #[cfg(target_arch = "x86_64")]
    {
        core::arch::x86_64::_pext_u32(n, 0b00011111000111110001111100000000)
    }

    #[cfg(not(target_arch = "x86_64"))]
    {
        let a = (n >> 8) & 0b11111;
        let b = (n >> 11) & 0b1111100000;
        let c = (n >> 14) & 0b111110000000000;
        a | b | c
    }
}

unsafe fn part1_inner(input: &str) -> i64 {
    let mut nodes = MaybeUninit::<[i32; 32 * 32 * 26]>::uninit();
    let nodes = nodes.as_mut_ptr().as_mut_ptr();

    const END_MARKER: u32 = u32::MAX;
    let mut edges = MaybeUninit::<[u32; 3000]>::uninit();
    let edges = edges.as_mut_ptr().as_mut_ptr();
    let mut edges_len = 1;

    let mut ptr = input.as_ptr();
    let end_ptr = ptr.add(input.len());

    while ptr != end_ptr {
        let start = parse_n3_s(ptr);
        ptr = ptr.add(4);

        *nodes.add(start as usize) = edges_len as _;

        loop {
            let end = parse_n3(ptr);
            ptr = ptr.add(4);

            edges.add(edges_len).write(end);
            edges_len += 1;

            if *ptr == b'\n' {
                break;
            }
        }

        edges.add(edges_len).write(END_MARKER);
        edges_len += 1;
        ptr = ptr.add(1);
    }

    unsafe fn solve_rec(nodes: *mut i32, edges: *const u32, n: u32) -> i32 {
        let mut idx = *nodes.add(n as usize) as usize;

        let mut sum = 0;

        loop {
            let m = *edges.add(idx);

            if *nodes.add(m as usize) <= 0 {
                sum += *nodes.add(m as usize);
            } else {
                sum += solve_rec(nodes, edges, m);
            }

            idx += 1;

            if *edges.add(idx) == END_MARKER {
                break;
            }
        }

        *nodes.add(n as usize) = sum;
        sum
    }

    let you = parse_n3(b" you".as_ptr());
    let out = parse_n3(b" out".as_ptr());

    *nodes.add(out as usize) = -1;

    // #[cfg(target_arch = "x86_64")]
    // {
    //     let sum: i64;
    //     core::arch::asm!(
    //         "mov {rsp}, rsp",
    //         "call 21f",
    //         "mov rsp, {rsp}",
    //         "jmp 29f",

    //         "21:",
    //         "movzx {idx}, word ptr [{nodes} + 2 * {n}]",
    //         "mov {sum}, 0",
    //         "movzx {m}, word ptr [{edges} + 2 * {idx}]",

    //         "22:",
    //         "movsx {ret_sum}, word ptr [{nodes} + 2 * {m}]",
    //         "test {ret_sum:x}, {ret_sum:x}",
    //         "jle 23f",

    //         "push {n}",
    //         "push {idx}",
    //         "push {sum}",
    //         "mov {n}, {m}",
    //         "call 21b",
    //         "pop {sum}",
    //         "pop {idx}",
    //         "pop {n}",

    //         "23:",
    //         "add {sum}, {ret_sum}",
    //         "inc {idx}",
    //         "movzx {m}, word ptr [{edges} + 2 * {idx}]",
    //         "cmp {m}, {end_marker}",
    //         "jne 22b",

    //         "mov word ptr [{nodes} + 2 * {n}], {sum:x}",
    //         "mov {ret_sum}, {sum}",
    //         "ret",

    //         "29:",

    //         nodes = in(reg) nodes.as_mut_ptr(),
    //         edges = in(reg) edges,

    //         n = in(reg) you as usize,
    //         m = out(reg) _,
    //         idx = out(reg) _,
    //         sum = out(reg) sum,
    //         ret_sum = out(reg) _,

    //         rsp = out(reg) _,

    //         end_marker = const END_MARKER
    //     );
    //     -sum as i64
    // }

    // #[cfg(not(target_arch = "x86_64"))]
    // {
    //     -solve_rec(nodes.as_mut_ptr(), edges, you) as i64
    // }

    -solve_rec(nodes, edges, you) as i64
}

unsafe fn part2_inner(input: &str) -> i64 {
    let mut nodes = MaybeUninit::<[u32; 32 * 32 * 26]>::uninit();
    let nodes = nodes.as_mut_ptr().as_mut_ptr();

    const END_MARKER: u32 = u32::MAX;
    let mut edges = MaybeUninit::<[u32; 3000]>::uninit();
    let edges = edges.as_mut_ptr().as_mut_ptr();
    let mut edges_len = 0;

    edges.add(edges_len).write(END_MARKER);
    edges_len += 1;

    let mut ptr = input.as_ptr();
    let end_ptr = ptr.add(input.len());

    while ptr != end_ptr {
        let start = parse_n3_s(ptr);
        ptr = ptr.add(4);

        *nodes.add(start as usize) = edges_len as _;

        loop {
            let end = parse_n3(ptr);
            ptr = ptr.add(4);

            edges.add(edges_len).write(end);
            edges_len += 1;

            if *ptr == b'\n' {
                break;
            }
        }

        edges.add(edges_len).write(END_MARKER);
        edges_len += 1;
        ptr = ptr.add(1);
    }

    let svr = parse_n3(b" svr".as_ptr());
    let dac = parse_n3(b" dac".as_ptr());
    let fft = parse_n3(b" fft".as_ptr());
    let out = parse_n3(b" out".as_ptr());

    *nodes.add(out as usize) = 0;

    unsafe fn solve_rec<const MASK: u32>(nodes: *mut u32, edges: *const u32, n: u32) -> u32 {
        let mut idx = *nodes.add(n as usize) as usize;

        let mut sum = 0u32;

        while *edges.add(idx) != END_MARKER {
            let m = *edges.add(idx);

            let cached = *nodes.add(m as usize);

            if cached & MASK != 0 {
                sum = sum.wrapping_add(cached ^ MASK);
            } else if cached & !(MASK - 1) == 0 {
                sum = sum.wrapping_add(solve_rec::<MASK>(nodes, edges, m));
            }

            idx += 1;
        }

        *nodes.add(n as usize) = sum | MASK;
        sum
    }

    *nodes.add(out as usize) = 1 | (1 << 31);
    let dac_to_out = solve_rec::<{ 1 << 31 }>(nodes, edges, dac) as i64;

    *nodes.add(dac as usize) = 1 | (1 << 30);
    let fft_to_dac = solve_rec::<{ 1 << 30 }>(nodes, edges, fft) as i64;

    *nodes.add(fft as usize) = 1 | (1 << 29);
    let svr_to_fft = solve_rec::<{ 1 << 29 }>(nodes, edges, svr) as i64;

    (svr_to_fft * fft_to_dac * dac_to_out) as i64
}
