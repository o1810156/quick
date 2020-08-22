extern crate rand;
extern crate rand_pcg;
use quick::quick_sort;
use rand::{Rng, SeedableRng};
use rand::distributions::Standard;
use rand_pcg::Pcg64Mcg;
use std::time::Instant;

fn main() {
    let n = 10000000;
    let rng = Pcg64Mcg::from_seed([0; 16]);
    let mut v = rng.sample_iter(&Standard)
        .take(n)
        .collect::<Vec<u32>>();

    let start = Instant::now();

    quick_sort(&mut v);

    let end = start.elapsed();

    println!("time: {}.{:03}", end.as_secs(), end.subsec_nanos() / 1000000);

    // check
    let mut flag = true;
    for i in 0..(n-1) {
        if v[i] > v[i+1] {
            flag = false;
            break;
        }
    }

    if flag {
        println!("ok");
    } else {
        println!("!!Some thing wrong!!");
    }
}
