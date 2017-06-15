#include "pcg_variants.h"
#include <stdio.h>

static int32_t steps[] = {
    1,
    5,
    21,
    85,
    341,
    1365,
    5461,
    21845,
    152917,
    677205,
    2774357,
    11162965,
    44717397,
    178935125,
    715806037,
    -715806037,
    -178935125,
    -44717397,
    -11162965,
    -2774357,
    -677205,
    -152917,
    -21845,
    -5461,
    -1365,
    -341,
    -85,
    -21,
    -5,
    -1
};

#define NB_STEP (sizeof(steps) / sizeof(steps[0]))

int main(int argc, const char **argv)
{
    uint32_t value;
    struct pcg_state_setseq_64 rng;
    int i;
    FILE *out = fopen( "pcg_setseq_64_xsh_rr_32.testscript", "wt" );

    fprintf(out, "seed 42 54\n");
    pcg_setseq_64_srandom_r(&rng, 42, 54);
    for ( i=0; i < 100; ++i ) {
        value = pcg_setseq_64_xsh_rr_32_random_r(&rng);
        fprintf( out, "next %u\n", value );
    }

    for ( i=0; i < 100; ++i ) {
        value = pcg_setseq_64_xsh_rr_32_boundedrand_r(&rng, 100);
        fprintf( out, "next_bounded 100 %u\n", value );
    }

    for ( i=0; i < 100; ++i ) {
        value = pcg_setseq_64_xsh_rr_32_boundedrand_r(&rng, 0x40000001u);
        fprintf( out, "next_bounded %u %u\n", 0x40000001u, value );
    }

    for ( i=0; i < NB_STEP; ++i ) {
        pcg_setseq_64_advance_r(&rng, steps[i]);
        fprintf( out, "advance %u\n", steps[i] );
        value = pcg_setseq_64_xsh_rr_32_random_r(&rng);
        fprintf( out, "next %u\n", value );
    }
    fprintf(out, "seed 1001 7654321\n");
    pcg_setseq_64_srandom_r(&rng, 1001, 7654321);
    for ( i=0; i < 100; ++i ) {
        value = pcg_setseq_64_xsh_rr_32_random_r(&rng);
        fprintf( out, "next %u\n", value );
    }
    fclose(out);

    return 0;
}