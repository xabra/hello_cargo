use chrono::prelude::*;

fn main() {
    let utc: DateTime<Utc> = Utc::now(); // e.g. `2014-11-28T12:45:59.324310806Z`
    println!("Time = {utc}\n");

    let t2 = utc - chrono::Duration::seconds(3600);
    println!("Time-3600s = {t2}\n");

    let time_from_timestamp = Utc.timestamp(1642547760, 0);
    println!("Time from timestamp = {time_from_timestamp}\n");

    let stamp = utc.timestamp();
    println!("Timestamp seconds = {stamp}");

    let n = 5;
    for i in -40..40 {
        println!(
            "i = {}, i mod n = {}, circ = {}",
            i,
            i % n,
            circular_index(i, n)
        );
    }
}

fn circular_index(i: i32, n: i32) -> i32 {
    let mut j = i % n;
    if j < 0 {
        j += n
    };
    j
}
