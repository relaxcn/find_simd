#![feature(portable_simd)]
use std::simd::prelude::*;
use std::env;

const NEEDLE_STR: &str = "newsletter";

fn main() {
    let args: Vec<String> = env::args().collect();
    let iter_cnt = 10000;
    // Read file
    let haystack = std::fs::read_to_string("./haystack.txt").expect("Failed to read file");
    let haystack_str = haystack.as_str();
    // Start test
    if args.iter().any(|arg| arg == "--simd") {
        test_simd_v1(NEEDLE_STR, haystack_str, iter_cnt);
    } else {
        test_baseline(NEEDLE_STR, haystack_str, iter_cnt);
    }
}

#[test]
fn test_two_methods() {
    use std::time::Instant;
    fn test(needle_str: &str, haystack_str: &str, num_iterations: u32) {
        // Benchmark find_substr
        let start = Instant::now();
        test_baseline(needle_str, haystack_str, num_iterations);
        let duration = start.elapsed();
        println!("find_substr took: {:?}", duration);
        println!("Average: {:?}\n", duration / num_iterations);

        // Benchmark find_substr_simd
        let start = Instant::now();
        test_simd_v1(needle_str, haystack_str, num_iterations);
        let duration = start.elapsed();
        println!("find_substr_simd took: {:?}", duration);
        println!("Average: {:?}\n", duration / num_iterations);
    }

    // Read large context
    let haystack = std::fs::read_to_string("./haystack.txt").expect("Failed to read file");
    let haystack_str = haystack.as_str();

    println!("-----------------------------------");
    println!("Test large context:");
    test(NEEDLE_STR, haystack_str, 10000);
    println!("-----------------------------------");
    // Read small context
    let haystack = std::fs::read_to_string("./haystack-small.txt").expect("Failed to read file");
    let haystack_str = haystack.as_str();
    println!("Test small context:");
    test(NEEDLE_STR, haystack_str, 10000);
    println!("-----------------------------------");
}

fn test_baseline(needle_str: &str, haystack_str: &str, num_iterations: u32) {
    for _ in 0..num_iterations {
        let _ = find_substr(needle_str, haystack_str);
    }
}

fn test_simd_v1(needle_str: &str, haystack_str: &str, num_iterations: u32) {
    for _ in 0..num_iterations {
        let _ = find_substr_simd(needle_str, haystack_str);
    }
}

// Baseline
fn find_substr(needle_str: &str, haystack_str: &str) -> Option<usize> {
    haystack_str.find(needle_str)
}

// A SIMD implementation
fn find_substr_simd(needle_str: &str, haystack_str: &str) -> Option<usize> {
    let needle_len = needle_str.len();
    let haystack_len = haystack_str.len();

    let needle = needle_str.as_bytes();
    let haystack = haystack_str.as_bytes();

    let first_letter = u8x32::splat(needle[0]);
    let last_letter = u8x32::splat(needle[needle_len - 1]);

    let mut i: usize = 0;
    // Scan 32 bytes each iteration
    while i + needle_len + 32 <= haystack_len {
        let first_block: u8x32 = u8x32::from_slice(&haystack[i..i+32]);
        let second_block: u8x32 = u8x32::from_slice(&haystack[i+needle_len-1..i+needle_len-1+32]);

        // Get mask of the two sub blocks
        let eq_first = first_block.simd_eq(first_letter);
        let eq_second = second_block.simd_eq(last_letter);

        let mask = eq_first & eq_second;
        // Found the last set bit's index position and compare inner bits.
        let mut bitmask = mask.to_bitmask();
        while bitmask != 0 {
            // Count the number of trailing zeros
            let tail_zero_cnt = bitmask.trailing_zeros() as usize;
            let start_pos = i + tail_zero_cnt;
            if &haystack[start_pos..start_pos + needle_len] == needle {
                return Some(start_pos);
            }
            // reset the rightmost `1`
            bitmask &= bitmask - 1;
        }
        i += 32; // move forward 32 bytes
    }
    // Handle the remaining bytes
    if i < haystack_len {
        if let Some(rel_idx) = haystack_str[i..].find(needle_str) {
            return Some(i + rel_idx);
        }
    }
    None
}
