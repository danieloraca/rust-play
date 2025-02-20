fn fib(n: i32, memo: &mut Vec<Option<i32>>) -> i32 {
    if let Some(val) = memo[n as usize] {
        return val;
    }
    let result = if n < 2 {
        n
    } else {
        fib(n - 1, memo) + fib(n - 2, memo)
    };
    memo[n as usize] = Some(result);
    result
}

fn main() {
    let n = 45;
    let mut memo = vec![None; (n + 1) as usize];
    for i in 0..n {
        println!("fib({}) = {}", i, fib(i, &mut memo));
    }
}
