fn check_palindrome( s: &str) -> bool {
    let mut iter = s.chars();

    while let (Some(front), Some(back)) = (iter.next(), iter.next_back()){
        if front != back {
            return false
        }
    }
    true
}

fn main() {
    let s = String::from("ABBBA");
    let solution = check_palindrome(&s);
    let test2 = check_palindrome("TEETh");

    print!("{}", solution);
    print!("{}", test2);
}
