use paste::paste;

#[derive(Debug)]
pub struct Pair {
    v1: i32,
    v2: i32,
}

// NOTE: A silly example :)
macro_rules! pair {
    (_MAX) => {2000};
    (_MIN) => {-2000};
    (_Z) => {0};

    (Z) => {Pair{ v1:pair!(_Z), v2:pair!(_Z) }};
    (R) => {Pair { v1:pair!(_MAX), v2:pair!(_Z) }};
    (TR) => {Pair { v1:pair!(_MAX), v2:pair!(_MAX) }};

    ($p:ident) => { Pair { v1:$p.v2, v2:$p.v1}};
    ($($c:tt)+) => {[
        $(pair!($c)),+
    ]}
}

macro_rules! board_zip {
    // This does not work as the input stream is processed in "zip" fashion as noted below
    ( ($($cols:ident $colnos:literal),*), ($($rows:literal),*)) => {
        $(
            paste! {
                #[allow(dead_code)]
                const [< $cols $rows >]: (usize,) = ($colnos * 10 + $rows - 1,);
            }
        )*
    };
}

macro_rules! board {
    ( ($($cols:ident $colnos:literal),*), $rows:tt ) => {
        $( column!($cols, $colnos, $rows); )*
    };
}

/// Helper for board!
macro_rules! column {
    ( $col:ident, $colno:literal, ($($rows:literal),*) ) => {
        $(
            paste! {
                // [< >] are special brackets that tell the `paste!` macro to
                // paste together all the pieces appearing within them into
                // a single identifier.
                #[allow(dead_code)]
                const [< $col $rows >]: (usize, usize) = ($colno, $rows - 1);
            }
        )*
    };
}

macro_rules! dbg_with_name {
    ($c:ident) => {
        paste! {
            println!("{} = {:?}", stringify!($c), $c);
        }
    };

    [$($id:ident),*] => {
        $( { dbg_with_name!($id); })*
    };
}

// NOTE: Can't evaluate expressions in a macro

macro_rules! fib {
    (0) => {
        1
    };
    (1) => {
        1
    };

    ($id:ident) => {
        paste! {
            if $id == 0 { 1 }
            else if $id == 1 { 1 }
            else {
                // NOTE: Can't do this
                fib!($id - 1) + fib!($id - 2)
            }
        }
    };
}

// ============================================================================

macro_rules! csvs_len {
    () => (0);
    ($head:expr) => (1);
    ($head:expr, $($tail:expr),*) => (1 + csvs_len!($($tail),*));
}

macro_rules! measure_this_csv {
    ( $name:ident = [ $($inits:literal),+ ]) => {
        println!("{} = [ {} ]; len({}) = {}", stringify!($name), stringify!($($inits),+), stringify!($name), csvs_len!($($inits),+) );
    };
}

// Recurrence macro - creates an iterator from a recurrence defintion.

macro_rules! recurrence {
    ( $seq:ident [ $ind:ident ]: $sty:ty = $($init_csvs:expr),+ => $recur:expr ) => {
        {
            use std::ops::Index;

            const MEM_SIZE: usize = csvs_len!($($init_csvs),+);

            struct Recurrence {
                mem: [$sty; MEM_SIZE],
                pos: usize,
            }

            // Create a way of allowing the supplied $recur to be evaluated
            // for Recurrence above - which has only MEM_SIZE elements.

            // IndexOffset and its Index impl allows indexing to apply only
            // to the last MEM_SIZE elements (which should be all that are specified)
            // FIXME: NOTE: This is not enforced at compile time!
            struct IndexOffset<'a> {
                slice: &'a [$sty; MEM_SIZE],
                offset: usize,
            }

            impl<'a> Index<usize> for IndexOffset<'a> {
                type Output = $sty;

                #[inline(always)]
                fn index<'b>(&'b self, index: usize) -> &'b $sty {
                    use std::num::Wrapping;

                    let index = Wrapping(index);
                    let offset = Wrapping(self.offset);
                    let window = Wrapping(MEM_SIZE);

                    let real_index = index - offset + window;
                    // NOTE: This could be an out of range access and fail at runtime
                    &self.slice[real_index.0]
                }
            }

            impl Iterator for Recurrence {
                type Item = $sty;

                #[inline]
                fn next(&mut self) -> Option<$sty> {
                    if self.pos < MEM_SIZE {
                        let next_val = self.mem[self.pos];
                        self.pos += 1;
                        Some(next_val)
                    } else {
                        let next_val = {
                            let $ind = self.pos;
                            let $seq = IndexOffset { slice: &self.mem, offset: $ind };
                            $recur
                        };

                        {
                            for i in 0..MEM_SIZE-1 {
                                self.mem[i] = self.mem[i+1];
                            }
                            self.mem[MEM_SIZE-1] = next_val;
                        }

                        self.pos += 1;
                        Some(next_val)
                    }
                }
            }

            Recurrence { mem: [$($init_csvs),+], pos: 0 }
        }
    };
}

// ============================================================================

// board!((A 0, B 1, C 2), (5, 6, 7));

// board_zip!((J 0, K 1, L 2), (1, 2, 3));

fn main() {
    let p = pair!(R);
    let p2 = pair!(p);

    println!("p: {:?}, p2: {:?}\n", p, p2);

    // dbg_with_name![A5, A6, A7, B5, B6, B7, C5, C6, C7];
    // dbg_with_name![J1, K2, L3];
    // dbg_with_name![p, p2];

    measure_this_csv!(a = [1, 1, 1]);
    measure_this_csv!(a = [1, 1, 1, 0, 0, 0]);

    let fib = recurrence!(a[n]: u64 = 0, 1 => a[n - 2] + a[n - 1]);

    let mut n: usize = 20;

    let args = std::env::args().skip(1).collect::<Vec<_>>();

    if args.len() == 1 {
        n = args[0].parse().unwrap_or(n);
    }

    println!("\n\nfib.take({})", n);

    for e in fib.take(n).enumerate() {
        println!("{:^4}: {:}  -- {:?}", e.0, e.1, e);
    }
}
