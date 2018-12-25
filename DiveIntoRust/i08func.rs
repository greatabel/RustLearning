fn add1(t: (i32, i32)) -> i32 {
    t.0 + t.1
}

fn add2((x, y): (i32, i32)) -> i32 {
    x + y
}

fn main() {
    let p = (1, 3);

    let func = add2;
    println!("evaluation ouput {}", func(p));
}