use std::time::{Duration, Instant};

pub fn time_learning() {
    println!("Starts time waiting");
    let start_time = Instant::now();
    let dur1 = Duration::from_secs(150);
    println!("finished waiting for {:?} seconds", dur1.as_millis());
    let dur2 = Duration::from_millis(5500);
    let result = dur2.checked_sub(dur1);

    println!("{:?}", result);
    // let end_time = Instant::now();
    // let time_difference = end_time - start_time;
    println!("Time difference: {:?}", start_time.elapsed());
}
