use std::{hint::black_box, sync::OnceLock};

use adventofcode_2025_rs_fast::*;

fn bench<D: std::fmt::Display>(mut f: impl FnMut() -> D) {
    // rayon::broadcast(|_| {});

    const ITERS: u32 = 100_000;
    let n = if cfg!(debug_assertions) { 1 } else { ITERS };
    let now = std::time::Instant::now();
    for _ in 1..n {
        black_box(f());
    }
    let sol = f();
    println!("Took: {:?}", now.elapsed() / n);
    println!("Solution: {sol}");
}

macro_rules! run {
    ($day:ident) => {{
        static INPUT: OnceLock<&'static str> = OnceLock::new();
        let input = INPUT.get_or_init(|| {
            #[cfg(not(miri))]
            {
                std::fs::read_to_string(concat!("input/2025/", stringify!($day), ".txt"))
                    .unwrap()
                    .leak()
            }
            #[cfg(miri)]
            {
                include_str!(concat!("../input/2025/", stringify!($day), ".txt"))
            }
        });

        bench(|| crate::$day::part1(input));
        run!(part2 $day bench(|| crate::$day::part2(input)));
    }};
    (part2 day25 $($rest:tt)*) => {};
    (part2 $day:ident $($rest:tt)*) => { $($rest)* };
}

fn main() {
    run!(day6);
}
