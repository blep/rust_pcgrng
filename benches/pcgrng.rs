#[macro_use]
extern crate bencher;
extern crate pcgrng;

use bencher::{black_box, Bencher};
use pcgrng::PCG32;


#[bench]
pub fn pcg32_next_u32(b: &mut Bencher) {
    let rng = PCG32::seed(0,0);
    b.iter(|| {
        black_box(rng.next_u32());
    });
}


//benchmark_group!(benches, pcg32_next_u32);
benchmark_main!(pcg32_next_u32);
