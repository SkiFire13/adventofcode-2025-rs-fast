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
    let mut ptr = input.as_ptr();
    let end_ptr = ptr.add(input.len());

    let mut tot = 0;

    loop {
        let block = ptr.add(1).cast::<u8x16>().read_unaligned();
        let lights = block.simd_eq(u8x16::splat(b'#')).to_bitmask() as u16;
        let len = block
            .simd_eq(u8x16::splat(b']'))
            .to_bitmask()
            .trailing_zeros();
        ptr = ptr.add(1 + len as usize + 1);

        ptr = ptr.add(1);

        let mut buttons = [0; 16];
        let mut buttons_len = 16;
        for i in 0..16 {
            if *ptr != b'(' {
                buttons_len = i;
                break;
            }

            ptr = ptr.add(1);

            let mut button = 0u16;
            for _ in 0..10 {
                if *ptr == b' ' {
                    break;
                }

                button |= 1 << (*ptr - b'0');
                ptr = ptr.add(2);
            }

            buttons[i] = button;
            ptr = ptr.add(1);
        }

        let mut queue = MaybeUninit::<[u16; 1 << 16]>::uninit();
        let queue = queue.as_mut_ptr().as_mut_ptr();
        queue.write(lights);

        let mut steps = u32::MAX;
        let mut steps_acc = u16x16::splat(u16::MAX);

        'lop: {
            macro_rules! loop_body {
                ($i:expr) => {{
                    let i = $i;

                    if i >= buttons_len {
                        break 'lop;
                    }

                    if i < 4 {
                        for j in 0..1 << i {
                            let k = (1 << i) + j;
                            let new_lights = *queue.add(j) ^ buttons[i];
                            if new_lights == 0 {
                                steps = steps.min(k.count_ones());
                            }
                            queue.add(k).write(new_lights);
                        }
                    } else {
                        for j in 0..(1 << i) / 16 {
                            let k = (1 << i) + 16 * j;
                            let lights = queue.add(16 * j).cast::<u16x16>().read_unaligned();
                            let new_lights = lights ^ u16x16::splat(buttons[i]);
                            let mask = new_lights.simd_eq(u16x16::splat(0));
                            if mask.any() {
                                let indexes = u16x16::splat(k as u16)
                                    + u16x16::from_array([
                                        0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15,
                                    ]);
                                steps_acc = steps_acc.simd_min(
                                    mask.select(indexes, u16x16::splat(u16::MAX)).count_ones(),
                                );
                            }
                            queue.add(k).cast::<u16x16>().write_unaligned(new_lights);
                        }
                    }
                }};
            }

            loop_body!(0);
            loop_body!(1);
            loop_body!(2);
            loop_body!(3);
            loop_body!(4);
            loop_body!(5);
            loop_body!(6);
            loop_body!(7);
            loop_body!(8);
            loop_body!(9);
            loop_body!(10);
            loop_body!(11);
            loop_body!(12);
            loop_body!(13);
            loop_body!(14);
            loop_body!(15);
        }

        tot += steps.min(steps_acc.reduce_min() as u32);

        if ptr >= end_ptr.sub(40) {
            break;
        }

        let block = ptr.cast::<u8x64>().read_unaligned();
        let offset = block
            .simd_eq(u8x64::splat(b'\n'))
            .to_bitmask()
            .trailing_zeros() as usize;
        ptr = ptr.add(offset + 1);
    }

    tot as i64
}

unsafe fn part2_inner(input: &str) -> i64 {
    use rayon::prelude::*;
    input
        .par_lines()
        .map(|line| {
            let (lights, rest) = line[1..].split_once("] (").unwrap();
            let lights = lights.chars().map(|c| c == '#').collect::<Vec<_>>();

            let (buttons, rest) = rest.split_once(") {").unwrap();
            let buttons = buttons
                .split(") (")
                .map(|bs| bs.split(',').map(|b| b.parse::<usize>().unwrap()).collect())
                .collect::<Vec<Vec<_>>>();

            let joltages = rest[..rest.len() - 1]
                .split(',')
                .map(|j| j.parse::<i32>().unwrap())
                .collect::<Vec<_>>();

            (lights, buttons, joltages)
        })
        .map(|(_, buttons, joltages)| {
            let max_value = joltages.iter().copied().max().unwrap() as i32;

            // Create the matrix for the system of linear equations
            let cols = buttons.len() + 1;
            let rows = joltages.len();
            let mut matrix = vec![0; cols * rows];
            for (i, button) in buttons.iter().enumerate() {
                for &b in button {
                    matrix[b * cols + i] = 1;
                }
            }
            for (i, &j) in joltages.iter().enumerate() {
                matrix[i * cols + (cols - 1)] = j as i32;
            }

            // Put the matrix into integer RREF
            let mut pivot = 0;
            for c in 0..cols - 1 {
                let Some(row) = (pivot..rows).find(|&row| matrix[row * cols + c] != 0) else {
                    continue;
                };

                if pivot != row {
                    (0..cols).for_each(|c| matrix.swap(pivot * cols + c, row * cols + c));
                }
                if pivot != c {
                    (0..rows).for_each(|r| matrix.swap(r * cols + c, r * cols + pivot));
                }
                let pivot_val = matrix[pivot * cols + pivot];

                for r in 0..rows {
                    if r == pivot {
                        continue;
                    }
                    let factor = matrix[r * cols + pivot];
                    if factor != 0 {
                        for k in 0..cols {
                            matrix[r * cols + k] *= pivot_val;
                            matrix[r * cols + k] -= matrix[pivot * cols + k] * factor;
                        }
                    }
                }

                pivot += 1;
                if pivot >= rows {
                    break;
                }
            }

            // Find the free variables.
            let mut vars = Vec::new();
            'c: for _ in pivot..cols - 1 {
                // Prefer variables in equations with no other unchosen free variables.
                // These equations allows us to get some nice bounds on the free variable later on.
                'r: for r in 0..rows {
                    let mut k = 0;
                    for c in pivot..cols - 1 {
                        if matrix[r * cols + c] != 0 && !vars.contains(&c) {
                            if k != 0 {
                                continue 'r;
                            }
                            k = c;
                        }
                    }
                    if k != 0 {
                        vars.push(k);
                        continue 'c;
                    }
                }

                // If the above fails, pick a variable present in the most equations
                // to increase the chance that this allows another free variable to be chosen
                // with the first way.
                let mut best_c = pivot;
                let mut best_count = 0;
                for c in pivot..cols - 1 {
                    if !vars.contains(&c) {
                        let mut count = 0;
                        for r in 0..rows {
                            if matrix[r * cols + c] != 0 {
                                count += 1;
                            }
                        }
                        if count > best_count {
                            best_c = c;
                            best_count = count;
                        }
                    }
                }
                vars.push(best_c);
            }

            // Solve recursively for all free variables.
            fn solve_rec(
                vars: &[usize],
                values: &mut [i32],
                matrix: &[i32],
                cols: usize,
                rows: usize,
                mut best: Result<i32, ()>,
                max_value: i32,
            ) -> Result<i32, ()> {
                // If we have no more free variables compute the total.
                if vars.len() == 0 {
                    let mut tot = values.iter().sum();
                    for r in 0..cols - 1 - values.len() {
                        let mut sum = matrix[r * cols + (cols - 1)];
                        for i in 0..values.len() {
                            let c = cols - 1 - values.len() + i;
                            sum -= matrix[r * cols + c] * values[i];
                        }
                        if sum % matrix[r * cols + r] != 0 {
                            return best;
                        }
                        sum /= matrix[r * cols + r];
                        if sum < 0 {
                            return best;
                        }
                        tot += sum;
                    }
                    return best.min(Ok(tot));
                }

                let x = vars[0];

                let mut min = 0;
                let mut max = max_value;

                // Go through each equation where this appears as the only free variable
                // and use that to get a bound for it.
                'r: for r in 0..rows {
                    if matrix[r * cols + x] == 0 {
                        continue;
                    }

                    let mut n = matrix[r * cols + r];
                    let m = matrix[r * cols + x];
                    let mut rhs = matrix[r * cols + (cols - 1)];
                    for i in 0..values.len() {
                        let c = cols - 1 - values.len() + i;
                        if c != x && matrix[r * cols + c] != 0 {
                            if vars.contains(&c) {
                                if (matrix[r * cols + c] > 0) == (n > 0) {
                                    n += matrix[r * cols + c];
                                } else {
                                    continue 'r;
                                }
                            } else {
                                rhs -= matrix[r * cols + c] * values[i];
                            }
                        }
                    }

                    // Require the corresponding pivot variable to be positive or zero.
                    if (n > 0) ^ (m > 0) {
                        min = min.max(rhs / m);
                        max = max.min((rhs - max_value * n) / m);
                    } else {
                        max = max.min(rhs / m);
                        min = min.max((rhs - max_value * n + (m - 1)) / m);
                    }
                }

                // Go through all possible values for this variable.
                let mut v = min;
                while v <= max {
                    values[x - (cols - 1 - values.len())] = v;
                    best = solve_rec(&vars[1..], values, matrix, cols, rows, best, max_value);
                    if let Ok(best) = best {
                        max = max.min(best);
                    }
                    v += 1;
                }

                best
            }

            solve_rec(
                &vars,
                &mut vec![0; vars.len()],
                &matrix,
                cols,
                rows,
                Err(()),
                max_value,
            )
            .unwrap()
        })
        .sum::<i32>() as i64
}
