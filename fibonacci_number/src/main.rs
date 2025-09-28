
fn find_fibonacci_number(&n: &u8) -> u64 {
    if n <= 1 {
        return n as u64;
    }

    let (mut first, mut second) = (0, 1);

    for _ in 1..n {
        let ans = first + second;
        first = second;
        second = ans;
    }

    second
}


fn main() {
    let n = 10;
    let res = find_fibonacci_number(&n);
    println!("result {}", res);
}
