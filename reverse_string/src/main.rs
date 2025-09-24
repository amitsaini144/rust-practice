
fn reverse_it( s: &str ) -> String {
    s.chars().rev().collect()
}

fn main() {
    let s = String::from("Hello world");

    let solution = reverse_it(&s);
    let test2 = reverse_it("NEWWWW");

    print!("The reversed string ==> {}", solution);
    print!("test2 {}", test2);
}
