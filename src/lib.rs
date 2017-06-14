// Port of a subset of: http://www.pcg-random.org/
// http://www.pcg-random.org/
// pcg64_srandom_r
// pcg64_random_r
// pcg32_srandom_r
// pcg32_random_r

#![allow(dead_code)]

mod pcgrng {

    use std::num::Wrapping;


    pub struct PCG32 {
        state: u64,
        seq: u64,
    }

    const PCG_MULTIPLIER_64: u64 = 6364136223846793005;

    impl PCG32 {

        pub fn new(init_state: u64, init_seq: u64) -> PCG32 {
            let mut rng = PCG32{ state: 0, seq: (init_seq << 1) | 1 };
            rng.step();
            rng.state = (Wrapping(rng.state) + Wrapping(init_state)).0;
            rng.step();
            rng
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
            self.state = (Wrapping(self.state) * Wrapping(PCG_MULTIPLIER_64) + Wrapping(self.seq)).0;
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
    }

}


#[cfg(test)]
mod tests {
    use std::mem;
    use pcgrng;

    #[test]
    fn vs_pcg() {
        println!("      -  result:      32-bit unsigned int (uint32_t)");
        println!("      -  period:      2^64   (* 2^63 unique stream per rng)");
        println!("      -  state type:  struct PCG32 ({} bytes)", mem::size_of::<pcgrng::PCG32>());
        println!("      -  output func: XSH-RR");
        let mut rng = pcgrng::PCG32::new(42, 0);
        
    }
    
    
    /*
            
#define XX_NUMBITS                  "  32bit:"
#define XX_NUMVALUES                6
#define XX_NUMWRAP                  6
#define XX_PRINT_RNGVAL(value)      printf(" 0x%08x", value)
#define XX_RAND_DECL                struct pcg_state_64 rng;
#define XX_SEEDSDECL(seeds)         uint64_t seeds[1];
#define XX_SRANDOM_SEEDARGS(seeds)  seeds[0]
#define XX_SRANDOM_SEEDCONSTS       42u
#define XX_SRANDOM(...)             \
            pcg_unique_64_srandom_r(&rng, __VA_ARGS__)
#define XX_RANDOM()                 \
            pcg_unique_64_xsh_rr_32_random_r(&rng)
#define XX_BOUNDEDRAND(bound)       \
            pcg_unique_64_xsh_rr_32_boundedrand_r(&rng, bound)
#define XX_ADVANCE(delta)           \
            pcg_unique_64_advance_r(&rng, delta)

int main(int argc, char** argv)
{
    // Read command-line options
     
    int rounds = 5;
    bool nondeterministic_seed = false;
    
    ++argv; --argc;
    if (argc > 0 && strcmp( argv[ 0 ], "--redirect-output" ) == 0) {
        ++argv; --argc;
        if ( argc == 0 )
            return 2;
        const char *path = argv[0];
        ++argv; --argc;
        if ( freopen( path, "wt", stdout ) == NULL )
            return 1;
        if ( freopen( path, "wt", stderr ) == NULL )
            return 1;
    }
    if (argc > 0 && strcmp(argv[0], "-r") == 0) {
         nondeterministic_seed = true;
         ++argv; --argc;
    }
    if (argc > 0) {
         rounds = atoi(argv[0]);
    }
    
    // In this version of the code, we'll use a local rng, rather than the
    // global one.
    
    XX_RAND_DECL
    
    // You should *always* seed the RNG.  The usual time to do it is the
    // point in time when you create RNG (typically at the beginning of the
    // program).
    //
    // XX_SRANDOM_R takes two YY-bit constants (the initial state, and the
    // rng sequence selector; rngs with different sequence selectors will
    // *never* have random sequences that coincide, at all) - the code below 
    // shows three possible ways to do so.

    if (nondeterministic_seed) {
        // Seed with external entropy
        
        XX_SEEDSDECL(seeds)
        entropy_getbytes((void*) seeds, sizeof(seeds)); 
        XX_SRANDOM(XX_SRANDOM_SEEDARGS(seeds));
    } else {
        // Seed with a fixed constant
        
        XX_SRANDOM(XX_SRANDOM_SEEDCONSTS);
    }
    
    printf(XX_INFO);
 
    for (int round = 1; round <= rounds; ++round) {
        printf("Round %d:\n", round);

        /* Make some XX-bit numbers */
        printf(XX_NUMBITS);
        for (int i = 0; i < XX_NUMVALUES; ++i) {
            if (i > 0 && i % XX_NUMWRAP == 0)
               printf("\n\t");
            XX_PRINT_RNGVAL(XX_RANDOM());
        }
        printf("\n");

        printf("  Again:");
        XX_ADVANCE(-XX_NUMVALUES);
        for (int i = 0; i < XX_NUMVALUES; ++i) {
            if (i > 0 && i % XX_NUMWRAP == 0)
               printf("\n\t");
            XX_PRINT_RNGVAL(XX_RANDOM());
        }
        printf("\n");
        

        /* Toss some coins */
        printf("  Coins: ");
        for (int i = 0; i < 65; ++i)
            printf("%c", XX_BOUNDEDRAND(2) ? 'H' : 'T');
        printf("\n");
        
        /* Roll some dice */
        printf("  Rolls:");
        for (int i = 0; i < 33; ++i)
            printf(" %d", (int) XX_BOUNDEDRAND(6)+1);
        printf("\n");
        
        /* Deal some cards */
        enum { SUITS = 4, NUMBERS = 13, CARDS = 52 };
        char cards[CARDS];
        
        for (int i = 0; i < CARDS; ++i)
           cards[i] = i;
        
        for (int i = CARDS; i > 1; --i) {
           int chosen = XX_BOUNDEDRAND(i);
           char card     = cards[chosen];
           cards[chosen] = cards[i-1];
           cards[i-1]  = card;
        }
        
        printf("  Cards:");
        static const char number[] = {'A', '2', '3', '4', '5', '6', '7',
                                      '8', '9', 'T', 'J', 'Q', 'K'};
        static const char suit[] = {'h', 'c', 'd', 's'};
        for (int i = 0; i < CARDS; ++i) {
           printf(" %c%c", number[cards[i] / SUITS], suit[cards[i] % SUITS]);
           if ((i+1) % 22 == 0)
               printf("\n\t");
        }
        printf("\n");
        
        printf("\n");
    }

    return 0;
}             
    
    
    */
}
