extern crate pcgrng;


#[cfg(test)]
mod tests {
    use std::mem;
    use pcgrng::PCG32;

    #[test]
    fn pcg_setseq_64_xsh_rr_32() {
        println!("      -  result:      32-bit unsigned int (uint32_t)");
        println!("      -  period:      2^64   (* 2^63 unique stream per rng)");
        println!("      -  state type:  struct PCG32 ({} bytes)", mem::size_of::<PCG32>());
        println!("      -  output func: XSH-RR");
        let mut rng = PCG32::seed(0, 0);
        let test_script = include_str!("../testdata/pcg_setseq_64_xsh_rr_32.testscript");
        check_script_results( &mut rng, test_script );
    }
    
    #[test]
    fn pcg32_advance_to_repeat_last() {
        let mut rng = PCG32::seed(0, 0);
        let v1 = rng.next_u32();
        rng.advance(-1);
        let v2 = rng.next_u32();
        assert_eq!(v1, v2);
    }
    
    fn check_script_results( rng: &mut PCG32, test_script: &str ) {
        for (line_no, line) in test_script.lines().enumerate() {
            println!("Processing test script line {}", line_no+1);
            let line = line.trim();
            if !line.is_empty() {
                let mut parts = line.split(' ');
                let command = parts.next().unwrap(); // string is not blank so guaranted to have at least one item
                let mut params = Vec::new();
                for part in parts {
                    let value = part.parse::<u64>().expect( format!("Expected an unsigned integer but got {}.", part ).as_str() );
                    params.push( value );
                }
                match command {
                    "seed" => {
                        assert!(params.len() == 2);
                        rng.reseed( params[0], params[1] );
                    },
                    "next" => {
                        assert!(params.len() == 1);
                        println!("state=0x{:016x}", rng.state);
                        let actual = rng.next_u32();
                        let expected = params[0] as u32;
                        assert_eq!( expected, actual );
                    },
                    "next_bounded" => {
                        assert!(params.len() == 2);
                        let (bound, expected) = (params[0] as u32, params[1] as i32);
                        println!("next_bounded {} {}", bound, expected);
                        let actual = rng.i32_in_range(0, bound as i32);
                        assert_eq!( expected, actual );
                    },
                    "advance" => {
                        assert!(params.len() == 1);
                        let delta = params[0] as i64;
                        println!("advance delta=0x{:016x}, state=0x{:016x}", delta, rng.state);
                        rng.advance(delta);
                    }
                    _ => panic!("Unknown command: {}.", command ),
                }
            }
        }
        
    }
}
