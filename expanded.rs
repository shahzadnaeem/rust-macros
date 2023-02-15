#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
use paste::paste;
pub struct Pair {
    v1: i32,
    v2: i32,
}
#[automatically_derived]
impl ::core::fmt::Debug for Pair {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field2_finish(
            f,
            "Pair",
            "v1",
            &&self.v1,
            "v2",
            &&self.v2,
        )
    }
}
fn main() {
    let fib = {
        use std::ops::Index;
        const MEM_SIZE: usize = 1 + 1;
        struct Recurrence {
            mem: [u64; MEM_SIZE],
            pos: usize,
        }
        struct IndexOffset<'a> {
            slice: &'a [u64; MEM_SIZE],
            offset: usize,
        }
        impl<'a> Index<usize> for IndexOffset<'a> {
            type Output = u64;
            #[inline(always)]
            fn index<'b>(&'b self, index: usize) -> &'b u64 {
                use std::num::Wrapping;
                let index = Wrapping(index);
                let offset = Wrapping(self.offset);
                let window = Wrapping(MEM_SIZE);
                let real_index = index - offset + window;
                &self.slice[real_index.0]
            }
        }
        impl Iterator for Recurrence {
            type Item = u64;
            #[inline]
            fn next(&mut self) -> Option<u64> {
                if self.pos < MEM_SIZE {
                    let next_val = self.mem[self.pos];
                    self.pos += 1;
                    Some(next_val)
                } else {
                    let next_val = {
                        let n = self.pos;
                        let a = IndexOffset {
                            slice: &self.mem,
                            offset: n,
                        };
                        a[n - 2] + a[n - 1]
                    };
                    {
                        use std::mem::swap;
                        let mut swap_tmp = next_val;
                        for i in (0..MEM_SIZE).rev() {
                            swap(&mut swap_tmp, &mut self.mem[i]);
                        }
                    }
                    self.pos += 1;
                    Some(next_val)
                }
            }
        }
        Recurrence { mem: [1, 1], pos: 0 }
    };
    for e in fib.take(10) {
        {
            ::std::io::_print(format_args!("{0}\n", e));
        }
    }
    {
        ::std::io::_print(format_args!("len({0}) = {1}\n", "a", 1 + (1 + 1)));
    };
}
