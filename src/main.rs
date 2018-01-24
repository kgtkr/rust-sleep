use std::env;
use std::thread;
use std::time::{Duration, Instant};

fn main() {
    //引数処理
    let ms = env::args().nth(1).unwrap().parse::<u64>().unwrap();

    let start = Instant::now();

    thread::sleep(Duration::from_millis(ms));

    let end = start.elapsed();
    let time = end.as_secs() * 1000 + (end.subsec_nanos() as u64) / 1_000_000;

    println!("理想 {}ms", ms);
    println!("現実 {}ms", time);
}
