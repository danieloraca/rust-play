fn main() {
    // Get the current time
    let start = std::time::Instant::now();

    // Do some work
    // ...

    // Get the elapsed time
    let elapsed = start.elapsed();

    // Convert elapsed time to nanoseconds
    let nanoseconds = elapsed.as_nanos();

    println!("Elapsed time: {} nanoseconds", nanoseconds);
}
