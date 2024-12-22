fn main() {
    let mut a = 10u32;
    let b = &mut a;
    *b = 20;
    println!("{b}");
}

fn foo(s: String) -> String {
    s
}
