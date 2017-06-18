#include "pcg_variants.h"
#include <stdio.h>
#include <math.h>

/*
const uint64_t N = 100'000'000'000;
pi    ~= 3.14159183511999984
error ~= 0.00000081846979327
 */


inline double next_double(struct pcg_state_setseq_64 *rng) 
{
    const double MAX_VALUE_PLUS_1_INVERSE = 1.08420217248550443400745280086994171142578125E-19; // 1.0/(i64::MAX+1); it is the inverse of a power of 2
    auto uv64 = ((uint64_t)pcg_setseq_64_xsh_rr_32_random_r(rng)) << 32
                | (uint64_t)pcg_setseq_64_xsh_rr_32_random_r(rng);
    double vf = double(int64_t(uv64 & INT64_MAX));
    return vf * MAX_VALUE_PLUS_1_INVERSE;
}


int main(int argc, const char **argv)
{
    struct pcg_state_setseq_64 rng;

    pcg_setseq_64_srandom_r(&rng, 0, 0);
    const uint64_t N = 1'000'000'000;

    uint64_t nb_inside = 0;
    for ( int64_t i=0; i < N; ++i ) {
        double x = next_double(&rng);
        double y = next_double(&rng);
        double d = x*x + y*y;
        nb_inside += uint64_t(d < 1.0);
    }
    double pi_estimate = double(4 * nb_inside) / double(N);
    double err = fabs(3.141592653589793 - pi_estimate);
    printf( "pi    ~= %.17f\n", pi_estimate);
    printf( "error ~= %.17f\n", err);

    return 0;
}