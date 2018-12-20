fn f1() -> bool {
    println!("Call f1");
    true
}

fn f2() -> bool {
    println!("Call f2");
    false
}

fn main() {
    println!("Bit and: {}\n", f2() & f1());
    println!("Logic and: {}\n", f2() && f1());
    println!("Bit or: {}\n", f1() | f2());
    println!("Logic or: {}\n", f1() || f2());
}