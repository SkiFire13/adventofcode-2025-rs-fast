#![feature(portable_simd)]
#![feature(slice_ptr_get)]
#![feature(array_ptr_get)]
#![feature(likely_unlikely)]

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
unsafe fn parse_node(n: u32) -> u32 {
    #[cfg(target_arch = "x86_64")]
    {
        core::arch::x86_64::_pext_u32(n, 0b00000000000111110001111100011111)
    }

    #[cfg(not(target_arch = "x86_64"))]
    {
        let a = n & 0b11111;
        let b = (n >> 3) & 0b1111100000;
        let c = (n >> 6) & 0b111110000000000;
        a | b | c
    }
}

unsafe fn part1_inner(input: &str) -> i64 {
    static mut NODES: [i32; 32 * 32 * 27] = [0; 32 * 32 * 27];
    let nodes = (&raw mut NODES).as_mut_ptr();

    let ptr = input.as_ptr();
    let mut offset = 0;

    let mut block = ptr.add(offset).cast::<u8x64>().read_unaligned();
    loop {
        let mut mask = std::hint::black_box(block.simd_eq(u8x64::splat(b':')).to_bitmask());

        while mask != 0 {
            let offset = offset + mask.trailing_zeros() as usize;
            mask &= mask - 1;
            let node = parse_node(ptr.add(offset - 3).cast::<u32>().read_unaligned());
            nodes.add(node as usize).write(offset as i32);
        }

        offset += 64;

        if offset < input.len() & !63 {
            block = ptr.add(offset).cast::<u8x64>().read_unaligned();
            continue;
        } else if offset < input.len() {
            let len = input.len() - offset;
            let enable = mask8x64::from_bitmask((1 << len) - 1);
            block = u8x64::load_select_ptr(ptr.add(offset), enable, u8x64::splat(0));
            continue;
        } else {
            break;
        }
    }

    let you = parse_node(u32::from_ne_bytes(*b"you "));
    let out = parse_node(u32::from_ne_bytes(*b"out "));

    *nodes.add(out as usize) = -1;

    #[cfg(target_arch = "x86_64")]
    {
        let sum: i32;
        core::arch::asm!(
            "mov {rsp}, rsp",
            "call 21f",
            "mov rsp, {rsp}",
            "jmp 29f",

            "21:",
            "mov {idx:e}, dword ptr [{nodes} + 4 * {n}]",
            "mov {sum:e}, 0",

            "22:",
            "mov {m:e}, dword ptr [{input} + {idx} + 2]",
            "pext {m:e}, {m:e}, {bitmask:e}",
            "mov {ret_sum:e}, dword ptr [{nodes} + 4 * {m}]",
            "test {ret_sum:e}, {ret_sum:e}",
            "jg 24f",
            "25:",
            "add {sum:e}, {ret_sum:e}",
            "lea {idx}, [{idx} + 4]",
            "cmp byte ptr [{input} + {idx} + 1], {newline}",
            "jne 22b",
            "jmp 26f",

            "24:",
            "push {n}",
            "push {idx}",
            "push {sum:r}",
            "mov {n}, {m}",
            "call 21b",
            "mov {ret_sum:e}, {sum:e}",
            "pop {sum:r}",
            "pop {idx}",
            "pop {n}",
            "jmp 25b",

            "26:",
            "mov dword ptr [{nodes} + 4 * {n}], {sum:e}",
            "ret",

            "29:",

            nodes = in(reg) nodes,
            input = in(reg) ptr,

            n = in(reg) you as usize,
            m = out(reg) _,
            idx = out(reg) _,
            sum = out(reg) sum,
            ret_sum = out(reg) _,

            rsp = out(reg) _,

            bitmask = in(reg) 0b00000000000111110001111100011111u32,
            newline = const b'\n',
        );
        -sum as i64
    }

    #[cfg(not(target_arch = "x86_64"))]
    {
        unsafe fn solve_rec(nodes: *mut i32, input: *const u8, n: u32) -> i32 {
            let mut idx = *nodes.add(n as usize) as usize;

            let mut sum = 0;

            loop {
                let m = parse_node(input.add(idx + 2).cast::<u32>().read_unaligned());

                if *nodes.add(m as usize) <= 0 {
                    sum += *nodes.add(m as usize);
                } else {
                    sum += solve_rec(nodes, input, m);
                }

                idx += 4;

                if *input.add(idx + 1) == b'\n' {
                    break;
                }
            }

            *nodes.add(n as usize) = sum;
            sum
        }
        -solve_rec(nodes, ptr, you) as i64
    }
}

unsafe fn part2_inner(input: &str) -> i64 {
    static mut NODES: [u32; 32 * 32 * 27] = [0; 32 * 32 * 27];
    let nodes = (&raw mut NODES).as_mut_ptr();

    const END_MARKER: u32 = u32::MAX;
    static mut EDGES: [u32; 3000] = [0; 3000];
    let edges = (&raw mut EDGES).as_mut_ptr();
    let mut edges_len = 0;

    edges.add(edges_len).write(END_MARKER);
    edges_len += 1;

    let mut ptr = input.as_ptr();
    let end_ptr = ptr.add(input.len());

    while ptr != end_ptr {
        let start = parse_node(ptr.cast::<u32>().read_unaligned());
        ptr = ptr.add(5);

        *nodes.add(start as usize) = edges_len as _;

        loop {
            let end = parse_node(ptr.cast::<u32>().read_unaligned());
            ptr = ptr.add(4);

            edges.add(edges_len).write(end);
            edges_len += 1;

            if *ptr.sub(1) == b'\n' {
                break;
            }
        }

        edges.add(edges_len).write(END_MARKER);
        edges_len += 1;
    }

    let svr = parse_node(u32::from_ne_bytes(*b"svr "));
    let dac = parse_node(u32::from_ne_bytes(*b"dac "));
    let fft = parse_node(u32::from_ne_bytes(*b"fft "));
    let out = parse_node(u32::from_ne_bytes(*b"out "));

    *nodes.add(out as usize) = 0;

    unsafe fn solve_rec<const MASK: u32>(nodes: *mut u32, edges: *const u32, n: u32) -> u32 {
        let mut idx = *nodes.add(n as usize) as usize;

        let mut sum = 0u32;

        while *edges.add(idx) != END_MARKER {
            let m = *edges.add(idx);

            let cached = *nodes.add(m as usize);

            if cached & MASK != 0 {
                sum = sum.wrapping_add(cached ^ MASK);
            } else if MASK == 1 << 31 || cached & !(MASK - 1) == 0 {
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
