// Port of a subset of: http://www.pcg-random.org/
// http://www.pcg-random.org/
// pcg64_srandom_r
// pcg64_random_r
// pcg32_srandom_r
// pcg32_random_r

#![allow(dead_code)]

use std::num::Wrapping;


#[derive(Copy, Clone)] 
pub struct PCG32 {
    pub state: u64,
    pub seq: u64,
}

const PCG_DEFAULT_MULTIPLIER_64: u64 = 6364136223846793005;

/** Multi-step advance functions (jump-ahead, jump-back).
 *
 * The method used here is based on Brown, "Random Number Generation
 * with Arbitrary Stride,", Transactions of the American Nuclear
 * Society (Nov. 1994).  The algorithm is very similar to fast
 * exponentiation.
 *
 * Even though delta is an unsigned integer, we can pass a
 * signed integer to go backwards, it just goes "the long way round".
 */
fn pcg_advance_lcg_64( state: u64, delta: u64, mult: u64, plus: u64) -> u64 {
    let mut acc_mult = Wrapping(1u64);
    let mut acc_plus = Wrapping(0u64);
    let mut cur_mult = Wrapping(mult);
    let mut cur_plus = Wrapping(plus);
    let mut cur_delta = Wrapping(delta);
    while cur_delta > Wrapping(0) {
        if (cur_delta & Wrapping(1)) != Wrapping(0) {
            acc_mult *= cur_mult;
            acc_plus = acc_plus * cur_mult + cur_plus;
        }
        cur_plus = (cur_mult + Wrapping(1)) * cur_plus;
        cur_mult *= cur_mult;
        cur_delta /= Wrapping(2);
    }
    (acc_mult * Wrapping(state) + acc_plus).0
}

impl PCG32 {

    pub fn seed(init_state: u64, init_seq: u64) -> PCG32 {
        let mut rng = PCG32{ state: 0, seq: 0 };
        rng.reseed( init_state, init_seq );
        rng
    }
    
    pub fn reseed(&mut self, init_state: u64, init_seq: u64) {
        self.state = 0;
        self.seq = (init_seq << 1) | 1;
        self.step();
        self.state = (Wrapping(self.state) + Wrapping(init_state)).0;
        self.step();
    }
    
    pub fn next_u32(&mut self) -> u32 {
        // based on pcg_setseq_64_xsh_rr_32_random_r()
        let state = self.state;
        self.step();
        ((((state >> 18) ^ state) >> 27) as u32).rotate_right( (state >> 59) as u32 )
    }
    
    pub fn next_u64(&mut self) -> u64 {
        ((self.next_u32() as u64) << 32) | (self.next_u32() as u64)
    }
    
    pub fn next_f64(&mut self) -> f64 {
        // TODO compare performance with bitshift / masking technics
        const MAX_VALUE_PLUS_1_INVERSE: f64 = 1.08420217248550443400745280086994171142578125E-19; // 1.0/(i64::MAX+1); it is the inverse of a power of 2
        (self.next_u64() as i64 & i64::max_value()) as f64 * MAX_VALUE_PLUS_1_INVERSE
    }

    fn step(&mut self) {
        self.state = (Wrapping(self.state) * Wrapping(PCG_DEFAULT_MULTIPLIER_64) + Wrapping(self.seq)).0;
    }
    
    pub fn i32_in_range(&mut self, low: i32, high: i32) -> i32 {
        let Wrapping(range) = Wrapping(high as u32) - Wrapping(low as u32);
        let unsigned_max: u32 = ::std::u32::MAX; 
        // zone is the largest multiple of `range` that fits into u32
        // if we've sampled `n` uniformly from this region, then `n % range` is
        // uniform in [0, range).
        let zone = unsigned_max - unsigned_max % range;        
        loop {
            let v = self.next_u32();
            // Discard any random sample which is not within [0, zone).
            // As samples are (should be) independent, discarding does not skew the overall distribution.
            // If this was not done then the distribution would be skewed. For example,
            // if number if range [0,200) were desired with an 8 bits RNG, then returning
            // next_i32() % 200 would skew distribution of [0,56).
            if v < zone {
                return (Wrapping(low as u32) + Wrapping(v % range)).0 as i32;
            }
        }
    }
    
    pub fn advance( &mut self, delta: i64) {
        self.state = pcg_advance_lcg_64( self.state, delta as u64, PCG_DEFAULT_MULTIPLIER_64, self.seq);
    }

}
