

fn main() {
    //let sum: u64 = (0..100_000_000).into_par_iter().sum();
    //println!("Sum: {}", sum);
    let x: u64 = (0..100_000_000).sum();
    println!("Hello, world! {}", x);

}
