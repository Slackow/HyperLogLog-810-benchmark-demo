use hyperloglogplus::{HyperLogLog, HyperLogLogPlus};
use num_format::{Locale, ToFormattedString};
use std::env;
use std::hash::RandomState;
use std::io::{self, Write};
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering::Relaxed;
use std::thread::{sleep, spawn};
use std::time::Duration;

/**
 * Inserts numbers from 0..N into a HyperLogLog
 */
fn insert_upto(hll: &mut impl HyperLogLog<u64>, upto: u64) {
    for i in 0..upto {
        hll.insert(&i);
    }
}
/**
 * Calculate and print error for the actual and expected values of a HyperLogLog
 */
fn benchmark(hll: &mut impl HyperLogLog<u64>, upto: u64) {
    let mut s = String::new();
    let count = hll.count();
    let diff = f64::abs(count - (upto as f64));
    let percent = (diff / ((upto / 10_000) as f64)).trunc();
    print!("Expected: {:>12}", upto.to_formatted_string(&Locale::en));
    _ = io::stdout().flush();
    _ = io::stdin().read_line(&mut s);
    println!(
        "Actual:   {:>12}, Error: {:.2}%",
        (count.trunc() as u64).to_formatted_string(&Locale::en),
        percent / 100.
    );
    _ = io::stdin().read_line(&mut s);
}

fn main() {
    let precision = env::args()
        .skip(1)
        .next()
        .and_then(|x| x.parse::<u8>().ok())
        .unwrap_or(16);
    let mut hll = HyperLogLogPlus::<u64, _>::new(precision, RandomState::new()).unwrap();
    println!("\nUsing 2^{precision} ({}) registers\n", 1_u32 << precision);
    insert_upto(&mut hll, 100_000);
    benchmark(&mut hll, 100_000);

    insert_upto(&mut hll, 1_000_000);
    benchmark(&mut hll, 1_000_000);

    println!("Inserting first 100k again (10 times)");
    for _ in 0..10 {
        insert_upto(&mut hll, 100_000);
    }
    benchmark(&mut hll, 1_000_000);

    insert_upto(&mut hll, 10_000_000);
    benchmark(&mut hll, 10_000_000);

    insert_upto(&mut hll, 100_000_000);
    benchmark(&mut hll, 100_000_000);
    {
        // Loading dots
        B.store(true, Relaxed);
        spawn(|| {
            while B.load(Relaxed) {
                print!(".");
                _ = io::stdout().flush();
                sleep(Duration::from_secs(4));
            }
        });
        insert_upto(&mut hll, 10_000_000_000);
        B.store(false, Relaxed);
        println!();
    }
    benchmark(&mut hll, 10_000_000_000);
}

static B: AtomicBool = AtomicBool::new(false);
