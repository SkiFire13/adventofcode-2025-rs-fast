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

    while ptr < end {
        {
            // let mut s = (*ptr - b'0') as u64;
            // ptr = ptr.add(1);

            // let mut e = 0;
            // let mut slen = 1;

            // core::arch::asm!(
            //     "movzx {c}, byte ptr [{ptr}]",
            //     "cmp {c}, {DASH}",
            //     "je 91f",
            //     "lea {s}, [{s} + 4 * {s}]",
            //     "add {ptr}, 1",
            //     "add {slen}, 1",
            //     "lea {s}, [{c} + 2 * {s} - {ZERO}]",

            //     "movzx {c}, byte ptr [{ptr}]",
            //     "cmp {c}, {DASH}",
            //     "je 92f",
            //     "lea {s}, [{s} + 4 * {s}]",
            //     "add {ptr}, 1",
            //     "add {slen}, 1",
            //     "lea {s}, [{c} + 2 * {s} - {ZERO}]",

            //     "movzx {c}, byte ptr [{ptr}]",
            //     "cmp {c}, {DASH}",
            //     "je 93f",
            //     "lea {s}, [{s} + 4 * {s}]",
            //     "add {ptr}, 1",
            //     "add {slen}, 1",
            //     "lea {s}, [{c} + 2 * {s} - {ZERO}]",

            //     "movzx {c}, byte ptr [{ptr}]",
            //     "cmp {c}, {DASH}",
            //     "je 94f",
            //     "lea {s}, [{s} + 4 * {s}]",
            //     "add {ptr}, 1",
            //     "add {slen}, 1",
            //     "lea {s}, [{c} + 2 * {s} - {ZERO}]",

            //     "movzx {c}, byte ptr [{ptr}]",
            //     "cmp {c}, {DASH}",
            //     "je 95f",
            //     "lea {s}, [{s} + 4 * {s}]",
            //     "add {ptr}, 1",
            //     "add {slen}, 1",
            //     "lea {s}, [{c} + 2 * {s} - {ZERO}]",

            //     "movzx {c}, byte ptr [{ptr}]",
            //     "cmp {c}, {DASH}",
            //     "je 96f",
            //     "lea {s}, [{s} + 4 * {s}]",
            //     "add {ptr}, 1",
            //     "add {slen}, 1",
            //     "lea {s}, [{c} + 2 * {s} - {ZERO}]",

            //     "movzx {c}, byte ptr [{ptr}]",
            //     "cmp {c}, {DASH}",
            //     "je 97f",
            //     "lea {s}, [{s} + 4 * {s}]",
            //     "add {ptr}, 1",
            //     "add {slen}, 1",
            //     "lea {s}, [{c} + 2 * {s} - {ZERO}]",

            //     "movzx {c}, byte ptr [{ptr}]",
            //     "cmp {c}, {DASH}",
            //     "je 98f",
            //     "lea {s}, [{s} + 4 * {s}]",
            //     "add {ptr}, 1",
            //     "add {slen}, 1",
            //     "lea {s}, [{c} + 2 * {s} - {ZERO}]",

            //     "movzx {c}, byte ptr [{ptr}]",
            //     "cmp {c}, {DASH}",
            //     "je 99f",
            //     "lea {s}, [{s} + 4 * {s}]",
            //     "add {ptr}, 1",
            //     "add {slen}, 1",
            //     "lea {s}, [{c} + 2 * {s} - {ZERO}]",

            //     "add {ptr}, 1",
            //     "lea {e}, [{e} + 4 * {e}]",
            //     "movzx {c}, byte ptr [{ptr}]",
            //     "lea {e}, [{c} + 2 * {e} - {ZERO}]",

            //     "99:",
            //     "add {ptr}, 1",
            //     "lea {e}, [{e} + 4 * {e}]",
            //     "movzx {c}, byte ptr [{ptr}]",
            //     "lea {e}, [{c} + 2 * {e} - {ZERO}]",

            //     "98:",
            //     "add {ptr}, 1",
            //     "lea {e}, [{e} + 4 * {e}]",
            //     "movzx {c}, byte ptr [{ptr}]",
            //     "lea {e}, [{c} + 2 * {e} - {ZERO}]",

            //     "97:",
            //     "add {ptr}, 1",
            //     "lea {e}, [{e} + 4 * {e}]",
            //     "movzx {c}, byte ptr [{ptr}]",
            //     "lea {e}, [{c} + 2 * {e} - {ZERO}]",

            //     "96:",
            //     "add {ptr}, 1",
            //     "lea {e}, [{e} + 4 * {e}]",
            //     "movzx {c}, byte ptr [{ptr}]",
            //     "lea {e}, [{c} + 2 * {e} - {ZERO}]",

            //     "95:",
            //     "add {ptr}, 1",
            //     "lea {e}, [{e} + 4 * {e}]",
            //     "movzx {c}, byte ptr [{ptr}]",
            //     "lea {e:r}, [{c} + 2 * {e} - {ZERO}]",

            //     "94:",
            //     "add {ptr}, 1",
            //     "lea {e}, [{e} + 4 * {e}]",
            //     "movzx {c}, byte ptr [{ptr}]",
            //     "lea {e}, [{c} + 2 * {e} - {ZERO}]",

            //     "93:",
            //     "add {ptr}, 1",
            //     "lea {e}, [{e} + 4 * {e}]",
            //     "movzx {c}, byte ptr [{ptr}]",
            //     "lea {e}, [{c} + 2 * {e} - {ZERO}]",

            //     "92:",
            //     "add {ptr}, 1",
            //     "lea {e}, [{e} + 4 * {e}]",
            //     "movzx {c}, byte ptr [{ptr}]",
            //     "lea {e}, [{c} + 2 * {e} - {ZERO}]",

            //     "91:",

            //     s = inout(reg) s,
            //     e = inout(reg) e,
            //     slen = inout(reg) slen,
            //     ptr = inout(reg) ptr,
            //     c = out(reg) _,
            //     DASH = const b'-',
            //     ZERO = const b'0',
            // );

            // ptr = ptr.add(1);
            // e = 10 * e + (*ptr - b'0') as u64;

            // ptr = ptr.add(1);
            // let mut elen = slen;
            // if *ptr >= b'0' {
            //     e = 10 * e + (*ptr - b'0') as u64;
            //     ptr = ptr.add(1);
            //     elen += 1;
            // }

            // ptr = ptr.add(1);
        }

        {
            // let mut s = (*ptr - b'0') as u64;
            // ptr = ptr.add(1);

            // let mut e = 0;
            // let mut slen = 1;

            // core::arch::asm!(
            //     "ldrb {c:w}, [{ptr}]",
            //     "cmp {c}, {DASH}",
            //     "b.eq 91f",
            //     "madd {s}, {s}, {TEN}, {c}",
            //     "add {ptr}, {ptr}, 1",
            //     "add {slen}, {slen}, 1",
            //     "sub {s}, {s}, {ZERO}",

            //     "ldrb {c:w}, [{ptr}]",
            //     "cmp {c}, {DASH}",
            //     "b.eq 92f",
            //     "madd {s}, {s}, {TEN}, {c}",
            //     "add {ptr}, {ptr}, 1",
            //     "add {slen}, {slen}, 1",
            //     "sub {s}, {s}, {ZERO}",

            //     "ldrb {c:w}, [{ptr}]",
            //     "cmp {c}, {DASH}",
            //     "b.eq 93f",
            //     "madd {s}, {s}, {TEN}, {c}",
            //     "add {ptr}, {ptr}, 1",
            //     "add {slen}, {slen}, 1",
            //     "sub {s}, {s}, {ZERO}",

            //     "ldrb {c:w}, [{ptr}]",
            //     "cmp {c}, {DASH}",
            //     "b.eq 94f",
            //     "madd {s}, {s}, {TEN}, {c}",
            //     "add {ptr}, {ptr}, 1",
            //     "add {slen}, {slen}, 1",
            //     "sub {s}, {s}, {ZERO}",

            //     "ldrb {c:w}, [{ptr}]",
            //     "cmp {c}, {DASH}",
            //     "b.eq 95f",
            //     "madd {s}, {s}, {TEN}, {c}",
            //     "add {ptr}, {ptr}, 1",
            //     "add {slen}, {slen}, 1",
            //     "sub {s}, {s}, {ZERO}",

            //     "ldrb {c:w}, [{ptr}]",
            //     "cmp {c}, {DASH}",
            //     "b.eq 96f",
            //     "madd {s}, {s}, {TEN}, {c}",
            //     "add {ptr}, {ptr}, 1",
            //     "add {slen}, {slen}, 1",
            //     "sub {s}, {s}, {ZERO}",

            //     "ldrb {c:w}, [{ptr}]",
            //     "cmp {c}, {DASH}",
            //     "b.eq 97f",
            //     "madd {s}, {s}, {TEN}, {c}",
            //     "add {ptr}, {ptr}, 1",
            //     "add {slen}, {slen}, 1",
            //     "sub {s}, {s}, {ZERO}",

            //     "ldrb {c:w}, [{ptr}]",
            //     "cmp {c}, {DASH}",
            //     "b.eq 98f",
            //     "madd {s}, {s}, {TEN}, {c}",
            //     "add {ptr}, {ptr}, 1",
            //     "add {slen}, {slen}, 1",
            //     "sub {s}, {s}, {ZERO}",

            //     "ldrb {c:w}, [{ptr}]",
            //     "cmp {c}, {DASH}",
            //     "b.eq 99f",
            //     "madd {s}, {s}, {TEN}, {c}",
            //     "add {ptr}, {ptr}, 1",
            //     "add {slen}, {slen}, 1",
            //     "sub {s}, {s}, {ZERO}",

            //     "add {ptr}, {ptr}, 1",
            //     "ldrb {c:w}, [{ptr}]",
            //     "madd {e}, {e}, {TEN}, {c}",
            //     "sub {e}, {e}, {ZERO}",

            //     "99:",
            //     "add {ptr}, {ptr}, 1",
            //     "ldrb {c:w}, [{ptr}]",
            //     "madd {e}, {e}, {TEN}, {c}",
            //     "sub {e}, {e}, {ZERO}",

            //     "98:",
            //     "add {ptr}, {ptr}, 1",
            //     "ldrb {c:w}, [{ptr}]",
            //     "madd {e}, {e}, {TEN}, {c}",
            //     "sub {e}, {e}, {ZERO}",

            //     "97:",
            //     "add {ptr}, {ptr}, 1",
            //     "ldrb {c:w}, [{ptr}]",
            //     "madd {e}, {e}, {TEN}, {c}",
            //     "sub {e}, {e}, {ZERO}",

            //     "96:",
            //     "add {ptr}, {ptr}, 1",
            //     "ldrb {c:w}, [{ptr}]",
            //     "madd {e}, {e}, {TEN}, {c}",
            //     "sub {e}, {e}, {ZERO}",

            //     "95:",
            //     "add {ptr}, {ptr}, 1",
            //     "ldrb {c:w}, [{ptr}]",
            //     "madd {e}, {e}, {TEN}, {c}",
            //     "sub {e}, {e}, {ZERO}",

            //     "94:",
            //     "add {ptr}, {ptr}, 1",
            //     "ldrb {c:w}, [{ptr}]",
            //     "madd {e}, {e}, {TEN}, {c}",
            //     "sub {e}, {e}, {ZERO}",

            //     "93:",
            //     "add {ptr}, {ptr}, 1",
            //     "ldrb {c:w}, [{ptr}]",
            //     "madd {e}, {e}, {TEN}, {c}",
            //     "sub {e}, {e}, {ZERO}",

            //     "92:",
            //     "add {ptr}, {ptr}, 1",
            //     "ldrb {c:w}, [{ptr}]",
            //     "madd {e}, {e}, {TEN}, {c}",
            //     "sub {e}, {e}, {ZERO}",

            //     "91:",

            //     s = inout(reg) s,
            //     e = inout(reg) e,
            //     slen = inout(reg) slen,
            //     ptr = inout(reg) ptr,
            //     c = out(reg) _,
            //     TEN = in(reg) 10u64,
            //     DASH = const b'-',
            //     ZERO = const b'0' as u64,
            // );

            // ptr = ptr.add(1);
            // e = 10 * e + (*ptr - b'0') as u64;

            // ptr = ptr.add(1);
            // let mut elen = slen;
            // if *ptr >= b'0' {
            //     e = 10 * e + (*ptr - b'0') as u64;
            //     ptr = ptr.add(1);
            //     elen += 1;
            // }

            // ptr = ptr.add(1);
        }

        {
            // let mut s = (*ptr - b'0') as u64;
            // ptr = ptr.add(1);

            // let mut e = 0;

            // let mut slen = 1;

            // if *ptr != b'-' {
            //     s = 10 * s + (*ptr - b'0') as u64;
            //     ptr = ptr.add(1);
            //     slen += 1;

            //     if *ptr != b'-' {
            //         s = 10 * s + (*ptr - b'0') as u64;
            //         ptr = ptr.add(1);
            //         slen += 1;

            //         if *ptr != b'-' {
            //             s = 10 * s + (*ptr - b'0') as u64;
            //             ptr = ptr.add(1);
            //             slen += 1;

            //             if *ptr != b'-' {
            //                 s = 10 * s + (*ptr - b'0') as u64;
            //                 ptr = ptr.add(1);
            //                 slen += 1;

            //                 if *ptr != b'-' {
            //                     s = 10 * s + (*ptr - b'0') as u64;
            //                     ptr = ptr.add(1);
            //                     slen += 1;

            //                     if *ptr != b'-' {
            //                         s = 10 * s + (*ptr - b'0') as u64;
            //                         ptr = ptr.add(1);
            //                         slen += 1;

            //                         if *ptr != b'-' {
            //                             s = 10 * s + (*ptr - b'0') as u64;
            //                             ptr = ptr.add(1);
            //                             slen += 1;

            //                             if *ptr != b'-' {
            //                                 s = 10 * s + (*ptr - b'0') as u64;
            //                                 ptr = ptr.add(1);
            //                                 slen += 1;

            //                                 if *ptr != b'-' {
            //                                     s = 10 * s + (*ptr - b'0') as u64;
            //                                     ptr = ptr.add(1);
            //                                     slen += 1;

            //                                     ptr = ptr.add(1);
            //                                     e = 10 * e + (*ptr - b'0') as u64;
            //                                 }

            //                                 ptr = ptr.add(1);
            //                                 e = 10 * e + (*ptr - b'0') as u64;
            //                             }

            //                             ptr = ptr.add(1);
            //                             e = 10 * e + (*ptr - b'0') as u64;
            //                         }

            //                         ptr = ptr.add(1);
            //                         e = 10 * e + (*ptr - b'0') as u64;
            //                     }

            //                     ptr = ptr.add(1);
            //                     e = 10 * e + (*ptr - b'0') as u64;
            //                 }

            //                 ptr = ptr.add(1);
            //                 e = 10 * e + (*ptr - b'0') as u64;
            //             }

            //             ptr = ptr.add(1);
            //             e = 10 * e + (*ptr - b'0') as u64;
            //         }

            //         ptr = ptr.add(1);
            //         e = 10 * e + (*ptr - b'0') as u64;
            //     }

            //     ptr = ptr.add(1);
            //     e = 10 * e + (*ptr - b'0') as u64;
            // }

            // ptr = ptr.add(1);
            // e = 10 * e + (*ptr - b'0') as u64;

            // ptr = ptr.add(1);
            // let mut elen = slen;
            // if *ptr >= b'0' {
            //     e = 10 * e + (*ptr - b'0') as u64;
            //     ptr = ptr.add(1);
            //     elen += 1;
            // }

            // ptr = ptr.add(1);

            // println!("s={s}, e={e}, slen={slen}, elen={elen}");
        }

        let mut s = (*ptr - b'0') as u64;
        let mut slen = 1;
        ptr = ptr.add(1);
        while *ptr != b'-' {
            s = 10 * s + (*ptr - b'0') as u64;
            ptr = ptr.add(1);
            slen += 1;
        }

        ptr = ptr.add(1);

        let mut e = (*ptr - b'0') as u64;
        let mut elen = 1;
        ptr = ptr.add(1);
        while *ptr >= b'0' {
            e = 10 * e + (*ptr - b'0') as u64;
            ptr = ptr.add(1);
            elen += 1;
        }

        ptr = ptr.add(1);

        const NEXT: [u64; 11] = [
            11,
            11,
            1010,
            1010,
            100100,
            100100,
            10001000,
            10001000,
            1000010000,
            1000010000,
            100000100000,
        ];

        const PREV: [u64; 11] = [0, 0, 0, 99, 0, 9999, 0, 999999, 0, 99999999, 0];

        if slen % 2 == 1 {
            s = *NEXT.get_unchecked(slen);
            slen += 1;
        }
        if elen % 2 == 1 {
            e = *PREV.get_unchecked(elen);
            elen -= 1;
        }

        if slen > elen {
            continue;
        }

        const DIV: [(u64, u64); 11] = [
            (0, 0),
            (0, 0),
            (10, compute_m_u32(11)),
            (0, 0),
            (100, compute_m_u32(101)),
            (0, 0),
            (1000, compute_m_u32(1001)),
            (0, 0),
            (10000, compute_m_u32(10001)),
            (0, 0),
            (100000, compute_m_u32(100001)),
        ];

        let (d, bs, be) = if slen < 10 {
            let (d, m) = DIV[slen];
            let bs = fastdiv_u32((s + d) as u32, m);
            let be = fastdiv_u32(e as u32, m);
            (d, bs as u64, be as u64)
        } else {
            let (d, m) = const { (100000, compute_m_u64(100001)) };
            let bs = fastdiv_u64(s + d, m);
            let be = fastdiv_u64(e, m);
            (d, bs, be)
        };

        if bs > be {
            continue;
        }

        let k = (d + 1) as u64;
        let b = bs as u64 * k;
        let n = (be - bs + 1) as u64;
        tot += n * b + n * (n - 1) / 2 * k;
    }

    tot as i64
}

unsafe fn part2_inner(input: &str) -> i64 {
    let bytes = input.as_bytes();
    let mut ptr = bytes.as_ptr();
    let end = ptr.add(bytes.len());

    let mut tot = 0;

    let mut buckets = [[(0, 0); 32]; 11];
    let mut buckets_len = [0; 11];

    while ptr < end {
        {
            // let mut s = (*ptr - b'0') as u64;
            // ptr = ptr.add(1);

            // let mut e = 0;
            // let mut slen = 1;

            // core::arch::asm!(
            //     "movzx {c}, byte ptr [{ptr}]",
            //     "cmp {c}, {DASH}",
            //     "je 91f",
            //     "lea {s}, [{s} + 4 * {s}]",
            //     "add {ptr}, 1",
            //     "add {slen}, 1",
            //     "lea {s}, [{c} + 2 * {s} - {ZERO}]",

            //     "movzx {c}, byte ptr [{ptr}]",
            //     "cmp {c}, {DASH}",
            //     "je 92f",
            //     "lea {s}, [{s} + 4 * {s}]",
            //     "add {ptr}, 1",
            //     "add {slen}, 1",
            //     "lea {s}, [{c} + 2 * {s} - {ZERO}]",

            //     "movzx {c}, byte ptr [{ptr}]",
            //     "cmp {c}, {DASH}",
            //     "je 93f",
            //     "lea {s}, [{s} + 4 * {s}]",
            //     "add {ptr}, 1",
            //     "add {slen}, 1",
            //     "lea {s}, [{c} + 2 * {s} - {ZERO}]",

            //     "movzx {c}, byte ptr [{ptr}]",
            //     "cmp {c}, {DASH}",
            //     "je 94f",
            //     "lea {s}, [{s} + 4 * {s}]",
            //     "add {ptr}, 1",
            //     "add {slen}, 1",
            //     "lea {s}, [{c} + 2 * {s} - {ZERO}]",

            //     "movzx {c}, byte ptr [{ptr}]",
            //     "cmp {c}, {DASH}",
            //     "je 95f",
            //     "lea {s}, [{s} + 4 * {s}]",
            //     "add {ptr}, 1",
            //     "add {slen}, 1",
            //     "lea {s}, [{c} + 2 * {s} - {ZERO}]",

            //     "movzx {c}, byte ptr [{ptr}]",
            //     "cmp {c}, {DASH}",
            //     "je 96f",
            //     "lea {s}, [{s} + 4 * {s}]",
            //     "add {ptr}, 1",
            //     "add {slen}, 1",
            //     "lea {s}, [{c} + 2 * {s} - {ZERO}]",

            //     "movzx {c}, byte ptr [{ptr}]",
            //     "cmp {c}, {DASH}",
            //     "je 97f",
            //     "lea {s}, [{s} + 4 * {s}]",
            //     "add {ptr}, 1",
            //     "add {slen}, 1",
            //     "lea {s}, [{c} + 2 * {s} - {ZERO}]",

            //     "movzx {c}, byte ptr [{ptr}]",
            //     "cmp {c}, {DASH}",
            //     "je 98f",
            //     "lea {s}, [{s} + 4 * {s}]",
            //     "add {ptr}, 1",
            //     "add {slen}, 1",
            //     "lea {s}, [{c} + 2 * {s} - {ZERO}]",

            //     "movzx {c}, byte ptr [{ptr}]",
            //     "cmp {c}, {DASH}",
            //     "je 99f",
            //     "lea {s}, [{s} + 4 * {s}]",
            //     "add {ptr}, 1",
            //     "add {slen}, 1",
            //     "lea {s}, [{c} + 2 * {s} - {ZERO}]",

            //     "add {ptr}, 1",
            //     "lea {e}, [{e} + 4 * {e}]",
            //     "movzx {c}, byte ptr [{ptr}]",
            //     "lea {e}, [{c} + 2 * {e} - {ZERO}]",

            //     "99:",
            //     "add {ptr}, 1",
            //     "lea {e}, [{e} + 4 * {e}]",
            //     "movzx {c}, byte ptr [{ptr}]",
            //     "lea {e}, [{c} + 2 * {e} - {ZERO}]",

            //     "98:",
            //     "add {ptr}, 1",
            //     "lea {e}, [{e} + 4 * {e}]",
            //     "movzx {c}, byte ptr [{ptr}]",
            //     "lea {e}, [{c} + 2 * {e} - {ZERO}]",

            //     "97:",
            //     "add {ptr}, 1",
            //     "lea {e}, [{e} + 4 * {e}]",
            //     "movzx {c}, byte ptr [{ptr}]",
            //     "lea {e}, [{c} + 2 * {e} - {ZERO}]",

            //     "96:",
            //     "add {ptr}, 1",
            //     "lea {e}, [{e} + 4 * {e}]",
            //     "movzx {c}, byte ptr [{ptr}]",
            //     "lea {e}, [{c} + 2 * {e} - {ZERO}]",

            //     "95:",
            //     "add {ptr}, 1",
            //     "lea {e}, [{e} + 4 * {e}]",
            //     "movzx {c}, byte ptr [{ptr}]",
            //     "lea {e:r}, [{c} + 2 * {e} - {ZERO}]",

            //     "94:",
            //     "add {ptr}, 1",
            //     "lea {e}, [{e} + 4 * {e}]",
            //     "movzx {c}, byte ptr [{ptr}]",
            //     "lea {e}, [{c} + 2 * {e} - {ZERO}]",

            //     "93:",
            //     "add {ptr}, 1",
            //     "lea {e}, [{e} + 4 * {e}]",
            //     "movzx {c}, byte ptr [{ptr}]",
            //     "lea {e}, [{c} + 2 * {e} - {ZERO}]",

            //     "92:",
            //     "add {ptr}, 1",
            //     "lea {e}, [{e} + 4 * {e}]",
            //     "movzx {c}, byte ptr [{ptr}]",
            //     "lea {e}, [{c} + 2 * {e} - {ZERO}]",

            //     "91:",

            //     s = inout(reg) s,
            //     e = inout(reg) e,
            //     slen = inout(reg) slen,
            //     ptr = inout(reg) ptr,
            //     c = out(reg) _,
            //     DASH = const b'-',
            //     ZERO = const b'0',
            // );

            // ptr = ptr.add(1);
            // e = 10 * e + (*ptr - b'0') as u64;

            // ptr = ptr.add(1);
            // let mut elen = slen;
            // if *ptr >= b'0' {
            //     e = 10 * e + (*ptr - b'0') as u64;
            //     ptr = ptr.add(1);
            //     elen += 1;
            // }

            // ptr = ptr.add(1);
        }

        let mut s = (*ptr - b'0') as u64;
        let mut slen = 1;
        ptr = ptr.add(1);
        while *ptr != b'-' {
            s = 10 * s + (*ptr - b'0') as u64;
            ptr = ptr.add(1);
            slen += 1;
        }

        ptr = ptr.add(1);

        let mut e = (*ptr - b'0') as u64;
        let mut elen = 1;
        ptr = ptr.add(1);
        while *ptr >= b'0' {
            e = 10 * e + (*ptr - b'0') as u64;
            ptr = ptr.add(1);
            elen += 1;
        }

        ptr = ptr.add(1);

        const NEXT: [u64; 10] = [
            1, 10, 100, 1000, 10000, 100000, 1000000, 10000000, 100000000, 1000000000,
        ];

        if slen == elen {
            let len = buckets_len.get_unchecked_mut(slen);
            *buckets.get_unchecked_mut(slen).get_unchecked_mut(*len) = (s, e);
            *len += 1;
        } else {
            let next = *NEXT.get_unchecked(slen);

            let len = buckets_len.get_unchecked_mut(slen);
            *buckets.get_unchecked_mut(slen).get_unchecked_mut(*len) = (s, next - 1);
            *len += 1;

            let len = buckets_len.get_unchecked_mut(slen + 1);
            *buckets.get_unchecked_mut(slen + 1).get_unchecked_mut(*len) = (next, e);
            *len += 1;
        }
    }

    macro_rules! count_invalid {
        ($K:literal, $s:expr, $e:expr) => {{
            let k = $K;
            let bs = ($s + k - 1) / k;
            let be = $e / k;

            if bs > be {
                0
            } else {
                let k = k as u64;
                let b = bs as u64 * k;
                let n = (be - bs + 1) as u64;
                n * b + n * (n - 1) / 2 * k
            }
        }};
    }

    for i in 0..buckets_len[2] {
        let (s, e) = *buckets[2].get_unchecked(i);
        tot += count_invalid!(11, s as u32, e as u32);
    }

    for i in 0..buckets_len[3] {
        let (s, e) = *buckets[3].get_unchecked(i);
        tot += count_invalid!(111, s as u32, e as u32);
    }

    for i in 0..buckets_len[4] {
        let (s, e) = *buckets[4].get_unchecked(i);
        tot += count_invalid!(101, s as u32, e as u32);
    }

    for i in 0..buckets_len[5] {
        let (s, e) = *buckets[5].get_unchecked(i);
        tot += count_invalid!(11111, s as u32, e as u32);
    }

    for i in 0..buckets_len[6] {
        let (s, e) = *buckets[6].get_unchecked(i);
        tot += count_invalid!(10101, s as u32, e as u32) + count_invalid!(1001, s as u32, e as u32)
            - count_invalid!(111111, s as u32, e as u32);
    }

    for i in 0..buckets_len[7] {
        let (s, e) = *buckets[7].get_unchecked(i);
        tot += count_invalid!(1111111, s as u32, e as u32);
    }

    for i in 0..buckets_len[8] {
        let (s, e) = *buckets[8].get_unchecked(i);
        tot += count_invalid!(10001, s as u32, e as u32);
    }

    for i in 0..buckets_len[9] {
        let (s, e) = *buckets[9].get_unchecked(i);
        tot += count_invalid!(1001001, s as u32, e as u32);
    }

    for i in 0..buckets_len[10] {
        let (s, e) = *buckets[10].get_unchecked(i);
        tot += count_invalid!(101010101, s, e) + count_invalid!(100001, s, e)
            - count_invalid!(1111111111, s, e);
    }

    tot as i64
}

use fastdiv::*;
mod fastdiv {
    #[inline]
    const fn mul128_u32(lowbits: u64, d: u32) -> u64 {
        (lowbits as u128 * d as u128 >> 64) as u64
    }
    #[inline]
    const fn mul128_u64(lowbits: u128, d: u64) -> u64 {
        let mut bottom_half = (lowbits & 0xFFFFFFFFFFFFFFFF) * d as u128;
        bottom_half >>= 64;
        let top_half = (lowbits >> 64) * d as u128;
        let both_halves = bottom_half + top_half;
        (both_halves >> 64) as u64
    }

    #[inline]
    pub const fn compute_m_u32(d: u32) -> u64 {
        (0xFFFFFFFFFFFFFFFF / d as u64) + 1
    }
    #[inline]
    pub const fn fastdiv_u32(a: u32, m: u64) -> u32 {
        mul128_u32(m, a) as u32
    }

    #[inline]
    pub const fn compute_m_u64(d: u64) -> u128 {
        (0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF / d as u128) + 1
    }
    #[inline]
    pub const fn fastdiv_u64(a: u64, m: u128) -> u64 {
        mul128_u64(m, a)
    }
}
